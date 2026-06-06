use crate::util::Color;

// 6 sub-cell sample positions in cell-relative (dx, dy) coords where
// (0, 0) is the cell's top-left corner and (1, 1) is the bottom-right.
// Layout: 3 rows x 2 cols.
//   [0] [1]    upper
//   [2] [3]    middle
//   [4] [5]    lower
// Terminal cells are roughly 2:1 (taller than wide) so y rows are spread
// more than x columns. Inspired by Alex Harri's blog post on ASCII rendering.
pub const SAMPLE_POSITIONS: [(f32, f32); 6] = [
    (1.0 / 3.0, 1.0 / 6.0),
    (2.0 / 3.0, 1.0 / 6.0),
    (1.0 / 3.0, 3.0 / 6.0),
    (2.0 / 3.0, 3.0 / 6.0),
    (1.0 / 3.0, 5.0 / 6.0),
    (2.0 / 3.0, 5.0 / 6.0),
];

// Per-character "ink density" at the same 6 sample positions, hand-tuned.
// Values are in [0, 1]. The selection step finds the nearest neighbor by
// Euclidean distance after the sample vector has been contrast-enhanced.
// 30 characters cover the major directional/density patterns. Replacing
// these with measurements taken from an actual font bitmap is the natural
// follow-up if visual quality matters.
pub const CHAR_FOOTPRINTS: &[(char, [f32; 6])] = &[
    (' ', [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    ('.', [0.0, 0.0, 0.0, 0.0, 0.3, 0.3]),
    (',', [0.0, 0.0, 0.0, 0.0, 0.4, 0.1]),
    ('\'', [0.4, 0.0, 0.0, 0.0, 0.0, 0.0]),
    ('`', [0.4, 0.0, 0.0, 0.0, 0.0, 0.0]),
    ('"', [0.4, 0.4, 0.0, 0.0, 0.0, 0.0]),
    ('-', [0.0, 0.0, 0.5, 0.5, 0.0, 0.0]),
    ('_', [0.0, 0.0, 0.0, 0.0, 0.5, 0.5]),
    ('=', [0.0, 0.0, 0.4, 0.4, 0.3, 0.3]),
    ('~', [0.0, 0.0, 0.3, 0.3, 0.0, 0.0]),
    (':', [0.3, 0.3, 0.0, 0.0, 0.3, 0.3]),
    (';', [0.3, 0.3, 0.0, 0.0, 0.4, 0.1]),
    ('|', [0.4, 0.4, 0.4, 0.4, 0.4, 0.4]),
    ('/', [0.0, 0.5, 0.2, 0.3, 0.5, 0.0]),
    ('\\', [0.5, 0.0, 0.3, 0.2, 0.0, 0.5]),
    ('+', [0.1, 0.1, 0.5, 0.5, 0.1, 0.1]),
    ('*', [0.3, 0.3, 0.4, 0.4, 0.3, 0.3]),
    ('<', [0.3, 0.1, 0.5, 0.1, 0.3, 0.1]),
    ('>', [0.1, 0.3, 0.1, 0.5, 0.1, 0.3]),
    ('^', [0.4, 0.4, 0.1, 0.1, 0.0, 0.0]),
    ('v', [0.0, 0.0, 0.1, 0.1, 0.4, 0.4]),
    ('o', [0.3, 0.3, 0.4, 0.4, 0.3, 0.3]),
    ('x', [0.5, 0.5, 0.3, 0.3, 0.5, 0.5]),
    ('X', [0.6, 0.6, 0.4, 0.4, 0.6, 0.6]),
    ('#', [0.7, 0.7, 0.7, 0.7, 0.7, 0.7]),
    ('%', [0.5, 0.5, 0.5, 0.5, 0.5, 0.5]),
    ('@', [0.8, 0.8, 0.8, 0.8, 0.8, 0.8]),
    ('M', [0.7, 0.7, 0.5, 0.5, 0.5, 0.5]),
    ('W', [0.5, 0.5, 0.5, 0.5, 0.7, 0.7]),
    ('&', [0.6, 0.4, 0.5, 0.5, 0.4, 0.6]),
];

// Global contrast enhancement: normalize each component by the cell's max,
// raise to a power, denormalize. Squashes dim samples relative to the
// brightest one — this is what lets a partially-covered cell pick an
// asymmetric character instead of a low-brightness solid one.
pub fn apply_global_contrast(samples: &mut [f32; 6]) {
    let max = samples.iter().cloned().fold(0.0_f32, f32::max);
    if max < 1e-6 {
        return;
    }
    for s in samples.iter_mut() {
        let n = *s / max;
        *s = n * n * max;
    }
}

// Pick the character whose footprint is closest in 6D Euclidean space.
pub fn match_char(samples: &[f32; 6]) -> Color {
    let mut best_char = ' ';
    let mut best_dist = f32::INFINITY;
    for (ch, footprint) in CHAR_FOOTPRINTS {
        let mut d = 0.0_f32;
        for k in 0..6 {
            let diff = samples[k] - footprint[k];
            d += diff * diff;
        }
        if d < best_dist {
            best_dist = d;
            best_char = *ch;
        }
    }
    best_char
}

// Reduce a per-cell lum_samples buffer to chars in the framebuffer.
pub fn finalize_frame(
    lum_samples: &Vec<Vec<[f32; 6]>>,
    framebuffer: &mut Vec<Vec<Color>>,
    w: usize,
    h: usize,
) {
    for i in 0..h {
        for j in 0..w {
            let mut samples = lum_samples[i][j];
            apply_global_contrast(&mut samples);
            framebuffer[i][j] = match_char(&samples);
        }
    }
}
