use crate::exports::test::plugin::exports::Guest;
use crate::test::plugin::logging;

wit_bindgen::generate!({path: "../../wit/v1_old"});

struct MyPlugin {}

impl Guest for MyPlugin {
    fn execute() {
        logging::log("Executing `MyPlugin`!");
    }
}

export!(MyPlugin);
