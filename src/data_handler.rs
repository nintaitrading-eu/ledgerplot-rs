/*
 * data_handler
 *     Loads and saves data from files.
 */
pub mod data 
{
    use crate::error_handler::error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    pub fn read_lines<F>(filename: F) -> Result<BufReader<File>, error::ApplicationError>
        where F: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(BufReader::new(file))
    }
}
