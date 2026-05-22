//! Performance comparison of different MPS parsing approaches

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn benchmark_nom_parser(c: &mut Criterion) {
    let files = [
        ("afiro", include_str!("../tests/data/netlib/afiro")),
        ("agg", include_str!("../tests/data/netlib/agg")),
        ("pilot", include_str!("../tests/data/netlib/pilot")),
    ];

    let mut group = c.benchmark_group("nom_parser");
    for (name, content) in files {
        group.throughput(criterion::Throughput::Bytes(content.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), content, |b, content| {
            b.iter(|| {
                mps::Parser::<f32>::parse(content).unwrap();
            });
        });
    }
    group.finish();
}

fn benchmark_custom_parser(c: &mut Criterion) {
    let files = [
        ("afiro", include_str!("../tests/data/netlib/afiro")),
        ("agg", include_str!("../tests/data/netlib/agg")),
        ("pilot", include_str!("../tests/data/netlib/pilot")),
    ];

    let mut group = c.benchmark_group("custom_parser");
    for (name, content) in files {
        group.throughput(criterion::Throughput::Bytes(content.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), content, |b, content| {
            b.iter(|| {
                custom_parse(content).unwrap();
            });
        });
    }
    group.finish();
}

fn benchmark_simd_parser(c: &mut Criterion) {
    let files = [
        ("afiro", include_str!("../tests/data/netlib/afiro")),
        ("agg", include_str!("../tests/data/netlib/agg")),
        ("pilot", include_str!("../tests/data/netlib/pilot")),
    ];

    let mut group = c.benchmark_group("simd_parser");
    for (name, content) in files {
        group.throughput(criterion::Throughput::Bytes(content.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), content, |b, content| {
            b.iter(|| {
                simd_parse(content).unwrap();
            });
        });
    }
    group.finish();
}

fn benchmark_mmap_parser(c: &mut Criterion) {
    let files = [
        ("afiro", "../tests/data/netlib/afiro"),
        ("agg", "../tests/data/netlib/agg"),
        ("pilot", "../tests/data/netlib/pilot"),
    ];

    let mut group = c.benchmark_group("mmap_parser");
    for (name, path) in files {
        let content = std::fs::read_to_string(path).unwrap();
        group.throughput(criterion::Throughput::Bytes(content.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), &content, |b, content| {
            b.iter(|| {
                mmap_parse(content.as_bytes()).unwrap();
            });
        });
    }
    group.finish();
}

fn benchmark_parallel_parser(c: &mut Criterion) {
    let files = [
        ("afiro", include_str!("../tests/data/netlib/afiro")),
        ("agg", include_str!("../tests/data/netlib/agg")),
        ("pilot", include_str!("../tests/data/netlib/pilot")),
    ];

    let mut group = c.benchmark_group("parallel_parser");
    for (name, content) in files {
        group.throughput(criterion::Throughput::Bytes(content.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), content, |b, content| {
            b.iter(|| {
                parallel_parse(content).unwrap();
            });
        });
    }
    group.finish();
}

// Custom parser implementation (simplified for benchmarking)
fn custom_parse(input: &str) -> Result<usize, &'static str> {
    let bytes = input.as_bytes();
    let mut pos = 0;
    let len = bytes.len();
    
    let mut sections_parsed = 0;
    
    while pos < len {
        while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\n' || bytes[pos] == b'\r' || bytes[pos] == b'*') {
            if bytes[pos] == b'*' {
                while pos < len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }
        
        if pos >= len { break; }
        
        let mut section_end = pos;
        while section_end < len && bytes[section_end] != b' ' && bytes[section_end] != b'\t' && bytes[section_end] != b'\n' && bytes[section_end] != b'\r' {
            if section_end + 4 <= len && &bytes[section_end..section_end+4] == b"ROWS" {
                sections_parsed += 1;
                pos = section_end + 4;
                break;
            } else if section_end + 7 <= len && &bytes[section_end..section_end+7] == b"COLUMNS" {
                sections_parsed += 1;
                pos = section_end + 7;
                break;
            } else if section_end + 3 <= len && &bytes[section_end..section_end+3] == b"RHS" {
                sections_parsed += 1;
                pos = section_end + 3;
                break;
            } else if section_end + 6 <= len && &bytes[section_end..section_end+6] == b"BOUNDS" {
                sections_parsed += 1;
                pos = section_end + 6;
                break;
            } else if section_end + 6 <= len && &bytes[section_end..section_end+6] == b"ENDATA" {
                return Ok(sections_parsed);
            } else {
                section_end += 1;
            }
        }
        
        if section_end == pos {
            pos += 1;
        }
    }
    
    Ok(sections_parsed)
}

