pub mod entry;
pub mod utils;
//pub mod parser;
pub mod db_driver;
pub mod manager;

use clap::{Arg, ArgAction, Command};
use rusqlite::{params, Connection};

fn main() {
    let matches = Command::new("Bookmark Manager")
        .version("1.0")
        .author("Pavel")
        .about("CLI Bookmark Manager with encryption")
        .subcommand(Command::new("add")
            .about("Add a new bookmark")
            .arg(Arg::new("clipboard")
                .short('c')
                .long("clipboard")
                .action(ArgAction::SetTrue)))
        .subcommand(Command::new("search").about("Search bookmarks"))
        .subcommand(Command::new("edit")
            .about("Edit a bookmark")
            .arg(Arg::new("id").required(true)))
        .subcommand(Command::new("remove")
            .about("Remove a bookmark")
            .arg(Arg::new("id").required(true)))
        .subcommand(Command::new("clip")
            .about("Copy bookmark URL to clipboard"))
        .get_matches();

    let conn = Connection::open("bookmarks.db").expect("Failed to open database");
    manager::new(&conn);

    match matches.subcommand() {
        Some(("add", sub_m)) => manager::add(&conn, sub_m.get_flag("clipboard")),
        Some(("search", _)) => {
            if let Some(url) = manager::search(&conn) {
                println!("Selected URL: {}", url);
            }
        }
        Some(("edit", sub_m)) => {
            let id: i32 = sub_m.get_one::<String>("id").unwrap().parse().unwrap();
            manager::edit(&conn, id);
        }
        Some(("remove", sub_m)) => {
            let id: i32 = sub_m.get_one::<String>("id").unwrap().parse().unwrap();
            manager::remove(&conn, id);
        }
        Some(("clip", _)) => manager::clip(&conn),
        _ => eprintln!("Unknown command"),
    };
}
