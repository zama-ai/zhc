use std::io::Write;
use std::process::Command;
use std::time::Instant;

use zhc::compat::Iop;
use zhc_builder::CiphertextSpec;
use zhc_config::hpu::{HpuConfig, PhysicalConfig};
use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::LookupCheck};
use zhc_ir::{IR, ValId};
use zhc_langs::hpulang::{HpuInstructionSet, HpuLang, TDstId, TImmId, TSrcId};
use zhc_pipeline::{
    SchedPolicy,
    passes::{hpu_schedule, lower_iop_to_hpu},
};
use zhc_utils::FastMap;

type Lit = u32;
const FALSE: Lit = 0;
const TRUE: Lit = 1;

struct Aig {
    n_inputs: u32,
    n_vars: u32,
    ands: Vec<(Lit, Lit, Lit)>,
    hc: FastMap<(Lit, Lit), Lit>,
}

impl Aig {
    fn new() -> Self {
        Self {
            n_inputs: 0,
            n_vars: 0,
            ands: Vec::new(),
            hc: FastMap::default(),
        }
    }

    fn input(&mut self) -> Lit {
        assert!(self.ands.is_empty());
        self.n_inputs += 1;
        self.n_vars += 1;
        self.n_vars * 2
    }

    fn not(a: Lit) -> Lit {
        a ^ 1
    }

    fn and(&mut self, a: Lit, b: Lit) -> Lit {
        if a == FALSE || b == FALSE || a == Self::not(b) {
            return FALSE;
        }
        if a == TRUE || a == b {
            return b;
        }
        if b == TRUE {
            return a;
        }
        let key = if a < b { (a, b) } else { (b, a) };
        if let Some(l) = self.hc.get(&key) {
            return *l;
        }
        self.n_vars += 1;
        let lhs = self.n_vars * 2;
        self.ands.push((lhs, key.1, key.0));
        self.hc.insert(key, lhs);
        lhs
    }

    fn or(&mut self, a: Lit, b: Lit) -> Lit {
        Self::not(self.and(Self::not(a), Self::not(b)))
    }

    fn xor(&mut self, a: Lit, b: Lit) -> Lit {
        let t = self.and(a, Self::not(b));
        let u = self.and(Self::not(a), b);
        self.or(t, u)
    }

    fn mux(&mut self, sel: Lit, on_true: Lit, on_false: Lit) -> Lit {
        if on_true == on_false {
            return on_true;
        }
        let t = self.and(sel, on_true);
        let f = self.and(Self::not(sel), on_false);
        self.or(t, f)
    }

    fn write_aig(&self, outputs: &[Lit], path: &str) {
        let mut f = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
        writeln!(
            f,
            "aig {} {} 0 {} {}",
            self.n_vars,
            self.n_inputs,
            outputs.len(),
            self.ands.len()
        )
        .unwrap();
        for o in outputs {
            writeln!(f, "{o}").unwrap();
        }
        let mut put = |mut x: u32| {
            loop {
                let byte = (x & 0x7f) as u8;
                x >>= 7;
                if x == 0 {
                    f.write_all(&[byte]).unwrap();
                    break;
                }
                f.write_all(&[byte | 0x80]).unwrap();
            }
        };
        for (lhs, a, b) in &self.ands {
            assert!(lhs > a && a >= b);
            put(lhs - a);
            put(a - b);
        }
    }

    fn to_circuit(&self, outputs: &[Lit]) -> zhc_abc::Circuit {
        let mut c = zhc_abc::Circuit::with_capacity(self.n_vars as usize + 2);
        let mut map: Vec<zhc_abc::Lit> = vec![zhc_abc::Lit::FALSE; self.n_vars as usize + 1];
        for i in 1..=self.n_inputs {
            map[i as usize] = c.input();
        }
        let tr = |c: &zhc_abc::Circuit, map: &Vec<zhc_abc::Lit>, l: Lit| {
            let base = map[(l / 2) as usize];
            if l & 1 == 1 { c.not(base) } else { base }
        };
        for (lhs, a, b) in &self.ands {
            let x = tr(&c, &map, *a);
            let y = tr(&c, &map, *b);
            map[(lhs / 2) as usize] = c.and(x, y);
        }
        for o in outputs {
            let l = tr(&c, &map, *o);
            c.output(l);
        }
        c
    }

