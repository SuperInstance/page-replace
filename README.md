# page-replace

Virtual memory page replacement algorithm simulator for research and education.

Implements four classic page replacement strategies with detailed hit/miss statistics:

- **FIFO** — First In, First Out
- **LRU** — Least Recently Used
- **Clock** — Second-chance / clock algorithm
- **LFU** — Least Frequently Used

## Usage

```rust
use page_replace::{fifo::FifoReplacer, stats::PageStats};

let mut replacer = FifoReplacer::new(3); // 3 frame capacity
let stats: PageStats = replacer.run(&[1, 2, 3, 1, 4, 1, 5]);
println!("Hit rate: {:.2}%", stats.hit_rate() * 100.0);
```

Each algorithm implements the `PageReplacer` trait and returns a `PageStats` report with hits, misses, and the full access log.

## Algorithms

| Algorithm | Module | Description |
|-----------|--------|-------------|
| FIFO | `fifo` | Evicts the oldest loaded page |
| LRU | `lru` | Evicts the least recently accessed page |
| Clock | `clock` | Second-chance circular buffer |
| LFU | `lfu` | Evicts the least frequently accessed page |

## Stats

The `stats` module provides `PageStats` with:
- Hit / miss counts
- Hit rate percentage
- Per-step access log (`AccessRecord`)

No external dependencies — pure `std`.

## License

MIT
