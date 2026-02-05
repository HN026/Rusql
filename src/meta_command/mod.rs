//! Meta-command handling for REPL control (.help, .exit, .cls).

use crate::error::{RUSQLError, Result};
use crate::repl::REPLHelper;
use rustyline::history::FileHistory;
use rustyline::Editor;
use std::fmt;
use std::process::Command as ProcessCommand;

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
    Exit,
    Help,
    Cls,
    Unknown,
}

impl fmt::Display for MetaCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetaCommand::Exit => f.write_str(".exit"),
            MetaCommand::Help => f.write_str(".help"),
            MetaCommand::Cls => f.write_str(".cls"),
            MetaCommand::Unknown => f.write_str("Unknown command"),
        }
    }
}

impl MetaCommand {
    pub fn new(command: String) -> MetaCommand {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        match cmd.as_ref() {
            ".exit" => MetaCommand::Exit,
            ".help" => MetaCommand::Help,
            ".cls" => MetaCommand::Cls,
            _ => MetaCommand::Unknown,
        }
    }
}

pub fn handle_meta_command(
    command: MetaCommand,
    repl: &mut Editor<REPLHelper, FileHistory>,
) -> Result<String> {
    match command {
        MetaCommand::Exit => {
            repl.append_history("history").unwrap();
            std::process::exit(0);
        }
        MetaCommand::Help => Ok(format!(
            "{}{}{}{}{}{}",
            "Special Commands: \n",
            ".exit: Exit the REPL\n",
            ".help: Display this help message\n",
            ".cls:  Clear the screen\n",
            "RUSQL is a simple SQL database engine written in Rust.\n",
            "Version: 0.1.0\n"
        )),
        MetaCommand::Cls => {
            if cfg!(target_os = "windows") {
                ProcessCommand::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .unwrap();
            } else {
                ProcessCommand::new("clear").status().unwrap();
            }
            Ok("".to_string())
        }
        MetaCommand::Unknown => Err(RUSQLError::UnknownCommand(format!(
            "Unknown command or Invalid syntax. Type .help for more information."
        ))),
    }
}
