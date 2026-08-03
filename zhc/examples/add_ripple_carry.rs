use zhc::prelude::*;
use zhc_builder::{CiphertextSpec, IrKind};
use zhc_pipeline::compat::Iop;

static INT_SIZE: u16 = 8;

fn add_ripple_carry() -> Builder {
    // Working with 2 bits of carry / 2 bits of message.
    let bd = Builder::new(CiphertextBlockSpec(2,2));

    // Declare inputs and split them in blocks.
    let lhs = bd.ciphertext_input(INT_SIZE);
    let rhs = bd.ciphertext_input(INT_SIZE);
    let lhs_blocks = bd.ciphertext_split(lhs);
    let rhs_blocks = bd.ciphertext_split(rhs);

    // Ripple carry adder.
    let mut carry = bd.block_let_ciphertext(0);
    let mut output_blocks = Vec::new();
    for i in 0..lhs_blocks.iter().len() {
        let raw_sum = bd.block_add(lhs_blocks[i], rhs_blocks[i]);
        let sum = bd.block_add(raw_sum, carry);
        let message = bd.block_lookup(sum, Lut1Def::MsgOnly);
        carry = bd.block_lookup(sum, Lut1Def::CarryInMsg);
        output_blocks.push(message);
    }

    // Declare output.
    let output = bd.ciphertext_join(output_blocks, Some(INT_SIZE));
    bd.ciphertext_output(output);

    bd
}

fn main() {
    // 2 carry bits / 2 message bits
    let bd = Builder::new(CiphertextBlockSpec(2, 2));
    let pti =  bd.plaintext_input(2);
    let pt =   bd.plaintext_split(pti)[0];
    let ct =   bd.block_let_ciphertext(0);
    let trv =  bd.block_add_plaintext(ct, pt);
    let sh =   bd.block_lookup(
                    trv,
                    Lut1Def::custom("sh", |e| e.protect_shr(1))
               );
    // let oup =  bd.ciphertext_join([sh], None);
    //            bd.ciphertext_output(oup);

    // bd.dump(); // Dumps Block-level IR.
    bd.draw(IrKind::Original).open(); // Draw Interactive IR.
    // bd.draw(IrKind::Original).open().unwrap();
    // bd.interpret().with_inputs([pti.make_value(3)]).dump_and_wait();
    // bd.dump(); // Dumps Block-level IR.
    // bd.draw(zhc_builder::IrKind::Original).open().unwrap(); // Draw Interactive IR.
    // bd.interpret()
    //     .with_inputs([pti.make_value(3)])
    //     .dump();

    // let mut pl = Pipeline::new().with_builder(bd).with_hpu_config(Default::default());
    // pl.get_slack_drawing().open();
}
