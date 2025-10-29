/*
 * main
 *     The main starting point of the application.
 */
extern crate docopt;

mod consts;
mod enums;
mod models;
mod income_vs_expenses;
mod passive_income_vs_expenses;
mod wealthgrowth;
mod expenses_per_category;
mod income_per_category;
mod investment_heatmap;
mod error_handler;
mod config_handler;
mod data_handler;

use docopt::Docopt;
use enums::plot;
use models::model;
use error_handler::error;
use config_handler::config;
use data_handler::data;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

const VERSION: &'static str = "0.1.2";
const USAGE: &'static str = "
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
const TMPDIR: &'static str = "ledgerplot";

fn main()
{
    let mut mapping = model::Mapping
    {
        records: vec![model::Account { ..Default::default() }],
        ..Default::default()
    };

    match config::ensure_mapping(&mut mapping)
    {
        Ok(()) => (),
        Err(ex) =>
        {
            println!("Error: {}", ex);
            std::process::exit(1);
        }
    }

    mapping = match data::load(&mut mapping)
    {
        Ok(Some(m)) => m,
        Ok(None) => mapping,
        Err(ex) =>
        {
            println!("Error: {}", ex);
            std::process::exit(1);
        }
    };

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

    match plot_data(file, pricedb, &plot_type, startyear, endyear)
    {
        Ok(res) => res,
        Err(e) =>
        {
            println!("Error: data could not be plotted: {:?}", e);
            std::process::exit(1);
        }
    };

    //cleanup(); // Remove temporary files
    std::process::exit(0);
}

fn prepare_temp_dir() -> Result<(), error::ApplicationError>
{
    let paths = [env::temp_dir(), Path::new(TMPDIR).to_path_buf()];
    let tmpdir: PathBuf = paths.iter().collect();
    let tmpdir_str = tmpdir.to_str().unwrap();
    if Path::new(&tmpdir_str).exists()
    {
       return Ok(());
    }

    fs::create_dir_all(&tmpdir_str).map_err(error::ApplicationError::IoError)?;
    Ok(())
}

fn plot_data(
    afile: &str,
    apricedb: &str,
    aplot_type: &plot::PlotType,
    astartyear: i32,
    aendyear: i32,
) -> Result<(), error::ApplicationError>
{
    if *aplot_type == plot::PlotType::IncomeVsExpenses || *aplot_type == plot::PlotType::All
    {
        income_vs_expenses::income_vs_expenses::plot_data(afile, apricedb, astartyear, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::IncomeVsExpenses);
    }
    if *aplot_type == plot::PlotType::PassiveIncomeVsExpenses || *aplot_type == plot::PlotType::All
    {
        passive_income_vs_expenses::passive_income_vs_expenses::plot_data(afile, apricedb, astartyear, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::PassiveIncomeVsExpenses);
    }
    if *aplot_type == plot::PlotType::WealthGrowth || *aplot_type == plot::PlotType::All
    {
        wealthgrowth::wealthgrowth::plot_data(afile, apricedb, astartyear, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::WealthGrowth);
    }
    if *aplot_type == plot::PlotType::ExpensesPerCategory || *aplot_type == plot::PlotType::All
    {
        expenses_per_category::expenses_per_category::plot_data(afile, apricedb, astartyear, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::ExpensesPerCategory);
    }
    if *aplot_type == plot::PlotType::IncomePerCategory || *aplot_type == plot::PlotType::All
    {
        income_per_category::income_per_category::plot_data(afile, apricedb, astartyear, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::IncomePerCategory);
    }
    if *aplot_type == plot::PlotType::InvestmentHeatmap || *aplot_type == plot::PlotType::All
    {
        investment_heatmap::investment_heatmap::plot_data(afile, apricedb, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::InvestmentHeatmap);
    }
    Ok(())
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
