use crate::exports::test::plugin::exports::Guest;
use crate::test::plugin::logging;

wit_bindgen::generate!({path: "../../wit/v1"});

struct MyPlugin {}

impl Guest for MyPlugin {
    fn execute() {
        logging::log("Executing `MyPlugin`!");
    }
}

export!(MyPlugin);
