pub mod steps;


pub mod text {
    pub fn sanitize_text(text: &str) -> String {
        //DO WE NEED THESE?
        //let text = unescape_text(text);
        //let text = extract_text(text) if aruba.config.remove_ansi_escape_sequences
        text.trim_end().to_owned()
    }
}

pub mod api {
    pub mod command {
        use std::process::Command;
        use crate::text::sanitize_text;

        pub fn run_and_wait(command: &str) -> Command {
            let sanitized_command = sanitize_text(command);
            let mut cmd = Command::new("sh");
            cmd
                .arg("-c")
                .arg(sanitized_command);
            cmd
                // .output()
                // .expect("failed to execute process")
        }
    }
}

pub mod cucumber_world {
    use std::process::Command;

    #[derive(Debug, Default)]
    pub struct ArubaWorldState {
        last_command_run: Option<Command>
    }

    impl ArubaWorldState {
        pub fn when_run_and_wait(&mut self, command: &str) {
            let command_run = crate::api::command::run_and_wait(command);
            self.last_command_run = Some(command_run);
        }
    }
}

pub mod prelude {
    pub use crate::steps::parameters::*;
    pub use super::cucumber_world::*;
    pub use cucumber::{given, then, when, World};
}
