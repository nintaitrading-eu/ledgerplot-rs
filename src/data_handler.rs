/*
 * data_handler
 *     Loads and saves the data of the kanban issues created via the application.
 */
pub mod data 
{
    use crate::config_handler::config;
    use crate::error_handler::error;
    use crate::models::model;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    pub fn load(model: &mut model::Mapping) -> Result<Option<model::Mapping>, error::ApplicationError>
    {
        let json_data = fs::read_to_string(config::get_mapping_file()).map_err(error::ApplicationError::IoError)?;
        let i: Vec<model::Account> = serde_json::from_str(&json_data).map_err(error::ApplicationError::JsonError)?;
        model.records = i;
        Ok(Some(model.clone()))
    }

    pub fn save(model: &mut model::Mapping) -> Result<(), error::ApplicationError>
    {
        let json_data = serde_json::to_string_pretty(&model.records).unwrap();
        let mut file = File::create(config::get_mapping_file()).map_err(error::ApplicationError::IoError)?;
        file.write_all(json_data.as_bytes()).map_err(error::ApplicationError::IoError)?;
        Ok(())
    }
}
