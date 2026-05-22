use std::time::Instant;

fn main() {
    let content = include_str!("../../tests/data/netlib/pilot");
    let iterations = 200;

    // Warm up
    for _ in 0..10 {
        let _ = mps::Parser::<f32>::parse(content).unwrap();
    }

    // Baseline
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = mps::Parser::<f32>::parse(content).unwrap();
    }
    let baseline = start.elapsed();
    println!("pilot: {:8.3}ms {:10.1} MB/s", 
             (baseline / iterations).as_secs_f64() * 1000.0,
             (content.len() as f64 / 1e6) / (baseline.as_secs_f64() / iterations as f64));

    // Test with smaller files
    let small = include_str!("../../tests/data/netlib/afiro");
    let start = Instant::now();
    for _ in 0..10000 {
        let _ = mps::Parser::<f32>::parse(small).unwrap();
    }
    let elapsed = start.elapsed();
    println!("afiro: {:8.3}μs {:10.1} MB/s", 
             (elapsed / 10000).as_secs_f64() * 1_000_000.0,
             (small.len() as f64 / 1e6) / (elapsed.as_secs_f64() / 10000.0));
}
