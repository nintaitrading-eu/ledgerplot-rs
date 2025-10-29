/*
 * consts
 *     Global vars and resource strings.
 */
pub mod const_
{
    // General
    pub const VERSION: &'static str = "0.1.2";
    pub const USAGE: &'static str = "
    Ledgerplot.

    Usage:
        ledgerplot --file=<file_name> --pricedb=<file_name> --startyear=<year_start> --endyear=<year_end> --type=<All|IncomeVsExpenses|PassiveIncomeVsExpenses|IncomePerCategory|ExpensesPerCategory|WealthGrowth|InvestmentHeatMap>
        ledgerplot --help
        ledgerplot --version

    Options:
        --file=<file_name>          Ledger dat file to use.
        --pricedb=<file_name>       Price database file to use.
        --startyear=<year_start>    Plot from this year.
        --endyear=<year_end>        Plot until this year (inclusive).
        --type=<All|IncomeVsExpenses|PassiveIncomeVsExpenses|IncomePerCategory|ExpensesPerCategory|WealthGrowth|InvestmentHeatMap>                          Create the given plot type.
        -h --help                   Show this screen.
        --version                   Show version.
    ";
    pub const TMPDIR: &'static str = "ledgerplot";

    // Mapping
    pub const JSON_MAPPING: &str = "mapping.json";
}
