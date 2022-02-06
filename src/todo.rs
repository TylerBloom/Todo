use crate::urgency::{string_to_urgency, Urgency};

use serde::{Deserialize, Serialize};
use termcolor;

use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Todos {
    pub tasks: Vec<Todo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Todo {
    urg: Urgency,
    name: String,
    project: String,
    description: String,
}

impl Todos {
    pub fn find(&self, name: String) -> Option<Todo> {
        let mut digest: Option<Todo> = None;
        for t in self.tasks {
            if t.name == name {
                digest = Some(t.clone());
                break;
            }
        }
        digest
    }

    pub fn remove(&mut self, name: String) {
        match self.tasks.iter().position(|t| t.name == name) {
            Some(i) => { self.tasks.remove(i); },
            None => (),
        }
    }
}

impl Todo {
    pub fn new(urg_str: String, name: String, project: String, description: String) -> Self {
        Todo {
            urg: string_to_urgency(&urg_str),
            name,
            project,
            description,
        }
    }
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
