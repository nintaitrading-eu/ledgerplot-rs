/*
 * data_handler
 *     Loads and saves data from files.
 */
pub mod data 
{
    use crate::error_handler::error;
    use std::fs::{File,OpenOptions};
    use std::io::{Write,BufReader};
    use std::path::Path;

    pub fn read_lines<F>(filename: F) -> Result<BufReader<File>, error::ApplicationError>
        where F: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(BufReader::new(file))
    }

    pub fn append(data: &str, filepath: &str) -> Result<(), error::ApplicationError>
    {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath).map_err(error::ApplicationError::IoError)?;
        file.write_all(data.as_bytes()).map_err(error::ApplicationError::IoError)?;
        Ok(())
    }
}
