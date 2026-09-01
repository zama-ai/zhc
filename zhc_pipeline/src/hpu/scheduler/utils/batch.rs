use std::rc::Rc;

use zhc_ir::{AsOpRef, IR, OpId, ValRef};
use zhc_langs::hpulang::{HpuInstructionSet, HpuLang};
use zhc_utils::{
    FastMap,
    iter::{CollectInVec, DedupedByKey, MultiZip},
    small::SmallMap,
    svec,
};

fn flush_pbs(instruction: &HpuInstructionSet) -> HpuInstructionSet {
    match instruction {
        HpuInstructionSet::Pbs { lut } | HpuInstructionSet::PbsF { lut } => {
            HpuInstructionSet::PbsF { lut: lut.clone() }
        }
        HpuInstructionSet::Pbs2 { lut } | HpuInstructionSet::Pbs2F { lut } => {
            HpuInstructionSet::Pbs2F { lut: lut.clone() }
        }
        HpuInstructionSet::Pbs4 { lut } | HpuInstructionSet::Pbs4F { lut } => {
            HpuInstructionSet::Pbs4F { lut: lut.clone() }
        }
        HpuInstructionSet::Pbs8 { lut } | HpuInstructionSet::Pbs8F { lut } => {
            HpuInstructionSet::Pbs8F { lut: lut.clone() }
        }
        _ => unreachable!(),
    }
}

#[derive(Clone)]
pub struct Batch<T: AsOpRef<Dialect = HpuLang>> {
    pub ops: Vec<T>,
    pub cap: usize,
}

impl<T: AsOpRef<Dialect = HpuLang>> Batch<T> {
    pub fn new(batch_size: usize) -> Self {
        let output = Batch {
            ops: Vec::with_capacity(batch_size),
            cap: batch_size,
        };
        output
    }

    pub fn is_full(&self) -> bool {
        self.ops.len() == self.cap
    }

    pub fn push<'a>(&mut self, op: T) {
        assert!(op.op_ref().get_instruction().is_pbs());
        if self.is_full() {
            panic!()
        }
        self.ops.push(op);
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    pub fn iter_members(&self) -> impl Iterator<Item = &T> {
        self.ops.iter()
    }

    pub fn gen_batch_ir(
        &self,
    ) -> (
        IR<HpuLang>,
        Vec<ValRef<'_, HpuLang>>,
        Vec<ValRef<'_, HpuLang>>,
    ) {
        let ops = self.ops.iter().map(|op| op.op_ref()).covec();

        // We collect the inputs and outputs of the batch.
        let mut inputs = self
            .ops
            .iter()
            .map(|op| op.op_ref().get_args_iter())
            .flatten()
            .filter(|arg| {
                // To be a batch input, an op arg origin must not be in the batch.
                !ops.as_slice().contains(&arg.get_origin().opref)
            })
            .dedup_by_key(|op| op.get_id())
            .covec();
        inputs.sort_unstable_by_key(|a| a.get_id());
        let mut outputs = ops
            .iter()
            .map(|op| op.get_returns_iter())
            .flatten()
            .filter(|arg| {
                // To be a batch ouptut, a value must be produced by an operation that has users,
                // and which have at least one user outside of the batch.
                arg.get_origin()
                    .opref
                    .get_users_iter()
                    .any(|user| !ops.as_slice().contains(&user))
            })
            .dedup_by_key(|op| op.get_id())
            .covec();
        outputs.sort_unstable_by_key(|a| a.get_id());

        // Now we write the batch IR
        let mut batch = IR::empty();
        let mut batch_map = SmallMap::new();
        for (i, val) in inputs.iter().enumerate() {
            let (_, batch_arg) = batch.add_op(
                HpuInstructionSet::BatchArg {
                    pos: i.try_into().unwrap(),
                    ty: val.get_type(),
                },
                svec![],
            );
            batch_map.insert(val.get_id(), batch_arg[0]);
        }
        for (idx, op) in ops.iter().enumerate() {
            let instr = if idx == self.ops.len() - 1 {
                // Ensures the last is a flush...
                flush_pbs(op.get_instruction())
            } else {
                op.get_instruction().clone()
            };
            let (_, batch_op_rets) = batch.add_op(
                instr,
                op.get_arg_valids()
                    .iter()
                    .map(|k| batch_map.get(k).unwrap())
                    .copied()
                    .collect(),
            );
            for (k, v) in (op.get_return_valids().iter(), batch_op_rets.into_iter()).mzip() {
                batch_map.insert(*k, v);
            }
        }
        for (i, val) in outputs.iter().enumerate() {
            batch.add_op(
                HpuInstructionSet::BatchRet {
                    pos: i.try_into().unwrap(),
                    ty: val.get_type(),
                },
                svec![*batch_map.get(&val.get_id()).unwrap()],
            );
        }

        (batch, inputs, outputs)
    }
}

#[derive(Clone)]
pub struct Batches<T: AsOpRef<Dialect = HpuLang>>(pub Vec<Batch<T>>);

impl<T: AsOpRef<Dialect = HpuLang>> Batches<T> {
    pub fn new() -> Self {
        Batches(Vec::new())
    }

    pub fn push(&mut self, batch: Batch<T>) {
        self.0.push(batch);
    }

    pub fn into_batch_iter(self) -> impl Iterator<Item = Batch<T>> {
        self.0.into_iter()
    }

    pub fn batch_iter(&self) -> impl Iterator<Item = &Batch<T>> {
        self.0.iter()
    }

    pub fn into_batch_map(self) -> FastMap<OpId, Rc<Batch<T>>> {
        self.into_batch_iter()
            .map(Rc::new)
            .flat_map(|batch| {
                (0..batch.len()).map(move |i| (batch.ops[i].op_ref().get_id(), batch.clone()))
            })
            .collect()
    }
}
