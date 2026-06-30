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
