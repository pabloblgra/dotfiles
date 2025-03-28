// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{AnimationState, AnimationTarget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwAnimation")]
    pub struct Animation(Object<ffi::AdwAnimation, ffi::AdwAnimationClass>);

    match fn {
        type_ => || ffi::adw_animation_get_type(),
    }
}

impl Animation {
    pub const NONE: Option<&'static Animation> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Animation>> Sealed for T {}
}

pub trait AnimationExt: IsA<Animation> + sealed::Sealed + 'static {
    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "adw_animation_get_follow_enable_animations_setting")]
    #[doc(alias = "get_follow_enable_animations_setting")]
    fn follows_enable_animations_setting(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_animation_get_follow_enable_animations_setting(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_animation_get_state")]
    #[doc(alias = "get_state")]
    fn state(&self) -> AnimationState {
        unsafe { from_glib(ffi::adw_animation_get_state(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "adw_animation_get_target")]
    #[doc(alias = "get_target")]
    fn target(&self) -> AnimationTarget {
        unsafe {
            from_glib_none(ffi::adw_animation_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_animation_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> f64 {
        unsafe { ffi::adw_animation_get_value(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "adw_animation_get_widget")]
    #[doc(alias = "get_widget")]
    fn widget(&self) -> gtk::Widget {
        unsafe {
            from_glib_none(ffi::adw_animation_get_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_animation_pause")]
    fn pause(&self) {
        unsafe {
            ffi::adw_animation_pause(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_animation_play")]
    fn play(&self) {
        unsafe {
            ffi::adw_animation_play(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_animation_reset")]
    fn reset(&self) {
        unsafe {
            ffi::adw_animation_reset(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_animation_resume")]
    fn resume(&self) {
        unsafe {
            ffi::adw_animation_resume(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "adw_animation_set_follow_enable_animations_setting")]
    fn set_follow_enable_animations_setting(&self, setting: bool) {
        unsafe {
            ffi::adw_animation_set_follow_enable_animations_setting(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_animation_set_target")]
    fn set_target(&self, target: &impl IsA<AnimationTarget>) {
        unsafe {
            ffi::adw_animation_set_target(
                self.as_ref().to_glib_none().0,
                target.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_animation_skip")]
    fn skip(&self) {
        unsafe {
            ffi::adw_animation_skip(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "done")]
    fn connect_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn done_trampoline<P: IsA<Animation>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwAnimation,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Animation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"done\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    done_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "follow-enable-animations-setting")]
    fn connect_follow_enable_animations_setting_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_follow_enable_animations_setting_trampoline<
            P: IsA<Animation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwAnimation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Animation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::follow-enable-animations-setting\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_follow_enable_animations_setting_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "state")]
    fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P: IsA<Animation>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwAnimation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Animation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "target")]
    fn connect_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_target_trampoline<P: IsA<Animation>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwAnimation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Animation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::target\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P: IsA<Animation>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwAnimation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Animation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Animation>> AnimationExt for O {}
