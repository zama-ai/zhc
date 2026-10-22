use zhc::prelude::*;

static INT_SIZE: u16 = 8;

fn add_ripple_carry() -> Builder {
    // Working with 2 bits of carry / 2 bits of message.
    let bd = Builder::new(SpecBlock(2, 2));

    // Declare inputs and split them in blocks.
    let lhs = bd.ciphertext_integer_input(INT_SIZE);
    let rhs = bd.ciphertext_integer_input(INT_SIZE);
    let lhs_blocks = bd.ciphertext_integer_split(lhs);
    let rhs_blocks = bd.ciphertext_integer_split(rhs);

    // Ripple carry adder.
    let mut carry = bd.block_let_ciphertext(0);
    let mut output_blocks = Vec::new();
    for i in 0..lhs_blocks.len() {
        let raw_sum = bd.block_add(lhs_blocks[i], rhs_blocks[i]);
        let sum = bd.block_add(raw_sum, carry);
        let message = bd.block_lookup(sum, Lut1::msg_only(*bd.spec()));
        carry = bd.block_lookup(sum, Lut1::carry_in_msg(*bd.spec()));
        output_blocks.push(message);
    }

    // Declare output.
    let output = bd.ciphertext_integer_join(output_blocks, Some(INT_SIZE));
    bd.ciphertext_integer_output(output);

    bd
}

fn main() {
    let bd = add_ripple_carry();

    // Noise analysis of the circuit.
    bd.dump_noise();

    // Interactive drawing of the block-level IR.
    bd.draw().open().unwrap();

    // Compilation for a single HPU, with the predicted latency.
    let mut pl = Pipeline::new()
        .with_builder(bd)
        .with_hpu_config(Default::default());
    pl.get_hpu_metrics().dump();
}
