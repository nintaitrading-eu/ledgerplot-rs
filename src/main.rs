extern crate docopt;

//#[macro_use]
mod enums;

use docopt::Docopt;
use enums::plot;
use std::io::{Write, Error};
use std::fs::File;
use std::process::Command;
use std::path::Path;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Ledgerplot.

Usage:
    ledgerplot --file=<file_name> --startyear=<year_start> --endyear=<year_end> [--type=<IncomeVsExpenses|IncomePerCategory|ExpensesPerCategory|WealthGrowth>] [--yearly|--monthly|--weekly]
    ledgerplot --help
    ledgerplot --version

Options:
    --file=<file_name>          Ledger dat filename to use.
    --startyear=<year_start>    Plot from this year.
    --endyear=<year_end>        Plot until this year (inclusive).
    --type=<IncomeVsExpenses|IncomePerCategory|ExpensesPerCategory|WealthGrowth>                          Create the given plot type.
    --yearly                    Plot totals per year.
    --monthly                   Plot totals per month.
    --weekly                    Plot totals per week.
    -h --help                   Show this screen.
    --version                   Show version.
";
const PLOT_AMOUNT_FORMAT: &'static str =
    "%(format_date(date, \"%Y-%m-%d\")) %(abs(quantity(scrub(display_amount))))\n";

fn main() -> Result<(), Error>
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version")
    {
        println!("Ledgerplot v{}", VERSION);
        std::process::exit(0);
    };

    let file = args.get_str("--file");
    if !(file.len() > 0) || !Path::new(file).exists()
    {
        println!("File {} not found.", file);
        std::process::exit(1);
    };

    let startyear = match args.get_str("--startyear").parse::<i32>()
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("Invalid startyear {}.", args.get_str("--startyear"));
            std::process::exit(1);
        }
    };

    let endyear = match args.get_str("--endyear").parse::<i32>()
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("Invalid endyear {}.", args.get_str("--endyear"));
            std::process::exit(1);
        }
    };

    let plot_type = match args.get_str("--type").parse::<plot::PlotType>()
    {
        Ok(pt) => pt,
        Err(e) =>
        {
            println!("Error parsing plot type: {:?}", e);
            std::process::exit(1);
        }
    };

    let is_prepared = match prepare_data(file, plot_type, startyear, endyear)
    {
        Ok(res) => res,
        Err(e) =>
        {
            println!("Error: data could not be prepared: {:?}", e);
            std::process::exit(1);
        }
    };

    plot_data();
    cleanup(); // Remove temporary files
    std::process::exit(0);
}

fn prepare_data(
    afile: &str,
    aplot_type: plot::PlotType,
    astartyear: i32,
    aendyear: i32,
) -> Result<bool, Error>
{
    println!("TEST - prepare_data: {} for plot {:?}", afile, aplot_type);
    let mut path1: &str = "./";
    let mut path2: &str = "./";
    let mut output1: std::vec::Vec<u8> = std::vec::Vec::<u8>::new();
    let mut output2: std::vec::Vec<u8> = std::vec::Vec::<u8>::new();
    if aplot_type == plot::PlotType::IncomeVsExpenses
    {
        output1 = Command::new("ledger")
            .arg("-f")
            .arg(afile)
            .arg("--strict")
            .arg("-j")
            .arg("reg")
            .arg("--real")
            .arg("-X")
            .arg("EUR")
            .arg("-H")
            .arg("^income")
            .arg("-b")
            .arg(astartyear.to_string())
            .arg("-e")
            .arg(aendyear.to_string())
            .arg("--collapse")
            .arg("--plot-amount-format")
            .arg(PLOT_AMOUNT_FORMAT)
            .output()
            .expect("Failed to execute ledger command for output1.")
            .stdout;
        output2 = Command::new("ledger")
            .arg("-f")
            .arg(afile)
            .arg("--strict")
            .arg("-j")
            .arg("reg")
            .arg("--real")
            .arg("-X")
            .arg("EUR")
            .arg("-H")
            .arg("^expenses")
            .arg("-b")
            .arg(astartyear.to_string())
            .arg("-e")
            .arg(aendyear.to_string())
            .arg("--collapse")
            .arg("--plot-amount-format")
            .arg(PLOT_AMOUNT_FORMAT)
            .output()
            .expect("Failed to execute ledger command for output2.")
            .stdout;
        path1 = "/var/tmp/ledgerplot/ledgeroutput1.tmp";
        path2 = "/var/tmp/ledgerplot/ledgeroutput2.tmp";
    }
    let mut output_file1 = File::create(path1)?;
    let mut result: std::result::Result<bool, Error> = match output_file1.write_all(&output1)
    {
        Ok(_) => Ok(true),
        Err(e) => return Err(e),
    };

    let mut output_file2 = File::create(path2)?;
    result = match output_file2.write_all(&output2)
    {
        Ok(_) => Ok(true),
        Err(e) => return Err(e),
    };
    result = match Command::new("gnuplot")
        .arg("/usr/local/share/ledgerplot/gp_income_vs_expenses.gnu")
        .status()
    {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    };
    result
}

fn plot_data()
{
    println!("TEST - plot_data");
}

fn cleanup()
{
    println!("TEST - cleanup");
}
