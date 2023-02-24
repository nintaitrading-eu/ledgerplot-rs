extern crate docopt;

mod enums;
mod income_vs_expenses;
mod wealthgrowth;

use docopt::Docopt;
use enums::plot;
use std::io::Error;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Ledgerplot.

Usage:
    ledgerplot --file=<file_name> --pricedb=<file_name> --startyear=<year_start> --endyear=<year_end> --type=<IncomeVsExpenses|IncomePerCategory|ExpensesPerCategory|WealthGrowth> [--yearly|--monthly|--weekly]
    ledgerplot --help
    ledgerplot --version

Options:
    --file=<file_name>          Ledger dat file to use.
    --pricedb=<file_name>       Price database file to use.
    --startyear=<year_start>    Plot from this year.
    --endyear=<year_end>        Plot until this year (inclusive).
    --type=<IncomeVsExpenses|IncomePerCategory|ExpensesPerCategory|WealthGrowth>                          Create the given plot type.
    --yearly                    Plot totals per year.
    --monthly                   Plot totals per month.
    --weekly                    Plot totals per week.
    -h --help                   Show this screen.
    --version                   Show version.
";
const TMPDIR: &'static str = "ledgerplot";

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

    let pricedb = args.get_str("--pricedb");
    if !(pricedb.len() > 0) || !Path::new(pricedb).exists()
    {
        println!("Price database {} not found.", pricedb);
        std::process::exit(1);
    };

    if args.get_bool("--yearly")
        || args.get_bool("--monthly")
        || args.get_bool("--weekly")
    {
        println!("NotImplemented: --yearly, --monthly or --weekly options.");
    }

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
        Err(_) =>
        {
            println!("Error parsing plot type: {}", args.get_str("--type"));
            std::process::exit(1);
        }
    };

    match prepare_temp_dir()
    {
        Ok(res) => res,
        Err(e) =>
        {
            println!("Error: temporary directory could not be created: {:?}", e);
            std::process::exit(1);
        }
    };

    match prepare_data(file, pricedb, &plot_type, startyear, endyear)
    {
        Ok(res) => res,
        Err(e) =>
        {
            println!("Error: data could not be prepared: {:?}", e);
            std::process::exit(1);
        }
    };

    match plot_data(&plot_type, startyear, endyear)
    {
        Ok(res) => res,
        Err(e) =>
        {
            println!("Error: data could not be plotted: {:?}", e);
            std::process::exit(1);
        }
    };
    cleanup(); // Remove temporary files
    std::process::exit(0);
}

fn prepare_temp_dir() -> Result<bool, Error>
{
    let paths = [env::temp_dir(), Path::new(TMPDIR).to_path_buf()];
    let tmpdir: PathBuf = paths.iter().collect();
    let tmpdir_str = tmpdir.to_str().unwrap();
    if Path::new(&tmpdir_str).exists()
    {
       return Ok(true);
    }

    match fs::create_dir_all(&tmpdir_str)
    {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    Ok(true)
}

fn prepare_data(
    afile: &str,
    apricedb: &str,
    aplot_type: &plot::PlotType,
    astartyear: i32,
    aendyear: i32,
) -> Result<bool, Error>
{
    if *aplot_type == plot::PlotType::IncomeVsExpenses
    {
        match income_vs_expenses::income_vs_expenses::prepare_data(afile, astartyear, aendyear)
        {
            Ok(_) => println!("Data for {:?} prepared.", aplot_type),
            Err(e) => return Err(e),
        };
    }
    if *aplot_type == plot::PlotType::WealthGrowth
    {
        match wealthgrowth::wealthgrowth::prepare_data(afile, apricedb, astartyear, aendyear)
        {
            Ok(_) => println!("Data for {:?} prepared.", aplot_type),
            Err(e) => return Err(e),
        };
    }
    Ok(true)
}

fn plot_data(aplot_type: &plot::PlotType, astartyear: i32, aendyear: i32) -> Result<bool, Error>
{
    if *aplot_type == plot::PlotType::IncomeVsExpenses
    {
        match income_vs_expenses::income_vs_expenses::plot_data()
        {
            Ok(_) => println!("Data for {:?} plotted.", *aplot_type),
            Err(e) => return Err(e),
        };
    }
    if *aplot_type == plot::PlotType::WealthGrowth
    {
        match wealthgrowth::wealthgrowth::plot_data(astartyear, aendyear)
        {
            Ok(_) => println!("Data for {:?} plotted.", *aplot_type),
            Err(e) => return Err(e),
        };
    }
    Ok(true)
}

fn cleanup()
{
    for path in fs::read_dir(env::temp_dir()).unwrap()
    {
        let path = path.unwrap().path();
        if path.file_stem() == Some(OsStr::new(TMPDIR))
        {
            fs::remove_dir_all(path).unwrap();
        }
    }
}
