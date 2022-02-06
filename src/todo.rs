use crate::urgency::Urgency;

use serde::{Deserialize, Serialize};
use termcolor;

use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Todos {
    pub tasks: Vec<Todo>,
}

#[derive(Deserialize, Serialize)]
pub struct Todo {
    urg: Urgency,
    name: String,
    project: String,
    description: String,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.urg == other.urg
    }
}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.urg.partial_cmp(&other.urg)
    }
}