    fn eval(&self, lits: &[Lit], inputs: &[bool]) -> Vec<bool> {
        let mut vals = vec![false; self.n_vars as usize + 1];
        for (i, v) in inputs.iter().enumerate() {
            vals[i + 1] = *v;
        }
        let get = |vals: &Vec<bool>, l: Lit| vals[(l / 2) as usize] ^ (l & 1 == 1);
        for (lhs, a, b) in &self.ands {
            let v = get(&vals, *a) && get(&vals, *b);
            vals[(lhs / 2) as usize] = v;
        }
        lits.iter().map(|l| get(&vals, *l)).collect()
    }
}

type Bundle = Vec<Lit>;

struct Blocks {
    width: usize,
    alt: bool,
}

impl Blocks {
    fn constant(&self, v: u16) -> Bundle {
        (0..self.width)
            .map(|i| if (v >> i) & 1 == 1 { TRUE } else { FALSE })
            .collect()
    }

    fn add_with_carry(&self, g: &mut Aig, a: &Bundle, b: &Bundle, mut c: Lit) -> Bundle {
        let mut out = Vec::with_capacity(self.width);
        for i in 0..self.width {
            if self.alt {
                let axb = g.xor(a[i], b[i]);
                out.push(g.mux(c, Aig::not(axb), axb));
                let ab = g.and(a[i], b[i]);
                let ac = g.and(a[i], c);
                let bc = g.and(b[i], c);
                let t = g.or(ab, ac);
                c = g.or(t, bc);
            } else {
                let axb = g.xor(a[i], b[i]);
                out.push(g.xor(axb, c));
                let t = g.and(a[i], b[i]);
                let u = g.and(c, axb);
                c = g.or(t, u);
            }
        }
        out
    }

    fn add(&self, g: &mut Aig, a: &Bundle, b: &Bundle) -> Bundle {
        self.add_with_carry(g, a, b, FALSE)
    }

    fn sub(&self, g: &mut Aig, a: &Bundle, b: &Bundle) -> Bundle {
        let nb: Bundle = b.iter().map(|l| Aig::not(*l)).collect();
        if self.alt {
            let one = self.constant(1);
            let neg = self.add(g, &nb, &one);
            self.add(g, a, &neg)
        } else {
            self.add_with_carry(g, a, &nb, TRUE)
        }
    }

    fn shl(&self, a: &Bundle, k: usize) -> Bundle {
        (0..self.width)
            .map(|i| if i >= k { a[i - k] } else { FALSE })
            .collect()
    }

    fn mul_const(&self, g: &mut Aig, a: &Bundle, c: u16) -> Bundle {
        let mut acc = self.constant(0);
        for i in 0..self.width {
            if (c >> i) & 1 == 1 {
                let s = self.shl(a, i);
                acc = self.add(g, &acc, &s);
            }
        }
        acc
    }

    fn mul(&self, g: &mut Aig, a: &Bundle, b: &Bundle) -> Bundle {
        let mut acc = self.constant(0);
        for i in 0..self.width {
            let s = self.shl(a, i);
            let gated: Bundle = s.iter().map(|l| g.and(*l, b[i])).collect();
            acc = self.add(g, &acc, &gated);
        }
        acc
    }

    fn table(&self, g: &mut Aig, a: &Bundle, entries: &[u16]) -> Bundle {
        assert_eq!(entries.len(), 1 << self.width);
        if self.alt {
            return (0..self.width)
                .map(|bit| {
                    let mut acc = FALSE;
                    for (v, e) in entries.iter().enumerate() {
                        if (e >> bit) & 1 == 1 {
                            let mut term = TRUE;
                            for i in 0..self.width {
                                let lit = if (v >> i) & 1 == 1 { a[i] } else { Aig::not(a[i]) };
                                term = g.and(term, lit);
                            }
                            acc = g.or(acc, term);
                        }
                    }
                    acc
                })
                .collect();
        }
        (0..self.width)
            .map(|bit| {
                let leaves: Vec<Lit> = entries
                    .iter()
                    .map(|e| if (e >> bit) & 1 == 1 { TRUE } else { FALSE })
                    .collect();
                self.mux_tree(g, a, &leaves)
            })
            .collect()
    }

