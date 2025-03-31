use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::network::Network;

lazy_static! {
    pub static ref NETWORK: Mutex<Option<Network>> = Mutex::new(None);
}
