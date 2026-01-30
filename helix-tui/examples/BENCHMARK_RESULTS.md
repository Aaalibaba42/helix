# Benchmark

Command: `cargo run -p helix-tui --example buffer_benchmark --release --no-default-features [--features X]`

### Cell Operations (10k iterations)

| Operation          | String | CompactString | SmartString |
|--------------------|--------|---------------|-------------|
| `set_symbol('x')`  | 9µs    | 5µs           | 5µs         |
| `set_symbol('界')` | 10µs   | 5µs           | 24µs        |
| `set_symbol('🎉')` | 7µs    | 5µs           | 6µs         |
| `set_char('x')`    | 6µs    | 5µs           | 5µs         |
| `set_char('界')`   | 8µs    | 5µs           | 6µs         |
| `reset()`          | 15µs   | 11µs          | 10µs        |
| `Cell::default()`  | 117µs  | 10µs          | 10µs        |

### Buffer Small (80×24 = 1,920 cells)

| Operation        | String | CompactString | SmartString |
|------------------|--------|---------------|-------------|
| `Buffer::empty()`| 550ms  | 47ms          | 45ms        |
| `reset()`        | 17ms   | 19ms          | 20ms        |
| fill             | 104ms  | 114ms         | 117ms       |
| diff (identical) | 119ms  | 137ms         | 143ms       |
| full cycle       | 250ms  | 262ms         | 276ms       |

### Buffer Medium (120×40 = 4,800 cells)

| Operation        | String | CompactString | SmartString |
|------------------|--------|---------------|-------------|
| `Buffer::empty()`| 1.34s  | 104ms         | 109ms       |
| `reset()`        | 53ms   | 46ms          | 50ms        |
| fill             | 175ms  | 181ms         | 196ms       |
| diff (identical) | 299ms  | 330ms         | 345ms       |
| full cycle       | 538ms  | 562ms         | 600ms       |

### Buffer Large (200×50 = 10,000 cells)

| Operation        | String | CompactString | SmartString |
|------------------|--------|---------------|-------------|
| `Buffer::empty()`| 2.77s  | 213ms         | 225ms       |
| `reset()`        | 110ms  | 95ms          | 105ms       |
| fill             | 224ms  | 227ms         | 240ms       |
| diff (identical) | 625ms  | 684ms         | 725ms       |
| full cycle       | 967ms  | 1.02s         | 1.07s       |

### Summary

| Operation         | CompactString   | SmartString        |
|-------------------|-----------------|--------------------|
| `Cell::default()` | **12× faster**  | **12× faster**     |
| `Buffer::empty()` | **13× faster**  | **12× faster**     |
| `reset()`         | ~same           | ~same              |
| `set_symbol()`    | ~same           | ~same (except CJK) |
| fill              | ~same           | ~same              |
| diff              | 9% slower       | 14% slower         |
| full cycle        | 5% slower       | 10% slower         |
