# Cell Operations (10k iterations)

| Operation          | String  | CompactString | SmartString |
|--------------------|---------|---------------|-------------|
| `set_symbol('x')`  | 133µs   | 5.0µs         | 5.1µs       |
| `set_symbol('界')` | 129µs   | 4.9µs         | 24µs        |
| `set_symbol('🎉')` | 143µs   | 5.0µs         | 6.4µs       |
| `set_char('x')`    | 142µs   | 4.9µs         | 5.1µs       |
| `set_char('界')`   | 143µs   | 6.0µs         | 6.1µs       |
| `reset()`          | 137µs   | 14µs          | 10µs        |
| `Cell::default()`  | 141µs   | 11µs          | 10µs        |

# Buffer Small (80×24 = 1,920 cells)

| Operation        | String | CompactString | SmartString |
|------------------|--------|---------------|-------------|
| `Buffer::empty()`| 551ms  | 45ms          | 46ms        |
| `reset()`        | 238ms  | 19ms          | 18ms        |
| fill             | 239ms  | 108ms         | 112ms       |
| diff (identical) | 128ms  | 128ms         | 140ms       |
| full cycle       | 608ms  | 254ms         | 266ms       |

# Buffer Medium (120×40 = 4,800 cells)

| Operation        | String | CompactString | SmartString |
|------------------|--------|---------------|-------------|
| `Buffer::empty()`| 1.33s  | 101ms         | 106ms       |
| `reset()`        | 612ms  | 44ms          | 50ms        |
| fill             | 395ms  | 180ms         | 185ms       |
| diff (identical) | 301ms  | 323ms         | 349ms       |
| full cycle       | 1.33s  | 569ms         | 584ms       |

# Buffer Large (200×50 = 10,000 cells)

| Operation        | String | CompactString | SmartString |
|------------------|--------|---------------|-------------|
| `Buffer::empty()`| 2.82s  | 217ms         | 225ms       |
| `reset()`        | 1.29s  | 93ms          | 104ms       |
| fill             | 502ms  | 222ms         | 233ms       |
| diff (identical) | 627ms  | 672ms         | 719ms       |
| full cycle       | 2.43s  | 1.03s         | 1.07s       |

# Summary

| Operation         | CompactString   | SmartString     |
|-------------------|-----------------|-----------------|
| `Buffer::empty()` | **13× faster**  | **13× faster**  |
| `reset()`         | **13× faster**  | **12× faster**  |
| `Cell::default()` | **13× faster**  | **14× faster**  |
| full cycle        | **2.3× faster** | **2.3× faster** |

CompactString and SmartString perform similarly. Both eliminate heap allocations
for small strings (up to 24 bytes for SmartString, 24 for CompactString).
Terminal cell symbols are 1-4 bytes, so they always fit inline.

SmartString is slightly slower on CJK `set_symbol('界')` due to its encoding.
CompactString is slightly slower on `reset()`. Overall difference is negligible.
