// Reference: https://exercism.org/tracks/rust/exercises/clock/solutions/bobahop

const MINUTES_PER_DAY: i32 = 24 * 60;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock{
    hour: i32,
    minute: i32,
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = ((hours * MINUTES_PER_HOUR) + minutes).rem_euclid(MINUTES_PER_DAY);
        let hour = total_minutes / MINUTES_PER_HOUR;
        let minute = total_minutes % MINUTES_PER_HOUR;
        
        Self {
            hour,
            minute,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:02?}:{:02?}", self.hour, self.minute)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hour, self.minute + minutes)
    }
}
