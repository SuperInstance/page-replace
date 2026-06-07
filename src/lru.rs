//! Least Recently Used page replacement.

use crate::stats::AccessType;
use crate::PageReplacer;

/// LRU page replacer.
///
/// Evicts the page whose most recent access is the oldest.
/// Uses a timestamp counter for O(n) simulation.
#[derive(Debug)]
pub struct LruReplacer {
    frames: Vec<(u32, u64)>, // (page, last_access_tick)
    capacity: usize,
    tick: u64,
}

impl LruReplacer {
    pub fn new(capacity: usize) -> Self {
        Self {
            frames: Vec::with_capacity(capacity),
            capacity,
            tick: 0,
        }
    }
}

impl PageReplacer for LruReplacer {
    fn access(&mut self, page: u32) -> AccessType {
        self.tick += 1;
        if let Some(entry) = self.frames.iter_mut().find(|(p, _)| *p == page) {
            entry.1 = self.tick;
            return AccessType::Hit;
        }
        // Miss
        if self.frames.len() >= self.capacity {
            // Find LRU
            let lru_idx = self
                .frames
                .iter()
                .enumerate()
                .min_by_key(|(_, (_, tick))| *tick)
                .map(|(i, _)| i)
                .unwrap();
            self.frames[lru_idx] = (page, self.tick);
        } else {
            self.frames.push((page, self.tick));
        }
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
        let r = LruReplacer::new(3);
        assert_eq!(r.frame_count(), 0);
        assert_eq!(r.capacity(), 3);
    }

    #[test]
    fn miss_then_hit() {
        let mut r = LruReplacer::new(3);
        assert_eq!(r.access(1), AccessType::Miss);
        assert_eq!(r.access(1), AccessType::Hit);
    }

    #[test]
    fn lru_evicts_oldest_access() {
        let mut r = LruReplacer::new(2);
        r.access(1); // frames: [(1,t1)]
        r.access(2); // frames: [(1,t1), (2,t2)]
        r.access(1); // hit, updates 1 -> t3
        assert_eq!(r.access(3), AccessType::Miss); // evicts 2 (t2 < t3)
        assert_eq!(r.access(2), AccessType::Miss); // 2 was evicted
        assert_eq!(r.access(3), AccessType::Hit); // 3 still here
    }

    #[test]
    fn run_full_trace() {
        let mut r = LruReplacer::new(3);
        let stats = r.run(&[1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5]);
        // LRU with 3 frames on this trace: 10 misses
        assert_eq!(stats.misses, 10);
    }

    #[test]
    fn no_belady_anomaly() {
        // LRU is stack algorithm — no Belady's anomaly
        let mut r3 = LruReplacer::new(3);
        let stats3 = r3.run(&[1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5]);

        let mut r4 = LruReplacer::new(4);
        let stats4 = r4.run(&[1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5]);

        assert!(stats4.misses <= stats3.misses);
    }
}
