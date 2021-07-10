pub mod wealthgrowth
{
    use std::io::{Write,Error};
    use std::fs::File;
    use std::process::Command;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(format_date(date, \"%Y-%m-%d\")) %(abs(quantity(scrub(display_amount))))\n";

    pub fn prepare_data(
        afile: &str,
        astartyear: i32,
        aendyear: i32,
    ) -> Result<bool, Error>
    {
        let path1: &str = "/var/tmp/ledgerplot/ledgeroutput1.tmp";
        let path2: &str = "/var/tmp/ledgerplot/ledgeroutput2.tmp";
        let output1: std::vec::Vec<u8> = Command::new("ledger")
            .arg("-f")
            .arg(afile)
            .arg("--price-db")
            .arg("price.db")
            .arg("--strict")
            .arg("-X")
            .arg("EUR")
            .arg("--real")
            .arg("-J")
            .arg("reg")
            .arg("assets")
            .arg("-D")
            .arg("--collapse")
            .arg("--no-rounding")
            .arg("--plot-total-format")
            .arg(PLOT_TOTAL_FORMAT)
            .arg("-b")
            .arg(astartyear.to_string())
            .arg("-e")
            .arg((aendyear + 1).to_string())
            .output()
            .expect("Failed to execute ledger command for output1.")
            .stdout;
        let output2: std::vec::Vec<u8> = Command::new("ledger")
            .arg("-f")
            .arg(afile)
            .arg("--price-db")
            .arg("prices.db")
            .arg("--strict")
            .arg("-X")
            .arg("EUR")
            .arg("--real")
            .arg("-J")
            .arg("reg")
            .arg("liabilities")
            .arg("-D")
            .arg("--collapse")
            .arg("--no-rounding")
            .arg("--plot-total-format")
            .arg(PLOT_TOTAL_FORMAT)
            .arg("-b")
            .arg(astartyear.to_string())
            .arg("-e")
            .arg((aendyear + 1).to_string())
            .output()
            .expect("Failed to execute ledger command for output2.")
            .stdout;

        let mut output_file1 = File::create(path1)?;
        match output_file1.write_all(&output1)
        {
            Ok(_) => println!("Wrote output1."),
            Err(e) => return Err(e),
        };

        let mut output_file2 = File::create(path2)?;
        match output_file2.write_all(&output2)
        {
            Ok(_) => println!("Wrote output2."),
            Err(e) => return Err(e),
        };
        Ok(true)
    }

    pub fn plot_data() -> Result<bool, Error>
    {
        match Command::new("gnuplot")
            .arg("/usr/local/share/ledgerplot/gp_wealthgrowth.gnu")
            .status()
        {
            Ok(_) => println!("Created gnuplot output."),
            Err(e) => return Err(e),
        };
        Ok(true)
    }
}
