use nu_plugin::serve_plugin;
use nu_plugin_from_dhall::FromDhall;

fn main() {
    serve_plugin(&mut FromDhall::new());
}
