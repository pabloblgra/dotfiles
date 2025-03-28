// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::BreakpointCondition;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwBreakpoint")]
    pub struct Breakpoint(Object<ffi::AdwBreakpoint, ffi::AdwBreakpointClass>) @implements gtk::Buildable;

    match fn {
        type_ => || ffi::adw_breakpoint_get_type(),
    }
}

impl Breakpoint {
    #[doc(alias = "adw_breakpoint_new")]
    pub fn new(condition: BreakpointCondition) -> Breakpoint {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::adw_breakpoint_new(condition.into_glib_ptr())) }
    }

    #[doc(alias = "adw_breakpoint_add_setter")]
    pub fn add_setter(&self, object: &impl IsA<glib::Object>, property: &str, value: &glib::Value) {
        unsafe {
            ffi::adw_breakpoint_add_setter(
                self.to_glib_none().0,
                object.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_breakpoint_get_condition")]
    #[doc(alias = "get_condition")]
    pub fn condition(&self) -> Option<BreakpointCondition> {
        unsafe { from_glib_none(ffi::adw_breakpoint_get_condition(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_breakpoint_set_condition")]
    pub fn set_condition(&self, condition: Option<&BreakpointCondition>) {
        unsafe {
            ffi::adw_breakpoint_set_condition(
                self.to_glib_none().0,
                mut_override(condition.to_glib_none().0),
            );
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "apply")]
    pub fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn apply_trampoline<F: Fn(&Breakpoint) + 'static>(
            this: *mut ffi::AdwBreakpoint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"apply\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    apply_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "unapply")]
    pub fn connect_unapply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unapply_trampoline<F: Fn(&Breakpoint) + 'static>(
            this: *mut ffi::AdwBreakpoint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unapply\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    unapply_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "condition")]
    pub fn connect_condition_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_condition_trampoline<F: Fn(&Breakpoint) + 'static>(
            this: *mut ffi::AdwBreakpoint,
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
                b"notify::condition\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_condition_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
