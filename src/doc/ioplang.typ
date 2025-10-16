#import "@preview/curryst:0.5.1": rule, prooftree

= Ioplang

== Rationale

We can expect `ioplang` code to exist in different locations/forms :

1) As definitions inside `tfhe-rs` :
  Scenario: Replace the current ILP/LLT code

2) As standalone source files :

Scenario: If a user wants to develop a custom IOP to be compiled at configuration time

3) In a serialized form, sent by user for on-the-fly compilation:
  Scenario: If a user wants to compile a big IOP (graph-dataflow case)

This context is strongly influenced by the rust programming language:
+ `tfhe-rs` uses rust, and as such the current main entry point to the HPU is rust
+ All non-driver-and-ucore code in the FPGA team is in rust.
+ The `co-processor`, which will likely become an important user, also uses rust.

In this context, implementing `ioplang` as a rust dialect would have a lot of benefits. By _rust dialect_, I mean a language that has a syntax that can be successfully parsed by a rust parser (I am thinking about syn). On the top of my head:
+ The syntax being already in use in a production environment, we avoid coming with a broken syntax.
+ As time goes, if more syntactical features are needed, we can tap into the rust syntax, with the guarantee that it will play with the rest (syntactically).
+ We get the lexing + parsing with correct span tracking for free (it can be tricky to get right, even more so as features are added).
+ We get free formatting/coloring/delimiters-mathing (which looks futile only if you never spent time debugging code without it lol).
+ We can leverage syn spans to emit diagnostics which are later picked up by rust-analyzer.
+ Users can use rust syntax in their editor to have matching parens, formatting, indent.
+ Code can be emitted with the quasi-quoting crate `quote` (which is way better than a builder api).

== Bird eyes view

This section gives an overview of the essential elements of the syntax.

*Function* syntax is used to define IOps. They have more than one arguments, and zero return values. They contain zero or more generic variables. By convention, iop names are lower case.
Example:
```rust
fn add_cst<I>(dst: &mut[CtBlock], lhs: &[CtBlock], rhs: &[ImBlock]) {
    // ...
}
```

*Generics* syntax is used to express IOps in a way that is generic over the Integer length. IOps can then be specialized using the `static $a = $b::<$c>;` syntax, where `$c` is a comma delimited list of positive integer literals, matching the number of generic variables.
Example:
```rust
fn some_op<I, J>(...) {
    // ...
}

static add_cst_128_64 = add_cst::<128, 64>;
```

*Arguments* are used to declare identifiers for destination (```rust &mut [CtBlock]```), source (```rust &[CtBlock]```), and immediate plaintext (```rust &[ImBlock]```) variables.
Example:
```rust
fn some_op<I>(dst: &mut[CtBlock], lhs: &[CtBlock], rhs: &[ImBlock]) {
    // ...
}
```

The slice/mut-slice rust syntax is used along with two different inner types to identify three types of arguments:
+ ```rust &mut [CtBlock]``` -> A destination integer ciphertext memory location
+ ```rust &[CtBlock]``` -> A source integer ciphertext memory location
+ ```rust &[ImBlock]``` -> A source integer plaintext immediate


*Let-binding* syntax can be used inside functions to introduce new identifiers. A label may reference either a virtual registers (when introduced with `let $a = new_reg();`) or a heap slot when introduced with `let $b = new_heap();`).
Example:
```rust
fn ... {
    let r1 = new_reg();
    let h1 = new_heap();
}
```

*Function call* syntax is used to represent the execution of a DOp instruction. By convention, DOp instructions have upper case names.
Example:
```rust
fn some_op<I>(dst: &mut[CtBlock], lhs: &[CtBlock], rhs: &[ImBlock]) {
    let ra = new_reg();
    LD(ra, lhs[9]);
    ...
}
```

*For loops* syntax is used to compactly represent the iterative execution of a sub-computation. The syntax recognized is `for $v in 0..$a { $b }` where:
+ `$a` is either a positive integer literal or one of the generic variable of the IOp.
+ `$v` is a label pointing to an induction variable that goes from 0 to `$a`
+ `$b` is a block of code in which the `$b` variable may be used freely.
Example:
```rust
fn some_op<I>(dst: &mut[CtBlock], lhs: &[CtBlock], rhs: &[ImBlock]) {
    let ra = new_reg();
    for i in 0..I {
        LD(ra, lhs[9]);
    }
    ...
}
```