    fn mux_tree(&self, g: &mut Aig, sel: &[Lit], leaves: &[Lit]) -> Lit {
        if leaves.len() == 1 {
            return leaves[0];
        }
        let half = leaves.len() / 2;
        let top = sel[sel.len() - 1];
        let rest = &sel[..sel.len() - 1];
        let lo = self.mux_tree(g, rest, &leaves[..half]);
        let hi = self.mux_tree(g, rest, &leaves[half..]);
        g.mux(top, hi, lo)
    }
}

fn enumerate_table(
    spec: CiphertextBlockSpec,
    f: impl Fn(zhc_crypto::integer_semantics::EmulatedCiphertextBlock) -> u16,
) -> Vec<u16> {
    let n = 1usize << spec.complete_size();
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let out = (0..n)
        .map(|v| {
            let block = spec.from_complete(v as u16);
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(block))).unwrap_or(0)
        })
        .collect();
    std::panic::set_hook(prev);
    out
}

struct Io {
    srcs: Vec<TSrcId>,
    imms: Vec<TImmId>,
    dsts: Vec<TDstId>,
}

fn collect_io(irs: &[&IR<HpuLang>]) -> Io {
    let mut srcs = Vec::new();
    let mut imms = Vec::new();
    let mut dsts = Vec::new();
    for ir in irs {
        for op in ir.walk_ops_linear() {
            match op.get_instruction() {
                HpuInstructionSet::SrcLd { from } => srcs.push(from.clone()),
                HpuInstructionSet::ImmLd { from } => imms.push(from.clone()),
                HpuInstructionSet::DstSt { to } => dsts.push(to.clone()),
                _ => {}
            }
        }
    }
    srcs.sort_by_key(|s| (s.src_pos, s.block_pos));
    srcs.dedup_by_key(|s| (s.src_pos, s.block_pos));
    imms.sort_by_key(|s| (s.imm_pos, s.block_pos));
    imms.dedup_by_key(|s| (s.imm_pos, s.block_pos));
    dsts.sort_by_key(|s| (s.dst_pos, s.block_pos));
    dsts.dedup_by_key(|s| (s.dst_pos, s.block_pos));
    Io { srcs, imms, dsts }
}

struct Export {
    aig: Aig,
    outputs: Vec<Lit>,
    n_ops: usize,
}

struct Exporter {
    g: Aig,
    blocks: Blocks,
    spec: CiphertextBlockSpec,
    sources: FastMap<TSrcId, Bundle>,
    immediates: FastMap<TImmId, Bundle>,
    destinations: FastMap<TDstId, Bundle>,
    mutate: bool,
    mutated: bool,
    n_ops: usize,
}

impl Exporter {
    fn walk(&mut self, ir: &IR<HpuLang>, batch_args: &FastMap<u8, Bundle>) -> FastMap<u8, Bundle> {
        let mut vals: FastMap<ValId, Bundle> = FastMap::default();
        let mut batch_rets: FastMap<u8, Bundle> = FastMap::default();
        for op in ir.walk_ops_topological() {
            self.n_ops += 1;
            let args: Vec<Bundle> = op
                .get_arg_valids()
                .iter()
                .map(|v| vals.get(v).expect("argument not yet defined").clone())
                .collect();
            let results = self.eval(op.get_instruction(), &args, batch_args, &mut batch_rets);
            for (v, r) in op.get_return_valids().iter().zip(results) {
                vals.insert(*v, r);
            }
        }
        batch_rets
    }

