mod ext;
mod recommended;
use andromeda_core::{Runtime, RuntimeConfig};
pub use ext::*;
use gdnative::{
    api::MeshInstance,
    derive::methods,
    export::{
        hint::{EnumHint, IntHint, StringHint},
        ClassBuilder,
    },
    init::{godot_init, InitHandle},
    prelude::{Color, Vector3},
};
pub use recommended::*;

// use gdnative::api::MeshInstance;
// use gdnative::export::hint::{EnumHint, IntHint, StringHint};
// use gdnative::prelude::*;

#[derive(gdnative::derive::NativeClass)]
#[inherit(MeshInstance)]
#[register_with(register_members)]
struct AndromedaTest {
    start: Vector3,
    time: f32,
    #[property(path = "base/rotate_speed")]
    rotate_speed: f64,
}

fn register_members(builder: &ClassBuilder<AndromedaTest>) {
    builder
        .property::<String>("test/test_enum")
        .with_hint(StringHint::Enum(EnumHint::new(vec![
            "Hello".into(),
            "World".into(),
            "Testing".into(),
        ])))
        .with_getter(|_: &AndromedaTest, _| "Hello".to_string())
        .done();

    builder
        .property("test/test_flags")
        .with_hint(IntHint::Flags(EnumHint::new(vec![
            "A".into(),
            "B".into(),
            "C".into(),
            "D".into(),
        ])))
        .with_getter(|_: &AndromedaTest, _| 0)
        .done();
}

#[methods]
impl AndromedaTest {
    fn new(_owner: &MeshInstance) -> Self {
        AndromedaTest {
            start: Vector3::new(0.0, 0.0, 0.0),
            time: 0.0,
            rotate_speed: 1.15,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] owner: &MeshInstance) {
        owner.set_physics_process(true);
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &MeshInstance, delta: f64) {
        use gdnative::api::SpatialMaterial;

        self.time += delta as f32;
        owner.rotate_y(self.rotate_speed * delta);

        let offset = Vector3::new(0.0, 1.0, 0.0) * self.time.cos() * 0.5;
        owner.set_translation(self.start + offset);

        if let Some(mat) = owner.get_surface_material(0) {
            let mat = unsafe { mat.assume_safe() };
            let mat = mat.cast::<SpatialMaterial>().expect("Incorrect material");
            mat.set_albedo(Color::from_rgba(self.time.cos().abs(), 0.0, 0.0, 1.0));
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<AndromedaTest>();

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
        let _ = runtime.run();
    });
}

godot_init!(init);
