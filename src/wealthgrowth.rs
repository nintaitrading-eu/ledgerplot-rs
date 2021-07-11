pub mod wealthgrowth
{
    use std::io::{Read,Write,Error};
    use std::fs::{File,OpenOptions};
    use std::process::Command;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(format_date(date, \"%Y-%m-%d\")) %(abs(quantity(scrub(display_total))))\n";

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
            .arg("/home/rockwolf/doc/ledger-personal/prices.db")
            .arg("--strict")
            .arg("-X")
            .arg("EUR")
            .arg("--real")
            .arg("-J")
            .arg("reg")
            .arg("assets")
            .arg("-D")
            .arg("--collapse")
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
            .arg("/home/rockwolf/doc/ledger-personal/prices.db")
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

    pub fn plot_data(astartyear: i32, aendyear: i32) -> Result<bool, Error>
    {
        let script_without_xrange = "/usr/local/share/ledgerplot/gp_wealthgrowth.gnu";
        let script_with_xrange: &str = "/var/tmp/ledgerplot/wealthgrowth.gnu";

        let xrange_line = format!("set xdata time\nset timefmt \"%Y-%m-%d\"\nset xrange [\"{}-01-01\":\"{}-12-31\"]\n", astartyear.to_string(), aendyear.to_string());
        let mut script_with_xrange_file = File::create(script_with_xrange)?;
        match script_with_xrange_file.write_all(&xrange_line.as_bytes())
        {
            Ok(_) => println!("Wrote gnuplot script_with_xrange."),
            Err(e) => return Err(e),
        };
        let mut file_in = std::fs::File::open(script_without_xrange).unwrap();
        let mut file_out = OpenOptions::new().append(true).open(script_with_xrange).unwrap();
        let mut buffer = [0u8; 4096];
        loop
        {
            let nbytes = file_in.read(&mut buffer).unwrap();
            file_out.write(&buffer[..nbytes]).unwrap();
            if nbytes < buffer.len() { break; }
        }
        match Command::new("gnuplot")
            .arg(script_with_xrange)
            .status()
        {
            Ok(_) => println!("Created gnuplot output."),
            Err(e) => return Err(e),
        };
        Ok(true)
    }
}