    fn eval(
        &mut self,
        instr: &HpuInstructionSet,
        args: &[Bundle],
        batch_args: &FastMap<u8, Bundle>,
        batch_rets: &mut FastMap<u8, Bundle>,
    ) -> Vec<Bundle> {
        use HpuInstructionSet::*;
        let spec = self.spec;
        let blocks = &self.blocks;
        let g = &mut self.g;
        match instr {
            SrcLd { from } => vec![self.sources[from].clone()],
            ImmLd { from } => vec![self.immediates[from].clone()],
            DstSt { to } => {
                assert!(self.destinations.insert(to.clone(), args[0].clone()).is_none());
                vec![]
            }
            CstCt { cst } => vec![blocks.constant(cst.0 as u16)],
            AddCt => vec![blocks.add(g, &args[0], &args[1])],
            SubCt => vec![blocks.sub(g, &args[0], &args[1])],
            Mac { cst } => {
                let s = blocks.shl(&args[0], cst.0.ilog2() as usize);
                vec![blocks.add(g, &s, &args[1])]
            }
            AddPt => vec![blocks.add(g, &args[0], &args[1])],
            SubPt => vec![blocks.sub(g, &args[0], &args[1])],
            PtSub => vec![blocks.sub(g, &args[0], &args[1])],
            MulPt => vec![blocks.mul(g, &args[0], &args[1])],
            AddCst { cst } => {
                let c = blocks.constant(cst.0 as u16);
                vec![blocks.add(g, &args[0], &c)]
            }
            SubCst { cst } => {
                let c = blocks.constant(cst.0 as u16);
                vec![blocks.sub(g, &args[0], &c)]
            }
            CstSub { cst } => {
                let c = blocks.constant(cst.0 as u16);
                vec![blocks.sub(g, &c, &args[0])]
            }
            MulCst { cst } => vec![blocks.mul_const(g, &args[0], cst.0 as u16)],
            Pbs { lut } | PbsF { lut } => {
                let mut t = enumerate_table(spec, |b| {
                    lut.lookup(b, LookupCheck::AllowBothPadding).raw_complete_bits()
                });
                if self.mutate && !self.mutated {
                    self.mutated = true;
                    t[2] ^= 1;
                }
                vec![blocks.table(g, &args[0], &t)]
            }
            Pbs2 { lut } | Pbs2F { lut } => (0..2)
                .map(|k| {
                    let t = enumerate_table(spec, |b| {
                        let r = lut.lookup(b, LookupCheck::AllowOutputPadding);
                        [r.0, r.1][k].raw_complete_bits()
                    });
                    blocks.table(g, &args[0], &t)
                })
                .collect(),
            Pbs4 { lut } | Pbs4F { lut } => (0..4)
                .map(|k| {
                    let t = enumerate_table(spec, |b| {
                        let r = lut.lookup(b, LookupCheck::AllowOutputPadding);
                        [r.0, r.1, r.2, r.3][k].raw_complete_bits()
                    });
                    blocks.table(g, &args[0], &t)
                })
                .collect(),
            Pbs8 { lut } | Pbs8F { lut } => (0..8)
                .map(|k| {
                    let t = enumerate_table(spec, |b| {
                        let r = lut.lookup(b, LookupCheck::AllowOutputPadding);
                        [r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7][k].raw_complete_bits()
                    });
                    blocks.table(g, &args[0], &t)
                })
                .collect(),
            BatchArg { pos, .. } => vec![batch_args[pos].clone()],
            BatchRet { pos, .. } => {
                assert!(batch_rets.insert(*pos, args[0].clone()).is_none());
                vec![]
            }
            Batch { block } => {
                let mut arg_positions: Vec<u8> = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_instruction() {
                        BatchArg { pos, .. } => Some(*pos),
                        _ => None,
                    })
                    .collect();
                arg_positions.sort_unstable();
                let inner_args: FastMap<u8, Bundle> = arg_positions
                    .iter()
                    .zip(args.iter())
                    .map(|(p, a)| (*p, a.clone()))
                    .collect();
                let mut rets = self.walk(block, &inner_args);
                let mut ret_positions: Vec<u8> = rets.keys().copied().collect();
                ret_positions.sort_unstable();
                ret_positions
                    .iter()
                    .map(|p| rets.remove(p).unwrap())
                    .collect()
            }
            other => panic!("unsupported instruction in spike: {other:?}"),
        }
    }
}

