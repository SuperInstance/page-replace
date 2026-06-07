//! Statistics and access logging for page replacement.

/// Whether a page access resulted in a hit or miss.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    Hit,
    Miss,
}

/// A single access event in the reference string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessRecord {
    pub page: u32,
    pub access_type: AccessType,
}

/// Aggregated statistics from a page replacement simulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageStats {
    pub hits: usize,
    pub misses: usize,
    pub log: Vec<AccessRecord>,
}

impl PageStats {
    /// Build stats from an access log.
    pub fn from_log(log: &[AccessRecord]) -> Self {
        let hits = log.iter().filter(|r| r.access_type == AccessType::Hit).count();
        let misses = log.len() - hits;
        Self {
            hits,
            misses,
            log: log.to_vec(),
        }
    }

    /// Hit rate as a fraction in [0.0, 1.0].
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Miss rate as a fraction in [0.0, 1.0].
    pub fn miss_rate(&self) -> f64 {
        1.0 - self.hit_rate()
    }

    /// Total number of accesses.
    pub fn total_accesses(&self) -> usize {
        self.hits + self.misses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_log() {
        let stats = PageStats::from_log(&[]);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.hit_rate(), 0.0);
    }

    #[test]
    fn all_hits() {
        let log = vec![
            AccessRecord { page: 1, access_type: AccessType::Hit },
            AccessRecord { page: 2, access_type: AccessType::Hit },
        ];
        let stats = PageStats::from_log(&log);
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 0);
        assert!((stats.hit_rate() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn all_misses() {
        let log = vec![
            AccessRecord { page: 1, access_type: AccessType::Miss },
            AccessRecord { page: 2, access_type: AccessType::Miss },
        ];
        let stats = PageStats::from_log(&log);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 2);
        assert!((stats.hit_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mixed() {
        let log = vec![
            AccessRecord { page: 1, access_type: AccessType::Miss },
            AccessRecord { page: 1, access_type: AccessType::Hit },
            AccessRecord { page: 2, access_type: AccessType::Miss },
            AccessRecord { page: 1, access_type: AccessType::Hit },
        ];
        let stats = PageStats::from_log(&log);
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 2);
        assert!((stats.hit_rate() - 0.5).abs() < f64::EPSILON);
        assert_eq!(stats.total_accesses(), 4);
    }
}
