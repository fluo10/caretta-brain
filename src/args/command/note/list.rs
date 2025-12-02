use caretta_brain_core::util::RunnableCommand;
use clap::Args;

#[derive(Args)]
pub struct NoteListCommandArgs {
    config: ConfigOptionArgs
}

impl RunnableCommand for NoteListCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}