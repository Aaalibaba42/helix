# Cell Operations

| Operation | String (before) | CompactString (after) | Change |
|-----------|-----------------|----------------------|--------|
| `set_symbol('x')` | 9.00µs | 5.00µs | **1.8× faster** |
| `set_symbol('界')` | 8.25µs | 4.96µs | **1.7× faster** |
| `set_symbol('🎉')` | 7.00µs | 4.92µs | **1.4× faster** |
| `set_char('x')` | 6.25µs | 4.92µs | **1.3× faster** |
| `set_char('界')` | 10.0µs | 4.92µs | **2.0× faster** |
| `reset()` | 14.9µs | 11.5µs | **1.3× faster** |
| `Cell::default()` | 145µs | 10.2µs | **14× faster** |

# Buffer Operations - Small (80×24 = 1,920 cells)

| Operation | String | CompactString | Change |
|-----------|--------|---------------|--------|
| `Buffer::empty()` | 537ms | 41ms | **13× faster** |
| `reset()` | 17ms | 21ms | ~same |
| fill | 104ms | 112ms | ~same |
| diff (identical) | 119ms | 132ms | ~same |
| full cycle | 253ms | 266ms | ~same |

# Buffer Operations - Medium (120×40 = 4,800 cells)

| Operation | String | CompactString | Change |
|-----------|--------|---------------|--------|
| `Buffer::empty()` | 1.33s | 106ms | **13× faster** |
| `reset()` | 53ms | 57ms | ~same |
| fill | 176ms | 179ms | ~same |
| diff (identical) | 301ms | 326ms | ~same |
| full cycle | 532ms | 577ms | ~same |

# Buffer Operations - Large (200×50 = 10,000 cells)

| Operation | String | CompactString | Change |
|-----------|--------|---------------|--------|
| `Buffer::empty()` | 2.78s | 212ms | **13× faster** |
| `reset()` | 110ms | 116ms | ~same |
| fill | 222ms | 229ms | ~same |
| diff (identical) | 635ms | 683ms | ~same |
| full cycle | 955ms | 1.04s | ~same |

# Summary

| Improvement Area | Speedup |
|-----------------|---------|
| `Buffer::empty()` | **13× faster** |
| `Cell::default()` | **14× faster** |
| `set_char()` (CJK) | **2× faster** |
| `set_symbol()` | **1.4-1.8× faster** |
