//! Run with: cargo run -p helix-tui --example buffer_benchmark --release

use helix_tui::buffer::{Buffer, Cell};
use helix_view::graphics::{Rect, Style};
use std::hint::black_box;
use std::time::{Duration, Instant};

const WARMUP_ITERS: u32 = 100;
const BENCH_ITERS: u32 = 10_000;

const SMALL_TERM: (u16, u16) = (80, 24);
const MEDIUM_TERM: (u16, u16) = (120, 40);
const LARGE_TERM: (u16, u16) = (200, 50);

fn bench<F: FnMut()>(name: &str, mut f: F) -> Duration {
    for _ in 0..WARMUP_ITERS {
        f();
    }
    let start = Instant::now();
    for _ in 0..BENCH_ITERS {
        f();
    }
    let elapsed = start.elapsed();
    println!("  {name}: {elapsed:?} total");
    elapsed
}

fn bench_cell_operations() {
    println!("Cell Operations");
    let mut cell = Cell::default();

    bench("set_symbol('x')", || {
        black_box(cell.set_symbol("x"));
    });
    bench("set_symbol('界')", || {
        black_box(cell.set_symbol("界"));
    });
    bench("set_symbol('🎉')", || {
        black_box(cell.set_symbol("🎉"));
    });
    bench("set_char('x')", || {
        black_box(cell.set_char('x'));
    });
    bench("set_char('界')", || {
        black_box(cell.set_char('界'));
    });
    bench("reset()", || {
        cell.reset();
        black_box(&cell);
    });
    bench("Cell::default()", || {
        black_box(Cell::default());
    });
}

fn bench_buffer_operations(size: (u16, u16), label: &str) {
    let cells = size.0 as u32 * size.1 as u32;
    println!("\nBuffer {label} ({}x{} = {cells} cells)", size.0, size.1);

    let area = Rect::new(0, 0, size.0, size.1);
    let style = Style::default();
    let line = "The quick brown fox jumps over the lazy dog. ";

    bench("Buffer::empty()", || {
        black_box(Buffer::empty(area));
    });

    let mut buffer = Buffer::empty(area);
    bench("reset()", || {
        buffer.reset();
        black_box(&buffer);
    });

    bench("fill (set_string all rows)", || {
        for y in 0..size.1 {
            buffer.set_string(0, y, line, style);
        }
        black_box(&buffer);
    });

    let buffer1 = Buffer::empty(area);
    let buffer2 = Buffer::empty(area);
    bench("diff (identical)", || {
        black_box(buffer1.diff(&buffer2));
    });

    let mut buffer_a = Buffer::empty(area);
    let mut buffer_b = Buffer::empty(area);
    for y in 0..size.1 {
        buffer_a.set_string(0, y, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", style);
        buffer_b.set_string(0, y, "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", style);
    }
    bench("diff (all different)", || {
        black_box(buffer_a.diff(&buffer_b));
    });

    let mut buffer_prev = Buffer::empty(area);
    let mut buffer_next = Buffer::empty(area);
    for y in 0..size.1 {
        buffer_prev.set_string(0, y, line, style);
        buffer_next.set_string(0, y, line, style);
    }
    if size.1 > 5 {
        buffer_next.set_string(0, 2, "MODIFIED LINE **********************************", style);
        buffer_next.set_string(0, 3, "MODIFIED LINE **********************************", style);
        buffer_next.set_string(0, 4, "MODIFIED LINE **********************************", style);
    }
    bench("diff (3 lines changed)", || {
        black_box(buffer_prev.diff(&buffer_next));
    });

    let mut buffers = [Buffer::empty(area), Buffer::empty(area)];
    let mut current = 0;
    for y in 0..size.1 {
        buffers[1].set_string(0, y, line, style);
    }
    bench("full cycle (reset+fill+diff+swap)", || {
        buffers[current].reset();
        for y in 0..size.1 {
            buffers[current].set_string(0, y, line, style);
        }
        black_box(buffers[1 - current].diff(&buffers[current]));
        current = 1 - current;
    });
}

fn main() {
    println!("helix-tui buffer benchmark ({BENCH_ITERS} iters, {WARMUP_ITERS} warmup)\n");

    bench_cell_operations();

    for (size, label) in [(SMALL_TERM, "small"), (MEDIUM_TERM, "medium"), (LARGE_TERM, "large")] {
        bench_buffer_operations(size, label);
    }
}
