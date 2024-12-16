mod from_dhall;
mod plugin;

use nu_plugin::{serve_plugin, MsgPackSerializer};

fn main() {
    serve_plugin(&crate::plugin::DhallPlugin, MsgPackSerializer {})
}
