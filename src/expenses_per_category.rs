pub mod expenses_per_category
{
    use crate::consts::const_;
    use crate::error_handler::error;
    use std::env;
    use std::io::Write;
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;
    use std::process::Command;
    use crate::enums::plot;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(partial_account(options.flat)) %(abs(quantity(scrub(display_total))))\n";
    const FILE_OUTPUT1: &'static str = "expenses_per_category.dat";
    const FILE_OUTPUT_FINAL: &'static str = "expenses_per_category.png"; // As defined in the gnu file.
    const PLOT_SORT_EXPRESSION: &'static str =
        "-abs(quantity(scrub(display_total)))";

    fn prepare_data(
        afile: &str,
        apricedb: &str,
        ayear: i32
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
            .arg("bal")
            .arg("expenses")
            .arg("-p")
            .arg(ayear.to_string())
            .arg("--sort")
            .arg(PLOT_SORT_EXPRESSION)
            .arg("--flat")
            .arg("--no-total")
            .arg("--plot-total-format")
            .arg(PLOT_TOTAL_FORMAT)
            .output()
            .expect("Failed to execute ledger command for output1.")
            .stdout;

        let path1: PathBuf = env::temp_dir().join(const_::TMPDIR).join(FILE_OUTPUT1);
        let path1_str = path1.to_str().unwrap();

        let mut output_file1 = File::create(path1_str).map_err(error::ApplicationError::IoError)?;
        output_file1.write_all(&output1).map_err(error::ApplicationError::IoError)?;
        println!("Wrote data to {}.", FILE_OUTPUT1);

        Ok(())
    }

    pub fn plot_data(
        afile: &str,
        apricedb: &str,
        astartyear: i32,
        aendyear: i32,
    ) -> Result<(), error::ApplicationError>
    {
        for year in astartyear .. aendyear + 1
        {
            prepare_data(afile, apricedb, year)?;
            println!("Data for {:?} prepared.", plot::PlotType::ExpensesPerCategory);

            Command::new("gnuplot")
                .arg("/usr/local/share/ledgerplot/gp_expenses_per_category.gnu")
                .status()?;
            println!("Created gnuplot output.");

            let output_file = FILE_OUTPUT_FINAL
                .to_string()
                .to_lowercase()
                .replace(".png", &format!("_{}.png", year));

            fs::rename(FILE_OUTPUT_FINAL, &output_file)?;
            println!("Wrote data to {}.", &output_file);
        }

        Ok(())
    }
}
