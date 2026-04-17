use crate::plugin_host::PluginHost;

wasmtime::component::bindgen!({
    path: "../../wit/v1_old"
});

impl test::plugin::logging::Host for PluginHost {
    fn log(&mut self, msg: String) {
        println!("[v1] {msg}");
    }
}
