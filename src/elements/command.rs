pub mod simple;
pub mod paren;

use crate::Feeder;
use crate::ShellCore;
use std::fmt;
use std::fmt::Debug;
use self::paren::ParenCommand;
use self::simple::SimpleCommand;

impl Debug for dyn Command {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("COMMAND").finish()
    }
}

pub trait Command {
    fn exec(&mut self, core: &mut ShellCore);
    fn get_text(&self) -> String;
}

pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<Box<dyn Command>> {
    if let Some(a) = ParenCommand::parse(feeder, core) { Some(Box::new(a)) }
    else if let Some(a) = SimpleCommand::parse(feeder, core) { Some(Box::new(a)) }
    else { None }
}
