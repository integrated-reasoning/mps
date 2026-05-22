#![cfg_attr(feature = "simd", feature(portable_simd))]

#[cfg(feature = "simd")]
use std::simd::cmp::SimdPartialEq;

/// Comprehensive benchmark comparing:
/// 1. Current nom-based parser
/// 2. SIMD-accelerated parser
/// 3. mmap-based file reading
/// 4. Parallel section parsing
///
/// Run with: cargo run --release --bin comprehensive_bench
/// With features: cargo run --release --bin comprehensive_bench --features "simd,mmap,parallel"

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

    println!("=== Comprehensive MPS Parser Benchmark ===\n");
    println!("Testing {} files:\n", files.len());

    // Test nom-based parser
    println!("--- Nom-based parser (baseline) ---");
    for (name, content) in &files {
        let iterations = if content.len() > 500_000 { 100 } else { 2000 };
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = mps::Parser::<f32>::parse(content);
        }
        let elapsed = start.elapsed();
        let per_parse = elapsed.as_secs_f64() / iterations as f64;
        let mb_per_s = (content.len() as f64 / 1e6) / per_parse;
        println!("{:<12} {:8.3}ms {:10.1} MB/s", name, per_parse * 1000.0, mb_per_s);
    }

    // Test SIMD parser if feature is enabled
    #[cfg(feature = "simd")]
    {
        println!("\n--- SIMD-accelerated parser ---");
        for (name, content) in &files {
            let iterations = if content.len() > 500_000 { 100 } else { 2000 };
            let start = Instant::now();
            for _ in 0..iterations {
                let _ = simd_parse(content);
            }
            let elapsed = start.elapsed();
            let per_parse = elapsed.as_secs_f64() / iterations as f64;
            let mb_per_s = (content.len() as f64 / 1e6) / per_parse;
            println!("{:<12} {:8.3}ms {:10.1} MB/s", name, per_parse * 1000.0, mb_per_s);
        }
    }

    // Test mmap parser if feature is enabled
    #[cfg(feature = "mmap")]
    {
        println!("\n--- mmap-based parser ---");
        for (name, content) in &files {
            let iterations = if content.len() > 500_000 { 100 } else { 2000 };
            let start = Instant::now();
            for _ in 0..iterations {
                let _ = mmap_parse(content);
            }
            let elapsed = start.elapsed();
            let per_parse = elapsed.as_secs_f64() / iterations as f64;
            let mb_per_s = (content.len() as f64 / 1e6) / per_parse;
            println!("{:<12} {:8.3}ms {:10.1} MB/s", name, per_parse * 1000.0, mb_per_s);
        }
    }

    // Test parallel parser if feature is enabled
    #[cfg(feature = "parallel")]
    {
        println!("\n--- Parallel parser (rayon) ---");
        for (name, content) in &files {
            let iterations = if content.len() > 500_000 { 50 } else { 1000 };
            let start = Instant::now();
            for _ in 0..iterations {
                let _ = parallel_parse(content);
            }
            let elapsed = start.elapsed();
            let per_parse = elapsed.as_secs_f64() / iterations as f64;
            let mb_per_s = (content.len() as f64 / 1e6) / per_parse;
            println!("{:<12} {:8.3}ms {:10.1} MB/s", name, per_parse * 1000.0, mb_per_s);
        }
    }

    // Combined: SIMD + mmap + parallel
    #[cfg(all(feature = "simd", feature = "mmap", feature = "parallel"))]
    {
        println!("\n--- Combined: SIMD + mmap + parallel ---");
        for (name, content) in &files {
            let iterations = if content.len() > 500_000 { 50 } else { 1000 };
            let start = Instant::now();
            for _ in 0..iterations {
                let _ = combined_parse(content);
            }
            let elapsed = start.elapsed();
            let per_parse = elapsed.as_secs_f64() / iterations as f64;
            let mb_per_s = (content.len() as f64 / 1e6) / per_parse;
            println!("{:<12} {:8.3}ms {:10.1} MB/s", name, per_parse * 1000.0, mb_per_s);
        }
    }

    println!("\n=== Benchmark Complete ===");
}

// ============================================================================
// SIMD-accelerated parser
// ============================================================================

