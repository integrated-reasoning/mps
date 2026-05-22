use std::time::Instant;

fn main() {
    let content = include_str!("../../tests/data/netlib/pilot");
    let iterations = 500;

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
    println!("Baseline: {:?}", baseline);
    println!("Per parse: {:?}", baseline / iterations);
    println!("MB/s: {:.1}", (content.len() as f64 / 1e6) / (baseline.as_secs_f64() / iterations as f64));
}
