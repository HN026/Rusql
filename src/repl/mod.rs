use crate::meta_command::*;

use std::borrow::Cow::{ self, Borrowed, Owned };

use rustyline::error::ReadlineError;
use rustyline::highlight::{ Highlighter, MatchingBracketHighlighter };
use rustyline::hint::{ Hinter, HistoryHinter };
use rustyline::validate::{ Validator, ValidationContext, ValidationResult };
use rustyline::{ CompletionType, Config, Context, EditMode };
use rustyline_derive::{ Completer, Helper };

/// For now we have only one type of command, but we can add more in the future.
#[derive(Debug, PartialEq)]
pub enum CommandType {
    MetaCommand(MetaCommand),
}

/// Returns the type of command inputed in the REPL
pub fn get_command_type(input: &String) -> Result<CommandType, &'static str> {
    match input.starts_with(".") {
        true => Ok(CommandType::MetaCommand(MetaCommand::new(input.to_owned()))),
        false => Err("Invalid command type."),
    }
}

// REPL Helper struct with components for a Rust REPL
#[derive(Helper, Completer)]
pub struct REPLHelper {
    pub colored_prompt: String,
    pub history_hinter: HistoryHinter,
    pub highlighter: MatchingBracketHighlighter,
}

// Default implementation for REPLHelper
impl Default for REPLHelper {
    fn default() -> Self {
        Self {
            colored_prompt: "".to_owned(),
            history_hinter: HistoryHinter {},
            highlighter: MatchingBracketHighlighter::new(),
        }
    }
}

// Implementing the trait responsible for providing hints during input
impl Hinter for REPLHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        self.history_hinter.hint(line, pos, _ctx)
    }
}

// Implementing the trait responsible for highlighting user input
impl Highlighter for REPLHelper {
    // Takes the prompt and returns the highlighted version
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool
    ) -> Cow<'b, str> {
        if default { Borrowed(&self.colored_prompt) } else { Borrowed(prompt) }
    }

    // Takes the user input hint and returns the highlighted version (ANSI color)
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    // Takes the currently edited line with the cursor position and returns the highlighted version (with ANSI color).
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    // Indicates whether line needs to be highlighted when a specific char is typed or when cursor is moved under a specific char.
    // Used to optimize refresh when a character is inserted or the cursor is moved.
    fn highlight_char(&self, line: &str, pos: usize, hint: bool) -> bool {
        self.highlighter.highlight_char(line, pos, hint)
    }
}

// Implementing the trait responsible for validating user input
impl Validator for REPLHelper {
    // Takes the currently edited input and returns a ValidationResult indicating its validity along with an optional message.
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        use ValidationResult::{ Incomplete, Valid };
        let input = ctx.input();
        let result = if input.starts_with(".") {
            Valid(None) // Valid if input starts with a dot (.)
        } else if !input.ends_with(';') {
            Incomplete // Incomplete if input does not end with a semicolon (;)
        } else {
            Valid(None) // Valid if input meets all conditions
        };
        Ok(result)
    }
}

// Function to get the configuration for Rust REPL
pub fn get_config() -> Config {
    Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_command_type_meta_command_test() {
        let input = String::from(".help");
        let expected = CommandType::MetaCommand(MetaCommand::Help);

        let result = get_command_type(&input);
        assert_eq!(result, expected);
    }
}
