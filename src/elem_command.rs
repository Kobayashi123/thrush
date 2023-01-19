use crate::{Feeder, ShellCore};
use nix::sys::wait;
use nix::unistd;
use nix::unistd::ForkResult;
use std::ffi::CString;
use std::process;

pub struct Command {
    text: String,
    args: Vec<String>,
    cargs: Vec<CString>,
}

impl Command {
    pub fn exec(&mut self, _core: &mut ShellCore) {
        if self.text == "exit\n" {
            process::exit(0);
        }

        match unsafe { unistd::fork() } {
            Ok(ForkResult::Child) => {
                let err = unistd::execvp(&self.cargs[0], &self.cargs);
                println!("Failed to execute. {:?}", err);
                process::exit(127);
            }
            Ok(ForkResult::Parent { child }) => {
                let _ = wait::waitpid(child, None);
            }
            Err(err) => panic!("Failed to fork. {}", err),
        }
    }

    pub fn parse(feeder: &mut Feeder, _core: &mut ShellCore) -> Option<Command> {
        let line = feeder.consume(feeder.remaining.len());
        let args: Vec<String> = line.trim_end().split(' ').map(|w| w.to_string()).collect();

        let cargs: Vec<CString> = args
            .iter()
            .map(|w| CString::new(w.clone()).unwrap())
            .collect();

        if args.len() > 0 {
            Some(Command {
                text: line,
                args: args,
                cargs: cargs,
            })
        } else {
            None
        }
    }
}
