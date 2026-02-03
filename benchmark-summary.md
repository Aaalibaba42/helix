Performance gain / loss compared to `String` with `CompactString`,
`SmartString`, `ArrayString<12>`, `ArrayString<28>` respectively, using
criterion data from the #15229 benchmarks.

You may check out the benchmarks and the raw json results on [my branch](https://github.com/Aaalibaba42/helix/tree/tui-bench-on-string-cell-type).

| Benchmark          | compactstring | smartstring | arraystr 12 | arraystr 28 |
|--------------------|---------------|-------------|-------------|-------------|
| **buffer_diff_**   |               |             |             |             |
| emoji/16           | +11.5%        | +15.9%      | DNF         | +2.9%       |
| emoji/64           | +11.9%        | +14.3%      | DNF         | +1.2%       |
| full_change/16     | +12.3%        | +14.4%      | +1.8%       | -0.7%       |
| full_change/64     | +14.4%        | +14.9%      | -0.4%       | -0.8%       |
| full_change/128    | +12.6%        | +12.9%      | -2.9%       | -0.3%       |
| multi_width/16     | +9.2%         | +11.4%      | -0.6%       | -1.5%       |
| multi_width/64     | +10.1%        | +15.8%      | -1.0%       | -2.3%       |
| no_change/16       | +7.3%         | +12.2%      | -2.0%       | -4.2%       |
| no_change/64       | +8.3%         | +11.5%      | -2.1%       | -4.4%       |
| no_change/128      | +9.0%         | +13.6%      | -0.9%       | -3.4%       |
| partial_change/16  | +9.6%         | +12.3%      | -1.6%       | -2.4%       |
| partial_change/64  | +9.1%         | +13.9%      | +0.8%       | -1.5%       |
| partial_change/128 | +9.6%         | +13.0%      | -0.5%       | -2.5%       |
| **buffer_empty_**  |               |             |             |             |
| 16                 | **-79.2%**    | **-78.3%**  | **-94.5%**  | **-94.4%**  |
| 64                 | **-80.9%**    | **-80.0%**  | **-95.4%**  | **-94.6%**  |
| 255                | **-74.3%**    | **-73.5%**  | **-88.5%**  | **-84.3%**  |
| **buffer_filled_** |               |             |             |             |
| 16                 | **-80.1%**    | **-78.9%**  | **-93.3%**  | **-91.3%**  |
| 64                 | **-81.0%**    | **-80.3%**  | **-94.1%**  | **-92.2%**  |
| 255                | **-75.2%**    | **-74.4%**  | **-87.9%**  | **-83.0%**  |

> [!WARNING]
arraystr 4 (ArrayString<4>) is not actually recommended, because unicode can
easiely go over 4 bytes (like we see on the emoji benchmark for instance). It
is included to showcase the kind of gain we could gain from going with less
bytes, given it's the only option that let us easiely choose this, and we can
somewhat interpolate values in between.

### Findings

`CompactString` and `SmartString` are toe to toe, losing about 8-12% on
operations, but having the huge gains from being stack-allocated in scenarios
where it matters.

`ArrayString` is the clear winner, removing the overhead of operations whilst
also having the lead in the situations where stack-allocations mattered.

Going from 20 bytes in the array string (so 24 bytes struct with the size) to 4
bytes (8 bytes total), gives a measurable (albeit small) advantage on large
buffer sizes, but measurably no difference in lower sizes. So reducing the
number of bytes to a minimum is not worth it.

So in conclusion `ArrayString` is the clear winner. The issue of the size we
give it can be `20` to be similar to the other SSO structures that exist with a
total size of `24` bytes, we can choose `28` to have a final struct sized `32`
bytes, or something else entirely, but I wouldn't go under `16`, for unicode
spec doesn't forbid chaining codepoints productively for a while on a single
symbol.
