mod overworld;
pub use overworld::Overworld;

mod nether;
pub use nether::Nether;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy::app::App) {
    app.register_type::<Nether>();
    app.register_type::<Overworld>();
}
