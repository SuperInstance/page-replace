//! Least Frequently Used page replacement.

use crate::stats::AccessType;
use crate::PageReplacer;
use std::collections::HashMap;

/// LFU page replacer.
///
/// Evicts the page with the lowest access frequency.
/// Ties are broken by insertion order (earliest inserted evicted first).
#[derive(Debug)]
pub struct LfuReplacer {
    frames: Vec<u32>,
    freq: HashMap<u32, usize>,
    capacity: usize,
    /// Global frequency tracking for pages not in frames too
    access_count: HashMap<u32, usize>,
}

impl LfuReplacer {
    pub fn new(capacity: usize) -> Self {
        Self {
            frames: Vec::with_capacity(capacity),
            freq: HashMap::new(),
            capacity,
            access_count: HashMap::new(),
        }
    }

    fn inc_freq(&mut self, page: u32) {
        *self.access_count.entry(page).or_insert(0) += 1;
        *self.freq.entry(page).or_insert(0) += 1;
    }
}

impl PageReplacer for LfuReplacer {
    fn access(&mut self, page: u32) -> AccessType {
        if self.frames.contains(&page) {
            self.inc_freq(page);
            return AccessType::Hit;
        }
        // Miss
        if self.frames.len() >= self.capacity {
            // Find the frame with lowest frequency
            let min_freq = self
                .frames
                .iter()
                .map(|p| *self.freq.get(p).unwrap_or(&0))
                .min()
                .unwrap();
            let evict_idx = self
                .frames
                .iter()
                .position(|p| *self.freq.get(p).unwrap_or(&0) == min_freq)
                .unwrap();
            let evicted = self.frames[evict_idx];
            self.freq.remove(&evicted);
            self.frames[evict_idx] = page;
        } else {
            self.frames.push(page);
        }
        self.inc_freq(page);
        AccessType::Miss
    }

    fn frame_count(&self) -> usize {
        self.frames.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PageReplacer;

    #[test]
    fn initial_state() {
        let r = LfuReplacer::new(3);
        assert_eq!(r.frame_count(), 0);
        assert_eq!(r.capacity(), 3);
    }

    #[test]
    fn miss_then_hit() {
        let mut r = LfuReplacer::new(3);
        assert_eq!(r.access(1), AccessType::Miss);
        assert_eq!(r.access(1), AccessType::Hit);
    }

    #[test]
    fn evicts_lowest_frequency() {
        let mut r = LfuReplacer::new(2);
        r.access(1); // freq(1)=1
        r.access(2); // freq(2)=1
        r.access(1); // hit, freq(1)=2
        r.access(3); // miss, evicts 2 (freq=1 < freq(1)=2)
        assert_eq!(r.access(2), AccessType::Miss);
        assert_eq!(r.access(1), AccessType::Hit);
    }

    #[test]
    fn tie_break_by_insertion_order() {
        let mut r = LfuReplacer::new(2);
        r.access(1); // freq(1)=1
        r.access(2); // freq(2)=1
        // Both have freq 1, evict first inserted (1)
        r.access(3); // evicts 1
        assert_eq!(r.access(1), AccessType::Miss);
        assert_eq!(r.access(2), AccessType::Hit);
    }

    #[test]
    fn run_trace() {
        let mut r = LfuReplacer::new(3);
        let stats = r.run(&[1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5]);
        assert!(stats.misses > 0);
        assert_eq!(stats.total_accesses(), 12);
    }

    #[test]
    fn frequency_accumulates() {
        let mut r = LfuReplacer::new(3);
        r.access(1); // miss, freq(1)=1
        r.access(1); // hit, freq(1)=2
        r.access(1); // hit, freq(1)=3
        r.access(2); // miss, freq(2)=1
        // Page 1 has freq 3, page 2 has freq 1
        assert_eq!(r.freq[&1], 3);
        assert_eq!(r.freq[&2], 1);
    }
}
