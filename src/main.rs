mod config_sources;

use config_evaluator::EvaluationScheduler::EvaluationScheduler;
use config_sources::EnvironmentVariableManager::EnvironmentVariableManager;
mod config_evaluator;
#[allow(warnings)]

use std::sync::{Arc, Mutex};
extern crate tokio;

pub fn main() {
    let mut scheduler = EvaluationScheduler::new();
    let mut env_manager: EnvironmentVariableManager = EnvironmentVariableManager::new();
    scheduler.add_source(1,Box::new(env_manager));
    scheduler.start();
}


