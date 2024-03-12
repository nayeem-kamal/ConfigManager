use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;


extern crate ed25519_dalek;
use ed25519_dalek::PublicKey;
use reqwest::Client;

//defaults taken from java tracer 
pub const RC_TARGET_KEY_ID: String = String::from("5c4ece41241a1bb513f6e3e5df74ab7d5183dfffbd71bfd43127920d880569fd");
pub const RC_MAX_PAYLOAD_SIZE: usize = 5120*1024;
pub const RC_TARGETS_KEY: String = String::from("e3f1f98c9da02a93bb547f448b472d727e14b22455235796fe49863856252508");
pub const INTEGRITY_CHECKS: bool = false; 
pub const CAPABILITY_APM_TRACING_SAMPLE_RATE: u64 = 1 << 12;
pub const CAPABILITY_APM_LOGS_INJECTION: u64 = 1 << 13;
pub const CAPABILITY_APM_HTTP_HEADER_TAGS: u64 = 1 << 14;
pub const CAPABILITY_APM_CUSTOM_TAGS: u64 = 1 << 15;
pub const CAPABILITY_ASM_PROCESSOR_OVERRIDES: u64 = 1 << 16;
pub const CAPABILITY_ASM_CUSTOM_DATA_SCANNERS: u64 = 1 << 17;
pub const CAPABILITY_ASM_EXCLUSION_DATA: u64 = 1 << 18;
pub const CAPABILITY_APM_TRACING_TRACING_ENABLED: u64 = 1 << 19;

//end of defaults

pub struct ConfigPoller{
    key_id: String,
    key: PublicKey,
    tracer_version: String,
    container_ID: String,
    entity_ID: String,
    http_client: Arc<Client>,
    url: String,
    max_payload_size: usize,
    integrity_checks: bool,
    productStates: HashMap<String, String>,
    startCount: AtomicUsize,
    capabilities: u64,
    //durationHint: Duration,
    fatal_on_inititialization: bool,
}
impl ConfigPoller {
    pub fn new(
        tracer_version: String,
        container_id: String,
        entity_id: String,
        url: String,
        http_client: Arc<Client>,
    ) -> Result<ConfigPoller, &'static str> {
        //values are hardcoded for now


        let key_id = RC_TARGET_KEY_ID;
        let key_str = RC_TARGETS_KEY;
        let key = PublicKey::from_byte_array(HexUtils::from_hex_string(key_str.clone()))
            .map_err(|_| "Bad public key")?;


        //start rc here?
        
        let max_payload_size = RC_MAX_PAYLOAD_SIZE;
        let integrity_checks = INTEGRITY_CHECKS;
        let mut capabilities: u64 = 0;

        let capabilities_bits=
                CAPABILITY_APM_CUSTOM_TAGS
                | CAPABILITY_APM_TRACING_TRACING_ENABLED 
                | CAPABILITY_APM_TRACING_SAMPLE_RATE
                | CAPABILITY_APM_LOGS_INJECTION
                ;


        capabilities |= capabilities_bits;

        Ok(ConfigPoller {
            tracer_version:tracer_version,
            container_ID:container_id,
            entity_ID:entity_id,
            url:url,
            key_id:key_id,
            key:key,
            max_payload_size:max_payload_size,
            integrity_checks:integrity_checks,
            http_client:http_client,
            capabilities: capabilities,
            fatal_on_inititialization:false,
            startCount:AtomicUsize::new(0),
            productStates:HashMap::new()
        })
    }
    pub fn add_capabilities(&self, flags: u64) {
        self.capabilities |= flags;
    }
    

}