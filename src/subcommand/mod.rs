use clap::Subcommand;

mod chat;


pub struct AppCommandArgs {
    command: AppSubcommand
}
#[derive(Subcommand)]
pub enum AppSubcommand {
    
}