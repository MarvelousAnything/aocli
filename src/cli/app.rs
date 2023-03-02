use aocli::config::Config;
use aocli::core::AppContext;
use clap::{Args, Parser, Subcommand};
use miette::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Initialize a new Advent of Code project")]
    Init(InitArgs),
    #[command(about = "Fetch the problem description and input for a given day")]
    Day(DayArgs),
    #[command(about = "Fetch the problem description and input for a given day")]
    Fetch(DayArgs),
}

#[derive(Args, Debug)]
struct InitArgs {
    #[clap(help = "The project directory")]
    #[clap(default_value = ".")]
    dir: String,
}

#[derive(Args, Debug)]
struct DayArgs {
    #[clap(short, long)]
    #[clap(help = "The year number")]
    year: u16,
    #[clap(short, long)]
    #[clap(help = "The day number")]
    day: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config {
        year: 2020,
        day: 1,
        session_id: "session_id".to_string(),
    };
    let app = AppContext::new(config);
    match cli.command {
        Commands::Init(args) => {
            println!("Initializing project in {}", args.dir);
        }
        Commands::Day(args) => {
            println!("Fetching problem for {}/{}", args.year, args.day);
        }
        Commands::Fetch(args) => {
            println!("Fetching input for {}/{}", args.year, args.day);
            let input = app.api.get_problem_input(args.year, args.day).await?;
            println!("input: {input}");
        }
    }
    Ok(())
}
