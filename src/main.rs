extern crate clap;
extern crate colored;
#[macro_use]
extern crate prettytable;
mod error;
mod meta_command;
mod repl;
mod replloop;
mod sql;
mod util;

use repl::{get_config, REPLHelper};
use replloop::run_repl_loop;
use sql::db::database::Database;
use util::intro::print_intro;

use rustyline::Editor;

use clap::{crate_authors, crate_description, crate_name, crate_version, Command};

fn main() -> rustyline::Result<()> {
    env_logger::init();

    let _matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let config = get_config();

    let helper = REPLHelper::default();

    let mut repl = match Editor::with_config(config) {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Error initializing editor: {}", e);
            return Err(From::from(e));
        }
    };

    repl.set_helper(Some(helper));

    if repl.load_history("history").is_err() {
        println!("No previous history.");
    }

    print_intro();

    let db = Database::new("tempdbase".to_string());

    run_repl_loop(repl, db)?;

    Ok(())
}
