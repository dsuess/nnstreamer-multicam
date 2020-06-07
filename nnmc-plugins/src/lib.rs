extern crate gstreamer as gst;
use glib;

pub mod meta;
mod muxer;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    muxer::register(plugin)?;
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
