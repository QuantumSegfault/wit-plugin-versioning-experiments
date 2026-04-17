use std::fmt::Display;

use crate::api_v2::test::plugin::logging::Severity;
use crate::plugin_host::PluginHost;

wasmtime::component::bindgen!({
    path: "../../wit/v2"
});

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Severity::Trace => "Trace",
            Severity::Debug => "Debug",
            Severity::Info => "Info",
            Severity::Warning => "Warning",
            Severity::Error => "Error",
            Severity::FatalError => "FatalError",
        })
    }
}

impl test::plugin::logging::Host for PluginHost {
    fn log(&mut self, severity: Severity, msg: String) {
        println!("[{severity} - v2] {msg}");
    }
    fn trace(&mut self, msg: String) {
        self.log(Severity::Trace, msg);
    }
    fn debug(&mut self, msg: String) {
        self.log(Severity::Debug, msg);
    }
    fn info(&mut self, msg: String) {
        self.log(Severity::Info, msg);
    }
    fn warning(&mut self, msg: String) {
        self.log(Severity::Warning, msg);
    }
    fn error(&mut self, msg: String) {
        self.log(Severity::Error, msg);
    }
    fn fatal_error(&mut self, msg: String) {
        self.log(Severity::FatalError, msg);
    }
}
