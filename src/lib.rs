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
//! ### Example
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
//! ### With custom config
//!
//! If you use a custom configuration for your benches and want to combine it with this crate, the [`decimal_byte_measurement`] will not
//! do, as it includes the default Criterion config. Instead, register the measurement with [`criterion::Criterion::with_measurement`]:
//!
//! #### Example
//!
//! ```
//! use core::time::Duration;
//! use criterion::{criterion_group, criterion_main};
//! use criterion_decimal_throughput::{Criterion, DecimalByteMeasurement};
//!
//! fn example_bench(c: &mut Criterion) {
//!     // ...
//! }
//!
//! // Your custom configuration would come here.
//! // As an example, we use a configuration that sets a non-default warm-up time of 10 seconds.
//! pub fn my_custom_config() -> Criterion {
//!     criterion::Criterion::default()
//!         .warm_up_time(Duration::from_secs(10))
//!         .with_measurement(DecimalByteMeasurement::new())
//! //      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ This enables the crate.
//! }
//! criterion_group!(
//!     name = example;
//!     config = my_custom_config();
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

        let (total_units, unit) = match *throughput {
            Bytes(bytes) => (bytes as f64, Byte),
            Elements(elements) => (elements as f64, Elem),
        };
        let units_per_second = total_units * (1e9 / typical_value);
        let multiple = if units_per_second >= 1e12 {
            Tera
        } else if units_per_second >= 1e9 {
            Giga
        } else if units_per_second >= 1e6 {
            Mega
        } else if units_per_second >= 1e3 {
            Kilo
        } else {
            One
        };
        let denominator = multiple.denominator();

        for val in values {
            let units_per_second = total_units * (1e9 / *val);
            *val = units_per_second / denominator;
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

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use Target::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Target {
        One,
        Kilo,
        Mega,
        Giga,
        Tera,
    }

    impl Target {
        fn get_base(self) -> f64 {
            match self {
                One => 1.0,
                Kilo => 1e3,
                Mega => 1e6,
                Giga => 1e9,
                Tera => 1e12,
            }
        }

        fn expected_bytes(self) -> &'static str {
            match self {
                One => " B/s",
                Kilo => "KB/s",
                Mega => "MB/s",
                Giga => "GB/s",
                Tera => "TB/s",
            }
        }

        fn expected_elems(self) -> &'static str {
            match self {
                One => " elem/s",
                Kilo => "Kelem/s",
                Mega => "Melem/s",
                Giga => "Gelem/s",
                Tera => "Telem/s",
            }
        }
    }

    fn arbitrary_target() -> impl Strategy<Value = Target> {
        prop_oneof![Just(One), Just(Kilo), Just(Mega), Just(Giga), Just(Tera)]
    }

    proptest! {
        #[test]
        fn scale_throughputs_bytes_gives_correct_unit(target in arbitrary_target(), bytes in any::<u64>()) {
            // bytes / seconds = target
            // seconds = bytes / target
            let thpt_config = Throughput::Bytes(bytes);
            let seconds = (bytes as f64) / target.get_base();
            let typical = (seconds * 1e9) * 0.999999;

            let measurement = DecimalByteMeasurement::default();
            let result = measurement.scale_throughputs(typical, &thpt_config, &mut []);

            assert_eq!(result, target.expected_bytes());
        }

        #[test]
        fn scale_throughputs_elems_gives_correct_unit(target in arbitrary_target(), elems in any::<u64>()) {
            // elems / seconds = target
            // seconds = elems / target
            let thpt_config = Throughput::Elements(elems);
            let seconds = (elems as f64) / target.get_base();
            let typical = (seconds * 1e9) * 0.999999;

            let measurement = DecimalByteMeasurement::default();
            let result = measurement.scale_throughputs(typical, &thpt_config, &mut []);

            assert_eq!(result, target.expected_elems());
        }
    }

    #[test]
    fn scale_throughputs_bytes() {
        let thpt_config = Throughput::Bytes(1_000_000);
        let typical = 1_000_000_000.0;
        let mut values = [
            100_000_000.0,
            500_000_000.0,
            999_999_999.0,
            1_000_000_000.0,
            1_000_000_001.0,
            2_000_000_000.0,
            10_000_000_000.0,
        ];

        let measurement = DecimalByteMeasurement::default();
        let result = measurement.scale_throughputs(typical, &thpt_config, &mut values);

        assert_eq!(result, "MB/s");
        assert_eq!(values, [10.0, 2.0, 1.000000001, 1.0, 0.999999999, 0.5, 0.1]);
    }

    #[test]
    fn scale_throughputs_elems_gives_correct_unit_regression1() {
        let elems = 13302377187617527;
        let target = Mega;

        let thpt_config = Throughput::Elements(elems);
        let seconds = (elems as f64) / target.get_base();
        let typical = (seconds * 1e9) * 0.999999;

        let measurement = DecimalByteMeasurement::default();
        let result = measurement.scale_throughputs(typical, &thpt_config, &mut []);

        assert_eq!(result, target.expected_elems());
    }
}