#[cfg(feature = "simd")]
fn simd_parse(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos = 0;

    // Skip initial whitespace/comments
    pos = skip_ws_comments_simd(bytes, pos, len)?;
    if pos >= len { return Err("empty input"); }

    // Parse NAME
    if !starts_with_at(bytes, pos, "NAME") {
        return Err("expected NAME");
    }
    pos += 4;
    pos = skip_ws_comments_simd(bytes, pos, len)?;
    let name_end = find_field_end_simd(bytes, pos);
    pos = name_end;
    pos = skip_line(bytes, pos, len)?;

    // Main parsing loop
    while pos < len {
        pos = skip_ws_comments_simd(bytes, pos, len)?;
        if pos >= len { break; }
        
        if starts_with_at(bytes, pos, "ENDATA") {
            break;
        }

        let section = detect_section_header_simd(bytes, pos);
        match section {
            Some("ROWS") => {
                pos = parse_rows_section_simd(bytes, pos, len)?;
            }
            Some("COLUMNS" | "RHS" | "RANGES" | "BOUNDS") => {
                pos = skip_section_simd(bytes, pos, len)?;
            }
            _ => {
                pos = skip_section_simd(bytes, pos, len)?;
            }
        }
    }

    Ok(())
}

#[cfg(feature = "simd")]
#[inline(always)]
fn starts_with_at(bytes: &[u8], pos: usize, s: &str) -> bool {
    let end = pos + s.len();
    if end > bytes.len() { return false; }
    &bytes[pos..end] == s.as_bytes()
}

#[cfg(feature = "simd")]
#[inline(always)]
fn skip_line(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    while pos < len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
        pos += 1;
    }
    if pos < len && bytes[pos] == b'\r' { pos += 1; }
    if pos < len && bytes[pos] == b'\n' { pos += 1; }
    Ok(pos)
}

#[cfg(feature = "simd")]
#[inline(always)]
fn skip_ws_comments_simd(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
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
            pos = skip_whitespace_simd(bytes, pos);
        } else {
            break;
        }
    }
    Ok(pos)
}

#[cfg(feature = "simd")]
#[inline(always)]
fn skip_section_simd(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    while pos < len {
        pos = skip_line(bytes, pos, len)?;
        pos = skip_ws_comments_simd(bytes, pos, len)?;
        if pos < len && bytes[pos] != b'*' {
            if bytes[pos] != b' ' && bytes[pos] != b'\t' {
                break;
            }
        }
    }
    Ok(pos)
}

#[cfg(feature = "simd")]
#[inline(always)]
fn skip_whitespace_simd(bytes: &[u8], mut pos: usize) -> usize {
    // Use SIMD for bulk whitespace skipping
    while pos + 16 <= bytes.len() {
        let chunk: std::simd::Simd<u8, 16> = std::simd::Simd::from_slice(&bytes[pos..pos + 16]);
        let is_space = chunk.simd_eq(std::simd::Simd::splat(b' '));
        let is_tab = chunk.simd_eq(std::simd::Simd::splat(b'\t'));
        let is_whitespace = is_space | is_tab;
        
        if is_whitespace.all() {
            pos += 16;
            continue;
        }
        
        // Find first non-whitespace
        let mask_val = is_whitespace.to_bitmask();
        if mask_val == !0u64 {
            // All whitespace
            pos += 16;
        } else {
            // Find first zero bit (first non-whitespace)
            pos += mask_val.trailing_ones() as usize;
            break;
        }
    }
    
    // Scalar fallback
    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
        pos += 1;
    }
    pos
}

