use clap::Parser;
use lambdalisp::action::run;

// use lambdalisp::common::fileinfo::CompileError;
// use lambdalisp::corelang::printer::simple::SimplePrinter;
// use lambdalisp::metalang::eval::MetaEvaluator;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Run(RunAction),
    Test(TestAction),
}

#[derive(clap::Args, Debug)]
struct RunAction {
    #[clap(value_parser)]
    filepath: String,
    #[clap(short, long)]
    verbose: bool,
}

#[derive(clap::Args, Debug)]
struct TestAction {
    #[clap(value_parser)]
    filepath: String,
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Run(RunAction {
            ref filepath,
            verbose,
        }) => {
            run::run(filepath.clone(), verbose, false);
        }
        Action::Test(TestAction {
            ref filepath,
            verbose,
        }) => {
            run::run(filepath.clone(), verbose, true);
        }
    }
}
