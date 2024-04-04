pub mod income_vs_expenses
{
    use TMPDIR;
    use std::env;
    use std::io::{Write,Error};
    use std::fs::File;
    use std::path::PathBuf;
    use std::process::Command;
    use enums::plot;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(format_date(date, \"%Y-%m-%d\")) %(roundto(abs(quantity(scrub(display_amount))), 0))\n";
    const FILE_OUTPUT1: &'static str = "income_vs_expenses1.dat";
    const FILE_OUTPUT2: &'static str = "income_vs_expenses2.dat";

    fn prepare_data(
        afile: &str,
        astartyear: i32,
        aendyear: i32,
    ) -> Result<bool, Error>
    {
        let output1: std::vec::Vec<u8> = Command::new("ledger")
            .arg("-f")
            .arg(afile)
            .arg("--strict")
            .arg("-X")
            .arg("EUR")
            .arg("--real")
            .arg("-J")
            .arg("reg")
            .arg("income")
            .arg("-Y")
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
            .arg("--strict")
            .arg("-X")
            .arg("EUR")
            .arg("--real")
            .arg("-J")
            .arg("reg")
            .arg("expenses")
            .arg("-Y")
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

        let path1: PathBuf = env::temp_dir().join(TMPDIR).join(FILE_OUTPUT1);
        let path1_str = path1.to_str().unwrap();
        let path2: PathBuf = env::temp_dir().join(TMPDIR).join(FILE_OUTPUT2);
        let path2_str= path2.to_str().unwrap();

        let mut output_file1 = File::create(path1_str)?;
        match output_file1.write_all(&output1)
        {
            Ok(_) => println!("Wrote data to {}.", FILE_OUTPUT1),
            Err(e) => return Err(e),
        };

        let mut output_file2 = File::create(path2_str)?;
        match output_file2.write_all(&output2)
        {
            Ok(_) => println!("Wrote data to {}.", FILE_OUTPUT2),
            Err(e) => return Err(e),
        };
        Ok(true)
    }

    pub fn plot_data(
        afile: &str,
        astartyear: i32,
        aendyear: i32,
    ) -> Result<bool, Error>
    {
        match prepare_data(afile, astartyear, aendyear)
        {
            Ok(_) => println!("Data for {:?} prepared.", plot::PlotType::IncomeVsExpenses),
            Err(e) => return Err(e),
        };

        match Command::new("gnuplot")
            .arg("/usr/local/share/ledgerplot/gp_income_vs_expenses.gnu")
            .status()
        {
            Ok(_) => println!("Created gnuplot output."),
            Err(e) => return Err(e),
        };
        Ok(true)
    }
}
