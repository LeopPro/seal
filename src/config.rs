use std::fs::File;
use std::io::Read;
use std::env;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> Config {
    let mut config_path = env::current_exe().expect("Can't Get Current Exe Path");
    config_path.pop();
    config_path.push("config");
    config_path.push("seald.conf");
    let mut config_file = File::open(config_path).expect("Open 'config/seald.conf' File Failure");
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).expect("Read 'config/seald.conf' File Failure");
    toml::from_str(&contents).expect("Read 'config/seald.conf' File Failure")
}