extern crate docopt;

//#[macro_use]
mod enums;

use docopt::Docopt;
use enums::plot;

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
            let plot_type = args.get_str("--type").parse::<plot::PlotType>();
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

fn prepare_data(afile: &str, aplot_type: plot::PlotType)
{
    if aplot_type == plot::PlotType::IncomeVsExpenses
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
