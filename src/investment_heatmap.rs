pub mod investment_heatmap
{
    use crate::TMPDIR;
    use std::env;
    use std::io::{Write,Error};
    use std::fs::File;
    use std::path::PathBuf;
    use std::process::Command;
    use crate::enums::plot;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(format_date(date, \"%Y-%m-%d\")) %(roundto(scrub(display_amount), 2))\n";
    const DAT_RAW: &'static str = "investment_heatmap_raw.dat";
    const DAT_CONVERTED: &'static str = "investment_heatmap.dat";

    fn prepare_data(
        afile: &str,
        apricedb: &str,
    ) -> Result<bool, Error>
    {
        let output1: std::vec::Vec<u8> = Command::new("ledger")
            .arg("-f")
            .arg(afile)
            .arg("--price-db")
            .arg(apricedb)
            .arg("--strict")
            .arg("-X")
            .arg("EUR")
            .arg("--real")
            .arg("bal")
            .arg("assets:stock")
            .arg("assets:etf")
            .arg("assets:bond")
            .arg("assets:crypto")
            .arg("--plot-total-format")
            .arg(PLOT_TOTAL_FORMAT)
            .arg("--flat")
            .output()
            .expect("Failed to execute ledger command for output1.")
            .stdout;

        let path_raw: PathBuf = env::temp_dir().join(TMPDIR).join(DAT_RAW);
        let path_raw_str = path_raw.to_str().unwrap();

        let mut output_raw = File::create(path_raw_str)?;
        match output_raw.write_all(&output1)
        {
            Ok(_) => println!("Wrote data to {}.", path_raw_str),
            Err(e) => return Err(e),
        };

        // TODO: convert the lines in the file from
        // 12.00 EUR assets:stock:xyz
        // to
        // assets:stock:xyz,0,1,0,0,0,0
        // Also add the header:
        // ,account01,account02,... (note empmty first col)
        // Needs a mapper: asset name -> account (name + col idx)
        // Needs a mapper: value range -> int value 1 - 5
        let path_converted: PathBuf = env::temp_dir().join(TMPDIR).join(DAT_CONVERTED);
        let path_converted_str = path_converted.to_str().unwrap();
        match convert_data(&path_raw_str)
        {
            Ok(_) => println!("Wrote converted data to {}.", path_converted_str),
            Err(e) => return Err(e), // TODO: Custom error handling
        }

        Ok(true)
    }

    fn convert_data(afile: &str) -> Result<bool, Error>
    {
        // TODO:
        // read file per line
        // first line: accounts
        // for each line:
        //     map_value of the value without the EUR
        //     set value of the asset in col 1
        //     map asset to account, to know in which col to write the value
        // 
        Ok(true)
    }

    fn map_asset_col_idx(asset: &str) -> Result<i32, Error>
    {
        // TODO: read json with mappings?
        // {[
        //     { "assets:asset1": "account00" },
        //     { "assets:asset2": "account01" },
        //     { "assets:asset3": "account02" },
        //     { "assets:asset4": "account00" },
        // ]} 
        match asset
        {
            "assets:asset1" => Ok(1),
            "assets:asset2" => Ok(2),
            "assets:asset3" => Ok(3),
            _ => panic!("Unknown asset, check asset_mapping.json"), // TODO: Implement custom error handling correctly
        }
    }

    fn map_value(avalue: f64) -> Result<i32, Error>
    {
        // TODO: switch ranges.
        match avalue
        {
            0.0..=9999.0 => Ok(0),
            10000.0..=24999.0 => Ok(1),
            25000.0..=49999.0 => Ok(2),
            50000.0..=74999.0 => Ok(3),
            75000.0..=99999.0 => Ok(4),
            99999.0.. => Ok(5),
            _ => panic!("Unknown range"), // TODO: Implement custom error handling correctly
        }
    }

    pub fn plot_data(
        afile: &str,
        apricedb: &str,
    ) -> Result<bool, Error>
    {
        match prepare_data(afile, apricedb)
        {
            Ok(_) => println!("Data for {:?} prepared.", plot::PlotType::InvestmentHeatmap),
            Err(e) => return Err(e),
        };

        match Command::new("gnuplot")
            .arg("/usr/local/share/ledgerplot/gp_investment_heatmap.gnu")
            .status()
        {
            Ok(_) => println!("Created gnuplot output."),
            Err(e) => return Err(e),
        };
        Ok(true)
    }
}
