use rand::distr::uniform::SampleUniform;
use std::thread::sleep as std_sleep;
use std::time::Duration;
use rand::Rng;
use num::Num;


pub fn sleep(time: u64) {
    std_sleep(Duration::from_millis(time));
}

pub fn random<T>(min: T, max: T) -> T 
where
    T: Num + Copy + SampleUniform + PartialOrd
{
    rand::rng().random_range(min..=max)
}
