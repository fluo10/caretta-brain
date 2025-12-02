mod delete;
mod list;
mod open;

pub use delete::NoteDeleteCommandArgs;
pub use list::NoteListCommandArgs;
pub use open::NoteOpenCommandArgs;

#[derive(Args)]
pub struct NoteCommandArgs {
    #[command(flatten)]
    subcommand: NoteSubcommand
}

#[derive(Subcommand)]
pub enum NoteSubcommand {
    Delete(NoteDeleteCommandArgs),
    List(NoteListCommandArgs),
    Open(NoteOpenCommandArgs)
}