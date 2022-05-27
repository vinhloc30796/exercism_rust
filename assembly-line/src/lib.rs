// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR: u32 = 221;

pub fn success_rate(speed: u8) -> f64 {
    //! Calculate the success rate of production with a given speed, as follows:
    //! 
    //! - 0: 0%
    //! - 1 to 4: 100% success rate.
    //! - 5 to 8: 90% success rate.
    //! - 9 and 10: 77% success rate.    
    match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => panic!("Invalid speed: {}, has to be between 0 and 10", speed),
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    //! Calculate the production rate per hour, depending on the success rate.
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
    let success_rate = success_rate(speed);
    success_rate * (speed as f64) * (CARS_PER_HOUR as f64)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    //! Calculate the number of working items per minute, 
    //! depending on the production rate per hour.
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    let production_rate_per_hour = production_rate_per_hour(speed);
    (production_rate_per_hour / 60.0) as u32
}