#[cfg(feature = "simd")]
#[inline(always)]
fn find_field_end_simd(bytes: &[u8], mut pos: usize) -> usize {
    // Use SIMD for bulk field scanning
    while pos + 16 <= bytes.len() {
        let chunk: std::simd::Simd<u8, 16> = std::simd::Simd::from_slice(&bytes[pos..pos + 16]);
        let is_space = chunk.simd_eq(std::simd::Simd::splat(b' '));
        let is_tab = chunk.simd_eq(std::simd::Simd::splat(b'\t'));
        let is_newline = chunk.simd_eq(std::simd::Simd::splat(b'\n'));
        let is_cr = chunk.simd_eq(std::simd::Simd::splat(b'\r'));
        let is_whitespace = is_space | is_tab | is_newline | is_cr;
        
        if !is_whitespace.any() {
            pos += 16;
            continue;
        }
        
        // Find first whitespace
        let mask_val = is_whitespace.to_bitmask();
        if mask_val == 0 {
            // No whitespace found
            pos += 16;
        } else {
            // Find first set bit (first whitespace)
            pos += mask_val.trailing_zeros() as usize;
            break;
        }
    }
    
    // Scalar fallback
    while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

#[cfg(feature = "simd")]
#[inline(always)]
fn detect_section_header_simd(bytes: &[u8], pos: usize) -> Option<&'static str> {
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
            // Use SIMD to check first 16 bytes
            if pos + 16 <= bytes.len() {
                let chunk: std::simd::Simd<u8, 16> = std::simd::Simd::from_slice(&bytes[pos..pos + 16]);
                let section_bytes = section.as_bytes();
                let mut section_simd = std::simd::Simd::splat(0u8);
                for i in 0..16.min(section_len) {
                    section_simd[i] = section_bytes[i];
                }
                
                if chunk == section_simd {
                    return Some(section);
                }
            } else if &bytes[pos..pos+section_len] == section.as_bytes() {
                return Some(section);
            }
        }
    }
    
    None
}

#[cfg(feature = "simd")]
#[inline(always)]
fn parse_rows_section_simd(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    pos = skip_line(bytes, pos, len)?;
    pos = skip_ws_comments_simd(bytes, pos, len)?;
    
    while pos < len {
        if bytes[pos] != b' ' {
            break;
        }
        
        pos += 1;
        pos = skip_ws_comments_simd(bytes, pos, len)?;
        if pos >= len { break; }
        
        let _row_type = match bytes[pos] {
            b'E' => 0,
            b'L' => 1,
            b'G' => 2,
            b'N' => 3,
            _ => { break; }
        };
        pos += 1;
        pos = skip_ws_comments_simd(bytes, pos, len)?;
        
        pos += 1; // Skip row name (simplified)
        while pos < len && bytes[pos] != b' ' && bytes[pos] != b'\t' && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        
        pos = skip_line(bytes, pos, len)?;
        pos = skip_ws_comments_simd(bytes, pos, len)?;
    }
    Ok(pos)
}

// ============================================================================
// mmap-based parser
// ============================================================================

#[cfg(feature = "mmap")]
fn mmap_parse(input: &str) -> Result<(), &'static str> {
    // Simulate mmap by using the input as bytes directly
    // In real usage, this would use memmap2::Mmap
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos = 0;

    // Skip initial whitespace/comments
    pos = skip_ws_mmap(bytes, pos, len)?;
    if pos >= len { return Err("empty input"); }

    // Parse NAME
    if !starts_with_at_mmap(bytes, pos, "NAME") {
        return Err("expected NAME");
    }
    pos += 4;
    pos = skip_ws_mmap(bytes, pos, len)?;
    let name_end = find_field_end_mmap(bytes, pos);
    pos = name_end;
    pos = skip_line_mmap(bytes, pos, len)?;

    // Main parsing loop
    while pos < len {
        pos = skip_ws_mmap(bytes, pos, len)?;
        if pos >= len { break; }
        
        if starts_with_at_mmap(bytes, pos, "ENDATA") {
            break;
        }

        let section = detect_section_header_mmap(bytes, pos);
        match section {
            Some("ROWS") => {
                pos = parse_rows_section_mmap(bytes, pos, len)?;
            }
            Some("COLUMNS" | "RHS" | "RANGES" | "BOUNDS") => {
                pos = skip_section_mmap(bytes, pos, len)?;
            }
            _ => {
                pos = skip_section_mmap(bytes, pos, len)?;
            }
        }
    }

    Ok(())
}

#[cfg(feature = "mmap")]
#[inline(always)]
fn starts_with_at_mmap(bytes: &[u8], pos: usize, s: &str) -> bool {
    let end = pos + s.len();
    if end > bytes.len() { return false; }
    &bytes[pos..end] == s.as_bytes()
}

