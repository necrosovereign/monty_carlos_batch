use monty_carlos::sample::{
    fitting::{DistributionFit, NormalFit},
    KSSample, LillieforsSample, Sample,
};
use serde::{Deserialize, Serialize};

/// An enum that describes possible simulation that can be executed
#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub(crate) enum Simulation {
    #[serde(rename = "Kolmogorov_Smirnov_test")]
    KSTest {
        samples: usize,
        distribution: Distribution,
    },
    #[serde(rename = "Lilliefors_test")]
    LillieforsTest {
        samples: usize,
        generating_distribution: Distribution,
        distribution_to_compare: Fit,
    },
}

impl Simulation {
    fn generate_sample(&self) -> Box<dyn Sample> {
        match self {
            Simulation::KSTest {
                samples,
                distribution,
            } => Box::new(KSSample::new(*distribution, *samples).unwrap()),
            Simulation::LillieforsTest {
                samples,
                generating_distribution,
                distribution_to_compare,
            } => Box::new(
                LillieforsSample::new(*generating_distribution, *samples, *distribution_to_compare)
                    .unwrap(),
            ),
        }
    }
}

/// An enum of possible distributions for Kolmogorov-Smirnov and Lilliefors tests
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum Distribution {
    Normal { mean: f64, stdev: f64 },
}

impl rand::distributions::Distribution<f64> for Distribution {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        match self {
            Distribution::Normal { mean, stdev } => {
                statrs::distribution::Normal::new(*mean, *stdev)
                    .unwrap()
                    .sample(rng)
            }
        }
    }
}

impl statrs::statistics::Max<f64> for Distribution {
    fn max(&self) -> f64 {
        match self {
            Distribution::Normal { mean, stdev } => {
                statrs::distribution::Normal::new(*mean, *stdev)
                    .unwrap()
                    .max()
            }
        }
    }
}

impl statrs::statistics::Min<f64> for Distribution {
    fn min(&self) -> f64 {
        match self {
            Distribution::Normal { mean, stdev } => {
                statrs::distribution::Normal::new(*mean, *stdev)
                    .unwrap()
                    .min()
            }
        }
    }
}

impl statrs::distribution::ContinuousCDF<f64, f64> for Distribution {
    fn cdf(&self, x: f64) -> f64 {
        match self {
            Distribution::Normal { mean, stdev } => {
                statrs::distribution::Normal::new(*mean, *stdev)
                    .unwrap()
                    .cdf(x)
            }
        }
    }

    fn sf(&self, x: f64) -> f64 {
        match self {
            Distribution::Normal { mean, stdev } => {
                statrs::distribution::Normal::new(*mean, *stdev)
                    .unwrap()
                    .sf(x)
            }
        }
    }
}

/// An enum of possible distribution kinds that Lilliefors test will fit to generated datasets
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub(crate) enum Fit {
    Normal,
}

impl DistributionFit for Fit {
    type Distr = statrs::distribution::Normal;

    fn fit(&self, samples: &[f64]) -> Self::Distr {
        match self {
            Fit::Normal => NormalFit.fit(samples),
        }
    }
}
