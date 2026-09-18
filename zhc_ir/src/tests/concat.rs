//! Joining several IRs into one with `IR::concat`.

use crate::tests::testlang::{TestInstructionSet, TestLang};
use crate::{IR, ValUse};
use zhc_utils::{assert_display_is, iter::CollectInVec, svec};

/// Checks that every link stored in the IR points both ways.
fn assert_links_consistent(ir: &IR<TestLang>) {
    for op in ir.raw_walk_ops_linear() {
        for (pos, arg) in op.args.iter().enumerate() {
            let use_site = ValUse {
                opid: op.get_id(),
                position: pos as u16,
            };
            assert!(ir.raw_get_val(arg).users.contains(&use_site));
        }
        for (pos, ret) in op.returns.iter().enumerate() {
            let origin = ir.raw_get_val(ret).origin;
            assert_eq!(origin.opid, op.get_id());
            assert_eq!(origin.position as usize, pos);
        }
    }
    for val in ir.raw_walk_vals_linear() {
        let origin = ir.raw_get_op(val.origin.opid);
        assert_eq!(origin.returns[val.origin.position as usize], val.get_id());
        for user in val.users.iter() {
            let user_op = ir.raw_get_op(user.opid);
            assert_eq!(user_op.args[user.position as usize], val.get_id());
        }
    }
}

/// Tests that concat shifts ids, keeps inactive elements, and keeps links intact
#[test]
fn test_concat() {
    let mut a: IR<TestLang> = IR::empty();
    let (_, a0) = a.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
    let (a_inc_id, a1) = a.add_op(TestInstructionSet::Inc, svec![a0[0]]);
    let (a_dead_id, a2) = a.add_op(TestInstructionSet::Inc, svec![a1[0]]);
    a.delete_op(a_dead_id);
    assert_display_is!(
        a.format().show_erased_ops(true),
        "
            %0 = int_input<pos: 0>();
            %1 = inc(%0);
            \x1b[9m%2 = inc(%1);\x1b[29m
        "
    );

    let mut b: IR<TestLang> = IR::empty();
    let (_, b0) = b.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
    let (_, b1) = b.add_op(TestInstructionSet::IntInput { pos: 2 }, svec![]);
    let (b_div_id, b2) = b.add_op(TestInstructionSet::DivRem, svec![b0[0], b1[0]]);
    let (b_ret_id, _) = b.add_op_with_comment(
        TestInstructionSet::Return,
        svec![b2[1]],
        "the end".to_string(),
    );
    assert_display_is!(
        b.format().show_erased_ops(true),
        r#"
                         | %0 = int_input<pos: 1>();
                         | %1 = int_input<pos: 2>();
                         | %2, %3 = div_rem(%0, %1);
            // the end   | return(%3);
        "#
    );

    // Edge cases.
    assert_eq!(IR::<TestLang>::concat(&[]), IR::empty());
    assert_eq!(IR::concat(&[a.clone()]), a);
    assert_eq!(IR::concat(&[b.clone()]), b);

    let c = IR::concat(&[a.clone(), b.clone()]);
    assert_links_consistent(&c);
    assert_display_is!(
        c.format().show_erased_ops(true),
        "
                         | %0 = int_input<pos: 0>();
                         | %1 = inc(%0);
            \x1b[9m             | %2 = inc(%1);\x1b[29m
                         | %3 = int_input<pos: 1>();
                         | %4 = int_input<pos: 2>();
                         | %5, %6 = div_rem(%3, %4);
            // the end   | return(%6);
        "
    );

    // Counts only count active elements; raw counts keep the inactive ones.
    assert_eq!(c.n_ops(), a.n_ops() + b.n_ops());
    assert_eq!(c.n_vals(), a.n_vals() + b.n_vals());
    assert_eq!(c.raw_n_ops(), a.raw_n_ops() + b.raw_n_ops());
    assert_eq!(c.raw_n_vals(), a.raw_n_vals() + b.raw_n_vals());

    // Ops of `a` keep their ids.
    let a_inc = c.get_op(a_inc_id);
    assert_eq!(a_inc.get_depth(), 2);
    assert_eq!(a_inc.get_args_iter().covec(), [c.get_val(a0[0])]);
    assert_eq!(a_inc.get_returns_iter().covec(), [c.get_val(a1[0])]);
    assert!(c.raw_get_op(a_dead_id).is_inactive());
    assert!(c.raw_get_val(a2[0]).is_inactive());
    assert_eq!(c.raw_get_op(a_dead_id).args, [a1[0]]);

    // Ops and vals of `b` are shifted by the raw counts of `a`.
    let op_shift = a.raw_n_ops();
    let val_shift = a.raw_n_vals();
    let b_div = c.get_op(b_div_id + op_shift);
    assert_eq!(b_div.get_depth(), 2);
    assert_eq!(
        b_div.get_args_iter().covec(),
        [c.get_val(b0[0] + val_shift), c.get_val(b1[0] + val_shift)]
    );
    assert_eq!(
        b_div.get_returns_iter().covec(),
        [c.get_val(b2[0] + val_shift), c.get_val(b2[1] + val_shift)]
    );
    let b_ret = c.get_op(b_ret_id + op_shift);
    assert_eq!(b_ret.get_depth(), 3);
    assert_eq!(b_ret.get_comment(), Some("the end"));
    assert_eq!(
        c.get_val(b2[1] + val_shift).get_users_iter().covec(),
        [b_ret.clone()]
    );
    assert_eq!(c.get_val(b2[0] + val_shift).get_users_iter().count(), 0);

    // Order of inputs matters.
    let d = IR::concat(&[b.clone(), a.clone()]);
    assert_links_consistent(&d);
    assert_display_is!(
        d.format().show_comments(false),
        r#"
            %0 = int_input<pos: 1>();
            %1 = int_input<pos: 2>();
            %2, %3 = div_rem(%0, %1);
            return(%3);
            %4 = int_input<pos: 0>();
            %5 = inc(%4);
        "#
    );
    assert_ne!(c, d);

    // Concat is associative.
    let abb = IR::concat(&[a.clone(), b.clone(), b.clone()]);
    assert_eq!(abb, IR::concat(&[c.clone(), b.clone()]));
    assert_eq!(
        abb,
        IR::concat(&[a.clone(), IR::concat(&[b.clone(), b.clone()])])
    );
    assert_links_consistent(&abb);
}
