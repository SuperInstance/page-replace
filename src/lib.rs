//! Virtual memory page replacement algorithms.
//!
//! Provides FIFO, LRU, Clock, and LFU page replacement strategies
//! with hit-rate tracking and detailed access logs.

pub mod fifo;
pub mod lru;
pub mod clock;
pub mod lfu;
pub mod stats;

pub use stats::{AccessRecord, AccessType, PageStats};

/// Trait for page replacement algorithms.
pub trait PageReplacer {
    /// Access a page, returning whether it was a hit.
    fn access(&mut self, page: u32) -> AccessType;

    /// Run a full reference string and return statistics.
    fn run(&mut self, references: &[u32]) -> PageStats {
        let mut log = Vec::with_capacity(references.len());
        for &page in references {
            let access_type = self.access(page);
            log.push(AccessRecord { page, access_type });
        }
        PageStats::from_log(&log)
    }

    /// Current number of frames in use.
    fn frame_count(&self) -> usize;

    /// Maximum frame capacity.
    fn capacity(&self) -> usize;
}
