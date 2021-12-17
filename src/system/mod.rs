mod logger;
use logger::configure_logger;

mod utils;
use utils::{read_env_vars};


pub fn bootstrap_system()  {
  read_env_vars();
  configure_logger();
}