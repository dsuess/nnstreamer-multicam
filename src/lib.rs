#[macro_use]
extern crate gstreamer as gst;

use glib;
use gstreamer_base as gst_base;
use once_cell;

pub mod meta;
mod streamid;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    streamid::register(plugin)?;
    Ok(())
}

gst::gst_plugin_define!(
    streamid,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MIT/X11",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
