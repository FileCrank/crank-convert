// Debugging code, formatters, etc.

use std::fmt::{Display, Formatter};
use crate::conversions::convertable::{ConversionSequence, ConversionStep, Convertable};

impl Display for dyn Convertable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.get_name())
    }
}

impl Display for ConversionStep {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} -> {}", self.from, self.to)
    }
}

impl Display for ConversionSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Conversion from {} to {}, in {} steps", self.start, self.end, self.steps.len());
        writeln!(f, "Conversion path: {}", self.steps.join(","))
    }
}