*Array indexing* syntax can be used to represent pointer arithmetic operations on IOp argument labels. The syntax recognized is `$v[$a]`, where:
+ `$v` is an argument label,
+ `$a` is either a positive integer literal, an iteration variable, or a generic variable (is this last useful?)
Example.
```rust
fn some_op<I>(dst: &mut[CtBlock], lhs: &[CtBlock], rhs: &[ImBlock]) {
    let ra = new_reg();
    LD(ra, lhs[9]);
    ...
}
```

== Typing

A *Type* is associated with variable and literal terms. We denote $cal(T)$ the set of all well-formed terms of the language. The set of available types is defined as:
#let comment(content) = text(fill: rgb(0, 0, 0, 30%), content)
$
  TT &= {\
    &text("DstCtArr"), &quad comment("Destination ciphertext adresses")\
    &text("DstCtBlock"), &quad comment("Destination ciphertext block adresses")\
    &text("SrcCtArr"), &quad comment("Source ciphertext adresses")\
    &text("SrcCtBlock"), &quad comment("Source ciphertext block adresses")\
    &text("SrcImArr"), &quad comment("Source immediate value")\
    &text("SrcImBlock"), &quad comment("Source immediate block value")\
    &text("Reg"), &comment("Virtual registers")\
    &text("Heap"), &comment("Heap slot")\
    &text("UInt"), &quad comment("Positive integers")\
  }
$

For the sake of simplicity, we define the typing of the program as a relation $Gamma subset cal(T) times TT$. We use the notation $a: A$ to denote the fact that $(a, A) in Gamma$.

Below are some semi-formal typing rules for the language. On top of the lines are the premises, and below are the conclusions. The red text can be thought of as terms templates.
#let t(content) = text(fill: rgb(255, 0, 125), raw(content))

Rules for the argument types:
$
  prooftree(
    rule(
      #t("a") : text("DstCtArr"),
      #t("a: &mut[CtBlock]"),
    ),
  )
  quad
  prooftree(
    rule(
      #t("a") : text("SrcCtArr"),
      #t("a: &[CtBlock]"),
    ),
  )
  quad
  prooftree(
    rule(
      #t("a") : text("SrcImArr"),
      #t("a: &[ImBlock]"),
    ),
  )
$

