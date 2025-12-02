mod chat;
mod config;
mod device;
mod note;
mod token;
mod init;

pub struct AppCommandArgs {
    command: AppSubcommand
}
#[derive(Subcommand)]
pub enum AppSubcommand {
    
}