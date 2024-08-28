use wasmtime::{Engine, Result, Store, Config};  
use wasmtime::component::{ResourceTable, Linker, bindgen, Component};  
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};  
  
#[tokio::main]  
async fn main() -> Result<()> {  
    let mut config = Config::new();  
    config.async_support(true);  
    config.wasm_component_model(true);  
    config.debug_info(true);  
    let engine = Engine::new(&config)?;  
  
    bindgen!({world: "new-world", path: "../plugin_api/wit/world.wit", async: true});  
  
    let mut linker = Linker::new(&engine);  
  
    wasmtime_wasi::add_to_linker_async(&mut linker)?;  
  
    // Only needed if our wit world had imports
    //NewWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;  
  
    // ... configure `builder` more to add env vars, args, etc ...    
    let mut builder = WasiCtxBuilder::new();
    // Without this, not even println! would work. What WASI functions plugins have access to is defined here  
    builder.inherit_stdio();  
    let mut store = Store::new(  
        &engine,  
        MyState {  
            ctx: builder.build(),  
            table: ResourceTable::new(),  
        },  
    );  
    // Load the plugin file from disk
    let component = Component::from_file(&engine, "./plugins/custom_plugin.wasm")?;  
    
    // Setup an Instance and a world Component that can be called
    // Instance needs to be not dropped when the world component is called.
    let (world, instance) = NewWorld::instantiate_async(&mut store, &component, &linker).await?;  
  
    // Here our `greet` function doesn't take any parameters for the component,  
    // but in the Wasmtime embedding API the first argument is always a `Store`. 
    let greeting = world.call_greeting(&mut store).await?;  
    println!("{greeting}");  
    Ok(())  
}  
  
struct MyState {  
    ctx: WasiCtx,  
    table: ResourceTable,  
}  
  
impl WasiView for MyState {  
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }  
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }  
}
