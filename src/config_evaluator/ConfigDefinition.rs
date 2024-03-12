use std::collections::HashMap;
// use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Result};
// use std::path::Path;


extern crate serde;
// use serde::{Deserialize, Serialize};


static CONFIG_VALS: [&'static str; 3] = ["DD_AGENT_HOST", "DD_TRACE_AGENT_PORT", "DD_TAGS"];
enum ConfigStates {
    ACTIVE,
    INACTIVE,
    PENDING
}

// #[derive(Serialize, Deserialize)]
struct Config {
    DD_AGENT_HOST: String,
    DD_TRACE_AGENT_PORT: String,
    DD_TAGS: HashMap<String,String>,
    config_state: ConfigStates,

    
}

struct ConfigEvaluator {
    active_config: Config,
    last_config: Config,
}

impl ConfigEvaluator{
    fn new() {

    }


    
}

pub fn evaluate_map_for_next_config(mut raw_config_map:HashMap<String,String>) -> Result<HashMap<String,String>,>{
    let mut result:HashMap<String,String> = HashMap::new();
    for &key in CONFIG_VALS.iter() {
        if let Some(val) = raw_config_map.remove(key) {
            result.insert(key.to_string(), val);
        }
    }
    Ok(result)
}