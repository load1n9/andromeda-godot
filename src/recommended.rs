use andromeda_core::{Extension, HostData};
use nova_vm::ecmascript::execution::agent::{GcAgent, RealmRoot};

use crate::ConsoleExt;

pub fn recommended_extensions() -> Vec<Extension> {
    vec![ConsoleExt::new_extension()]
}

pub fn recommended_builtins() -> Vec<&'static str> {
    vec![
        include_str!("../namespace/console.ts"),
        include_str!("../namespace/mod.ts"),
    ]
}


pub enum RuntimeMacroTask {
}


pub fn recommended_eventloop_handler(
    macro_task: RuntimeMacroTask,
    _agent: &mut GcAgent,
    _realm_root: &RealmRoot,
    _host_data: &HostData<RuntimeMacroTask>,
) {
    match macro_task {
        
    }
}