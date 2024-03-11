mod config_sources;
use config_sources::EnvironmentVariableManager::EnvironmentVariableManager;
fn main() {
    let env_manager: EnvironmentVariableManager = EnvironmentVariableManager::new();
    env_manager.print_vars();
}