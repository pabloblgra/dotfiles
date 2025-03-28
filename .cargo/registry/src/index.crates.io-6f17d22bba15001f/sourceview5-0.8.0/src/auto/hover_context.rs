// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{Buffer, View};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkSourceHoverContext")]
    pub struct HoverContext(Object<ffi::GtkSourceHoverContext, ffi::GtkSourceHoverContextClass>);

    match fn {
        type_ => || ffi::gtk_source_hover_context_get_type(),
    }
}

impl HoverContext {
    #[doc(alias = "gtk_source_hover_context_get_bounds")]
    #[doc(alias = "get_bounds")]
    pub fn bounds(&self) -> Option<(gtk::TextIter, gtk::TextIter)> {
        unsafe {
            let mut begin = gtk::TextIter::uninitialized();
            let mut end = gtk::TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_source_hover_context_get_bounds(
                self.to_glib_none().0,
                begin.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            ));
            if ret {
                Some((begin, end))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_source_hover_context_get_buffer")]
    #[doc(alias = "get_buffer")]
    pub fn buffer(&self) -> Buffer {
        unsafe {
            from_glib_none(ffi::gtk_source_hover_context_get_buffer(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_hover_context_get_iter")]
    #[doc(alias = "get_iter")]
    pub fn is_iter(&self, iter: &mut gtk::TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_hover_context_get_iter(
                self.to_glib_none().0,
                iter.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_hover_context_get_view")]
    #[doc(alias = "get_view")]
    pub fn view(&self) -> View {
        unsafe {
            from_glib_none(ffi::gtk_source_hover_context_get_view(
                self.to_glib_none().0,
            ))
        }
    }
}
