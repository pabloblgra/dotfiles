// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Display;
use crate::ModifierIntent;
use crate::ModifierType;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GdkKeymap")]
    pub struct Keymap(Object<ffi::GdkKeymap>);

    match fn {
        type_ => || ffi::gdk_keymap_get_type(),
    }
}

impl Keymap {
    #[doc(alias = "gdk_keymap_get_caps_lock_state")]
    #[doc(alias = "get_caps_lock_state")]
    pub fn is_caps_locked(&self) -> bool {
        unsafe { from_glib(ffi::gdk_keymap_get_caps_lock_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_get_direction")]
    #[doc(alias = "get_direction")]
    pub fn direction(&self) -> pango::Direction {
        unsafe { from_glib(ffi::gdk_keymap_get_direction(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_get_modifier_mask")]
    #[doc(alias = "get_modifier_mask")]
    pub fn modifier_mask(&self, intent: ModifierIntent) -> ModifierType {
        unsafe {
            from_glib(ffi::gdk_keymap_get_modifier_mask(
                self.to_glib_none().0,
                intent.into_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_keymap_get_modifier_state")]
    #[doc(alias = "get_modifier_state")]
    pub fn modifier_state(&self) -> u32 {
        unsafe { ffi::gdk_keymap_get_modifier_state(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_keymap_get_num_lock_state")]
    #[doc(alias = "get_num_lock_state")]
    pub fn is_num_locked(&self) -> bool {
        unsafe { from_glib(ffi::gdk_keymap_get_num_lock_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_get_scroll_lock_state")]
    #[doc(alias = "get_scroll_lock_state")]
    pub fn is_scroll_locked(&self) -> bool {
        unsafe { from_glib(ffi::gdk_keymap_get_scroll_lock_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_have_bidi_layouts")]
    pub fn have_bidi_layouts(&self) -> bool {
        unsafe { from_glib(ffi::gdk_keymap_have_bidi_layouts(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_translate_keyboard_state")]
    pub fn translate_keyboard_state(
        &self,
        hardware_keycode: u32,
        state: ModifierType,
        group: i32,
    ) -> Option<(u32, i32, i32, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut effective_group = mem::MaybeUninit::uninit();
            let mut level = mem::MaybeUninit::uninit();
            let mut consumed_modifiers = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_keymap_translate_keyboard_state(
                self.to_glib_none().0,
                hardware_keycode,
                state.into_glib(),
                group,
                keyval.as_mut_ptr(),
                effective_group.as_mut_ptr(),
                level.as_mut_ptr(),
                consumed_modifiers.as_mut_ptr(),
            ));
            if ret {
                Some((
                    keyval.assume_init(),
                    effective_group.assume_init(),
                    level.assume_init(),
                    from_glib(consumed_modifiers.assume_init()),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_keymap_get_for_display")]
    #[doc(alias = "get_for_display")]
    pub fn for_display(display: &Display) -> Option<Keymap> {
        skip_assert_initialized!();
        unsafe { from_glib_none(ffi::gdk_keymap_get_for_display(display.to_glib_none().0)) }
    }

    #[doc(alias = "direction-changed")]
    pub fn connect_direction_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn direction_changed_trampoline<F: Fn(&Keymap) + 'static>(
            this: *mut ffi::GdkKeymap,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"direction-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    direction_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "keys-changed")]
    pub fn connect_keys_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn keys_changed_trampoline<F: Fn(&Keymap) + 'static>(
            this: *mut ffi::GdkKeymap,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"keys-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    keys_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "state-changed")]
    pub fn connect_state_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<F: Fn(&Keymap) + 'static>(
            this: *mut ffi::GdkKeymap,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    state_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Keymap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Keymap")
    }
}
