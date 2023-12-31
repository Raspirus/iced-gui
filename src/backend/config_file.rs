use std::fs::{self, File};
use std::io::{self, Read};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub hashes_in_db: u32,
    pub last_db_update: String,
    pub logging_is_active: bool,
    pub obfuscated_is_active: bool,
    pub db_update_weekday: i32,
    pub db_update_time: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            hashes_in_db: 0,
            last_db_update: "Never".to_string(),
            logging_is_active: false,
            obfuscated_is_active: true,
            db_update_weekday: -1,
            db_update_time: "22:00:00".to_string(),
        }
    }

    pub fn set_path(&self) -> Result<String, io::Error> {
        // Sets the path/location of where to store the config files
        let project_dirs =
            ProjectDirs::from("com", "Raspirus", "Data").expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(&program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join("raspirus.config.json");
        let db_file_str = db_file_path.to_str().expect("Failed to get database path");
        Ok(db_file_str.to_string())
    }

    pub fn save(&self) -> Result<(), io::Error> {
        // Writes to the path set above
        let path = self.set_path().expect("Couldn't get path to Data directories");
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn load(&self) -> Result<Self, io::Error> {
        // Returns this class as object with data filled from file
        let path = Config::set_path(&self)?;
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
