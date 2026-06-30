pub const PREFERRED: [f64; 10] =
    [1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0];

pub const PREFERRED_WITH_MARGIN: [f64; 12] =
    [0.8, 1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];

pub fn margins_in_log_space() -> Vec<f64> {
    PREFERRED_WITH_MARGIN.map(f64::log10).to_vec()
}

#[derive(Clone, Copy, Debug)]
pub enum Boundaries {
    Lower,
    Mid,
    Upper,
}

impl Boundaries {
    pub fn calculate(&self) -> Vec<f64> {
        use Boundaries::*;

        match self {
            Lower => margins_in_log_space()
                .windows(2)
                .map(|window| window[0])
                .collect(),
            Mid => margins_in_log_space()
                .windows(2)
                .map(|window| (window[0] + window[1]) / 2.0)
                .collect(),
            Upper => margins_in_log_space()
                .windows(2)
                .map(|window| window[1].clone())
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Roots {
    #[cfg(feature = "f16")]
    Float16,
    Float32,
    Float64,
    #[cfg(feature = "f128")]
    Float128,
}

impl Roots {
    pub fn calculate(&self) -> Vec<String> {
        use Roots::*;

        match self {
            #[cfg(feature = "f16")]
            Float16 => {
                use crate::float16::constants::bounds;

                let five: f16 = 5.0;
                let least_power_of_five = bounds::smallest::EXPONENT - 1;
                let greatest_power_of_five = bounds::largest::EXPONENT + 1;
                let powers = least_power_of_five..=greatest_power_of_five;
                let floats = powers.map(|i| five.powi(i as i32));
                let ints = floats.map(|f| f.to_bits());

                ints.map(|u| format!("{u:#06x}")).collect()
            }
            Float32 => {
                use crate::float32::constants::bounds;

                let five: f32 = 5.0;
                let least_power_of_five = bounds::smallest::EXPONENT - 1;
                let greatest_power_of_five = bounds::largest::EXPONENT + 1;
                let powers = least_power_of_five..=greatest_power_of_five;
                let floats = powers.map(|i| five.powi(i as i32));
                let ints = floats.map(|f| f.to_bits());

                ints.map(|u| format!("{u:#010x}")).collect()
            }
            Float64 => {
                use crate::float64::constants::bounds;

                let five: f64 = 5.0;
                let least_power_of_five = bounds::smallest::EXPONENT - 1;
                let greatest_power_of_five = bounds::largest::EXPONENT + 1;
                let powers = least_power_of_five..=greatest_power_of_five;
                let floats = powers.map(|i| five.powi(i as i32));
                let ints = floats.map(|f| f.to_bits());

                ints.map(|u| format!("{u:#018x}")).collect()
            }
            #[cfg(feature = "f128")]
            Float128 => {
                use crate::float128::constants::bounds;

                let five: f128 = 5.0;
                let least_power_of_five = bounds::smallest::EXPONENT - 1;
                let greatest_power_of_five = bounds::largest::EXPONENT + 1;
                let powers = least_power_of_five..=greatest_power_of_five;
                let floats = powers.map(|i| five.powi(i as i32));
                let ints = floats.map(|f| f.to_bits());

                ints.map(|u| format!("{u:#034x}")).collect()
            }
        }
    }
}
