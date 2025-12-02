use caretta_brain_core::util::RunnableCommand;
use clap::Args;

#[derive(Args)]
pub struct NoteOpenCommandArgs {
    config: ConfigOptionArgs
}

impl RunnableCommand for NoteOpenCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}