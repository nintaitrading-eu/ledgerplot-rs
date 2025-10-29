pub mod wealthgrowth
{
    use crate::TMPDIR;
    use crate::error_handler::error;
    use std::env;
    use std::io::{Read,Write};
    use std::fs::{File,OpenOptions};
    use std::path::PathBuf;
    use std::process::Command;
    use crate::enums::plot;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(format_date(date, \"%Y-%m-%d\")) %(abs(roundto(quantity(scrub(display_total)),2)))\n";
    const FILE_OUTPUT1: &'static str = "wealthgrowth1.dat";
    const FILE_OUTPUT2: &'static str = "wealthgrowth2.dat";
    const FILE_OUTPUT3: &'static str = "wealthgrowth3.dat";

    fn prepare_data(
        afile: &str,
        apricedb: &str,
        astartyear: i32,
        aendyear: i32,
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
            .arg("-J")
            .arg("reg")
            .arg("assets")
            .arg("-D")
            .arg("--collapse")
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
            .arg(apricedb)
            .arg("--strict")
            .arg("-X")
            .arg("EUR")
            .arg("--real")
            .arg("-J")
            .arg("reg")
            .arg("liabilities")
            .arg("-D")
            .arg("--collapse")
            .arg("--plot-total-format")
            .arg(PLOT_TOTAL_FORMAT)
            .arg("-b")
            .arg(astartyear.to_string())
            .arg("-e")
            .arg((aendyear + 1).to_string())
            .output()
            .expect("Failed to execute ledger command for output2.")
            .stdout;
        let output3: std::vec::Vec<u8> = Command::new("ledger")
            .arg("-f")
            .arg(afile)
            .arg("--price-db")
            .arg(apricedb)
            .arg("--strict")
            .arg("-X")
            .arg("EUR")
            .arg("--real")
            .arg("-J")
            .arg("reg")
            .arg("assets")
            .arg("liabilities")
            .arg("-D")
            .arg("--collapse")
            .arg("--plot-total-format")
            .arg(PLOT_TOTAL_FORMAT)
            .arg("-b")
            .arg(astartyear.to_string())
            .arg("-e")
            .arg((aendyear + 1).to_string())
            .output()
            .expect("Failed to execute ledger command for output3.")
            .stdout;

        let path1: PathBuf = env::temp_dir().join(TMPDIR).join(FILE_OUTPUT1);
        let path1_str = path1.to_str().unwrap();
        let path2: PathBuf = env::temp_dir().join(TMPDIR).join(FILE_OUTPUT2);
        let path2_str = path2.to_str().unwrap();
        let path3: PathBuf = env::temp_dir().join(TMPDIR).join(FILE_OUTPUT3);
        let path3_str = path3.to_str().unwrap();

        let mut output_file1 = File::create(path1_str).map_err(error::ApplicationError::IoError)?;
        output_file1.write_all(&output1).map_err(error::ApplicationError::IoError)?;
        println!("Wrote data to {}.", FILE_OUTPUT1);

        let mut output_file2 = File::create(path2_str).map_err(error::ApplicationError::IoError)?;
        output_file2.write_all(&output2).map_err(error::ApplicationError::IoError)?;
        println!("Wrote data to {}.", FILE_OUTPUT2);

        let mut output_file3 = File::create(path3_str).map_err(error::ApplicationError::IoError)?;
        output_file3.write_all(&output3).map_err(error::ApplicationError::IoError)?;
        println!("Wrote data to {}.", FILE_OUTPUT3);

        Ok(())
    }

    pub fn plot_data(
        afile: &str,
        apricedb: &str,
        astartyear: i32,
        aendyear: i32
    ) -> Result<(), error::ApplicationError>
    {
        prepare_data(afile, apricedb, astartyear, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::WealthGrowth);

        let script_without_xrange = "/usr/local/share/ledgerplot/gp_wealthgrowth.gnu";
        let script_with_xrange: &str = "/tmp/ledgerplot/wealthgrowth.gnu";

        let xrange_line = format!("set xdata time\nset timefmt \"%Y-%m-%d\"\nset xrange [\"{}-01-01\":\"{}-12-31\"]\n", astartyear.to_string(), aendyear.to_string());
        let mut script_with_xrange_file = File::create(script_with_xrange).map_err(error::ApplicationError::IoError)?;
        script_with_xrange_file.write_all(&xrange_line.as_bytes()).map_err(error::ApplicationError::IoError)?;
        println!("Wrote gnuplot script_with_xrange.");

        let mut file_in = std::fs::File::open(script_without_xrange).unwrap();
        let mut file_out = OpenOptions::new().append(true).open(script_with_xrange).unwrap();
        let mut buffer = [0u8; 4096];
        loop
        {
            let nbytes = file_in.read(&mut buffer).unwrap();
            file_out.write(&buffer[..nbytes]).unwrap();
            if nbytes < buffer.len()
            {
                break;
            }
        }
        Command::new("gnuplot")
            .arg(script_with_xrange)
            .status()?;
        println!("Created gnuplot output.");

        Ok(())
    }
}
