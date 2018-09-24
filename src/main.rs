extern crate docopt;

use docopt::Docopt;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Ledgerplot.

Usage:
    ledgerplot --file=<file_name> --startyear=<year_start> --endyear=<year_end> [--income_vs_expenses|--income_per_category|--expenses_per_category] [--yearly|--monthly|--weekly]
    ledgerplot --help
    ledgerplot --version

Options:
    --file=<file_name>          Ledger dat filename to use.
    --startyear=<year_start>    Plot from this year.
    --endyear=<year_end>        Plot until this year (inclusive).
    --income_vs_expenses        Plot income vs expenses.
    --income_per_category       Plot income per category.
    --expenses_per_category     Plot expenses per category.
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
        println!("TBD");
    }
}
