/*
 * config_handler
 *     Loads the mapping.
 */
pub mod config
{
    use crate::consts::const_;
    use crate::models::model;
    use crate::error_handler::error;
    use directories_next::ProjectDirs;
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    pub fn get_config_dir() -> PathBuf
    {
        match ProjectDirs::from("com", "nintaitrading", "ledgerplot")
        {
            Some(proj_dirs) => proj_dirs.config_dir().to_path_buf(),
            _ => Path::new(".").to_path_buf(),
        }
    }

    pub fn get_mapping_file() -> PathBuf
    {
        get_config_dir().as_path().join(const_::JSON_MAPPING).to_path_buf()
    }

    pub fn ensure_mapping(model: &mut model::Mapping) -> Result<(), error::ApplicationError>
    {
        let config_dir: PathBuf = get_config_dir();
        if !config_dir.exists()
        {
            println!("Configuration directory does not exist yet, creating a default one at {:?}.", config_dir);
            fs::create_dir_all(config_dir.as_path()).map_err(error::ApplicationError::IoError)?;
        }

        let mapping_file: PathBuf = get_mapping_file();
        if !mapping_file.exists()
        {
            let model = model::Mapping
            {
                records: vec![
                    model::Account
                    {
                        name: "testaccount1".to_string(),
                        assets: vec![
                            model::Asset
                            {
                                name: "testasset1".to_string()
                            },
                            model::Asset
                            {
                                name: "testasset2".to_string()
                            }],
                            ..Default::default()
                    },
                    model::Account
                    {
                        name: "testaccount2".to_string(),
                        assets: vec![
                            model::Asset
                            {
                                name: "testasset3".to_string()
                            }],
                            ..Default::default()
                    }
                ],
                ..Default::default()
            };
            save(model)?;
            println!("Mapping file does not exist yet, creating a default one at {:?}.", Path::new(mapping_file.as_path()));
        }
        Ok(())
    }

    pub fn save(mapping: model::Mapping) -> Result<(), error::ApplicationError>
    {
        let json_data = serde_json::to_string_pretty(&mapping).unwrap();
        let mut file = File::create(get_mapping_file()).map_err(error::ApplicationError::IoError)?;
        file.write_all(json_data.as_bytes()).map_err(error::ApplicationError::IoError)?;
        Ok(())
    }

    pub fn load() -> Result<model::Mapping, error::ApplicationError>
    {
        let json_data = fs::read_to_string(get_mapping_file()).map_err(error::ApplicationError::IoError)?;
        let i: model::Mapping = serde_json::from_str(&json_data).map_err(error::ApplicationError::JsonError)?;
        Ok(i.clone())
    }
}
