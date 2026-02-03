#!/usr/bin/env bash

ROOT="${1:-target/criterion}"
OUT="${2:-criterion-report.json}"

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq not found. Install with: brew install jq" >&2
  exit 1
fi

if [ ! -d "$ROOT" ]; then
  echo "error: directory not found: $ROOT" >&2
  exit 1
fi

TMP="$(mktemp -t criterion-merge.XXXXXX)"
trap 'rm -f "$TMP"' EXIT

FOUND=0
find "$ROOT" -type f -path '*/new/estimates.json' -print0 |
while IFS= read -r -d '' EST; do
  FOUND=1
  BENCH_PARENT="$(dirname "$(dirname "$EST")")"
  REL="${BENCH_PARENT#"$ROOT"/}"
  jq -n \
    --arg id "$REL" \
    --arg estimates_path "$EST" \
    --slurpfile est "$EST" '
      def ci($x): ($x.confidence_interval | {
        level: .confidence_level,
        low: .lower_bound,
        high: .upper_bound
      });

      def stat($x): {
        point: $x.point_estimate,
        se: $x.standard_error,
        ci: ci($x)
      };

      def pick_stats($e):
        {
          mean: (if $e.mean != null then stat($e.mean) else null end),
          median: (if $e.median != null then stat($e.median) else null end),
          std_dev: (if $e.std_dev != null then stat($e.std_dev) else null end),
          median_abs_dev: (if $e.median_abs_dev != null then stat($e.median_abs_dev) else null end),
          slope: (if $e.slope != null then stat($e.slope) else null end)
        } | with_entries(select(.value != null));

      def typical($stats):
        if $stats.slope != null then $stats.slope else $stats.mean end;

      ($est[0]) as $e
      | (pick_stats($e)) as $stats
      | {
          id: $id,
          estimates_path: $estimates_path,
          typical: typical($stats),
          estimates: $stats
        }
    ' >>"$TMP"
done

if [ ! -s "$TMP" ]; then
  jq -n \
    --arg generated_at "$(date -u +"%Y-%m-%dT%H:%M:%SZ")" \
    --arg root "$ROOT" \
    '{generated_at: $generated_at, root: $root, benchmarks: []}' >"$OUT"
  exit 0
fi

jq -s \
  --arg generated_at "$(date -u +"%Y-%m-%dT%H:%M:%SZ")" \
  --arg root "$ROOT" '
    {
      generated_at: $generated_at,
      root: $root,
      benchmarks: (sort_by(.id))
    }
  ' "$TMP" >"$OUT"

echo "wrote: $OUT"
