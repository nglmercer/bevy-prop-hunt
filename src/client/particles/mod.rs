pub mod emitters;
pub mod magic;

pub fn plugins(app: &mut bevy::app::App) {
    app.add_plugins((emitters::trail::plugin, magic::plugin));
}
