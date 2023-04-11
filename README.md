# Example to reproduce `ops[name] is not a function` error 
Based off of `hello_world` example from deno core: https://github.com/denoland/deno/blob/main/core/examples/hello_world.rs

This changes two things:
 * using deno_runtime's MainWorker as interface instead of deno_core's JsRuntime
 * adding a JS file to the extension which provides JS wrapper functions around Deno.core.opAsync

## Run
Just do 
```
cargo run
```
