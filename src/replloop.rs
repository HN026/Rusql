use crate::meta_command::handle_meta_command;
use crate::repl::{get_command_type, CommandType, REPLHelper};
use crate::sql::db::database::Database;
use crate::sql::process_command;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{history::FileHistory, Editor};

pub fn run_repl_loop(
    mut repl: Editor<REPLHelper, FileHistory>,
    mut db: Database,
) -> rustyline::Result<()> {
    loop {
        let p = "RUSQL>> ".yellow().bold();
        repl.helper_mut().expect("No helper found").colored_prompt =
            format!("\x1b[1;32m{}\x1b[0m", p);

        let input = repl.readline(&p);
        match input {
            Ok(command) => {
                if let Err(e) = repl.add_history_entry(command.as_str()) {
                    eprintln!("Failed to add history entry: {}", e);
                }
                match get_command_type(&command.trim().to_owned()) {
                    CommandType::MetaCommand(cmd) => {
                        let _ = match handle_meta_command(cmd, &mut repl) {
                            Ok(msg) => println!("{}", msg),
                            Err(err) => eprintln!("An error occured: {}", err),
                        };
                    }
                    CommandType::SQLCommand(_cmd) => {
                        let _ = match process_command(&command, &mut db) {
                            Ok(msg) => println!("{}", msg),
                            Err(err) => eprintln!("An error occured: {}", err),
                        };
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    if let Err(err) = repl.save_history("history") {
        println!("Error saving history: {:?}", err);
    }

    Ok(())
}
