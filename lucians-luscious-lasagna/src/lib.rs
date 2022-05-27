// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]


pub fn expected_minutes_in_oven() -> i32 {
    //! Returns 40
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    //!  Takes the actual minutes in the oven and returns the remaining minutes
    let exp = expected_minutes_in_oven();
    exp - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    //! Takes in the number of lasagna layers 
    //! and returns the time in minutes it will take to prepare the lasagna
    //! assuming each layer takes 2 minutes to prepare
    /* unimplemented!(
        "calculate preparation time in minutes for number of layers: {}",
        number_of_layers
    )
    */
    number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    //! Takes in the number of lasagna layers and the actual minutes in the oven
    //! and returns the minutes spent cooking the lasagna
    //! which is the sum of prep time & the time the lasagna is in the oven
    /* unimplemented!(
        "calculate elapsed time in minutes for number of layers {} and actual minutes in oven {}",
        number_of_layers,
        actual_minutes_in_oven
    ) */
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
