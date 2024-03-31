use clap::{Args, Parser, Subcommand};

mod handelsbanken;
mod nordea;
mod util;
mod ynab;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// What bank format to use
    #[command(subcommand)]
    bank: Bank,
}

#[derive(Subcommand)]
enum Bank {
    Nordea(Arguments),
    Handelsbanken(Arguments),
}

#[derive(Args)]
struct Arguments {
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long, default_value_t = String::from("output.csv"))]
    output_file: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.bank {
        Bank::Nordea(arguments) => convert_csv(nordea::Parser::new(), arguments),
        Bank::Handelsbanken(arguments) => convert_csv(handelsbanken::Parser::new(), arguments),
    }
}

fn convert_csv(parser: impl ynab::Parser, arguments: Arguments) -> anyhow::Result<()> {
    let input_file = arguments.input_file;
    let output_file = arguments.output_file;

    let ynab_rows = parser.parse(input_file)?;
    ynab::write_csv(output_file, ynab_rows)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