fn export(
    ir: &IR<HpuLang>,
    spec: CiphertextBlockSpec,
    io: &Io,
    mutate: bool,
    alt: bool,
) -> Export {
    let width = spec.complete_size() as usize;
    let msg = spec.message_size() as usize;
    let blocks = Blocks { width, alt };
    let mut g = Aig::new();

    let mut sources: FastMap<TSrcId, Bundle> = FastMap::default();
    for s in &io.srcs {
        let mut b = blocks.constant(0);
        for bit in b.iter_mut().take(msg) {
            *bit = g.input();
        }
        sources.insert(s.clone(), b);
    }
    let mut immediates: FastMap<TImmId, Bundle> = FastMap::default();
    for s in &io.imms {
        let mut b = blocks.constant(0);
        for bit in b.iter_mut().take(msg) {
            *bit = g.input();
        }
        immediates.insert(s.clone(), b);
    }

    let mut ex = Exporter {
        g,
        blocks,
        spec,
        sources,
        immediates,
        destinations: FastMap::default(),
        mutate,
        mutated: false,
        n_ops: 0,
    };
    let rets = ex.walk(ir, &FastMap::default());
    assert!(rets.is_empty());

    let outputs: Vec<Lit> = io
        .dsts
        .iter()
        .flat_map(|d| {
            let b = ex
                .destinations
                .get(d)
                .unwrap_or_else(|| panic!("missing destination {d}"));
            b[..msg].to_vec()
        })
        .collect();
    Export {
        aig: ex.g,
        outputs,
        n_ops: ex.n_ops,
    }
}

