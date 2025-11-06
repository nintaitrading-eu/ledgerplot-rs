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

    pub fn get_config_file() -> PathBuf
    {
        get_config_dir().as_path().join(const_::JSON_CONFIG).to_path_buf()
    }

    pub fn ensure_config(model: &mut model::Configuration) -> Result<(), error::ApplicationError>
    {
        let config_dir: PathBuf = get_config_dir();
        if !config_dir.exists()
        {
            println!("Configuration directory does not exist yet, creating a default one at {:?}.", config_dir);
            fs::create_dir_all(config_dir.as_path()).map_err(error::ApplicationError::IoError)?;
        }

        let config_file: PathBuf = get_config_file();
        if !config_file.exists()
        {
            model.range1_low = 0.0;
            model.range1_high = 9999.0;
            model.range2_low = 10000.0;
            model.range2_high = 24999.0;
            model.range3_low = 25000.0;
            model.range3_high = 49999.0;
            model.range4_low = 50000.0;
            model.range4_high = 749999.0;
            model.range5_low = 75000.0;
            model.range5_high = 999999.0;
            model.range6_low = 100000.0;
            save(model)?;
            println!("Configuration file does not exist yet, creating a default one at {:?}.", Path::new(config_file.as_path()));
        }
        Ok(())
    }

    fn save(config: &mut model::Configuration) -> Result<(), error::ApplicationError>
    {
        let json_data = serde_json::to_string_pretty(&config).unwrap();
        let mut file = File::create(get_config_file()).map_err(error::ApplicationError::IoError)?;
        file.write_all(json_data.as_bytes()).map_err(error::ApplicationError::IoError)?;
        Ok(())
    }

    pub fn load() -> Result<model::Configuration, error::ApplicationError>
    {
        let json_data = fs::read_to_string(get_config_file()).map_err(error::ApplicationError::IoError)?;
        let m: model::Configuration = serde_json::from_str(&json_data).map_err(error::ApplicationError::JsonError)?;
        Ok(m.clone())
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
            model.records = vec![
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
                ];
            save_mapping(model)?;
            println!("Mapping file does not exist yet, creating a default one at {:?}.", Path::new(mapping_file.as_path()));
        }
        Ok(())
    }

    fn save_mapping(mapping: &mut model::Mapping) -> Result<(), error::ApplicationError>
    {
        let json_data = serde_json::to_string_pretty(&mapping).unwrap();
        let mut file = File::create(get_mapping_file()).map_err(error::ApplicationError::IoError)?;
        file.write_all(json_data.as_bytes()).map_err(error::ApplicationError::IoError)?;
        Ok(())
    }

    pub fn load_mapping() -> Result<model::Mapping, error::ApplicationError>
    {
        let json_data = fs::read_to_string(get_mapping_file()).map_err(error::ApplicationError::IoError)?;
        let m: model::Mapping = serde_json::from_str(&json_data).map_err(error::ApplicationError::JsonError)?;
        Ok(m.clone())
    }
}
