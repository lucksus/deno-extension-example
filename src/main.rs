// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
//!  This example shows you how to define ops in Rust and then call them from
//!  JavaScript.

use deno_core::url::Url;
use deno_core::Extension;
use deno_core::{include_js_files, op};
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::{worker::WorkerOptions, BootstrapOptions};

#[op]
fn op_sum(nums: Vec<f64>) -> Result<f64, deno_core::error::AnyError> {
    // Sum inputs
    let sum = nums.iter().fold(0.0, |a, v| a + v);
    // return as a Result<f64, AnyError>
    Ok(sum)
}

fn main() {
    // Build a deno_core::Extension providing custom ops
    let ext = Extension::builder("my_ext")
        // This JS file adds an EXT object to the global scope
        // with a sum() function that calls the op we defined above
        .js(include_js_files!(wallet "extension.js",))
        .ops(vec![
            // An op for summing an array of numbers
            // The op-layer automatically deserializes inputs
            // and serializes the returned Result & value
            op_sum::decl(),
        ])
        .build();

    // Initialize a runtime through MainWorker
    let mut worker = MainWorker::from_options(
        Url::parse("https://localhost/test").unwrap(),
        PermissionsContainer::allow_all(),
        WorkerOptions {
            extensions: vec![ext],
            ..Default::default()
        },
    );

    worker.bootstrap(&BootstrapOptions::default());

    // The following JS code fails with:
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: TypeError: ops[name] is not a function
    //   at Object.opAsync (ext:core/01_core.js:214:16)
    //   at Object.sum (ext:wallet/extension.js:6:25)
    //   at test:2:5', src/main.rs:60:10
    worker
        .execute_script(
            "test",
            r#"
EXT.sum([1, 2, 3, 4, 5]).then((sum) => {
  console.log("sum", sum);
});
"#,
        )
        .unwrap();
}
