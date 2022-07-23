//! Throughput measurement for criterion.rs using decimal multiple-byte units.
//!
//! By default, using [criterion.rs throughput measurement](https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html#throughput-measurements)
//! gives results in binary multiple-byte units, so KiB/s, MiB/s, etc. Some people, like me, prefer
//! to use the more intuitive decimal multiple-byte units of KB/s, MB/s, and so on. This crate enables that.
//!
//! ## Usage
//!
//! You need to:
//!
//! 1. Use the custom measurement type [`criterion_decimal_throughput::Criterion`](Criterion) from this crate,
//! exposed with the [`decimal_byte_measurement`] function.
//! 2. Enable throughput measurement in the benchmark group with [`criterion::BenchmarkGroup::throughput`].
//!
//! ## Example
//!
//! ```
//! use criterion::{criterion_group, criterion_main};
//! use criterion_decimal_throughput::{Criterion, decimal_byte_measurement};
//!
//! fn example_bench(c: &mut Criterion) {
//!     let mut group = c.benchmark_group("example_name");
//!     group.throughput(criterion::Throughput::Bytes(/* Your input size here */ 1_000_000u64));
//!
//!     // Add your benchmarks to the group here...
//!
//!     group.finish();
//! }
//!
//! criterion_group!(
//!     name = example;
//!     config = decimal_byte_measurement();
//!     targets = example_bench
//! );
//! criterion_main!(example);
//! ```
//!
//! ## Origin
//!
//! Related criterion.rs issue: <https://github.com/bheisler/criterion.rs/issues/581>.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(
    explicit_outlives_requirements,
    unreachable_pub,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    unused_lifetimes
)]

use criterion::{
    measurement::{Measurement, ValueFormatter, WallTime},
    Throughput,
};

/// Measurement type for decimal multiple-byte units.
pub struct DecimalByteMeasurement(WallTime);

/// Shorthand for the criterion manager with [`DecimalByteMeasurement`].
pub type Criterion = criterion::Criterion<DecimalByteMeasurement>;

/// Construct a default [`criterion::Criterion`] manager with [`DecimalByteMeasurement`].
pub fn decimal_byte_measurement() -> Criterion {
    criterion::Criterion::default().with_measurement(DecimalByteMeasurement::new())
}

impl Default for DecimalByteMeasurement {
    fn default() -> Self {
        Self::new()
    }
}

impl DecimalByteMeasurement {
    /// Create a new [`DecimalByteMeasurement`] struct.
    pub fn new() -> Self {
        DecimalByteMeasurement(WallTime)
    }
}

impl Measurement for DecimalByteMeasurement {
    type Intermediate = <WallTime as Measurement>::Intermediate;

    type Value = <WallTime as Measurement>::Value;

    fn start(&self) -> Self::Intermediate {
        self.0.start()
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        self.0.end(i)
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        self.0.add(v1, v2)
    }

    fn zero(&self) -> Self::Value {
        self.0.zero()
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        self.0.to_f64(value)
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Multiple {
    One,
    Kilo,
    Mega,
    Giga,
    Tera,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Unit {
    Byte,
    Elem,
}

impl Multiple {
    fn denominator(&self) -> f64 {
        match *self {
            Multiple::One => 1.0,
            Multiple::Kilo => 1_000.0,
            Multiple::Mega => 1_000_000.0,
            Multiple::Giga => 1_000_000_000.0,
            Multiple::Tera => 1_000_000_000_000.0,
        }
    }
}

impl ValueFormatter for DecimalByteMeasurement {
    fn scale_values(&self, typical_value: f64, values: &mut [f64]) -> &'static str {
        self.0.formatter().scale_values(typical_value, values)
    }

    fn scale_throughputs(
        &self,
        typical_value: f64,
        throughput: &criterion::Throughput,
        values: &mut [f64],
    ) -> &'static str {
        use Multiple::*;
        use Throughput::*;
        use Unit::*;

        let (value, unit) = match *throughput {
            Bytes(bytes) => (bytes as f64, Byte),
            Elements(elements) => (elements as f64, Elem),
        };
        let per_second = value * (1e9 / typical_value);
        let multiple = if per_second >= 1e12 {
            Tera
        } else if per_second >= 1e9 {
            Giga
        } else if per_second >= 1e6 {
            Mega
        } else if per_second >= 1e3 {
            Kilo
        } else {
            One
        };
        let denominator = multiple.denominator();

        for val in values {
            let per_second = value * (1e9 / *val);
            *val = per_second / denominator;
        }

        match (unit, multiple) {
            (Byte, One) => " B/s",
            (Byte, Kilo) => "KB/s",
            (Byte, Mega) => "MB/s",
            (Byte, Giga) => "GB/s",
            (Byte, Tera) => "TB/s",
            (Elem, One) => " elem/s",
            (Elem, Kilo) => "Kelem/s",
            (Elem, Mega) => "Melem/s",
            (Elem, Giga) => "Gelem/s",
            (Elem, Tera) => "Telem/s",
        }
    }

    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str {
        self.0.formatter().scale_for_machines(values)
    }
}
