#[allow(unused)]
use rand::distributions::{Alphanumeric, DistString};
#[allow(unused)]
use rand::{Rng, SeedableRng, rngs::StdRng};
#[allow(unused)]
use rand_pcg::Pcg32;

#[cfg(test)]
pub fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), length)
}

#[cfg(test)]
pub fn random_number() -> i32 {
    let mut rng = Pcg32::seed_from_u64(42);
    let random_number: i32 = rng.gen_range(20..1000);
    random_number
}