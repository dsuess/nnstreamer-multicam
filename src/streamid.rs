use glib;
use glib::subclass;
use glib::subclass::prelude::*;

use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer::subclass::prelude::*;

use gstreamer_base as gst_base;
use gstreamer_base::subclass::base_transform::PrepareOutputBufferSuccess;
use gstreamer_base::subclass::prelude::*;
use gstreamer_video as gst_video;

use once_cell::sync::Lazy;
use std::i32;
use std::sync::Mutex;

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "rsstreamid",
        gst::DebugColorFlags::empty(),
        Some("Adds stream-id to buffer"),
    )
});

#[derive(Debug, Clone, Copy)]
struct Settings {
    stream_id: u32,
}

const DEFAULT_STREAM_ID: u32 = 0;

impl Default for Settings {
    fn default() -> Self {
        Settings {
            stream_id: DEFAULT_STREAM_ID,
        }
    }
}

static PROPERTIES: [subclass::Property; 1] = [subclass::Property("stream_id", |name| {
    glib::ParamSpec::uint(
        name,
        "Stream ID",
        "Assigned stream ID",
        0,
        u32::MAX,
        DEFAULT_STREAM_ID,
        glib::ParamFlags::READWRITE,
    )
})];

struct State {
    in_info: gst_video::VideoInfo,
    out_info: gst_video::VideoInfo,
}

struct StreamIdElement {
    state: Mutex<Option<State>>,
    settings: Mutex<Settings>,
}

impl StreamIdElement {
    fn setup_pads(klass: &mut subclass::simple::ClassStruct<Self>) -> Result<(), glib::BoolError> {
        // FIXME Make more general
        let caps = gst::Caps::new_simple("video/x-raw", &[]);

        let src_pad_template = gst::PadTemplate::new(
            "src",
            gst::PadDirection::Src,
            gst::PadPresence::Always,
            &caps,
        )?;
        klass.add_pad_template(src_pad_template);

        let caps = gst::Caps::new_simple("video/x-raw", &[]);
        let sink_pad_template = gst::PadTemplate::new(
            "sink",
            gst::PadDirection::Sink,
            gst::PadPresence::Always,
            &caps,
        )?;
        klass.add_pad_template(sink_pad_template);

        Ok(())
    }
}

// GObject related boilerplate
impl ObjectSubclass for StreamIdElement {
    const NAME: &'static str = "RsStreamId";
    type ParentType = gst_base::BaseTransform;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib::glib_object_subclass!();

    fn new() -> Self {
        Self {
            state: Mutex::new(None),
            settings: Mutex::new(Default::default()),
        }
    }

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        //klass.install_properties(&PROPERTIES);
        klass.set_metadata(
            "StreamIdElement",
            "???",
            "Attaches metadata to buffer with stream id",
            "DS",
        );

        klass.configure(
            gst_base::subclass::BaseTransformMode::AlwaysInPlace,
            false,
            true,
        );

        klass.install_properties(&PROPERTIES);

        StreamIdElement::setup_pads(klass).unwrap();
    }
}

impl ObjectImpl for StreamIdElement {
    glib::glib_object_impl!();

    fn set_property(&self, obj: &glib::Object, id: usize, value: &glib::Value) {
        let prop = &PROPERTIES[id];
        let element = obj.downcast_ref::<gst_base::BaseTransform>().unwrap();

        match *prop {
            subclass::Property("stream_id", ..) => {
                let mut settings = self.settings.lock().unwrap();
                let stream_id = value.get_some().expect("type checked upstream");

                gst::gst_info!(
                    CAT,
                    obj: element,
                    "Setting stream_id from {} to {}",
                    settings.stream_id,
                    stream_id
                );

                settings.stream_id = stream_id;
            }
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id];

        match *prop {
            subclass::Property("stream_id", ..) => {
                Ok(self.settings.lock().unwrap().stream_id.to_value())
            }
            _ => unimplemented!(),
        }
    }
}

impl ElementImpl for StreamIdElement {}
impl BaseTransformImpl for StreamIdElement {
    fn set_caps(
        &self,
        element: &gst_base::BaseTransform,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        let in_info = match gst_video::VideoInfo::from_caps(incaps) {
            Err(..) => return Err(gst::gst_loggable_error!(CAT, "failed to parse input caps")),
            Ok(info) => info,
        };
        let out_info = match gst_video::VideoInfo::from_caps(outcaps) {
            Err(..) => return Err(gst::gst_loggable_error!(CAT, "failed to parse output caps")),
            Ok(info) => info,
        };

        gst::gst_debug!(
            CAT,
            obj: element,
            "Configured for caps {} -> {}",
            incaps,
            outcaps
        );
        *self.state.lock().unwrap() = Some(State { in_info, out_info });
        Ok(())
    }

    fn stop(&self, element: &gst_base::BaseTransform) -> Result<(), gst::ErrorMessage> {
        let _ = self.state.lock().unwrap().take();
        gst::gst_info!(CAT, obj: element, "Stopped");
        Ok(())
    }

    fn transform_ip(
        &self,
        element: &gst_base::BaseTransform,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        Ok(gst::FlowSuccess::Ok)
    }

    fn prepare_output_buffer(
        &self,
        _element: &gst_base::BaseTransform,
        inbuf: &gst::BufferRef,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError> {
        // FIXME This should be a constant
        let mut copy_flags = gst::BufferCopyFlags::all();
        copy_flags.remove(gst::BufferCopyFlags::DEEP);
        let mut buf = inbuf
            .copy_region(copy_flags, 0, None)
            .expect("Couldnt create copy of inbuffer");
        gst_video::VideoMeta::add(
            buf.get_mut().unwrap(),
            gst_video::VideoFrameFlags::NONE,
            gst_video::VideoFormat::Bgra,
            128,
            128,
        );
        Ok(PrepareOutputBufferSuccess::Buffer(buf))
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "rsstreamid",
        gst::Rank::None,
        StreamIdElement::get_type(),
    )
}
