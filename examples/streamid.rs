extern crate gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer::GstBinExt;
use gstreamer_app as gst_app;
use gstreamer_video as gst_video;
use std::error::Error;

const ENCODE_PIPELINE: &str =
    "videotestsrc is-live=false num-buffers=100 ! rsstreamid stream_id=32323 ! appsink name=sink";

fn build_pipeline() -> Result<(gst::Pipeline, gst_app::AppSink), Box<Error>> {
    let pipeline = gst::Pipeline::new(None);
    let elems = gst::parse_launch(ENCODE_PIPELINE)?;
    pipeline.add(&elems)?;

    let sink = pipeline
        .get_by_name("sink")
        .expect("Could not find sink in pipeline");
    // Setup appsink
    let appsink = sink
        .dynamic_cast::<gst_app::AppSink>()
        .expect("Sink should be appsink");
    appsink.set_caps(Some(&gst::Caps::new_simple("video/x-raw", &[])));
    Ok((pipeline, appsink))
}

fn handle_new_sample(appsink: &gst_app::AppSink) -> Result<gst::FlowSuccess, gst::FlowError> {
    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
    let buffer = sample.get_buffer().ok_or_else(|| {
        gst::gst_element_error!(
            appsink,
            gst::ResourceError::Failed,
            ("Failed to get buffer from appsink")
        );
        gst::FlowError::Error
    })?;
    let msg = match buffer.get_meta::<gst_video::VideoMeta>() {
        None => "No Meta",
        Some(meta) => "META",
    };
    println!("{}", msg);

    Ok(gst::FlowSuccess::Ok)
}

pub fn main() {
    gst::init().unwrap();

    let (major, minor, micro, nano) = gst::version();
    println!("Using GStreamer v{}.{}.{}.{}", major, minor, micro, nano);

    let (pipeline, appsink) = build_pipeline().expect("Unable to construct pipeline");
    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::new()
            .new_sample(handle_new_sample)
            .build(),
    );

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set pipeline playing");
    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => break,
            gst::MessageView::Error(err) => {
                eprintln!(
                    "Error from element: {:?}: {}",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error()
                );
                break;
            }
            gst::MessageView::Tag(tag) => println!("TAGS: {}", tag.get_tags()),
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to terminate pipeline");
}
