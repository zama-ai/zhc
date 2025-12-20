use std::hint::black_box;

use hpuc_pipeline::*;
use hpuc_sim::hpu::PhysicalConfig;

fn main() {
    use std::time::Instant;

    println!("Starting benchmark harness...");

    let iterations = 1000;
    let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
    let integer = IntegerConfig{
        integer_width: 128,
        message_width: 2,
        carry_width: 2,
        nu_msg: 1,
        nu_bool: 1,
    };

    let start = Instant::now();
    for i in 0..iterations {
        if i % 100 == 0 {
            println!("Completed {} iterations", i);
        }

        black_box(get_translation_table(&config, &integer, Iop::CmpEq));

    }
    let duration = start.elapsed();

    println!("Benchmark completed!");
    println!("Total iterations: {}", iterations);
    println!("Total time: {:?}", duration);
    println!("Average time per iteration: {:?}", duration / iterations);
}
