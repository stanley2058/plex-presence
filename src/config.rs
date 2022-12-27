pub mod config {
    use std::{fs, path::{Path, PathBuf}, process::exit};
    use toml;
    use directories::ProjectDirs;
    use serde::{Deserialize, Serialize};

    fn discord_application_id() -> String {
        String::from("1057190283017719828")
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Config {
        pub host: String,
        pub port: String,
        #[serde(skip)]
        pub origin: String,
        pub token: String,
        #[serde(skip, default="discord_application_id")]
        pub discord_application_id: String,
    }


    impl Default for Config {
        fn default() -> Self {
            let empty_string = String::from("");
            Config {
                host: empty_string.clone(),
                port: empty_string.clone(),
                origin: empty_string.clone(),
                token: empty_string.clone(),
                discord_application_id: discord_application_id(),
            }
        }
    }

    impl Config {
        pub fn get_project_dir() -> ProjectDirs {
            ProjectDirs::from("", "", "plex-presence").unwrap()
        }

        pub fn get_config_dir() -> String {
            let project_dir = Config::get_project_dir();
            let config_dir = project_dir.config_dir();
            if !config_dir.exists() {
                let res = fs::create_dir_all(config_dir);
                if res.is_err() {
                    panic!("failed to create config directory, {:#?}", res.unwrap_err())
                }
            }

            String::from(config_dir.as_os_str().to_str().unwrap())
        }

        pub fn get_lockfile() -> PathBuf {
            let config_dir = Config::get_config_dir();
            let lockfile = Path::new(config_dir.as_str()).join("plex-presense.lock");
            lockfile
        }


        fn read_config_file() -> String {
            let config_folder = Config::get_config_dir();

            let config_file_path = Path::new(config_folder.as_str()).join("config.toml");
            if !config_file_path.exists() {
                let default_conf = toml::to_string(&Config::default()).unwrap();
                let res = fs::write(config_file_path.clone(), default_conf);
                if res.is_err() {
                    panic!("failed to create default config file, {:#?}", res.unwrap_err())
                }
                println!("default config file created at: {}", config_file_path.to_str().unwrap());
                println!("please complete the config file and relaunch this program");
                exit(0);
            }

            let res = fs::read_to_string(config_file_path);
            if res.is_err() {
                panic!("cannot read config file, {:#?}", res.unwrap_err());
            }
            res.unwrap()
        }

        pub fn load() -> Self {
            let raw_config = Config::read_config_file();
            let parsed = toml::from_str::<Config>(&raw_config);
            if parsed.is_err() {
                panic!("cannot parse config, {:#?}", parsed.unwrap_err())
            }

            let mut config = parsed.unwrap();
            config.origin = format!("{}:{}", config.host, config.port);
            config
        }
    }
}
