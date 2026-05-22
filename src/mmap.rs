//! Memory-mapped file reading for MPS files
//!
//! This module provides zero-copy file reading using memory mapping,
//! which eliminates the need to allocate a String for the file contents.

use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

/// Memory-mapped MPS file
///
/// Provides zero-copy access to MPS file contents using memory mapping.
/// The file is mapped into memory when created and unmapped when dropped.
#[derive(Debug)]
pub struct MappedMpsFile {
    mmap: Mmap,
    path: Option<String>,
}

impl MappedMpsFile {
    /// Create a new memory-mapped MPS file from a path
    ///
    /// # Arguments
    /// * `path` - Path to the MPS file
    ///
    /// # Returns
    /// * `Ok(MappedMpsFile)` - Successfully mapped file
    /// * `Err(color_eyre::Report)` - Error mapping the file
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, color_eyre::Report> {
        let path_str = path.as_ref().to_string_lossy().into_owned();
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file) }?;
        
        Ok(MappedMpsFile {
            mmap,
            path: Some(path_str),
        })
    }
    
    /// Create a new memory-mapped MPS file from a file handle
    ///
    /// # Arguments
    /// * `file` - Open file handle
    ///
    /// # Returns
    /// * `Ok(MappedMpsFile)` - Successfully mapped file
    /// * `Err(color_eyre::Report)` - Error mapping the file
    pub fn from_file(file: &File) -> Result<Self, color_eyre::Report> {
        let mmap = unsafe { Mmap::map(file) }?;
        
        Ok(MappedMpsFile {
            mmap,
            path: None,
        })
    }
    
    /// Get the raw bytes of the mapped file
    ///
    /// This provides zero-copy access to the file contents.
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.mmap
    }
    
    /// Get the path of the mapped file, if available
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    
    /// Get the length of the mapped file
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.mmap.len()
    }
    
    /// Check if the mapped file is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.mmap.is_empty()
    }
}

impl AsRef<[u8]> for MappedMpsFile {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl std::fmt::Display for MappedMpsFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            Some(path) => write!(f, "MappedMpsFile({})", path),
            None => write!(f, "MappedMpsFile(<memory-mapped>)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mapped_file() {
        let content = include_str!("../tests/data/netlib/afiro");
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("mps_test_afiro.mps");
        std::fs::write(&file_path, content).unwrap();
        
        let mapped = MappedMpsFile::from_path(&file_path).unwrap();
        assert_eq!(mapped.len(), content.len());
        assert_eq!(mapped.as_bytes(), content.as_bytes());
        
        // Clean up
        let _ = std::fs::remove_file(&file_path);
    }
}
