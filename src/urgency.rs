
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Urgency {
    Urgent,
    Upcoming,
    Working,
    Backburner,
    Future,
}

pub fn string_to_urgency(urg_str: &str) -> Urgency {
    match urg_str {
        "Urgent" => Urgency::Urgent,
        "Upcoming" => Urgency::Upcoming,
        "Working" => Urgency::Working,
        "Backburner" => Urgency::Backburner,
        "Future" => Urgency::Future,
        _ => panic!("Unknown Urgency!")
    }
}
