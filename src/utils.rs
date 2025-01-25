use rand::distributions::uniform::SampleUniform;
use std::thread::sleep as std_sleep;
use std::time::Duration;
use rand::Rng;
use num::Num;


pub fn sleep(time: u64) {
    std_sleep(Duration::from_millis(time));
}

pub fn rand_int<T>(min: T, max: T) -> T 
where
    T: Num + Copy + SampleUniform + PartialOrd
{
    rand::thread_rng().gen_range(min..=max)
}
