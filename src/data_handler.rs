/*
 * data_handler
 *     Loads and saves data from files.
 */
pub mod data 
{
    use crate::error_handler::error;
    //use crate::models::model;
    //use std::fs;
    use std::fs::File;
    //use std::io::Write;
    use std::io::BufReader;
    use std::path::Path;

    /*fn load(model: &mut model::Mapping) -> Result<Option<model::Mapping>, error::ApplicationError>
    {
        let json_data = fs::read_to_string(config::get_mapping_file()).map_err(error::ApplicationError::IoError)?;
        let m: model::Mapping = serde_json::from_str(&json_data).map_err(error::ApplicationError::JsonError)?;
        Ok(Some(m.clone()))
    }

    fn save(model: &mut model::Mapping) -> Result<(), error::ApplicationError>
    {
        let json_data = serde_json::to_string_pretty(&model).unwrap();
        let mut file = File::create(config::get_mapping_file()).map_err(error::ApplicationError::IoError)?;
        file.write_all(json_data.as_bytes()).map_err(error::ApplicationError::IoError)?;
        Ok(())
    }*/

    pub fn read_lines<F>(filename: F) -> Result<BufReader<File>, error::ApplicationError>
        where F: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(BufReader::new(file))
    }
}
