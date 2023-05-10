mod action;
mod test;

use action::Action;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Test { name, count } => {
            test::perform(count, &name)
        },
        Action::Hoge { fuga } => {
            println!("Hoge {}", fuga)
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    // let args = Args::parse();
    Args::command().debug_assert()
}
