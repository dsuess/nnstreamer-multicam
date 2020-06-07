// This example demonstrates how custom GstMeta can be defined and used on buffers.
//
// It simply attaches a GstMeta with a Rust String to buffers that are passed into
// an appsrc and retrieves them again from an appsink.

//#[macro_use]
//use gst;
use gst::prelude::*;

use gst::gst_sys;
use std::fmt;
use std::ptr;

// Public Rust type for the custom meta.
#[repr(C)]
pub struct NnmcBufferMeta(imp::NnmcBufferMeta);

// Metas must be Send+Sync.
unsafe impl Send for NnmcBufferMeta {}
unsafe impl Sync for NnmcBufferMeta {}

impl NnmcBufferMeta {
    // Add a new custom meta to the buffer with the given label.
    pub fn add(
        buffer: &mut gst::BufferRef,
        stream_id: u32,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        unsafe {
            // First add it: this will store an empty label via nnmc_buffer_meta_init().
            let meta = gst_sys::gst_buffer_add_meta(
                buffer.as_mut_ptr(),
                imp::nnmc_buffer_meta_get_info(),
                ptr::null_mut(),
            ) as *mut imp::NnmcBufferMeta;

            // Then actually set the label.
            {
                let meta = &mut *meta;
                meta.stream_id = stream_id;
            }

            Self::from_mut_ptr(buffer, meta)
        }
    }

    // Retrieve the stored label.
    pub fn get_stream_id(&self) -> u32 {
        self.0.stream_id
    }
}

// Trait to allow using the gst::Buffer API with this meta.
unsafe impl MetaAPI for NnmcBufferMeta {
    type GstType = imp::NnmcBufferMeta;

    fn get_meta_api() -> glib::Type {
        imp::nnmc_buffer_meta_get_type()
    }
}

impl fmt::Debug for NnmcBufferMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NnmcBufferMeta")
            .field("stream_id", &self.get_stream_id())
            .finish()
    }
}

// Actual unsafe implementation of the meta.
pub mod imp {
    use glib::glib_sys;
    use glib::translate::*;
    use gst::gst_sys;
    use gst::prelude::*;
    use once_cell::sync::Lazy;
    use std::mem;
    use std::ptr;

    // This is the C type that is actually stored as meta inside the buffers.
    #[repr(C)]
    pub struct NnmcBufferMeta {
        parent: gst_sys::GstMeta,
        pub(super) stream_id: u32,
    }

    // Function to register the meta API and get a type back.
    #[no_mangle]
    pub extern "C" fn nnmc_buffer_meta_get_type() -> glib::Type {
        static TYPE: Lazy<glib::Type> = Lazy::new(|| unsafe {
            let t = from_glib(gst_sys::gst_meta_api_type_register(
                b"NnmcBufferMetaAPI\0".as_ptr() as *const _,
                // We provide no tags here as our meta is just a label and does
                // not refer to any specific aspect of the buffer
                [ptr::null::<std::os::raw::c_char>()].as_ptr() as *mut *const _,
            ));

            assert_ne!(t, glib::Type::Invalid);

            t
        });

        *TYPE
    }

    // Initialization function for our meta. This needs to ensure all fields are correctly
    // initialized. They will contain random memory before.
    unsafe extern "C" fn nnmc_buffer_meta_init(
        meta: *mut gst_sys::GstMeta,
        _params: glib_sys::gpointer,
        _buffer: *mut gst_sys::GstBuffer,
    ) -> glib_sys::gboolean {
        let _meta = &mut *(meta as *mut NnmcBufferMeta);
        // Need to initialize all our fields correctly here;
        // no heap-allocated field for now
        true.to_glib()
    }

    // Free function for our meta. This needs to free/drop all memory we allocated.
    unsafe extern "C" fn nnmc_buffer_meta_free(
        meta: *mut gst_sys::GstMeta,
        _buffer: *mut gst_sys::GstBuffer,
    ) {
        let _meta = &mut *(meta as *mut NnmcBufferMeta);
        // Need to free/drop all our fields here.
        // no heap-allocated field for now
    }

    // Transform function for our meta. This needs to get it from the old buffer to the new one
    // in a way that is compatible with the transformation type. In this case we just always
    // copy it over.
    unsafe extern "C" fn nnmc_buffer_meta_transform(
        dest: *mut gst_sys::GstBuffer,
        meta: *mut gst_sys::GstMeta,
        _buffer: *mut gst_sys::GstBuffer,
        _type_: glib_sys::GQuark,
        _data: glib_sys::gpointer,
    ) -> glib_sys::gboolean {
        let meta = &mut *(meta as *mut NnmcBufferMeta);

        // We simply copy over our meta here. Other metas might have to look at the type
        // and do things conditional on that, or even just drop the meta.
        super::NnmcBufferMeta::add(gst::BufferRef::from_mut_ptr(dest), meta.stream_id);

        true.to_glib()
    }

    // Register the meta itself with its functions.
    #[no_mangle]
    pub extern "C" fn nnmc_buffer_meta_get_info() -> *const gst_sys::GstMetaInfo {
        struct MetaInfo(ptr::NonNull<gst_sys::GstMetaInfo>);
        unsafe impl Send for MetaInfo {}
        unsafe impl Sync for MetaInfo {}

        static META_INFO: Lazy<MetaInfo> = Lazy::new(|| unsafe {
            MetaInfo(
                ptr::NonNull::new(gst_sys::gst_meta_register(
                    nnmc_buffer_meta_get_type().to_glib(),
                    b"NnmcBufferMeta\0".as_ptr() as *const _,
                    mem::size_of::<NnmcBufferMeta>(),
                    Some(nnmc_buffer_meta_init),
                    Some(nnmc_buffer_meta_free),
                    Some(nnmc_buffer_meta_transform),
                ) as *mut gst_sys::GstMetaInfo)
                .expect("Failed to register meta API"),
            )
        });

        META_INFO.0.as_ptr()
    }
}
