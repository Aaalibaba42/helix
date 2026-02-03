Performance gain / loss compared to `String` with `CompactString`,
`SmartString`, `ArrayString<4>`, `ArrayString<20>` respectively, using
criterion data from the #15229 benchmarks.

You may check out the benchmarks and the raw json results on [my branch](https://github.com/Aaalibaba42/helix/tree/tui-bench-on-string-cell-type).

| Benchmark          | compactstring | smartstring | arraystr 4 | arraystr 20 |
|--------------------|---------------|-------------|------------|-------------|
| **buffer_diff_**   |               |             |            |             |
| emoji/16           | +8.5%         | +12.2%      | DNF        | +0.6%       |
| emoji/64           | +11.8%        | +13.0%      | DNF        | +0.6%       |
| full_change/16     | +10.8%        | +12.7%      | +0.3%      | +0.2%       |
| full_change/64     | +14.1%        | +15.5%      | +0.9%      | -0.7%       |
| full_change/128    | +12.5%        | +13.1%      | -2.3%      | -1.2%       |
| multi_width/16     | +8.9%         | +10.9%      | -2.7%      | -1.5%       |
| multi_width/64     | +11.6%        | +15.4%      | -2.1%      | +0.2%       |
| no_change/16       | +12.0%        | +14.8%      | +0.7%      | +2.1%       |
| no_change/64       | +11.0%        | +12.7%      | +1.0%      | +2.2%       |
| no_change/128      | +10.1%        | +11.7%      | -1.2%      | +0.0%       |
| partial_change/16  | +9.5%         | +10.6%      | -0.7%      | -0.7%       |
| partial_change/64  | +8.3%         | +10.7%      | -0.2%      | -0.1%       |
| partial_change/128 | +8.3%         | +10.6%      | +0.0%      | +0.3%       |
| **buffer_empty_**  |               |             |            |             |
| 16                 | **-77.4%**    | **-76.4%**  | **-93.8%** | **-94.0%**  |
| 64                 | **-78.9%**    | **-77.5%**  | **-94.6%** | **-94.6%**  |
| 255                | **-73.2%**    | **-71.9%**  | **-94.4%** | **-86.0%**  |
| **buffer_filled_** |               |             |            |             |
| 16                 | **-79.5%**    | **-78.4%**  | **-94.2%** | **-91.8%**  |
| 64                 | **-80.3%**    | **-79.5%**  | **-94.9%** | **-92.3%**  |
| 255                | **-74.9%**    | **-74.5%**  | **-94.7%** | **-85.2%**  |

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