fn selfcheck(spec: CiphertextBlockSpec, alt: bool) {
    let width = spec.complete_size() as usize;
    let blocks = Blocks { width, alt };
    let n = 1u16 << width;
    let mut g = Aig::new();
    let a: Bundle = (0..width).map(|_| g.input()).collect();
    let b: Bundle = (0..width).map(|_| g.input()).collect();
    let add = blocks.add(&mut g, &a, &b);
    let sub = blocks.sub(&mut g, &a, &b);
    let mul = blocks.mul(&mut g, &a, &b);
    let mul3 = blocks.mul_const(&mut g, &a, 3);
    let to_bits = |v: u16| (0..width).map(|i| (v >> i) & 1 == 1).collect::<Vec<_>>();
    let from_bits = |bits: &[bool]| bits.iter().enumerate().fold(0u16, |acc, (i, b)| acc | ((*b as u16) << i));
    for x in 0..n {
        for y in 0..n {
            let mut inputs = to_bits(x);
            inputs.extend(to_bits(y));
            let bx = spec.from_complete(x);
            let by = spec.from_complete(y);
            let got = |l: &Bundle| from_bits(&g.eval(l, &inputs));
            assert_eq!(got(&add), bx.wrapping_add(by).raw_complete_bits());
            assert_eq!(got(&sub), bx.wrapping_sub(by).raw_complete_bits());
            assert_eq!(got(&mul), (x.wrapping_mul(y)) & spec.complete_mask());
            assert_eq!(got(&mul3), (x.wrapping_mul(3)) & spec.complete_mask());
        }
    }
    println!("selfcheck ok (width {width})");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iop = match args.get(1).map(|s| s.as_str()).unwrap_or("div") {
        "add" => Iop::Add,
        "mul" => Iop::Mul,
        "div" => Iop::Div,
        "cmp_gt" => Iop::CmpGt,
        other => panic!("unknown iop {other}"),
    };
    let size: u16 = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(128);
    let abc = std::env::var("ABC").unwrap_or_else(|_| "/tmp/abc-src/abc".to_string());
    let dir = "/tmp/abc_spike";
    std::fs::create_dir_all(dir).unwrap();

    let ct_spec = CiphertextSpec::new(size, 2, 2);
    let spec = ct_spec.block_spec();
    selfcheck(spec, false);
    selfcheck(spec, true);

    let t = Instant::now();
    let iop_ir = iop.to_builder(ct_spec).optimize_ir();
    let before = lower_iop_to_hpu(&iop_ir).output;
    let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
    let after = hpu_schedule(&before, &config, SchedPolicy::AsLateAsPossible);
    println!("compile: {:?}", t.elapsed());

    let io = collect_io(&[&before, &after]);
    println!(
        "io: {} src blocks, {} imm blocks, {} dst blocks",
        io.srcs.len(),
        io.imms.len(),
        io.dsts.len()
    );

    let mut files = Vec::new();
    let mut exports = Vec::new();
    for (name, ir, mutate, alt) in [
        ("before", &before, false, false),
        ("after", &after, false, false),
        ("after_mut", &after, true, false),
        ("before_alt", &before, false, true),
    ] {
        let t = Instant::now();
        let e = export(ir, spec, &io, mutate, alt);
        let path = format!("{dir}/{name}.aig");
        e.aig.write_aig(&e.outputs, &path);
        println!(
            "{name}: {} ops, {} inputs, {} outputs, {} ands, export {:?}",
            e.n_ops,
            e.aig.n_inputs,
            e.outputs.len(),
            e.aig.ands.len(),
            t.elapsed()
        );
        files.push(path);
        exports.push(e);
    }

    let mode = args.get(3).map(|s| s.as_str()).unwrap_or("all");
    let pairs: Vec<(usize, usize, &str)> = match mode {
        "same" => vec![(0, 1, "equivalent")],
        "alt" => vec![(3, 1, "equivalent")],
        "mut" => vec![(0, 2, "NOT equivalent")],
        "lib" | "lib_same" => vec![(0, 1, "equivalent"), (0, 2, "NOT equivalent")],
        _ => vec![
            (0, 1, "equivalent"),
            (3, 1, "equivalent"),
            (0, 2, "NOT equivalent"),
        ],
    };

    if mode.starts_with("lib") {
        for (i, j, expect) in pairs {
            let t = Instant::now();
            let a = exports[i].aig.to_circuit(&exports[i].outputs);
            let b = exports[j].aig.to_circuit(&exports[j].outputs);
            println!(
                "--- lib build {} vs {}: {} and {} ands in {:?}",
                files[i],
                files[j],
                a.n_ands(),
                b.n_ands(),
                t.elapsed()
            );
            let t = Instant::now();
            let verdict =
                zhc_abc::check_equivalence(&a, &b, &zhc_abc::CheckOptions::default()).unwrap();
            let short = match &verdict {
                zhc_abc::Verdict::Equivalent => "Equivalent".to_string(),
                zhc_abc::Verdict::NotEquivalent(cex) => {
                    let va = exports[i].aig.eval(&exports[i].outputs, &cex.inputs);
                    let vb = exports[j].aig.eval(&exports[j].outputs, &cex.inputs);
                    let differing: Vec<usize> = (0..va.len()).filter(|k| va[*k] != vb[*k]).collect();
                    format!(
                        "NotEquivalent at output {} (cex of {} bits); replay differs at outputs {:?}",
                        cex.output,
                        cex.inputs.len(),
                        differing
                    )
                }
                zhc_abc::Verdict::Undecided => "Undecided".to_string(),
            };
            println!("--- lib check (expect {expect}) in {:?}: {short}", t.elapsed());
        }
        return;
    }

    let comparisons: Vec<(&String, &String, &str)> =
        pairs.iter().map(|(i, j, e)| (&files[*i], &files[*j], *e)).collect();
    for (x, y, expect) in comparisons {
        let t = Instant::now();
        let out = Command::new(&abc)
            .arg("-c")
            .arg(format!("cec -v -T 3600 {x} {y}"))
            .output()
            .expect("failed to run abc");
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        println!("--- cec {x} {y} (expect {expect}) in {:?}", t.elapsed());
        println!("{}{}", stdout.trim(), stderr.trim());
    }
}