// SIMD parser implementation (simplified for benchmarking)
fn simd_parse(input: &str) -> Result<usize, &'static str> {
    let bytes = input.as_bytes();
    let mut pos = 0;
    let len = bytes.len();
    
    let mut sections_parsed = 0;
    
    while pos < len {
        while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\n' || bytes[pos] == b'\r' || bytes[pos] == b'*') {
            if bytes[pos] == b'*' {
                while pos < len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }
        
        if pos >= len { break; }
        
        let mut section_end = pos;
        while section_end < len && bytes[section_end] != b' ' && bytes[section_end] != b'\t' && bytes[section_end] != b'\n' && bytes[section_end] != b'\r' {
            if section_end + 4 <= len && &bytes[section_end..section_end+4] == b"ROWS" {
                sections_parsed += 1;
                pos = section_end + 4;
                break;
            } else if section_end + 7 <= len && &bytes[section_end..section_end+7] == b"COLUMNS" {
                sections_parsed += 1;
                pos = section_end + 7;
                break;
            } else if section_end + 3 <= len && &bytes[section_end..section_end+3] == b"RHS" {
                sections_parsed += 1;
                pos = section_end + 3;
                break;
            } else if section_end + 6 <= len && &bytes[section_end..section_end+6] == b"BOUNDS" {
                sections_parsed += 1;
                pos = section_end + 6;
                break;
            } else if section_end + 6 <= len && &bytes[section_end..section_end+6] == b"ENDATA" {
                return Ok(sections_parsed);
            } else {
                section_end += 1;
            }
        }
        
        if section_end == pos {
            pos += 1;
        }
    }
    
    Ok(sections_parsed)
}

// mmap parser implementation (simplified for benchmarking)
fn mmap_parse(bytes: &[u8]) -> Result<usize, &'static str> {
    let mut pos = 0;
    let len = bytes.len();
    
    let mut sections_parsed = 0;
    
    while pos < len {
        if pos + 4 <= len && &bytes[pos..pos+4] == b"ROWS" {
            sections_parsed += 1;
            pos += 4;
        } else if pos + 7 <= len && &bytes[pos..pos+7] == b"COLUMNS" {
            sections_parsed += 1;
            pos += 7;
        } else if pos + 3 <= len && &bytes[pos..pos+3] == b"RHS" {
            sections_parsed += 1;
            pos += 3;
        } else if pos + 6 <= len && &bytes[pos..pos+6] == b"BOUNDS" {
            sections_parsed += 1;
            pos += 6;
        } else if pos + 6 <= len && &bytes[pos..pos+6] == b"ENDATA" {
            return Ok(sections_parsed);
        } else {
            pos += 1;
        }
    }
    
    Ok(sections_parsed)
}

// Parallel parser implementation (simplified for benchmarking)
fn parallel_parse(input: &str) -> Result<usize, &'static str> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    
    let num_threads = 4;
    let chunk_size = len / num_threads;
    
    let mut total_sections = 0;
    
    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 { len } else { (i + 1) * chunk_size };
        
        let mut count = 0;
        let mut pos = start;
        while pos < end {
            if pos + 4 <= end && &bytes[pos..pos+4] == b"ROWS" {
                count += 1;
                pos += 4;
            } else if pos + 7 <= end && &bytes[pos..pos+7] == b"COLUMNS" {
                count += 1;
                pos += 7;
            } else if pos + 3 <= end && &bytes[pos..pos+3] == b"RHS" {
                count += 1;
                pos += 3;
            } else if pos + 6 <= end && &bytes[pos..pos+6] == b"BOUNDS" {
                count += 1;
                pos += 6;
            } else if pos + 6 <= end && &bytes[pos..pos+6] == b"ENDATA" {
                break;
            } else {
                pos += 1;
            }
        }
        
        total_sections += count;
    }
    
    Ok(total_sections)
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = 
        benchmark_nom_parser,
        benchmark_custom_parser,
        benchmark_simd_parser,
        benchmark_mmap_parser,
        benchmark_parallel_parser
);
criterion_main!(benches);
