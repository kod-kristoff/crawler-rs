use clap::{Parser, Subcommand};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let args = Args::parse();
}


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long)]
        list: bool,
    },
}
