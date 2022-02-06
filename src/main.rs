use std::fs::read_to_string;

use clap::Parser;

mod todo;
use crate::todo::{Todos, Todo};
mod urgency;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    all: bool,

    #[clap(short = 'l', long)]
    list: bool,

    #[clap(short = 'a', long)]
    add: Vec<String>,

    #[clap(short = 'r', long)]
    remove: Vec<String>,

    #[clap(short = 'u', long)]
    update: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut todos_file = home::home_dir().unwrap();
    todos_file.push(".config/adg/todos.json");
    let mut tds: Todos =
        serde_json::from_str(&mut read_to_string(&mut todos_file).expect("Missing todos file."))
            .expect("Malformed todos file");

    if args.add.len() != 0 {
        if args.add.len() < 4 {
            panic!("Not enough information about new todo!");
        }
        tds.tasks.push(Todo::new(
            args.add[0],
            args.add[1],
            args.add[2],
            args.add[3],
        ));
    }
    
    if args.remove.len() != 0 {
        for r in args.remove {
            tds.remove(r);
        }
    }

    if args.add.len() + args.remove.len() + args.update.len() != 0 {
        save_todos(&tds);
    }
    if args.all {
        pretty_print();
    } else if args.list {
        basic_print();
    }
}

fn save_todos(tds: &Todos) {
    todo!()
}

fn basic_print() {
    todo!()
}

fn pretty_print() {
    todo!()
}
