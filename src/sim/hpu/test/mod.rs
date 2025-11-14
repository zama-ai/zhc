use super::*;
use crate::sim::Simulator;

mod legacy;


macro_rules! test_hpu_simulation {
    ($($name: ident),+) => {
        #[test]
        #[allow(unused)]
        fn test_hpu_simulation() {
            $(
            let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
            let mut sim = Simulator::from_simulatable(config.freq, Hpu::new(config));
            let (stream, lat) = legacy::$name();
            sim.dispatch(Events::IscPushDOps(stream.collect()));
            sim.play();
            println!("Expected: {:?}, Performed: {:?}, Ratio: {}", lat, sim.now(), sim.now().0 as f64 / lat.0 as f64);
            )+
        }
    }
}

test_hpu_simulation!(
    ADDS,
    SUBS,
    SSUB,
    MULS,
    DIVS,
    MODS,
    OVF_ADDS,
    OVF_SUBS,
    OVF_SSUB,
    OVF_MULS,
    SHIFTS_R,
    SHIFTS_L,
    ROTS_R,
    ROTS_L,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    OVF_ADD,
    OVF_SUB,
    OVF_MUL,
    SHIFT_R,
    SHIFT_L,
    ROT_R,
    ROT_L,
    BW_AND,
    BW_OR,
    BW_XOR,
    CMP_GT,
    CMP_GTE,
    CMP_LT,
    CMP_LTE,
    CMP_EQ,
    CMP_NEQ,
    IF_THEN_ZERO,
    IF_THEN_ELSE,
    ERC_20,
    MEMCPY,
    ILOG2,
    COUNT0,
    COUNT1,
    LEAD0,
    LEAD1,
    TRAIL0,
    TRAIL1,
    ADD_SIMD,
    ERC_20_SIMD
);
