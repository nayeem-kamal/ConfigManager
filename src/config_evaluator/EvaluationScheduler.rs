use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use crate::config_sources::ConfigSource::ConfigSource;

pub struct EvaluationScheduler {
    sources: HashMap<u32, Box<dyn ConfigSource>>,
}

impl EvaluationScheduler {
    pub fn new() -> Self {
        Self { sources: HashMap::new() }
    }

    pub fn add_source(&mut self, priority: u32, source: Box<dyn ConfigSource>) {
        self.sources.insert(priority, source);
    }

    pub fn start(&mut self) {
        let mut sorted_sources = self.sources.iter_mut().collect::<Vec<_>>();
        sorted_sources.sort_by_key(|a: &(&u32, &mut Box<dyn ConfigSource>)| a.0);
        let mut i:u32 = 0;
        loop {
            for (_, source) in &mut sorted_sources {
                source.get_updated_config();
                source.print();

            }
            thread::sleep(Duration::from_secs(10));
        }
    }
}
