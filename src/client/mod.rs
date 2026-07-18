pub mod particles;
pub mod ui;

pub fn plugins(app: &mut bevy::app::App) {
    app.add_plugins((particles::plugins, ui::crosshair::plugin));
}
