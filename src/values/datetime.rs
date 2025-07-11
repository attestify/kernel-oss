#[cfg(target_arch = "wasm32")]
use web_sys::{window};

use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::{Error, Kind};
use crate::error::Kind::InvalidInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    value: u64,
}


impl DateTime {
    pub fn builder() -> DateTimeBuilder {
     DateTimeBuilder::default()
    }
    
    pub fn value(&self) -> &u64 {
        &self.value
    }
    
}

#[derive(Debug, Clone, Default)]
pub struct DateTimeBuilder {
    value: Option<u64>,
    now: bool,
}


impl DateTimeBuilder {

    pub fn now(mut self) -> Self {
        self.now = true;
        self
    }

    pub fn set_at(mut self, millis: u64) -> Self {
        self.value = Some(millis);
        self
    }

    pub fn build(self) -> Result<DateTime, Error> {
        
        let valid_time = match self.now { 
            true => { generate_now()? },
            false => { validate_value(self.value)? }
        };
        Ok(DateTime{ value: valid_time })
    }
    
}

fn generate_now() -> Result<u64, Error> {

    #[cfg(target_arch = "wasm32")]
    {
        let window = window().unwrap();
        let performance = window.performance().unwrap();
        let now_ms = performance.now();
        Ok(now_ms as u64)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH)
            .map_err(|error|
                Error::for_system(Kind::Unexpected, format!("Failed to generate the DateTime for now. {}", error)))?;
        Ok(duration.as_millis() as u64)
    }

}


fn validate_value(value: Option<u64>) -> Result<u64, Error> {
    match value {
        Some(value) => Ok(value),
        None => Err(Error::for_system(InvalidInput, "A value was not provided for the DateTime, please provide a valid DateTime value.".to_string()))
    }
}