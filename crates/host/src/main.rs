use anyhow::Result;

mod api_v1;
mod api_v1_old;
mod api_v2;
mod plugin_host;

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

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let plugin = api_v1_old::Plugin::instantiate(&mut store, &component, &linker)?;
        plugin.test_plugin_exports().call_execute(&mut store)?;
    }

    println!("Run plugin targetting 0.1.4, but using only 0.1.0 APIs");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_4_subset.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let plugin = api_v1_old::Plugin::instantiate(&mut store, &component, &linker)?;
        plugin.test_plugin_exports().call_execute(&mut store)?;
    }

    println!("Run plugin targetting 0.1.4");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_4.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let err = api_v1_old::Plugin::instantiate(&mut store, &component, &linker);
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

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let err = api_v1_old::Plugin::instantiate(&mut store, &component, &linker);
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

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let plugin = api_v1::Plugin::instantiate(&mut store, &component, &linker)?;
        plugin.test_plugin_exports().call_execute(&mut store)?;
    }

    println!("Run plugin targetting 0.1.4");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_4.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let plugin = api_v1::Plugin::instantiate(&mut store, &component, &linker)?;
        plugin.test_plugin_exports().call_execute(&mut store)?;
    }

    println!("Run plugin targetting 0.2.0");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_2_0.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let plugin = api_v2::Plugin::instantiate(&mut store, &component, &linker)?;

        plugin.test_plugin_exports().call_init(&mut store)?;
        plugin.test_plugin_exports().call_execute(&mut store)?;
        plugin.test_plugin_exports().call_deinit(&mut store)?;
    }

    println!("Run plugin targetting 0.2.0 but with 0.1.4 API!");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_2_0.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let err = api_v1::Plugin::instantiate(&mut store, &component, &linker);
        if err.is_ok() {
            anyhow::bail!("Succeeded???");
        } else {
            println!("Good! Failed: {}", err.err().unwrap());
        }
    }

    println!("Run plugin targetting 0.1.0 but with 0.2.0 API!");
    {
        let bytes = std::fs::read("target/wasm32-wasip2/debug/plugin_0_1_0.wasm")?;
        let component = wasmtime::component::Component::new(&engine, bytes)?;

        let mut store = wasmtime::Store::new(&engine, plugin_host::PluginHost::new());

        let err = api_v2::Plugin::instantiate(&mut store, &component, &linker);
        if err.is_ok() {
            anyhow::bail!("Succeeded???");
        } else {
            println!("Good! Failed: {}", err.err().unwrap());
        }
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
