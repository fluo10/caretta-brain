use caretta_agent_core::util::RunnableCommand;
use clap::Args;

#[derive(Args)]
pub struct InitCommandArgs {
    invitation_token: Option<String>
}

impl RunnableCommand for InitCommandArgs {
    fn run(self, app_name: &'static str) {
        todo!()
    }
}