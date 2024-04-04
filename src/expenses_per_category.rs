pub mod expenses_per_category
{
    use TMPDIR;
    use std::env;
    use std::io::{Write,Error};
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;
    use std::process::Command;
    use enums::plot;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(partial_account(options.flat)) %(abs(quantity(scrub(display_total))))\n";
    const FILE_OUTPUT1: &'static str = "expenses_per_category.dat";
    const FILE_OUTPUT_FINAL: &'static str = "expenses_per_category.png"; // As defined in the gnu file.
    const PLOT_SORT_EXPRESSION: &'static str =
        "-abs(amount)";

    fn prepare_data(
        afile: &str,
        ayear: i32
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

        let path1: PathBuf = env::temp_dir().join(TMPDIR).join(FILE_OUTPUT1);
        let path1_str = path1.to_str().unwrap();

        let mut output_file1 = File::create(path1_str)?;
        match output_file1.write_all(&output1)
        {
            Ok(_) => println!("Wrote data to {}.", FILE_OUTPUT1),
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
        for year in astartyear .. aendyear + 1
        {
            match prepare_data(afile, year)
            {
                Ok(_) => println!("Data for {:?} prepared.", plot::PlotType::ExpensesPerCategory),
                Err(e) => return Err(e),
            }

            match Command::new("gnuplot")
                .arg("/usr/local/share/ledgerplot/gp_expenses_per_category.gnu")
                .status()
            {
                Ok(_) => println!("Created gnuplot output."),
                Err(e) => return Err(e),
            }

            let output_file = FILE_OUTPUT_FINAL
                .to_string()
                .to_lowercase()
                .replace(".png", &format!("_{}.png", year));

            match fs::rename(FILE_OUTPUT_FINAL, &output_file)
            {
                Ok(_) => println!("Wrote data to {}.", &output_file),
                Err(e) => println!("Error writing data: {}", e),
            };
        }
        Ok(true)
    }
}
