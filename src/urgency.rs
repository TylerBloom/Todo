use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Urgency {
    Urgent,
    Upcoming,
    Working,
    Backburner,
    Future,
}
