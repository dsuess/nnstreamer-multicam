use gst::prelude::*;
use std::fmt;

// Public Rust type for the custom meta.
#[repr(C)]
pub struct NnmcBufferMeta(imp::NnmcBufferMeta);

// Metas must be Send+Sync.
unsafe impl Send for NnmcBufferMeta {}
unsafe impl Sync for NnmcBufferMeta {}

impl NnmcBufferMeta {
    // Retrieve the stored label.
    pub fn get_stream_id(&self) -> u32 {
        self.0.stream_id
    }
}

// Trait to allow using the gst::Buffer API with this meta.
unsafe impl MetaAPI for NnmcBufferMeta {
    type GstType = imp::NnmcBufferMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { imp::nnmc_buffer_meta_get_type() }
    }
}

impl fmt::Debug for NnmcBufferMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NnmcBufferMeta")
            .field("stream_id", &self.get_stream_id())
            .finish()
    }
}

mod imp {
    use gst::gst_sys;
    #[repr(C)]
    pub struct NnmcBufferMeta {
        parent: gst_sys::GstMeta,
        pub(super) stream_id: u32,
    }

    // Since we're using FFI just to bridge Rust -> Rust for now, it's okay
    // to not have proper ctypes for this
    // FIXME Proper ctype conversion to allow use of meta in other languages
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn nnmc_buffer_meta_get_type() -> glib::Type;
    }
}
