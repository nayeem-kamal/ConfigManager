mod config_sources;
use std::collections::HashMap;

use config_sources::EnvironmentVariableManager::EnvironmentVariableManager;
mod config_evaluator;
use config_evaluator::ConfigDefinition;
use crate::config_sources::ConfigSource::ConfigSource;
#[allow(warnings)]
fn main() {
    let mut env_manager: EnvironmentVariableManager = EnvironmentVariableManager::new();
    //env_manager.print_vars();
    
    let tmpMap: HashMap<String,String> =    
    ConfigDefinition::evaluate_map_for_next_config(
        env_manager.get_updated_config()
       ).expect("");

    for (key, value) in tmpMap {
        println!(" {}: {}", key, value);
    }
    }
