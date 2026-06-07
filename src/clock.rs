//! Clock (second-chance) page replacement.

use crate::stats::AccessType;
use crate::PageReplacer;

/// Clock page replacer.
///
/// Uses a circular buffer with reference bits. On eviction, pages with
/// their reference bit set get a "second chance" (bit cleared).
#[derive(Debug)]
pub struct ClockReplacer {
    /// (page, reference_bit)
    frames: Vec<Option<(u32, bool)>>,
    capacity: usize,
    hand: usize,
    len: usize,
}

impl ClockReplacer {
    pub fn new(capacity: usize) -> Self {
        Self {
            frames: vec![None; capacity],
            capacity,
            hand: 0,
            len: 0,
        }
    }
}

impl PageReplacer for ClockReplacer {
    fn access(&mut self, page: u32) -> AccessType {
        // Check if page is already loaded
        for slot in self.frames.iter_mut().flatten() {
            if slot.0 == page {
                slot.1 = true;
                return AccessType::Hit;
            }
        }
        // Miss — need to insert
        if self.len < self.capacity {
            // Find empty slot
            for slot in &mut self.frames {
                if slot.is_none() {
                    *slot = Some((page, true));
                    self.len += 1;
                    return AccessType::Miss;
                }
            }
            unreachable!();
        }
        // Evict: advance hand, give second chances
        loop {
            match &mut self.frames[self.hand] {
                Some((_, ref_bit)) if *ref_bit => {
                    *ref_bit = false;
                    self.hand = (self.hand + 1) % self.capacity;
                }
                Some((_, ref_bit)) if !*ref_bit => {
                    self.frames[self.hand] = Some((page, true));
                    self.hand = (self.hand + 1) % self.capacity;
                    return AccessType::Miss;
                }
                _ => unreachable!(),
            }
        }
    }

    fn frame_count(&self) -> usize {
        self.len
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
        let r = ClockReplacer::new(3);
        assert_eq!(r.frame_count(), 0);
        assert_eq!(r.capacity(), 3);
    }

    #[test]
    fn fill_frames() {
        let mut r = ClockReplacer::new(3);
        assert_eq!(r.access(1), AccessType::Miss);
        assert_eq!(r.access(2), AccessType::Miss);
        assert_eq!(r.access(3), AccessType::Miss);
        assert_eq!(r.frame_count(), 3);
    }

    #[test]
    fn hit_sets_ref_bit() {
        let mut r = ClockReplacer::new(3);
        r.access(1);
        r.access(2);
        r.access(3);
        // Access 1 again (hit) — sets ref bit
        assert_eq!(r.access(1), AccessType::Hit);
        // Now load 4 — should evict 2 (hand advances past 1 due to ref bit)
        assert_eq!(r.access(4), AccessType::Miss);
    }

    #[test]
    fn second_chance() {
        let mut r = ClockReplacer::new(3);
        r.access(1); // [Some((1,true)), None, None]
        r.access(2); // [Some((1,true)), Some((2,true)), None]
        r.access(3); // [Some((1,true)), Some((2,true)), Some((3,true))]
        r.access(1); // hit, ref bit already true
        r.access(4); // miss, evict: hand scans, clears all ref bits, evicts 1
        // frames: [Some(4,true), Some(2,false), Some(3,false)], hand=1
        assert_eq!(r.access(1), AccessType::Miss); // 1 was evicted, evicts 2
        assert_eq!(r.access(4), AccessType::Hit); // 4 still present
    }

    #[test]
    fn run_trace() {
        let mut r = ClockReplacer::new(3);
        let stats = r.run(&[1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5]);
        assert!(stats.misses > 0);
        assert!(stats.total_accesses() == 12);
    }
}
