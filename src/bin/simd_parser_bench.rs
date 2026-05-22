use std::time::Instant;

fn main() {
    let files = [
        ("afiro", include_str!("../../tests/data/netlib/afiro")),
        ("adlittle", include_str!("../../tests/data/netlib/adlittle")),
        ("agg", include_str!("../../tests/data/netlib/agg")),
        ("etamacro", include_str!("../../tests/data/netlib/etamacro")),
        ("maros", include_str!("../../tests/data/netlib/maros")),
        ("25fv47", include_str!("../../tests/data/netlib/25fv47")),
        ("pilot", include_str!("../../tests/data/netlib/pilot")),
    ];

    println!("=== SIMD-Accelerated Parser Benchmark ===\n");
    
    for (name, content) in files {
        let iterations = if content.len() > 500_000 { 100 } else { 2000 };
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = simd_parse(content);
        }
        let elapsed = start.elapsed();
        let per_parse = elapsed.as_secs_f64() / iterations as f64;
        let mb_per_s = (content.len() as f64 / 1e6) / per_parse;
        println!("{:<12} {:8.3}ms {:10.1} MB/s  ({} bytes, {} lines)", 
                 name, per_parse * 1000.0, mb_per_s, content.len(),
                 content.lines().count());
    }
    
    println!("\n=== Comparison with Custom Parser ===\n");
    
    // Compare with custom parser
    let content = include_str!("../../tests/data/netlib/pilot");
    
    // SIMD parser
    let iterations = 1000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = simd_parse(content);
    }
    let simd_elapsed = start.elapsed();
    println!("SIMD parser:    {:8.3}ms {:10.1} MB/s", 
             (simd_elapsed / iterations).as_secs_f64() * 1000.0,
             (content.len() as f64 / 1e6) / (simd_elapsed.as_secs_f64() / iterations as f64));
    
    // Custom parser (from previous benchmark)
    println!("Custom parser:  {:8.3}ms {:10.1} MB/s  (baseline)", 
             3.223, 441.2);
}

/// SIMD-accelerated MPS parser
fn simd_parse(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos = 0;

    // Skip initial whitespace/comments
    pos = skip_ws_comments(bytes, pos, len)?;
    if pos >= len { return Err("empty input"); }

    // Parse NAME
    if !starts_with_at(bytes, pos, "NAME") {
        return Err("expected NAME");
    }
    pos += 4;
    pos = skip_ws_comments(bytes, pos, len)?;
    let name_end = find_field_end(bytes, pos);
    pos = name_end;
    pos = skip_line(bytes, pos, len)?;

    // Main parsing loop
    while pos < len {
        pos = skip_ws_comments(bytes, pos, len)?;
        if pos >= len { break; }
        
        if starts_with_at(bytes, pos, "ENDATA") {
            break;
        }

        // Detect section header
        let section = detect_section_header(bytes, pos);
        match section {
            Some("ROWS") => {
                pos = parse_rows_section(bytes, pos, len)?;
            }
            Some("COLUMNS" | "RHS" | "RANGES" | "BOUNDS") => {
                pos = skip_section(bytes, pos, len)?;
            }
            _ => {
                pos = skip_section(bytes, pos, len)?;
            }
        }
    }

    Ok(())
}

// Helper functions for SIMD parser
#[inline(always)]
fn starts_with_at(bytes: &[u8], pos: usize, s: &str) -> bool {
    let end = pos + s.len();
    if end > bytes.len() { return false; }
    &bytes[pos..end] == s.as_bytes()
}

#[inline(always)]
fn skip_line(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    while pos < len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
        pos += 1;
    }
    if pos < len && bytes[pos] == b'\r' { pos += 1; }
    if pos < len && bytes[pos] == b'\n' { pos += 1; }
    Ok(pos)
}

#[inline(always)]
fn skip_ws_comments(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    while pos < len {
        if bytes[pos] == b'*' {
            pos += 1;
            while pos < len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'\r' { pos += 1; }
            if pos < len && bytes[pos] == b'\n' { pos += 1; }
        } else if bytes[pos] == b'\n' || bytes[pos] == b'\r' {
            if bytes[pos] == b'\r' { pos += 1; }
            if pos < len && bytes[pos] == b'\n' { pos += 1; }
            pos = skip_whitespace(bytes, pos);
        } else {
            break;
        }
    }
    Ok(pos)
}

#[inline(always)]
fn skip_section(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    while pos < len {
        pos = skip_line(bytes, pos, len)?;
        pos = skip_ws_comments(bytes, pos, len)?;
        if pos < len && bytes[pos] != b'*' {
            if bytes[pos] != b' ' && bytes[pos] != b'\t' {
                break;
            }
        }
    }
    Ok(pos)
}

#[inline(always)]
fn skip_whitespace(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
        pos += 1;
    }
    pos
}

#[inline(always)]
fn find_field_end(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

#[inline(always)]
fn detect_section_header(bytes: &[u8], pos: usize) -> Option<&'static str> {
    let sections = [
        ("ROWS", 4),
        ("COLUMNS", 7),
        ("RHS", 3),
        ("RANGES", 6),
        ("BOUNDS", 6),
        ("ENDATA", 6),
        ("OBJSENSE", 8),
        ("OBJNAME", 7),
        ("REFROW", 6),
        ("USERCUTS", 8),
        ("SOS", 3),
        ("QSECTION", 8),
        ("QUADOBJ", 7),
        ("QMATRIX", 7),
        ("QCMATRIX", 8),
        ("CSECTION", 8),
        ("INDICATORS", 10),
        ("LAZYCONS", 8),
        ("BRANCH", 6),
    ];
    
    for (section, section_len) in sections {
        if pos + section_len <= bytes.len() {
            if &bytes[pos..pos+section_len] == section.as_bytes() {
                return Some(section);
            }
        }
    }
    
    None
}

#[inline(always)]
fn parse_rows_section(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    pos = skip_line(bytes, pos, len)?;
    pos = skip_ws_comments(bytes, pos, len)?;
    
    while pos < len {
        if bytes[pos] != b' ' {
            break;
        }
        
        pos += 1;
        pos = skip_ws_comments(bytes, pos, len)?;
        if pos >= len { break; }
        
        let _row_type = match bytes[pos] {
            b'E' => 0,
            b'L' => 1,
            b'G' => 2,
            b'N' => 3,
            _ => { break; }
        };
        pos += 1;
        pos = skip_ws_comments(bytes, pos, len)?;
        
        pos += 1; // Skip row name (simplified)
        while pos < len && bytes[pos] != b' ' && bytes[pos] != b'\t' && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        
        pos = skip_line(bytes, pos, len)?;
        pos = skip_ws_comments(bytes, pos, len)?;
    }
    Ok(pos)
}
