mod ext;
mod recommended;
use andromeda_core::{Runtime, RuntimeConfig};
pub use ext::*;
use gdnative::init::{godot_init, InitHandle};
pub use recommended::*;

// use gdnative::api::MeshInstance;
// use gdnative::export::hint::{EnumHint, IntHint, StringHint};
// use gdnative::prelude::*;

fn init(_handle: InitHandle) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut runtime = Runtime::new(RuntimeConfig {
            no_strict: false,
            paths: vec!["./main.ts".to_string()],
            verbose: true,
            extensions: recommended_extensions(),
            builtins: recommended_builtins(),
            eventloop_handler: recommended_eventloop_handler,
        });
        let _runtime_result = runtime.run();
    });
}

godot_init!(init);