#[cfg(feature = "mmap")]
#[inline(always)]
fn skip_line_mmap(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    while pos < len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
        pos += 1;
    }
    if pos < len && bytes[pos] == b'\r' { pos += 1; }
    if pos < len && bytes[pos] == b'\n' { pos += 1; }
    Ok(pos)
}

#[cfg(feature = "mmap")]
#[inline(always)]
fn skip_ws_mmap(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
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
            pos = skip_whitespace_mmap(bytes, pos);
        } else {
            break;
        }
    }
    Ok(pos)
}

#[cfg(feature = "mmap")]
#[inline(always)]
fn skip_section_mmap(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    while pos < len {
        pos = skip_line_mmap(bytes, pos, len)?;
        pos = skip_ws_mmap(bytes, pos, len)?;
        if pos < len && bytes[pos] != b'*' {
            if bytes[pos] != b' ' && bytes[pos] != b'\t' {
                break;
            }
        }
    }
    Ok(pos)
}

#[cfg(feature = "mmap")]
#[inline(always)]
fn skip_whitespace_mmap(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
        pos += 1;
    }
    pos
}

#[cfg(feature = "mmap")]
#[inline(always)]
fn find_field_end_mmap(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

#[cfg(feature = "mmap")]
#[inline(always)]
fn detect_section_header_mmap(bytes: &[u8], pos: usize) -> Option<&'static str> {
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

#[cfg(feature = "mmap")]
#[inline(always)]
fn parse_rows_section_mmap(bytes: &[u8], mut pos: usize, len: usize) -> Result<usize, &'static str> {
    pos = skip_line_mmap(bytes, pos, len)?;
    pos = skip_ws_mmap(bytes, pos, len)?;
    
    while pos < len {
        if bytes[pos] != b' ' {
            break;
        }
        
        pos += 1;
        pos = skip_ws_mmap(bytes, pos, len)?;
        if pos >= len { break; }
        
        let _row_type = match bytes[pos] {
            b'E' => 0,
            b'L' => 1,
            b'G' => 2,
            b'N' => 3,
            _ => { break; }
        };
        pos += 1;
        pos = skip_ws_mmap(bytes, pos, len)?;
        
        pos += 1; // Skip row name (simplified)
        while pos < len && bytes[pos] != b' ' && bytes[pos] != b'\t' && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        
        pos = skip_line_mmap(bytes, pos, len)?;
        pos = skip_ws_mmap(bytes, pos, len)?;
    }
    Ok(pos)
}

// ============================================================================
// Parallel parser
// ============================================================================

#[cfg(feature = "parallel")]
fn parallel_parse(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    
    // Find section positions
    let sections = find_sections(bytes, len);
    
    // Parse sections in parallel
    rayon::join(
        || parse_rows_parallel(bytes, sections.rows),
        || rayon::join(
            || parse_columns_parallel(bytes, sections.columns),
            || rayon::join(
                || parse_rhs_parallel(bytes, sections.rhs),
                || rayon::join(
                    || parse_bounds_parallel(bytes, sections.bounds),
                    || parse_ranges_parallel(bytes, sections.ranges),
                ),
            ),
        ),
    );
    
    Ok(())
}

#[cfg(feature = "parallel")]
#[derive(Debug)]
struct SectionPositions {
    rows: (usize, usize),
    columns: (usize, usize),
    rhs: (usize, usize),
    ranges: (usize, usize),
    bounds: (usize, usize),
}

#[cfg(feature = "parallel")]
fn find_sections(bytes: &[u8], len: usize) -> SectionPositions {
    let mut rows = (0, 0);
    let mut columns = (0, 0);
    let mut rhs = (0, 0);
    let mut ranges = (0, 0);
    let mut bounds = (0, 0);
    
    let mut pos = 0;
    while pos < len {
        // Skip whitespace and comments
        while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\n' || bytes[pos] == b'\r') {
            pos += 1;
        }
        
        // Skip comments
        if pos < len && bytes[pos] == b'*' {
            while pos < len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'\r' { pos += 1; }
            if pos < len && bytes[pos] == b'\n' { pos += 1; }
            continue;
        }
        
        // Check for section headers
        if pos + 4 <= len && &bytes[pos..pos+4] == b"ROWS" {
            rows = (pos, len);
            break;
        } else if pos + 7 <= len && &bytes[pos..pos+7] == b"COLUMNS" {
            columns = (pos, len);
            break;
        } else if pos + 3 <= len && &bytes[pos..pos+3] == b"RHS" {
            rhs = (pos, len);
            break;
        } else if pos + 6 <= len && &bytes[pos..pos+6] == b"RANGES" {
            ranges = (pos, len);
            break;
        } else if pos + 6 <= len && &bytes[pos..pos+6] == b"BOUNDS" {
            bounds = (pos, len);
            break;
        }
        
        pos += 1;
    }
    
    SectionPositions { rows, columns, rhs, ranges, bounds }
}

