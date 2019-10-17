extern crate flexi_logger;

use flexi_logger::{opt_format, Logger};

pub fn init() {
    Logger::with_env_or_str("rust_test_task=debug")
        .format(opt_format)
        .start()
        .unwrap();
}