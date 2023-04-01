use super::job::Job;
use crate::{Feeder, ShellCore};

pub struct Script {
    pub jobs: Vec<Job>,
    pub text: String,
}

impl Script {
    pub fn exec(&mut self, core: &mut ShellCore) {
        for job in self.jobs.iter_mut() {
            job.exec(core);
        }
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<Script> {
        if let Some(job) = Job::parse(feeder, core) {
            return Some(Script {
                text: job.text.clone(),
                jobs: vec![job],
            });
        }
        None
    }
}
