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
    ADDS => 79924,
    SUBS => 88148,
    SSUB => 88172,
    MULS => 153224,
    DIVS => 5418883,
    MODS => 5296699,
    OVF_ADDS => 72239,
    OVF_SUBS => 80463,
    OVF_SSUB => 80487,
    OVF_MULS => 624735,
    SHIFTS_R => 14519,
    SHIFTS_L => 14519,
    ROTS_R => 14519,
    ROTS_L => 14519,
    ADD => 64517,
    SUB => 72250,
    MUL => 137606,
    DIV => 5047830,
    MOD => 4924701,
    OVF_ADD => 56831,
    OVF_SUB => 60465,
    OVF_MUL => 609388,
    SHIFT_R => 351568,
    SHIFT_L => 347273,
    ROT_R => 368195,
    ROT_L => 368130,
    BW_AND => 23114,
    BW_OR => 23114,
    BW_XOR => 23114,
    CMP_GT => 54972,
    CMP_GTE => 54972,
    CMP_LT => 54972,
    CMP_LTE => 54972,
    CMP_EQ => 54972,
    CMP_NEQ => 54972,
    IF_THEN_ZERO => 23078,
    IF_THEN_ELSE => 38225,
    ERC_20 => 160757,
    MEMCPY => 4301,
    ILOG2 => 271181,
    COUNT0 => 174549,
    COUNT1 => 174549,
    LEAD0 => 404685,
    LEAD1 => 416614,
    TRAIL0 => 400190,
    TRAIL1 => 402377,
    ADD_SIMD => 194069,
    ERC_20_SIMD => 920950
);
