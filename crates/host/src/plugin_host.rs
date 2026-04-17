pub struct PluginHost {
    pub wasi_ctx: wasmtime_wasi::WasiCtx,
    pub resource_table: wasmtime_wasi::ResourceTable,
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            wasi_ctx: wasmtime_wasi::WasiCtxBuilder::new().inherit_stdio().build(),
            resource_table: wasmtime_wasi::ResourceTable::new(),
        }
    }
}

impl wasmtime_wasi::WasiView for PluginHost {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        wasmtime_wasi::WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}
