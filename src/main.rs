use std::fs::read_to_string;

use clap::Parser;

mod todo;
use crate::todo::Todos;
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
