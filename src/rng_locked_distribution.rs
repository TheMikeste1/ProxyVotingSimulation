use rand::distributions::DistIter;
use rand::distributions::Distribution;
use rand::rngs::StdRng;
use rand::Rng;

// Source: https://users.rust-lang.org/t/vec-of-rand-distribution-trait-objects/58727/2
pub trait RngLockedDistribution<T> {
    type R: Rng + ?Sized;
    fn sample(&self, rng: &mut Self::R) -> T;
    fn sample_iter(self, rng: Self::R) -> DistIter<Self, Self::R, T>
    where
        Self::R: Sized,
        Self: Sized;
}

impl<D, T> RngLockedDistribution<T> for D
where
    D: Distribution<T>,
{
    type R = StdRng;

    fn sample(&self, rng: &mut Self::R) -> T {
        <Self as Distribution<T>>::sample(self, rng)
    }

    fn sample_iter(self, rng: Self::R) -> DistIter<Self, Self::R, T>
    where
        Self::R: Sized,
        Self: Sized,
    {
        <Self as Distribution<T>>::sample_iter(self, rng)
    }
}