use std::fmt;

const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];

#[derive(Clone, Copy)]
pub struct HumanBytes(pub u64);

impl fmt::Display for HumanBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 1000 {
            return write!(f, "{} B", self.0);
        }

        let mut value = self.0 as f64 / 1000.0;
        let mut unit = 0;

        while value >= 1000.0 && unit < UNITS.len() - 1 {
            value /= 1000.0;
            unit += 1;
        }

        write!(f, "{value:.1} {}", UNITS[unit])
    }
}
