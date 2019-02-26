extern crate docopt;

use docopt::Docopt;
//use enums::{PlotType};
use std::str::FromStr;

#[macro_use]
pub mod enums;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Ledgerplot.

Usage:
    ledgerplot --file=<file_name> --startyear=<year_start> --endyear=<year_end> [--type=<IncomeVsExpenses|IncomePerCategory|ExpensesPerCategory|wealthgrowth>] [--yearly|--monthly|--weekly]
    ledgerplot --help
    ledgerplot --version

Options:
    --file=<file_name>          Ledger dat filename to use.
    --startyear=<year_start>    Plot from this year.
    --endyear=<year_end>        Plot until this year (inclusive).
    --type=<IncomeVsExpenses|IncomePerCategory|ExpensesPerCategory|Wealthgrowth>                          Create the given plot type.
    --yearly                    Plot totals per year.
    --monthly                   Plot totals per month.
    --weekly                    Plot totals per week.
    -h --help                   Show this screen.
    --version                   Show version.
";

#[derive(Debug, PartialEq)]
pub enum PlotType
{
  IncomeVsExpenses,
  IncomePerCategory,
  ExpensesPerCategory,
  WealthGrowth
}

impl FromStr for PlotType
{
    type Err = ();

    fn from_str(a_str: &str) -> Result<Self, Self::Err>
    {
        match a_str
        {
            "IncomeVsExpenses" => Ok(PlotType::IncomeVsExpenses),
            "IncomePerCategory" => Ok(PlotType::IncomePerCategory),
            "ExpensesPerCategory" => Ok(PlotType::ExpensesPerCategory),
            "WealthGrowth" => Ok(PlotType::WealthGrowth),
            _ => Err(()),
        }
    }
}

fn main()
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version")
    {
        println!("Ledgerplot v{}", VERSION);
    }
    else
    {
        let file = args.get_str("--file");
        if file.len() > 0
        {
            //let plot_type = enums::PlotType::from_str(args.get_str("--type"));
            let plot_type = PlotType::from_str(args.get_str("--type"));
            match plot_type
            {
                Ok(pt) => prepare_data(file, pt),
                Err(e) => println!("Error parsing plot type: {:?}", e),
            }
            
            plot_data();
            cleanup(); // Remove temporary files
        }
    }
    std::process::exit(0);
}

//fn prepare_data(afile: &str, aplot_type: enums::PlotType)
fn prepare_data(afile: &str, aplot_type: PlotType)
{
    //if aplot_type == enums::PlotType::IncomeVsExpenses
    if aplot_type == PlotType::IncomeVsExpenses
    {
      println!("PlotType enum = {:?}", aplot_type);
    }
    println!("TEST - prepare_data: {} for plot {:?}", afile, aplot_type);
}

fn plot_data()
{
    println!("TEST - plot_data");
}

fn cleanup()
{
    println!("TEST - cleanup");
}