Rules for integer generics and integer literals:
$
  prooftree(
    rule(
      text(fill: #rgb(255, 0, 125), n) \: text("UInt"),
      text(fill: #rgb(255, 0, 125), n),
      n in NN,
    )
  )
  quad
  prooftree(
    rule(
      #t("i"): text("UInt"),
      #t("<i>"),
    )
  )
  quad
  prooftree(
    rule(
      #t("i"): text("UInt")\, #t("j"): text("UInt")\, dots\, #t("l"): text("UInt"),
      #t("<i, j,") dots #t("l>"),
    )
  )
  quad
  prooftree(
    rule(
      #t("i"): text("UInt"),
      #t("for i in"),
    )
  )
$

Rules for the indexing of arguments labels:
$
  prooftree(
    rule(
      #t("a[i]") : text("SrcCtBlock"),
      #t("a") : text("SrcCtArr"),
      #t("i") : text("Uint"),
    ),
  )
  quad
  prooftree(
    rule(
      #t("a[i]") : text("DstCtBlock"),
      #t("a") : text("DstCtArr"),
      #t("i") : text("Uint"),
    ),
  )
  quad
  prooftree(
    rule(
      #t("a[i]") : text("SrcImBlock"),
      #t("a") : text("SrcImArr"),
      #t("i") : text("Uint"),
    ),
  )
$

Rules for let bindings:
$
  prooftree(
    rule(
      #t("a") : text("Reg"),
      #t("let a = new_reg();"),
    ),
  )
  quad
  prooftree(
    rule(
      #t("a") : text("Heap"),
      #t("let a = new_heap();"),
    ),
  )
$

== Semantics

Assuming:
- $R$ a set of register labels.
- $H$ a set of heap slot labels.
- $M$ a set of memory location labels.
- $frak(C)$ a set of ciphertext values.
- $cal(R): R arrow.hook frak(C)$ is a partial map from the registers names to ciphertext values.
- $cal(H): H arrow.hook frak(C)$ is a partial map from the heap slots names to ciphertext values.
- $cal(M): M arrow.hook frak(C)$ is a partial map from the memory locs to ciphertext values.

The semantic of a program can be defined with respect to an abstract machine whose state is defined by a tuple $sigma = (cal(R), cal(H), cal(M))$.

TODO: Wait for approval of what's before defining the semantics formally.

== Eventual extensions ?

Here are a few extensions I imagine will be needed. Some may need to be included in the original language.

*Integer arithmetic* syntax could be used to represent operations on terms of UInt type.
Example:
```rust
LD(r1, a[i+3]);
```

*Function call* syntax could be used to represent the execution of another existing IOp.
Example:
```rust
fn some_op<I>(dst: &mut[CtBlock], lhs: &[CtBlock], rhs: &[ImBlock]) {
    some_other_op::<I>(dst, lhs);
}
```

*Array* syntax could be used in place of slice syntax, to make argument types dependant over integer generics. This would allow to perform bound checking and would increase the safety of the program.
Example:
```rust
fn some_op<I>(dst: &mut[CtBlock;I], lhs: &[CtBlock;I], rhs: &[ImBlock;I]) {
    ...
}
```

== V2

After discussing with JJ and BT, it appeared that this first design greatly oversimplified the circuits implemented. To come with a better approach, I tried to implement multiple real-life circuits, to derive all the constraints.

=== If-Then-Else

```rust
// Assuming we have a way to define lookup tables.
const LUT_IF_TRUE_ZEROED: Lut<1> = ...;
const LUT_IF_FALSE_ZEROED: Lut<1> = ...;

fn if_then_else<
    const nb_blocks: int,
    const block_width: int,
>(
    dst: &mut Mem<CtBlock, I>,
    src_a: &Mem<CtBlock, I>,
    src_b: &Mem<CtBlock, I>,
    cond: &Mem<CtBlock, 1>
){
    for i in 0..nb_blocks {
        let v1 = MAC(cond, 1 << block_width, src_a[i]);
        let v1 = PBS(v1, LUT_IF_FALSE_ZEROED);
        let v2 = MAC(cond, 1 << block_width, src_b[i]);
        let v2 = PBS(v2, LUT_IF_TRUE_ZEROED);
        dst[i] = add(v1, v2);
    }
}
```

=== Cmp-Gt

```rust
const LUT_NONE: Lut<1> = ...;
const LUT_SIGN: Lut<1> = ...;
const LUT_REDUCE: Lut<1> = ...;

fn cmp<const block_width: uint, const I: uint>(
    dst: &mut Mem<CtBlock, I>,
    src_a: &Mem<CtBlock, I>,
    src_b: &Mem<CtBlock, I>,
) {
    let packed = pack(block_width, src_a, src_b);
    let subed = sub(packed);
    let reduced = reduce(block_width, subed);
    let cmp = PBS(reduced, LUT_SIGN);
    dst[0] = cmp;
}

const fn reduce(block_width: uint, src: [[CtBlock]]) -> CtBlock {
    match src {
        [a | []] => a,
        [a | rst] => {
            let maced = MAC(a, 1 << block_width, reduce(block_width, rst));
            PBS(maced, LUT_REDUCE)
        }
    }
}

const fn sub(src: [[CtBlock]]) -> [[CtBlock]] {
    match src {
        [a, b | rst] => [PBS(b - a, LUT_SIGN) + 1 | sub(rst)],
        [a | rst] => impossible!("Input length not divisible by two..."),
        [] => []
    }
}

const fn pack(block_width: uint, src: [[CtBlock]]) -> [[CtBlock]] {
    match src {
        [a, b | rst] => {
            let maced = MAC(b, 1 << block_width, a);
            let packed = PBS(maced, LUT_NONE);
            [packed | pack(block_width, rst)]
        },
        [a | rst] => [a | pack(block_width, rst)],
        [] => []
    }
}
```

=== Multiplication

```rust
const LUT_MULT_MSG: Lut<1> = ...;
const LUT_MULT_CARRY: Lut<1> = ...;


fn mult<
    const I: int,
>(
    dst: &mut Mem<CtBlock, I>,
    src_a: &Mem<CtBlock, I>,
    src_b: &Mem<CtBlock, I>,
) {

}

const fn get_reduced_col_for_degree(deg: uint, a: [[CtBlock]], b: [[CtBlock]]) -> [[CtBlock]] {
}

/// Gather all operands that must be sumed for a given degree
const fn get_all_terms_of_degree(deg: uint, a: [[CtBlock]], b: [[CtBlock]]) -> [[CtBlock]] {
    get_all_operands_scan(deg, 0, [], a, b)
}

const fn get_all_operands_scan(
    deg: int,
    a_deg: int,
    acc: [[CtBlock]],
    a: [[CtBlock]],
    b: [[CtBlock]]
) -> [[CtBlock]] {
    match a {
        [v | rst..] => {
            let acc = get_all_operands_for_a(deg, a_deg, 0, acc, v, b);
            get_all_operands_scan(deg, a_deg+1,acc, rst, b);
        },
        [] => acc
    }
}

const fn get_all_operands_for_a(
    deg: int,
    a_deg: int,
    b_deg: int,
    acc: [[CtBlock]],
    a: CtBlock,
    b: [[CtBlock]]
) -> [[CtBlock]] {
    match b {
        [vb | rst] => {
            let acc = get_all_operands_for_a_b(deg, a_deg, b_deg, acc, a, vb);
            get_all_operands_for_a(deg, a_deg, b_deg+1, acc, a, rst)
        },
        [] => acc
    }
}

const fn get_all_operands_for_a_b(
    deg: int,
    a_deg: int,
    b_deg: int,
    acc: [[CtBlock]],
    a: CtBlock,
    b: CtBlock
) -> [[CtBlock]] {
    if a_deg + b_deg == deg {
        [elm_wise_prod(a, b).1 | acc]
    } else if a_deg + b_deg == deg - 1 {
        [elm_wise_prod(a, b).0 | acc]
    } else {
        acc
    }
}

/// 575:577
const fn elm_wise_prod(block_width: uint, a: CtBlock, b: CtBlock) -> (CtBlock, CtBlock)  {
    let packed = MAC(a, 1 << block_width, b);
    let carry = PBS(packed, LUT_MULT_CARRY);
    let msg = PBS(packed, LUT_MULT_MSG);
    (carry, msg)
}
```

=== Naive carry prop

```rust
fn add_carry_propagation(
    dst: &mut Mem<CtBlock>,
    src_a: &Mem<CtBlock>,
    src_b: &Mem<CtBlock>
) {
    let output = s1(CtBlock::zero(), src_a.convert(), src_b.convert());
    for i in 0..blk_nb {
        dst[i] = output[i];
    }
}

const fn s1(carry: CtBlock, src_a: [[CtBlock]], src_b: [[CtBlock]]) -> [[CtBlock]] {
    match (src_a, src_b) {
        ([a | rst_a], [b | rst_b]) => {
            let add0 = ALU(a, b);
            let add1 = ALU(add0, carry);
            let (msg, carry) = PBS(add1, LUT_MANY_MESSAGE_CARRY);
            [msg, s1(carry, rst_a, rst_b)]
        },
        ([], []) => []
        _ => error()
    }
}
```

=== Block addition tree

```rust
const fn block_sum(inputs: [[CtBlock]]) -> CtBlock {
    _block_sum(1, inputs)
}

const fn _block_sum(lev: uint, inputs: [[CtBlock]]) -> CtBlock {
    match inputs {
        [a | []] => {
            if lev == 5 {
                PBS(a, LUT_CLEAR)
            } else {
                a
            }
        },
        [a | rst] => {
            let added = _block_sum_two_by_two(inputs);
            if lev == 5 {
                let reset = _block_sum_reset(added);
                _block_sum(1, refreshed)
            } else {
                _block_sum(lev+1, added)
            }
        }
        [] => error!()
    }
}

const fn _block_sum_reset(inputs: [[CtBlock]]) -> [[CtBlock]] {
    match inputs {
        [a | rst] => [PBS(a, LUT_CLEAR) | _block_sum_reset(rst)],
        [] => []
    }
}

const fn _block_sum_two_by_two(inputs: [[CtBlock]]) -> [[CtBlock]] {
    match inputs {
        [a, b | rst] => {
            let r = ADD(a, b);
            [r | _block_sum_two_by_two(rst)]
        },
        [a | rst] => [a | rst],
        [] => []
    }
}
```

=== Hillis-Steel addition

```rust
fn add_hillis_steele<
    const I: int,
>(
    dst: &mut Mem<CtBlock, I>,
    src_a: &Mem<CtBlock, I>,
    src_b: &Mem<CtBlock, I>,
) {
    let raws = raw_add(src_a, src_b);
    let statuses = get_status(raws);
    let (block_prop, block_int_prop) = prop(statuses);
}

// Performs the raw block-wise addition.
const fn raw_add(a: [[CtBlock]], b: [[CtBlock]]) -> [[CtBlock]] {
    match (a, b) {
        ([a | rst_a], [b | rst_b]) =>  {
            let sum = ALU(a, b);
            [sum | s0(rst_a, rst_b)]
        },
        ([], []) => []
    }
}

const fn prop(is_first: bool, first_step: [[CtBlock]]) -> ([[CtBlock]], [[CtBlock]]) {
    match first_step {
        [a, b, c, d | rst] => {
            let sum = add_log([a,b,c,d]);
            let (l, r) = prop(false, rst);
            if is_first {
                ([PBS(sum, LUT_RESOLVE_PROP_FIRST_BLOCK) | l], [sum | r])
            } else {
                let _1 = PBS(sum, LUT_RESOLVE_PROP_BLOCK);
                ([ADD(_1, 1)| l], [sum | r])
            }
        },
        [a | rst] => error!("not of length multiple of 4"),
        [] => []
    }
}

const fn get_status(index: uint, sums: [[CtBlock]]) -> [[CtBlock]] {
    match sum {
        [a | rst] => {
            if index == 0 {
                let (_, p) = PBS(a, LUT_MANY_MSG_CARRY);
                [p | first_step(index+1, rst)]
            } else if index == 1 {
                let p = PBS(a, LUT_EXTRACT_PROPAGATE_FIRST_BLOCK_1);
                [p | first_step(index+1, rst)]
            } else if index == 2 {
                let p = PBS(a, LUT_EXTRACT_PROPAGATE_FIRST_BLOCK_2);
                [p | first_step(index+1, rst)]
            } else if index == 3 {
                let p = PBS(a, LUT_EXTRACT_PROPAGATE_FIRST_BLOCK_3);
                [p | first_step(index+1, rst)]
            } else if index % 4 = 0 {
                let p = PBS(a, LUT_EXTRACT_PROPAGATE_BLOCK_0);
                [p | first_step(index+1, rst)]
            } else if index % 4 = 1 {
                let p = PBS(a, LUT_EXTRACT_PROPAGATE_BLOCK_1);
                [p | first_step(index+1, rst)]
            } else if index % 4 = 2 {
                let p = PBS(a, LUT_EXTRACT_PROPAGATE_BLOCK_2);
                [p | first_step(index+1, rst)]
            } else if index % 4 = 3 {
                let p = PBS(a, LUT_EXTRACT_PROPAGATE_BLOCK_3);
                [p | first_step(index+1, rst)]
            }
        }
    }
}
```


=== Lessons learned

-> Packing and flushing deferred to the compiler (instruction selection / scheduling).
-> Move to a value based semantics working on `CtBlock` values.
-> Can gradually move to dependant types (shouldn't be too hard given how simple are the unification rules)
-> Needs dead-code analysis
-> Needs common-subexpression elimination
-> Needs const-folding
-> The IR must be able to represent functions, lists, integers, booleans.

=== Dynamic vs Static

Static/Compiler/Language approach:
+ Static analysis available
+ Early detection of errors
+ Clear semantics
+ Predictable performances (which matters for driver).
+ Amenable to verification.
+ Parallelism can be extracted upfront from the language.
+ Elements of the language are guaranteed to compose well.
+ Debugging is simpler (evaluation is inlining).
+ Expressivity may be limited.
+ Initial development cost is higher.

Dynamic/Runtime/Hosted approach:
+ Fast implementation
+ Maximal expressivity
+ High risk of API surface explosion, and semantic break
+ Debugging can become difficult, because you only see the execution of the host language
+ Impossible to perform verification
+ Parallelism must be retrieved
