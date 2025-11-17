use super::*;
use crate::sim::Simulator;
use crate::sim::Cycle;

mod legacy;


macro_rules! test_hpu_simulation {
    ($($name: ident => $cycles: literal),+) => {
        #[test]
        #[allow(unused)]
        fn test_hpu_simulation() {
            $(
            let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
            let mut sim = Simulator::from_simulatable(config.freq, Hpu::new(config));
            let (stream, lat) = legacy::$name();
            sim.dispatch(Events::IscPushDOps(stream.collect()));
            sim.play_until_event(Events::IscProcessOver);
            assert_eq!(sim.now(), Cycle($cycles));
            println!("{}::> Expected: {:?}, Performed: {:?}, Ratio: {}", stringify!($name), lat, sim.now(), sim.now().0 as f64 / lat.0 as f64);
            )+
        }
    }
}

test_hpu_simulation!(
    ADDS => 79838,
    SUBS => 88070,
    SSUB => 88094,
    MULS => 153146,
    DIVS => 5418230,
    MODS => 5296106,
    OVF_ADDS => 72158,
    OVF_SUBS => 80378,
    OVF_SSUB => 80402,
    OVF_MULS => 624614,
    SHIFTS_R => 14510,
    SHIFTS_L => 14510,
    ROTS_R => 14510,
    ROTS_L => 14510,
    ADD => 64430,
    SUB => 72158,
    MUL => 137510,
    DIV => 5047250,
    MOD => 4924118,
    OVF_ADD => 56750,
    OVF_SUB => 60374,
    OVF_MUL => 609254,
    SHIFT_R => 351530,
    SHIFT_L => 347210,
    ROT_R => 368150,
    ROT_L => 368078,
    BW_AND => 23090,
    BW_OR => 23090,
    BW_XOR => 23090,
    CMP_GT => 54902,
    CMP_GTE => 54902,
    CMP_LT => 54902,
    CMP_LTE => 54902,
    CMP_EQ => 54902,
    CMP_NEQ => 54902,
    IF_THEN_ZERO => 23054,
    IF_THEN_ELSE => 38186,
    ERC_20 => 160622,
    MEMCPY => 4298,
    ILOG2 => 271046,
    COUNT0 => 174422,
    COUNT1 => 174422,
    LEAD0 => 404522,
    LEAD1 => 416450,
    TRAIL0 => 400046,
    TRAIL1 => 402242,
    ADD_SIMD => 194030,
    ERC_20_SIMD => 920594
);
