use std::collections::HashMap;
use std::env;
use crate::config_sources::ConfigSource::ConfigSource;

pub struct EnvironmentVariableManager {
    name: String,
    unread: bool,
    vars: HashMap<String, String>,
}

impl ConfigSource for EnvironmentVariableManager {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn unread(&self) -> bool {
        self.unread
    }

    fn update_from_source(&mut self) {
        // Implement updating from source here
        unread = true;
        return
    }

    fn get_updated_config(&self) -> HashMap<String,String> {
        //parse for dd config here
        unread = false;
        self.vars.clone()
    }
}

impl EnvironmentVariableManager {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        for (key, value) in env::vars() {
            vars.insert(key, value);
        }
        Self {
            name: String::from("Environment-Variable-Manager"),
            unread: true,
            vars
        }
    }
    
    pub fn print_vars(&self) {
        for (key, value) in &self.vars {
            println!("{}: {}", key, value);
        }
    }
}