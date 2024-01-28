extern crate clap;
extern crate colored;
#[macro_use]
mod error;
mod repl;
mod meta_command;

use repl::{ get_command_type, get_config, REPLHelper, CommandType };
use meta_command::handle_meta_command;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use colored::*;

use clap::{ crate_authors, crate_description, crate_name, crate_version, Command };

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

    println!("{} - {}\n{}", crate_name!(), crate_version!(), "Developed by Huzaifa Naseer.");

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
                    Ok(CommandType::MetaCommand(cmd)) => {
                        let _ = match handle_meta_command(cmd, &mut repl) {
                            Ok(msg) => println!("{}", msg),
                            Err(err) => eprintln!("An error occured: {}", err),
                        };
                    }
                    Err(e) => {
                        eprintln!("Failed to get command type: {}", e);
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
