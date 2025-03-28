// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use std::fmt;

bitflags! {
    #[doc(alias = "GdkPixbufFormatFlags")]
    pub struct PixbufFormatFlags: u32 {
        #[doc(alias = "GDK_PIXBUF_FORMAT_WRITABLE")]
        const WRITABLE = ffi::GDK_PIXBUF_FORMAT_WRITABLE as _;
        #[doc(alias = "GDK_PIXBUF_FORMAT_SCALABLE")]
        const SCALABLE = ffi::GDK_PIXBUF_FORMAT_SCALABLE as _;
        #[doc(alias = "GDK_PIXBUF_FORMAT_THREADSAFE")]
        const THREADSAFE = ffi::GDK_PIXBUF_FORMAT_THREADSAFE as _;
    }
}

impl fmt::Display for PixbufFormatFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PixbufFormatFlags {
    type GlibType = ffi::GdkPixbufFormatFlags;

    fn into_glib(self) -> ffi::GdkPixbufFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPixbufFormatFlags> for PixbufFormatFlags {
    unsafe fn from_glib(value: ffi::GdkPixbufFormatFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}
