use std::collections::HashMap;

pub trait ConfigSource {
    // Getters
    fn name(&self) -> String;
    fn unread(&self) -> bool;
    fn print(&self);

    // Methods
    fn update_from_source(&mut self);
    fn get_updated_config(&mut self) -> HashMap<String,String> ;
}