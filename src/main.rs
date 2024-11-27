use crate::ynab::YnabWriter;
use clap::{arg, Parser, ValueEnum};

mod amex;
mod csv;
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
    Amex,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let ynab_writer = YnabWriter::new();

    match cli.input_file {
        input_file if is_csv(&input_file) => convert_csv(ynab_writer, input_file, cli.output_file),
        input_file if is_excel(&input_file) => {
            convert_excel(ynab_writer, input_file, cli.output_file)
        }
        unrecognized => Err(anyhow::Error::msg(format!(
            "Unrecognized file: {:?}",
            unrecognized
        ))),
    }
}

fn is_excel(input_file: &str) -> bool {
    input_file.ends_with(".xls") || input_file.ends_with(".xlsx")
}

fn is_csv(input_file: &str) -> bool {
    input_file.ends_with(".csv")
}

fn convert_excel(
    ynab_writer: YnabWriter,
    input_file: String,
    output_file: String,
) -> anyhow::Result<()> {
    let parser = handelsbanken::Parser::new();
    let ynab_rows = parser.read(input_file)?;
    ynab_writer.write_csv(output_file, ynab_rows)
}

fn convert_csv(
    ynab_writer: YnabWriter,
    input_file: String,
    output_file: String,
) -> anyhow::Result<()> {
    let nordea_parser = nordea::Parser::new();
    if nordea_parser.is_parsable(&input_file) {
        let nordea_rows = nordea_parser.parse(&input_file)?;
        ynab_writer.write_csv(output_file.clone(), nordea_rows)?
    }

    let amex_parser = amex::Parser::new();
    if amex_parser.is_parsable(&input_file) {
        let amex_rows = amex_parser.parse(&input_file)?;
        ynab_writer.write_csv(output_file.clone(), amex_rows)?
    }

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
