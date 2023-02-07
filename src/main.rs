use std::path::PathBuf;

use anyhow::{Context, Result};
use host::add_to_linker;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};

use crate::handler::{Method, Request};

wasmtime::component::bindgen!({
    world: "world",
    path: "./wit",
    async: true,}
);

#[tokio::main]
async fn main() -> Result<()> {
    let input = PathBuf::from(
        std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .context("must provide an input file")?,
    );

    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    // add wasi to linker
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(
        &engine,
        WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stdout()
            .build(),
    );
    add_to_linker(&mut linker, |x| x)?;

    // get component
    let component = Component::from_file(&engine, &input)?;
    let (s, _) = Serverless::instantiate_async(&mut store, &component, &linker).await?;
    let handler = s.handler();

    // create a request
    let req = Request {
        method: Method::Get,
        uri: "http://localhost:8080/",
        headers: &vec![("content-type", "application/json")],
        params: &vec![("name", "world")],
        body: None,
    };

    let res = handler.call_handle_http(&mut store, req).await?;

    println!("Host: We have a response!");
    match res {
        Ok(res) => {
            println!("Host: status: {}", res.status);
            println!("Host: headers: {:?}", res.headers);
            if let Some(body) = res.body {
                println!("Host: body: {}", String::from_utf8_lossy(&body));
            }
        }
        Err(err) => println!("{:?}", err),
    }
    Ok(())
}
