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
            let mut sim = Simulator::from_simulatable(config.freq, Hpu::new(&config));
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
            // let filename = format!("/tmp/hpu_compiler/tests/hpu_{}.json", stringify!($name));
            // let path = std::path::Path::new(&filename);
            // if let Some(parent) = path.parent() {
            //     std::fs::create_dir_all(parent).expect("Issue while creating output folder");
            // }
            // sim.dump_trace(&format!("/tmp/hpu_compiler/tests/hpu_{}.json", stringify!($name)));
            )+
        }
    }
}

test_hpu_simulation!(
    ADDS => 79876,
    SUBS => 88112,
    SSUB => 88124,
    MULS => 153212,
    DIVS => 5418703,
    MODS => 5296483,
    OVF_ADDS => 72215,
    OVF_SUBS => 80451,
    OVF_SSUB => 80463,
    OVF_MULS => 624711,
    SHIFTS_R => 14507,
    SHIFTS_L => 14507,
    ROTS_R => 14507,
    ROTS_L => 14507,
    ADD => 64481,
    SUB => 72214,
    MUL => 137594,
    DIV => 5047662,
    MOD => 4924521,
    OVF_ADD => 56819,
    OVF_SUB => 60453,
    OVF_MUL => 609376,
    SHIFT_R => 351556,
    SHIFT_L => 347261,
    ROT_R => 368183,
    ROT_L => 368118,
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
    ILOG2 => 271169,
    COUNT0 => 174537,
    COUNT1 => 174537,
    LEAD0 => 404673,
    LEAD1 => 416554,
    TRAIL0 => 400178,
    TRAIL1 => 402353,
    ADD_SIMD => 194057,
    ERC_20_SIMD => 920710
);
