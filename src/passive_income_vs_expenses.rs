pub mod passive_income_vs_expenses
{
    use crate::consts::const_;
    use crate::error_handler::error;
    use std::env;
    use std::io::Write;
    use std::fs::File;
    use std::path::PathBuf;
    use std::process::Command;
    use crate::enums::plot;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(format_date(date, \"%Y-%m-%d\")) %(abs(roundto(scrub(display_amount), 2)))\n";
    const FILE_OUTPUT1: &'static str = "passive_income_vs_expenses1.dat";
    const FILE_OUTPUT2: &'static str = "passive_income_vs_expenses2.dat";

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
            .arg("income:stock:dividend")
            .arg("income:etf:dividend")
            .arg("income:interest")
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
            .arg("and not expenses:stock")
            .arg("and not expenses:crypto")
            .arg("and not expenses:etf")
            .arg("and not expenses:bond")
            .arg("and not expenses:fund")
            .arg("and not expenses:turbo")
            .arg("and not expenses:nintai_bvba")
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

        let path1: PathBuf = env::temp_dir().join(const_::TMPDIR).join(FILE_OUTPUT1);
        let path1_str = path1.to_str().unwrap();
        let path2: PathBuf = env::temp_dir().join(const_::TMPDIR).join(FILE_OUTPUT2);
        let path2_str= path2.to_str().unwrap();

        let mut output_file1 = File::create(path1_str).map_err(error::ApplicationError::IoError)?;
        output_file1.write_all(&output1).map_err(error::ApplicationError::IoError)?;
        println!("Wrote data to {}.", FILE_OUTPUT1);

        let mut output_file2 = File::create(path2_str).map_err(error::ApplicationError::IoError)?;
        output_file2.write_all(&output2).map_err(error::ApplicationError::IoError)?;
        println!("Wrote data to {}.", FILE_OUTPUT2);

        Ok(())
    }

    pub fn plot_data(
        afile: &str,
        apricedb: &str,
        astartyear: i32,
        aendyear: i32,
    ) -> Result<(), error::ApplicationError>
    {
        prepare_data(afile, apricedb, astartyear, aendyear)?;
        println!("Data for {:?} prepared.", plot::PlotType::PassiveIncomeVsExpenses);

        Command::new("gnuplot")
            .arg("/usr/local/share/ledgerplot/gp_passive_income_vs_expenses.gnu")
            .status()?;
        println!("Created gnuplot output.");

        Ok(())
    }
}
