use monty_carlos::sample::{
    fitting::{DistributionFit, NormalFit},
    KSSample, LillieforsSample,
};
use serde::{Deserialize, Serialize};

use crate::snowfall::Snowfall;

/// An enum that describes possible simulation that can be executed
#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub(crate) enum Simulation {
    #[serde(rename = "Kolmogorov_Smirnov_test")]
    KSTest {
        samples: usize,
        distribution: DistrDescription,
    },
    #[serde(rename = "Lilliefors_test")]
    LillieforsTest {
        samples: usize,
        generating_distribution: DistrDescription,
        distribution_to_compare: Fit,
    },
    Snowfall {
        average_snowfall_number: f64,
        average_log_snow_amount: f64,
        stdev_log_snow_amount: f64,
    },
}

impl Simulation {
    pub(crate) fn generate_sample(&self) -> BatchSample {
        match self {
            Simulation::KSTest {
                samples,
                distribution,
            } => BatchSample::KS(KSSample::new(distribution.distribution(), *samples).unwrap()),
            Simulation::LillieforsTest {
                samples,
                generating_distribution,
                distribution_to_compare,
            } => BatchSample::Lilliefors(
                LillieforsSample::new(
                    generating_distribution.distribution(),
                    *samples,
                    *distribution_to_compare,
                )
                .unwrap(),
            ),
            Simulation::Snowfall {
                average_snowfall_number,
                average_log_snow_amount,
                stdev_log_snow_amount,
            } => BatchSample::Snowfall(Snowfall::new(
                *average_snowfall_number,
                *average_log_snow_amount,
                *stdev_log_snow_amount,
            )),
        }
    }
}

pub(crate) enum BatchSample {
    KS(KSSample<BatchDistribution>),
    Lilliefors(LillieforsSample<BatchDistribution, Fit>),
    Snowfall(Snowfall),
}

macro_rules! impl_sample_for_batch_sample {
    ($($constr:ident),+) => {
        impl monty_carlos::sample::Sample for crate::input::simulation::BatchSample {
            fn evaluate(&self) -> f64 {
                match self {
                    $(crate::input::simulation::BatchSample::$constr(sample) => sample.evaluate()),+
                }
            }

            fn generate(&mut self, rng: &mut dyn rand::RngCore) {
                match self {
                    $(crate::input::simulation::BatchSample::$constr(sample) => sample.generate(rng)),+
                }
            }
        }

    };
}

impl_sample_for_batch_sample!(KS, Lilliefors, Snowfall);

/// An enum of descriptions possible distributions for Kolmogorov-Smirnov and Lilliefors tests
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum DistrDescription {
    Normal { mean: f64, stdev: f64 },
}

impl DistrDescription {
    /// Converts the description into the corresponding probability distribution.
    fn distribution(&self) -> BatchDistribution {
        match self {
            DistrDescription::Normal { mean, stdev } => {
                BatchDistribution::Normal(statrs::distribution::Normal::new(*mean, *stdev).unwrap())
            }
        }
    }
}

/// Implements the traits that are required for [`BatchDistribution`] to be used in [`KSSample`]
/// and [`LillieforsSample`].
macro_rules! distribution_traits {
    ($($constr:ident),+) => {

        impl rand::distributions::Distribution<f64> for crate::input::simulation::BatchDistribution {
            fn sample<R>(&self, rng: &mut R) -> f64 where R: rand::Rng + ?Sized {
                match self {
                    $(crate::input::simulation::BatchDistribution::$constr(distr) => distr.sample(rng)),+
                }
            }
        }

        impl statrs::statistics::Min<f64> for crate::input::simulation::BatchDistribution {
            fn min(&self) -> f64 {
                match self {
                    $(crate::input::simulation::BatchDistribution::$constr(distr) => distr.min()),+
                }
            }
        }

        impl statrs::statistics::Max<f64> for crate::input::simulation::BatchDistribution {
            fn max(&self) -> f64 {
                match self {
                    $(crate::input::simulation::BatchDistribution::$constr(distr) => distr.max()),+
                }
            }
        }

        impl statrs::distribution::ContinuousCDF<f64, f64> for crate::input::simulation::BatchDistribution {
            fn cdf(&self, x: f64) -> f64 {
                match self {
                    $(crate::input::simulation::BatchDistribution::$constr(distr) => distr.cdf(x)),+
                }
            }

            fn sf(&self, x: f64) -> f64 {
                match self {
                    $(crate::input::simulation::BatchDistribution::$constr(distr) => distr.sf(x)),+
                }
            }
        }
    };
}

distribution_traits!(Normal);

/// A coproduct of probability distribution described by [`DistrDescription`].
pub(crate) enum BatchDistribution {
    Normal(statrs::distribution::Normal),
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
