pub mod particles;

pub fn plugins(app: &mut bevy::app::App) {
    app.add_plugins((particles::emitters::trail::plugin, particles::magic::plugin));
}
