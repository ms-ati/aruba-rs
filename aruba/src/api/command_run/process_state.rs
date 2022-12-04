use std::io;
use std::process::{Child, Output};

#[derive(Debug, Default)]
pub enum ProcessState {
    Running(Child),
    Stopped(Output),
    #[default]
    Unknown,
}

impl ProcessState {
    pub fn wait_for_output(&mut self) -> io::Result<&Output> {
        // See https://stackoverflow.com/questions/68247811/is-there-a-safe-way-to-map-an-enum-variant-to-another-with-just-a-mutable-refere
        match std::mem::replace(self, Self::Unknown) {
            Self::Running(child) => *self = Self::Stopped(child.wait_with_output()?),
            s @ Self::Stopped(_) => *self = s,
            Self::Unknown => return Err(io::Error::new(io::ErrorKind::Other, "Unknown run state")),
        }

        match self {
            ProcessState::Stopped(ref output) => Ok(output),
            _ => unreachable!(),
        }
    }
}
