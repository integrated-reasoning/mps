use std::time::Instant;

fn main() {
    // Test with a file that likely uses flexible parsing
    let files = [
        ("afiro", include_str!("../../tests/data/netlib/afiro")),
        ("pilot", include_str!("../../tests/data/netlib/pilot")),
        ("maros", include_str!("../../tests/data/netlib/maros")),
    ];

    for (name, content) in files {
        let iterations = 100;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = mps::Parser::<f32>::parse(content).unwrap();
        }
        let elapsed = start.elapsed();
        println!("{:12} {:8.3}ms {:10.1} MB/s", 
                 name, 
                 (elapsed / iterations).as_secs_f64() * 1000.0,
                 (content.len() as f64 / 1e6) / (elapsed.as_secs_f64() / iterations as f64));
    }
}
