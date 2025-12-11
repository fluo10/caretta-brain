use caretta_brain::args::ConfigOptionArgs;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    config: ConfigOptionArgs
}

fn main() {
    let args = Cli::parse();
    print!("{:?}", args)
}