pub mod expenses_per_category
{
    use TMPDIR;
    use std::env;
    use std::io::{Write,Error};
    use std::fs::File;
    use std::path::PathBuf;
    use std::process::Command;
    use enums::plot;

    const PLOT_TOTAL_FORMAT: &'static str =
        "%(partial_account(options.flat)) %(abs(quantity(scrub(display_total))))\n";
    const FILE_OUTPUT1: &'static str = "expenses_per_category.dat";
    const PLOT_SORT_EXPRESSION: &'static str =
        "-abs(amount)";

    fn prepare_data(
        afile: &str,
        ayear: i32
    ) -> Result<bool, Error>
    {
        // TODO: Loop over years from astartyear to aendyear (iter?)
        // TODO: change outputfile name to include each year
        // TODO: Do the same for the data preparation
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
        // TODO: Write loop over the years.
        // Call plot on each iteration.
        // Move file after plotting.
        // Remove prepare_data call in main.rs.
        match prepare_data(afile, aendyear) // TODO: change to loop var
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
        };
        Ok(true)
    }
}
