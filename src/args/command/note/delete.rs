use caretta_brain_core::util::RunnableCommand;
use clap::Args;

#[derive(Args)]
pub struct NoteDeleteCommandArgs {
    config: ConfigOptionArgs
}

impl RunnableCommand for NoteDeleteCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}