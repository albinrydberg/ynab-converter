use clap::{arg, Args, Parser, Subcommand, ValueEnum};

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
    api: Option<YnabApi>,
}

#[derive(Subcommand)]
enum YnabApi {
    Budgets(YnabApiArguments),
}

#[derive(Args)]
struct YnabApiArguments {
    #[arg(short, long)]
    pat: String,
}

#[derive(ValueEnum, Clone)]
enum Bank {
    Nordea,
    Handelsbanken,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.api {
        Some(YnabApi::Budgets(args)) => {
            let client = ynab::Client::new(args.pat)?;
            let response = client.get_budgets()?;
            println!("{:?}", response);
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
