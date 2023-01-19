use crate::{Feeder, ShellCore};
use nix::unistd::execvp;
use std::ffi::CString;
use std::process;

pub struct Command {
    pub text: String,
}

impl Command {
    pub fn exec(&mut self, _core: &mut ShellCore) {
        //引数_coreはまだ使いません
        if self.text == "exit\n" {
            process::exit(0);
        }

        let mut words: Vec<CString> = vec![];
        for w in self.text.trim_end().split(' ') {
            words.push(CString::new(w.to_string()).unwrap());
        }
        print!("{:?}", words);

        if words.len() > 0 {
            println!("{:?}", execvp(&words[0], &words));
        }
    }

    pub fn parse(feeder: &mut Feeder, _core: &mut ShellCore) -> Option<Command> {
        let line = feeder.consume(feeder.remaining.len());
        Some(Command { text: line })
    }
}
