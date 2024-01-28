use crate::error::{ Result, RUSQLError };
use std::process::Command as ProcessCommand;
use crate::repl::REPLHelper;
use rustyline::Editor;
use rustyline::history::FileHistory;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
    Exit,
    Help,
    Cls,
    Unknown,
}

// Trait responsible for translating type into a formated text.
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
    repl: &mut Editor<REPLHelper, FileHistory>
) -> Result<String> {
    match command {
        MetaCommand::Exit => {
            repl.append_history("history").unwrap();
            std::process::exit(0);
        }
        MetaCommand::Help =>
            Ok(
                format!(
                    "{}{}{}{}{}{}",
                    "Special Commands: \n",
                    ".exit: Exit the REPL\n",
                    ".help: Display this help message\n",
                    ".cls:  Clear the screen\n",
                    "RUSQL is a simple SQL database engine written in Rust.\n",
                    "Version: 0.1.0\n"
                )
            ),
        MetaCommand::Cls => {
            if cfg!(target_os = "windows") {
                ProcessCommand::new("cmd").args(&["/C", "cls"]).status().unwrap();
            } else {
                ProcessCommand::new("clear").status().unwrap();
            }
            Ok("".to_string())
        }
        MetaCommand::Unknown =>
            Err(
                RUSQLError::UnknownCommand(
                    format!("Unknown command or Invalid syntax. Type .help for more information.")
                )
            ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repl::{ get_config, REPLHelper };

    #[test]
    fn get_meta_command_exit_test() {
        let config = get_config();

        let helper = REPLHelper::default();

        let mut repl = Editor::with_config(config);
        repl.set_helper(Some(helper));

        let inputed_command = MetaCommand::Help;

        let result = handle_meta_command(inputed_command, &mut repl);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn get_meta_command_cls_test() {
        let config = get_config();

        let helper = REPLHelper::default();

        let mut repl = Editor::with_config(config);
        repl.set_helper(Some(helper));

        let inputed_command = MetaCommand::Cls;

        let result = handle_meta_command(inputed_command, &mut repl);

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn get_meta_command_unknown_command_test() {
        let config = get_config();

        let helper = REPLHelper::default();

        let mut repl = Editor::with_config(config);
        repl.set_helper(Some(helper));

        let inputed_command = MetaCommand::Unknown;

        let result = handle_meta_command(inputed_command, &mut repl);

        assert_eq!(result, Ok);
    }

    #[test]
    fn meta_command_display_trait_test() {
        let exit = MetaCommand::Exit;
        let help = MetaCommand::Help;
        let cls = MetaCommand::Cls;
        let unknown = MetaCommand::Unknown;

        assert_eq!(format!("{}", exit), ".exit");
        assert_eq!(format!("{}", help), ".help");
        assert_eq!(format!("{}", cls), ".cls");
        assert_eq!(format!("{}", unknown), "Unknown Command");
    }
}
