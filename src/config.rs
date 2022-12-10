use serde_yaml::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    log_level: String,
    db_url: String,
    port: u16,
    username: String,
    password: String,
    device_id: String,
    tracker_id: String,
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }
    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }
    pub fn get_db_url(&self) -> &str{
        &self.db_url
    }
    pub fn get_port(&self) -> u16{
        self.port
    }
    pub fn get_username(&self) -> &str{
        &self.username
    }
    pub fn get_password(&self) -> &str{
        &self.password
    }
    pub fn get_device_id(&self) -> &str{
        &self.device_id
    }
    pub fn get_tracker_id(&self) -> &str{
        &self.tracker_id
    }
}
