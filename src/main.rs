use clap::{arg, Parser, ValueEnum};

mod handelsbanken;
mod nordea;
mod util;
mod ynab;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    input_file: String,

    #[arg(short, long, default_value_t = String::from("output.csv"))]
    output_file: String,

    #[arg(value_enum, short, long, help = "Force read input as a specific bank")]
    force: Option<Bank>,
}

#[derive(ValueEnum, Clone)]
enum Bank {
    Nordea,
    Handelsbanken,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.input_file {
        input_file if is_nordea(&input_file) => {
            convert_csv(nordea::Parser::new(), input_file, cli.output_file)
        }
        input_file if is_handelsbanken(&input_file) => {
            convert_csv(handelsbanken::Parser::new(), input_file, cli.output_file)
        }
        unrecognized => Err(anyhow::Error::msg(format!(
            "Unrecognized file: {:?}",
            unrecognized
        ))),
    }
}

fn is_handelsbanken(input_file: &str) -> bool {
    input_file.ends_with(".xls") || input_file.ends_with(".xlsx")
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
