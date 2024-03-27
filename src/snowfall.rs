use monty_carlos::sample::Sample;
use rand::distributions::Distribution;
use statrs::distribution::{LogNormal, Poisson};

/// An implementor of [`Sample`] that simulate the total amount of snow when the number of
/// snowfalls is distributed with a Poisson distribution and the amount of snow in each snowfall is
/// distributed with a Lognormal distribution.
pub(crate) struct Snowfall {
    num_distr: Poisson,
    amount_distr: LogNormal,
    snow_amounts: Vec<f64>,
}

impl Snowfall {
    pub(crate) fn new(
        average_snowfall_number: f64,
        average_log_snow_amount: f64,
        stdev_log_snow_amount: f64,
    ) -> Self {
        Self {
            num_distr: Poisson::new(average_snowfall_number).unwrap(),
            amount_distr: LogNormal::new(average_log_snow_amount, stdev_log_snow_amount).unwrap(),
            snow_amounts: Vec::new(),
        }
    }
}

impl Sample for Snowfall {
    fn generate(&mut self, rng: &mut dyn rand::RngCore) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let snowfall_count = self.num_distr.sample(rng) as usize;
        self.snow_amounts.clear();
        self.snow_amounts
            .extend(self.amount_distr.sample_iter(rng).take(snowfall_count));
    }

    fn evaluate(&self) -> f64 {
        self.snow_amounts.iter().sum()
    }
}
