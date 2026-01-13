#![allow(non_snake_case)]

use super::*;
use crate::Cycle;
use crate::Simulator;

mod legacy;

macro_rules! test_hpu_simulation {
    ($($name: ident => $cycles: literal),+) => {
        $(
        #[test]
        #[allow(unused)]
        fn $name() {
            let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
            let mut sim = Simulator::from_simulatable(config.freq, Hpu::new(&config));
            let (stream, leg_lat) = legacy::$name();
            sim.dispatch(Events::IscPushDOps(stream.collect()));
            sim.play_until_event(Events::IscProcessOver);

            // Check that there are no diff with previous execution
            // If small modification are made to the models those value must be updated
            println!("{} => {},", stringify!($name), sim.now().0);
            assert_eq!(sim.now(), Cycle($cycles));

            // Uncomment if you want to have trace dump of each operations
            // let filename = format!("/tmp/hpu_compiler/tests/hpu_{}.json", stringify!($name));
            // let path = std::path::Path::new(&filename);
            // if let Some(parent) = path.parent() {
            //     std::fs::create_dir_all(parent).expect("Issue while creating output folder");
            // }
        }
        )+
    }
}
test_hpu_simulation!(
    ADDS => 79876,
    SUBS => 88112,
    SSUB => 88124,
    MULS => 153212,
    DIVS => 2796038,
    MODS => 2872224,
    OVF_ADDS => 72215,
    OVF_SUBS => 80451,
    OVF_SSUB => 80463,
    OVF_MULS => 270778,
    SHIFTS_R => 14507,
    SHIFTS_L => 14507,
    ROTS_R => 14507,
    ROTS_L => 14507,
    ADD => 64481,
    SUB => 72214,
    MUL => 137594,
    DIV => 4793256,
    MOD => 4670180,
    OVF_ADD => 56819,
    OVF_SUB => 60453,
    OVF_MUL => 255443,
    SHIFT_R => 351361,
    SHIFT_L => 347066,
    ROT_R => 367988,
    ROT_L => 367923,
    BW_AND => 23102,
    BW_OR => 23102,
    BW_XOR => 23102,
    CMP_GT => 54960,
    CMP_GTE => 54960,
    CMP_LT => 54960,
    CMP_LTE => 54960,
    CMP_EQ => 54960,
    CMP_NEQ => 54960,
    IF_THEN_ZERO => 23066,
    IF_THEN_ELSE => 38213,
    ERC_20 => 160709,
    MEMCPY => 4289,
    ILOG2 => 271039,
    COUNT0 => 129094,
    COUNT1 => 129094,
    LEAD0 => 356220,
    LEAD1 => 368101,
    TRAIL0 => 356220,
    TRAIL1 => 358395,
    ADD_SIMD => 192421,
    ERC_20_SIMD => 877411
);