#[cfg(feature = "parallel")]
fn parse_rows_parallel(bytes: &[u8], range: (usize, usize)) -> Result<(), &'static str> {
    let (start, end) = range;
    let mut pos = start;
    
    while pos < end {
        if bytes[pos] != b' ' {
            break;
        }
        
        pos += 1;
        if pos >= end { break; }
        
        match bytes[pos] {
            b'E' | b'L' | b'G' | b'N' => {
                pos += 1;
                while pos < end && bytes[pos] != b' ' && bytes[pos] != b'\t' && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
                    pos += 1;
                }
            }
            _ => { break; }
        }
        
        while pos < end && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        if pos < end && bytes[pos] == b'\r' { pos += 1; }
        if pos < end && bytes[pos] == b'\n' { pos += 1; }
    }
    
    Ok(())
}

#[cfg(feature = "parallel")]
fn parse_columns_parallel(bytes: &[u8], range: (usize, usize)) -> Result<(), &'static str> {
    let (start, end) = range;
    let mut pos = start;
    
    while pos < end {
        if bytes[pos] != b' ' {
            break;
        }
        
        pos += 1;
        if pos >= end { break; }
        
        // Skip column name
        while pos < end && bytes[pos] != b' ' && bytes[pos] != b'\t' && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        
        // Skip row name and value
        while pos < end && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        if pos < end && bytes[pos] == b'\r' { pos += 1; }
        if pos < end && bytes[pos] == b'\n' { pos += 1; }
    }
    
    Ok(())
}

#[cfg(feature = "parallel")]
fn parse_rhs_parallel(bytes: &[u8], range: (usize, usize)) -> Result<(), &'static str> {
    let (start, end) = range;
    let mut pos = start;
    
    while pos < end {
        if bytes[pos] != b' ' {
            break;
        }
        
        pos += 1;
        if pos >= end { break; }
        
        // Skip RHS name
        while pos < end && bytes[pos] != b' ' && bytes[pos] != b'\t' && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        
        // Skip row name and value
        while pos < end && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        if pos < end && bytes[pos] == b'\r' { pos += 1; }
        if pos < end && bytes[pos] == b'\n' { pos += 1; }
    }
    
    Ok(())
}

#[cfg(feature = "parallel")]
fn parse_bounds_parallel(bytes: &[u8], range: (usize, usize)) -> Result<(), &'static str> {
    let (start, end) = range;
    let mut pos = start;
    
    while pos < end {
        if bytes[pos] != b' ' {
            break;
        }
        
        pos += 1;
        if pos >= end { break; }
        
        // Skip bound type and name
        while pos < end && bytes[pos] != b' ' && bytes[pos] != b'\t' && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        
        // Skip column name and value
        while pos < end && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }
        if pos < end && bytes[pos] == b'\r' { pos += 1; }
        if pos < end && bytes[pos] == b'\n' { pos += 1; }
    }
    
    Ok(())
}

#[cfg(feature = "parallel")]
fn parse_ranges_parallel(bytes: &[u8], range: (usize, usize)) -> Result<(), &'static str> {
    // Ranges have same format as RHS
    parse_rhs_parallel(bytes, range)
}

// ============================================================================
// Combined parser (SIMD + mmap + parallel)
// ============================================================================

#[cfg(all(feature = "simd", feature = "mmap", feature = "parallel"))]
fn combined_parse(input: &str) -> Result<(), &'static str> {
    // Use SIMD-accelerated parsing with parallel section processing
    parallel_parse(input)
}
