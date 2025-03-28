// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Toast;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwToastOverlay")]
    pub struct ToastOverlay(Object<ffi::AdwToastOverlay, ffi::AdwToastOverlayClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_toast_overlay_get_type(),
    }
}

impl ToastOverlay {
    #[doc(alias = "adw_toast_overlay_new")]
    pub fn new() -> ToastOverlay {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_toast_overlay_new()).unsafe_cast() }
    }

    #[doc(alias = "adw_toast_overlay_add_toast")]
    pub fn add_toast(&self, toast: Toast) {
        unsafe {
            ffi::adw_toast_overlay_add_toast(self.to_glib_none().0, toast.into_glib_ptr());
        }
    }

    #[doc(alias = "adw_toast_overlay_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_toast_overlay_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toast_overlay_set_child")]
    pub fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_toast_overlay_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&ToastOverlay) + 'static>(
            this: *mut ffi::AdwToastOverlay,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ToastOverlay {
    fn default() -> Self {
        Self::new()
    }
}
