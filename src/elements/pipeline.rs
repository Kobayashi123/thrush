use super::command;
use crate::{Feeder, ShellCore};
use crate::elements::command::Command;

#[derive(Debug)]
pub struct Pipeline {
    pub commands: Vec<Box<dyn Command>>,
    pub text: String,
}

impl Pipeline {
    pub fn exec(&mut self, core: &mut ShellCore) {
        for command in self.commands.iter_mut() {
            command.exec(core);
        }
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<Pipeline> {
        if let Some(command) = command::parse(feeder, core) {
            return Some(Pipeline {
                text: command.get_text(),
                commands: vec!(command),
            });
        }
        None
    }
}
