use anyhow::Result;

mod api_v1;
mod api_v1_old;
mod api_v2;
mod plugin_host;

pub enum PluginInstance {
    v1(api_v1::Plugin),
    v2(api_v2::Plugin),
}

pub struct WasmPlugin {
    pub plugin_instance: PluginInstance,
    pub store: wasmtime::Store<plugin_host::PluginHost>,
}

fn instantiate_plugin_v1(
    engine: &wasmtime::Engine,
    linker: &wasmtime::component::Linker<plugin_host::PluginHost>,
    component: &wasmtime::component::Component,
) -> Result<WasmPlugin> {
    let inst_pre = linker.instantiate_pre(&component)?;

    // We make the store preemptively, under the assumption that the plugin is valid.
    // Though this does waste some resources for incompatible components
    let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

    {
        let result = api_v1::PluginPre::new(inst_pre.clone());
        if result.is_ok() {
            let plugin_pre = result.unwrap();
            return Ok(WasmPlugin {
                plugin_instance: PluginInstance::v1(plugin_pre.instantiate(&mut store)?),
                store,
            });
        }
    }

    Err(anyhow::format_err!(
        "Plugin could not be loaded. Unsupported version."
    ))
}

fn instantiate_plugin_v2(
    engine: &wasmtime::Engine,
    linker: &wasmtime::component::Linker<plugin_host::PluginHost>,
    component: &wasmtime::component::Component,
) -> Result<WasmPlugin> {
    let inst_pre = linker.instantiate_pre(&component)?;

    // We make the store preemptively, under the assumption that the plugin is valid.
    // Though this does waste some resources for incompatible components
    let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

    {
        let result = api_v2::PluginPre::new(inst_pre.clone());
        if result.is_ok() {
            let plugin_pre = result.unwrap();
            return Ok(WasmPlugin {
                plugin_instance: PluginInstance::v2(plugin_pre.instantiate(&mut store)?),
                store,
            });
        }
    }

    {
        let result = api_v1::PluginPre::new(inst_pre.clone());
        if result.is_ok() {
            let plugin_pre = result.unwrap();
            return Ok(WasmPlugin {
                plugin_instance: PluginInstance::v1(plugin_pre.instantiate(&mut store)?),
                store,
            });
        }
    }

    Err(anyhow::format_err!(
        "Plugin could not be loaded. Unsupported version."
    ))
}

fn execute_plugin(plugin: &mut WasmPlugin) -> Result<()> {
    match &plugin.plugin_instance {
        PluginInstance::v1(inst) => {
            inst.test_plugin_exports().call_execute(&mut plugin.store)?;
        }
        PluginInstance::v2(inst) => {
            inst.test_plugin_exports().call_init(&mut plugin.store)?;
            inst.test_plugin_exports().call_execute(&mut plugin.store)?;
            inst.test_plugin_exports().call_deinit(&mut plugin.store)?;
        }
    };
    Ok(())
}

fn host_v1(engine: &wasmtime::Engine) -> Result<()> {
    println!("Plugin host v1 -- implements 0.1.0");

    let mut linker = wasmtime::component::Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    api_v1_old::Plugin::add_to_linker::<_, wasmtime::component::HasSelf<_>>(
        &mut linker,
        |state| state,
    )?;

    println!("Run plugin targetting 0.1.0");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_0.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut plugin = instantiate_plugin_v1(engine, &linker, &component)?;
        execute_plugin(&mut plugin)?;
    }

    println!("Run plugin targetting 0.1.4, but using only 0.1.0 APIs");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_4_subset.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut plugin = instantiate_plugin_v1(engine, &linker, &component)?;
        execute_plugin(&mut plugin)?;
    }

    println!("Run plugin targetting 0.1.4");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_4.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let err = instantiate_plugin_v1(engine, &linker, &component);
        if err.is_ok() {
            anyhow::bail!("Succeeded???");
        } else {
            println!("Good! Failed: {}", err.err().unwrap());
        }
    }

    println!("Run plugin targetting 0.2.0");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_2_0.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let err = instantiate_plugin_v1(engine, &linker, &component);
        if err.is_ok() {
            anyhow::bail!("Succeeded???");
        } else {
            println!("Good! Failed: {}", err.err().unwrap());
        }
    }

    Ok(())
}

fn host_v2(engine: &wasmtime::Engine) -> Result<()> {
    println!("Plugin host v2 -- implements 0.2.0 and 0.1.4");

    let mut linker = wasmtime::component::Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    api_v1::Plugin::add_to_linker::<_, wasmtime::component::HasSelf<_>>(&mut linker, |state| {
        state
    })?;
    api_v2::Plugin::add_to_linker::<_, wasmtime::component::HasSelf<_>>(&mut linker, |state| {
        state
    })?;

    println!("Run plugin targetting 0.1.0");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_0.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut plugin = instantiate_plugin_v2(engine, &linker, &component)?;
        execute_plugin(&mut plugin)?;
    }

    println!("Run plugin targetting 0.1.4");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_4.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut plugin = instantiate_plugin_v2(engine, &linker, &component)?;
        execute_plugin(&mut plugin)?;
    }

    println!("Run plugin targetting 0.2.0");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_2_0.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut plugin = instantiate_plugin_v2(engine, &linker, &component)?;
        execute_plugin(&mut plugin)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);

    let engine = wasmtime::Engine::default();

    host_v1(&engine)?;
    println!("");
    host_v2(&engine)?;

    Ok(())
}
