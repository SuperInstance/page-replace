//! First In, First Out page replacement.

use crate::stats::AccessType;
use crate::PageReplacer;
use std::collections::VecDeque;

/// FIFO page replacer.
///
/// Evicts the page that has been in memory the longest.
#[derive(Debug)]
pub struct FifoReplacer {
    frames: VecDeque<u32>,
    capacity: usize,
}

impl FifoReplacer {
    pub fn new(capacity: usize) -> Self {
        Self {
            frames: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
}

impl PageReplacer for FifoReplacer {
    fn access(&mut self, page: u32) -> AccessType {
        if self.frames.contains(&page) {
            return AccessType::Hit;
        }
        // Miss: need to load
        if self.frames.len() >= self.capacity {
            self.frames.pop_front();
        }
        self.frames.push_back(page);
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
        let r = FifoReplacer::new(3);
        assert_eq!(r.frame_count(), 0);
        assert_eq!(r.capacity(), 3);
    }

    #[test]
    fn single_miss() {
        let mut r = FifoReplacer::new(3);
        assert_eq!(r.access(1), AccessType::Miss);
        assert_eq!(r.frame_count(), 1);
    }

    #[test]
    fn hit_on_loaded() {
        let mut r = FifoReplacer::new(3);
        r.access(1);
        assert_eq!(r.access(1), AccessType::Hit);
    }

    #[test]
    fn fifo_eviction_order() {
        let mut r = FifoReplacer::new(2);
        r.access(1); // miss, frames: [1]
        r.access(2); // miss, frames: [1, 2]
        r.access(3); // miss, evicts 1, frames: [2, 3]
        assert_eq!(r.access(1), AccessType::Miss); // 1 was evicted
    }

    #[test]
    fn belady_anomaly() {
        // Classic Belady's anomaly: more frames can mean more faults with FIFO
        let mut r3 = FifoReplacer::new(3);
        let stats3 = r3.run(&[1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5]);

        let mut r4 = FifoReplacer::new(4);
        let stats4 = r4.run(&[1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5]);

        // With 3 frames: 9 faults. With 4 frames: 10 faults (Belady's anomaly)
        assert_eq!(stats3.misses, 9);
        assert_eq!(stats4.misses, 10);
    }

    #[test]
    fn full_capacity_no_eviction() {
        let mut r = FifoReplacer::new(3);
        r.access(1);
        r.access(2);
        r.access(3);
        // All hit now
        assert_eq!(r.access(2), AccessType::Hit);
        assert_eq!(r.access(3), AccessType::Hit);
        assert_eq!(r.access(1), AccessType::Hit);
    }
}
