use std::collections::HashMap;

pub trait ConfigSource {
    // Getters
    fn name(&self) -> String;
    fn unread(&self) -> bool;

    // Methods
    fn update_from_source(&mut self);
    fn get_updated_config(&self) -> HashMap<String,String> ;
}