/*
 * investment_heatmap
 *     Creates a heatmap of investments.
 *     It uses account names and asset names from the asset_mapping.json file.
 */
pub mod investment_heatmap
{
    use crate::consts::const_;
    use crate::models::model;
    use crate::error_handler::error;
    use crate::data_handler::data;
    use std::env;
    use std::io::{Write,BufRead};
    use std::fs::File;
    use std::path::PathBuf;
    use std::process::Command;
    use crate::enums::plot;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(format_date(date, \"%Y-%m-%d\")) %(roundto(scrub(display_amount), 2))\n";
    const DAT_RAW: &'static str = "investment_heatmap_raw.dat";
    const DAT_CONVERTED: &'static str = "investment_heatmap.dat";
    const TOTAL_SEPARATOR: &'static str = "---";

    fn prepare_data(
        aconfig: model::Configuration,
        amapping: model::Mapping,
        afile: &str,
        apricedb: &str,
        aendyear: i32
    ) -> Result<(), error::ApplicationError>
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
            .arg("-e")
            .arg((aendyear + 1).to_string())
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

        let path_raw: PathBuf = env::temp_dir().join(const_::TMPDIR).join(DAT_RAW);
        let path_raw_str = path_raw.to_str().unwrap();

        let mut output_raw = File::create(path_raw_str).map_err(error::ApplicationError::IoError)?;
        output_raw.write_all(&output1).map_err(error::ApplicationError::IoError)?;
        println!("Wrote data to {}.", path_raw_str);

        convert_data(aconfig, amapping, &path_raw_str)?;

        Ok(())
    }

    fn convert_data(aconfig: model::Configuration, amapping: model::Mapping, afile: &str) -> Result<(), error::ApplicationError>
    {
        let path_converted: PathBuf = env::temp_dir().join(const_::TMPDIR).join(DAT_CONVERTED);
        let path_converted_str = path_converted.to_str().unwrap();

        // TODO:
        // read file per line
        // first line: accounts
        // for each line:
        //     map_value of the value without the EUR
        //     set value of the asset in col 1
        //     map asset to account, to know in which col to write the value
        // 
        // TODO: convert the lines in the file from
        // 12.00 EUR assets:stock:xyz
        // to
        // assets:stock:xyz,0,1,0,0,0,0
        // Also add the header:
        // ,account01,account02,... (note empmty first col)
        // Needs a mapper: asset name -> account (name + col idx)
        // Needs a mapper: value range -> int value 1 - 5
        let mut f = File::open(afile)?;

        let buffer = match data::read_lines(afile)
        {
            Ok(br) => br,
            Err(e) => return Err(e),
        };

        let mut line_accounts: String = Default::default();
        let mut number_of_accounts: i32 = 0;
        for (index, account) in amapping.records.iter().enumerate()
        {
            line_accounts.push_str(format!(",{}", account.name).as_str());
            number_of_accounts = number_of_accounts + 1;
        }
        line_accounts.push_str("\n");
        let _ = data::append(line_accounts.as_str(), path_converted_str)?;

        // TODO: assets are per account
        // assets:stock:xyz,0,1,0,0,0,0
        let mut line_asset: String = Default::default();
        for line in buffer.lines().map_while(Result::ok)
        {
            if line.starts_with(TOTAL_SEPARATOR)
            {
                break;
            }
            let amounts: Vec<&str> = line.trim().split_whitespace().collect();
            let amount_as_f64 = match amounts[0].parse::<f64>()
            {
                Ok(amt) => amt,
                Err(e) => return Err(error::ApplicationError::ParsingError(e.to_string())),
            };
            let mapped_value = match map_value(&aconfig, amount_as_f64)
            {
                Ok(val) => val,
                Err(e) => return Err(error::ApplicationError::MappingError(e.to_string())),
            };
            // TODO: make string dynamic, based on number_of_accounts?
            line_asset.push_str(format!("{}", amounts[2]).as_str());
            for account_idx in 1 .. number_of_accounts
            {
                // TODO: add val if map_col_idx is equal to account_idx + 1
                let v = match map_asset_col_idx(amounts[2])
                {
                    Ok(val) => val,
                    Err(e) => return Err(error::ApplicationError::MappingError(e.to_string())),
                };
                if v == account_idx
                {
                    line_asset.push_str(format!(",{}", v).as_str())
                }
                else
                {
                    line_asset.push_str(",0")
                };
            }
            line_asset.push_str("\n");
            let _ = data::append(line_asset.as_str(), path_converted_str)?;
        }

        println!("Wrote converted data to {}.", path_converted_str);

        Ok(())
    }

    fn map_asset_col_idx(asset: &str) -> Result<i32, error::ApplicationError>
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
            "assets:stock:mystock" => Ok(1),
            "assets:stock:mystock2" => Ok(2),
            "assets:stock:mystock3" => Ok(3),
            "assets:asset2" => Ok(2),
            "assets:asset3" => Ok(3),
            _ => Err(error::ApplicationError::UnknownAssetError(asset.to_string())),
        }
    }

    fn map_value(aconfig: &model::Configuration, avalue: f64) -> Result<i32, error::ApplicationError>
    {
        let mut result: i32 = -1;
        if avalue >= aconfig.range1_low && avalue <= aconfig.range1_high
        {
            result = 0;
        };
        if avalue >= aconfig.range2_low && avalue <= aconfig.range2_high
        {
            result = 1;
        };
        if avalue >= aconfig.range3_low && avalue <= aconfig.range3_high
        {
            result = 2;
        };
        if avalue >= aconfig.range4_low && avalue <= aconfig.range4_high
        {
            result = 3;
        };
        if avalue >= aconfig.range5_low && avalue <= aconfig.range5_high
        {
            result = 4;
        };
        if avalue >= aconfig.range6_low
        {
            result = 5;
        };
        match result
        {
            n if n >= 0 && n <= 5 => Ok(n),
            _ => Err(error::ApplicationError::ValueOutOfRangeError(avalue.to_string())),
        }
    }

    pub fn plot_data(
        aconfig: model::Configuration,
        amapping: model::Mapping,
        afile: &str,
        apricedb: &str,
        aendyear: i32
    ) -> Result<(), error::ApplicationError>
    {
        prepare_data(aconfig, amapping, afile, apricedb, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::InvestmentHeatmap);

        Command::new("gnuplot")
            .arg("/usr/local/share/ledgerplot/gp_investment_heatmap.gnu")
            .status()?;
        println!("Created gnuplot output.");

        Ok(())
    }
}
