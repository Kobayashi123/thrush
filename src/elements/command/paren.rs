use crate::{ShellCore, Feeder, Script};
use super::Command;

#[derive(Debug)]
pub struct ParenCommand {
    pub text: String,
    pub script: Option<Script>,
}

impl Command for ParenCommand {
    fn exec(&mut self, _: &mut ShellCore) {}
    fn get_text(&self) -> String { self.text.clone() }
}

impl ParenCommand {
    pub fn parse(_: &mut Feeder, _: &mut ShellCore) -> Option<ParenCommand> {
        eprintln!("ParenCommand::parse");
        None
    }
}
