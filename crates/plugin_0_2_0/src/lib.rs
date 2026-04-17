use crate::exports::test::plugin::exports::Guest;
use crate::test::plugin::logging;

wit_bindgen::generate!({path: "../../wit/v2"});

struct MyPlugin {}

impl Guest for MyPlugin {
    fn init() {
        logging::debug("Init `MyPlugin`!");
    }
    fn execute() {
        logging::info("Executing `MyPlugin`!");
    }
    fn deinit() -> () {
        logging::debug("Deinit `MyPlugin`!");
    }
}

export!(MyPlugin);
