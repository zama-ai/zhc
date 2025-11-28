use super::*;
use crate::Cycle;
use crate::Simulator;

mod legacy;

const ACCURACY_TOLERANCE: f64 = 0.05;

macro_rules! test_hpu_simulation {
    ($($name: ident => $cycles: literal),+) => {
        #[test]
        #[allow(unused)]
        fn test_hpu_simulation() {
            $(
            let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
            let mut sim = Simulator::from_simulatable(config.freq, Hpu::new(config));
            let (stream, leg_lat) = legacy::$name();
            sim.dispatch(Events::IscPushDOps(stream.collect()));
            sim.play_until_event(Events::IscProcessOver);
            // Check that current model stay in range with previous implementation
            let accuracy_error = sim.now().0.abs_diff(leg_lat.0) as f64 / leg_lat.0 as f64;
            println!("{}::> Expected: {:?}, Performed: {:?}, error: {}", stringify!($name), leg_lat, sim.now(), accuracy_error);
            assert!(accuracy_error <= ACCURACY_TOLERANCE);

            // Check that there are no diff with previous execution
            // If small modification are made to the models those value must be updated
            // println!("{} => {},", stringify!($name), sim.now().0);
            assert_eq!(sim.now(), Cycle($cycles));

            // Uncomment if you want to have trace dump of each operations
            let filename = format!("/tmp/hpu_compiler/tests/hpu_{}.json", stringify!($name));
            let path = std::path::Path::new(&filename);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("Issue while creating output folder");
            }
            sim.dump_trace(&format!("/tmp/hpu_compiler/tests/hpu_{}.json", stringify!($name)));
            )+
        }
    }
}

test_hpu_simulation!(
    ADDS         => 79938,
    SUBS         => 88161,
    SSUB         => 88186,
    MULS         => 153239,
    DIVS         => 5418991,
    MODS         => 5296804,
    OVF_ADDS     => 72250,
    OVF_SUBS     => 80473,
    OVF_SSUB     => 80498,
    OVF_MULS     => 624753,
    SHIFTS_R     => 14521,
    SHIFTS_L     => 14521,
    ROTS_R       => 14521,
    ROTS_L       => 14521,
    ADD          => 64531,
    SUB          => 72264,
    MUL          => 137621,
    DIV          => 5047926,
    MOD          => 4924796,
    OVF_ADD      => 56841,
    OVF_SUB      => 60476,
    OVF_MUL      => 609406,
    SHIFT_R      => 351577,
    SHIFT_L      => 347282,
    ROT_R        => 368205,
    ROT_L        => 368140,
    BW_AND       => 23119,
    BW_OR        => 23119,
    BW_XOR       => 23119,
    CMP_GT       => 54982,
    CMP_GTE      => 54982,
    CMP_LT       => 54982,
    CMP_LTE      => 54982,
    CMP_EQ       => 54982,
    CMP_NEQ      => 54982,
    IF_THEN_ZERO => 23083,
    IF_THEN_ELSE => 38231,
    ERC_20       => 160780,
    MEMCPY       => 4303,
    ILOG2        => 271197,
    COUNT0       => 174566,
    COUNT1       => 174566,
    LEAD0        => 404706,
    LEAD1        => 416639,
    TRAIL0       => 400209,
    TRAIL1       => 402397,
    ADD_SIMD     => 194074,
    ERC_20_SIMD  => 921023
);
