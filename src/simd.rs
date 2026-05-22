//! SIMD-accelerated parsing utilities for MPS files
//!
//! This module provides SIMD-accelerated functions for:
//! - Whitespace detection and skipping
//! - Field boundary detection
//! - Section header parsing

#![cfg(feature = "simd")]

use std::simd::prelude::*;

/// SIMD lane width for parsing (16 bytes on most platforms)
pub const SIMD_WIDTH: usize = 16;

/// SIMD-accelerated whitespace skipping
///
/// Skips leading whitespace and returns the new position.
/// Much faster than scalar loop for inputs with large whitespace blocks.
#[inline(always)]
pub fn skip_whitespace_simd(bytes: &[u8], mut pos: usize) -> usize {
    let len = bytes.len();
    
    // Process in SIMD lanes for better throughput
    while pos + SIMD_WIDTH <= len {
        // Load 16 bytes into SIMD lane
        let chunk: Simd<u8, SIMD_WIDTH> = Simd::from_slice(&bytes[pos..pos + SIMD_WIDTH]);
        
        // Check if all bytes are whitespace (space or tab)
        let is_space = chunk.simd_eq(Simd::splat(b' '));
        let is_tab = chunk.simd_eq(Simd::splat(b'\t'));
        let is_whitespace = is_space | is_tab;
        
        // If all 16 bytes are whitespace, skip them all at once
        if is_whitespace.all() {
            pos += SIMD_WIDTH;
            continue;
        }
        
        // Otherwise, find the first non-whitespace byte
        let mask_val = is_whitespace.to_bitmask();
        if mask_val == !0u64 {
            // All whitespace (all bits set)
            pos += SIMD_WIDTH;
        } else {
            // Find first zero bit (first non-whitespace)
            pos += mask_val.trailing_ones() as usize;
            break;
        }
    }
    
    // Handle remaining bytes with scalar fallback
    while pos < len && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    
    pos
}

/// SIMD-accelerated field boundary detection
///
/// Finds the end of a field (non-whitespace sequence) starting at `pos`.
/// Returns the position after the last non-whitespace byte.
#[inline(always)]
pub fn find_field_end_simd(bytes: &[u8], mut pos: usize) -> usize {
    let len = bytes.len();
    
    // Skip to end of field
    while pos + SIMD_WIDTH <= len {
        let chunk: Simd<u8, SIMD_WIDTH> = Simd::from_slice(&bytes[pos..pos + SIMD_WIDTH]);
        
        // Check if all bytes are non-whitespace
        let is_space = chunk.simd_eq(Simd::splat(b' '));
        let is_tab = chunk.simd_eq(Simd::splat(b'\t'));
        let is_newline = chunk.simd_eq(Simd::splat(b'\n'));
        let is_cr = chunk.simd_eq(Simd::splat(b'\r'));
        
        let is_whitespace = is_space | is_tab | is_newline | is_cr;
        
        // If all bytes are non-whitespace, skip them all
        if !is_whitespace.any() {
            pos += SIMD_WIDTH;
            continue;
        }
        
        // Otherwise, find the first whitespace byte
        let mask_val = is_whitespace.to_bitmask();
        if mask_val == 0 {
            // No whitespace found
            pos += SIMD_WIDTH;
        } else {
            // Find first set bit (first whitespace)
            pos += mask_val.trailing_zeros() as usize;
            break;
        }
    }
    
    // Handle remaining bytes with scalar fallback
    while pos < len && !bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    
    pos
}

/// SIMD-accelerated section header detection
///
/// Checks if the input starts with a known section header.
/// Uses SIMD to compare multiple bytes at once.
#[inline(always)]
pub fn detect_section_header_simd(bytes: &[u8], pos: usize) -> Option<&'static str> {
    let len = bytes.len();
    if pos >= len {
        return None;
    }
    
    // Check for common section headers
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
        if pos + section_len > len {
            continue;
        }
        
        // Use SIMD to compare the section header if possible
        if pos + SIMD_WIDTH <= len && section_len <= SIMD_WIDTH {
            let chunk: Simd<u8, SIMD_WIDTH> = Simd::from_slice(&bytes[pos..pos + SIMD_WIDTH]);
            
            // Create SIMD array with section bytes
            let mut section_simd = Simd::splat(0u8);
            for i in 0..section_len {
                section_simd[i] = section.as_bytes()[i];
            }
            
            if chunk == section_simd {
                return Some(section);
            }
        } else if &bytes[pos..pos+section_len] == section.as_bytes() {
            return Some(section);
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_skip_whitespace_simd() {
        let bytes = b"    hello world";
        let pos = skip_whitespace_simd(bytes, 0);
        assert_eq!(pos, 4);
    }
    
    #[test]
    fn test_find_field_end_simd() {
        let bytes = b"hello world";
        let end = find_field_end_simd(bytes, 0);
        assert_eq!(end, 5);
        
        let end = find_field_end_simd(bytes, 6);
        assert_eq!(end, 11);
    }
    
    #[test]
    fn test_detect_section_header() {
        let bytes = b"ROWS\n E  R09";
        let header = detect_section_header_simd(bytes, 0);
        assert_eq!(header, Some("ROWS"));
        
        let bytes = b"COLUMNS\n    X1";
        let header = detect_section_header_simd(bytes, 0);
        assert_eq!(header, Some("COLUMNS"));
    }
}
