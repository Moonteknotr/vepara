use std::sync::Arc;
use crate::{ClientOptions};

#[derive(Clone)]
pub struct Payment {
    options: Arc<ClientOptions>
}

impl Payment {
    pub fn new(options: &Arc<ClientOptions>) -> Self {
        Payment {
            options: options.clone()
        }
    }
    
    
}