use anyhow::Context;
use clap::{arg, Args, Parser, Subcommand, ValueEnum};
use dotenvy;

mod handelsbanken;
mod nordea;
mod util;
mod ynab;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    input_file: Option<String>,

    #[arg(short, long, default_value_t = String::from("output.csv"))]
    output_file: String,

    #[arg(value_enum, short, long, help = "Force read input as a specific bank")]
    force: Option<Bank>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Api {
        #[command(subcommand)]
        command: ApiCommand
    },
}

#[derive(Subcommand)]
enum ApiCommand {
    Budgets(YnabApiArguments),
}

#[derive(Args)]
struct YnabApiArguments {
    #[arg(short, long)]
    pat: Option<String>,
}

impl YnabApiArguments {
    fn pat(self) -> anyhow::Result<String> {
        match self.pat {
            Some(x) => Ok(x),
            None => std::env::var(YNAB_PAT_VAR).with_context(|| "PAT environment variable not set"),
        }
    }
}

#[derive(ValueEnum, Clone)]
enum Bank {
    Nordea,
    Handelsbanken,
}

const YNAB_PAT_VAR: &'static str = "YNAB_PAT";

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Api { command: ApiCommand::Budgets(args) }) => {
            let pat = args.pat()?;
            let client = ynab::Client::new(pat)?;
            let response = client.get_budgets()?;
            println!("{}", serde_json::to_string(&response)?);
            Ok(())
        }
        None => match cli.input_file {
            Some(input_file) if is_nordea(&input_file) => {
                convert_csv(nordea::Parser::new(), input_file, cli.output_file)
            }
            Some(input_file) if is_handelsbanken(&input_file) => {
                convert_csv(handelsbanken::Parser::new(), input_file, cli.output_file)
            }
            Some(unrecognized) => Err(anyhow::Error::msg(format!(
                "Unrecognized file: {:?}",
                unrecognized
            ))),
            None => Err(anyhow::Error::msg("No file passed")),
        },
    }
}

fn is_handelsbanken(input_file: &str) -> bool {
    input_file.ends_with(".xls")
}

fn is_nordea(input_file: &str) -> bool {
    input_file.ends_with(".csv")
}

fn convert_csv(
    parser: impl ynab::Parser,
    input_file: String,
    output_file: String,
) -> anyhow::Result<()> {
    let ynab_rows = parser.parse(input_file)?;
    ynab::write_csv(output_file, ynab_rows)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
