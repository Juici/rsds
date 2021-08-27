use std::fmt;

// Though not technically correct, follow convention and use decimal scale
// notation with a binary divisor.
const DIVISOR: f64 = 1024.0;
static SCALE: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

/// A format utility for human readable file sizes.
#[derive(Clone, Copy, Debug)]
pub struct FileSize(pub usize);

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut size = self.0 as f64;
        let mut scale_idx = 0;

        while size >= DIVISOR {
            size /= DIVISOR;
            scale_idx += 1;
        }

        let scale = SCALE[scale_idx];

        if f64_eq(size.fract(), 0.0) {
            write!(f, "{:.00} {}", size, scale)
        } else {
            write!(f, "{:.02} {}", size, scale)
        }
    }
}

fn f64_eq(left: f64, right: f64) -> bool {
    left == right || (left - right).abs() <= f64::EPSILON
}
