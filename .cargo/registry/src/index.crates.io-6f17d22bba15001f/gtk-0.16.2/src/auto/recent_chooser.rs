// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RecentFilter;
use crate::RecentInfo;
use crate::RecentSortType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GtkRecentChooser")]
    pub struct RecentChooser(Interface<ffi::GtkRecentChooser, ffi::GtkRecentChooserIface>);

    match fn {
        type_ => || ffi::gtk_recent_chooser_get_type(),
    }
}

impl RecentChooser {
    pub const NONE: Option<&'static RecentChooser> = None;
}

pub trait RecentChooserExt: 'static {
    #[doc(alias = "gtk_recent_chooser_add_filter")]
    fn add_filter(&self, filter: &RecentFilter);

    #[doc(alias = "gtk_recent_chooser_get_current_item")]
    #[doc(alias = "get_current_item")]
    fn current_item(&self) -> Option<RecentInfo>;

    #[doc(alias = "gtk_recent_chooser_get_current_uri")]
    #[doc(alias = "get_current_uri")]
    fn current_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_recent_chooser_get_filter")]
    #[doc(alias = "get_filter")]
    fn filter(&self) -> Option<RecentFilter>;

    #[doc(alias = "gtk_recent_chooser_get_items")]
    #[doc(alias = "get_items")]
    fn items(&self) -> Vec<RecentInfo>;

    #[doc(alias = "gtk_recent_chooser_get_limit")]
    #[doc(alias = "get_limit")]
    fn limit(&self) -> i32;

    #[doc(alias = "gtk_recent_chooser_get_local_only")]
    #[doc(alias = "get_local_only")]
    fn is_local_only(&self) -> bool;

    #[doc(alias = "gtk_recent_chooser_get_select_multiple")]
    #[doc(alias = "get_select_multiple")]
    fn selects_multiple(&self) -> bool;

    #[doc(alias = "gtk_recent_chooser_get_show_icons")]
    #[doc(alias = "get_show_icons")]
    fn shows_icons(&self) -> bool;

    #[doc(alias = "gtk_recent_chooser_get_show_not_found")]
    #[doc(alias = "get_show_not_found")]
    fn shows_not_found(&self) -> bool;

    #[doc(alias = "gtk_recent_chooser_get_show_private")]
    #[doc(alias = "get_show_private")]
    fn shows_private(&self) -> bool;

    #[doc(alias = "gtk_recent_chooser_get_show_tips")]
    #[doc(alias = "get_show_tips")]
    fn shows_tips(&self) -> bool;

    #[doc(alias = "gtk_recent_chooser_get_sort_type")]
    #[doc(alias = "get_sort_type")]
    fn sort_type(&self) -> RecentSortType;

    #[doc(alias = "gtk_recent_chooser_get_uris")]
    #[doc(alias = "get_uris")]
    fn uris(&self) -> Vec<glib::GString>;

    #[doc(alias = "gtk_recent_chooser_list_filters")]
    fn list_filters(&self) -> Vec<RecentFilter>;

    #[doc(alias = "gtk_recent_chooser_remove_filter")]
    fn remove_filter(&self, filter: &RecentFilter);

    #[doc(alias = "gtk_recent_chooser_select_all")]
    fn select_all(&self);

    #[doc(alias = "gtk_recent_chooser_select_uri")]
    fn select_uri(&self, uri: &str) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_recent_chooser_set_current_uri")]
    fn set_current_uri(&self, uri: &str) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_recent_chooser_set_filter")]
    fn set_filter(&self, filter: Option<&RecentFilter>);

    #[doc(alias = "gtk_recent_chooser_set_limit")]
    fn set_limit(&self, limit: i32);

    #[doc(alias = "gtk_recent_chooser_set_local_only")]
    fn set_local_only(&self, local_only: bool);

    #[doc(alias = "gtk_recent_chooser_set_select_multiple")]
    fn set_select_multiple(&self, select_multiple: bool);

    #[doc(alias = "gtk_recent_chooser_set_show_icons")]
    fn set_show_icons(&self, show_icons: bool);

    #[doc(alias = "gtk_recent_chooser_set_show_not_found")]
    fn set_show_not_found(&self, show_not_found: bool);

    #[doc(alias = "gtk_recent_chooser_set_show_private")]
    fn set_show_private(&self, show_private: bool);

    #[doc(alias = "gtk_recent_chooser_set_show_tips")]
    fn set_show_tips(&self, show_tips: bool);

    #[doc(alias = "gtk_recent_chooser_set_sort_func")]
    fn set_sort_func<P: Fn(&RecentInfo, &RecentInfo) -> i32 + 'static>(&self, sort_func: P);

    #[doc(alias = "gtk_recent_chooser_set_sort_type")]
    fn set_sort_type(&self, sort_type: RecentSortType);

    #[doc(alias = "gtk_recent_chooser_unselect_all")]
    fn unselect_all(&self);

    #[doc(alias = "gtk_recent_chooser_unselect_uri")]
    fn unselect_uri(&self, uri: &str);

    #[doc(alias = "item-activated")]
    fn connect_item_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "selection-changed")]
    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "filter")]
    fn connect_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "limit")]
    fn connect_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "local-only")]
    fn connect_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "select-multiple")]
    fn connect_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-icons")]
    fn connect_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-not-found")]
    fn connect_show_not_found_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-private")]
    fn connect_show_private_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-tips")]
    fn connect_show_tips_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "sort-type")]
    fn connect_sort_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RecentChooser>> RecentChooserExt for O {
    fn add_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_add_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn current_item(&self) -> Option<RecentInfo> {
        unsafe {
            from_glib_full(ffi::gtk_recent_chooser_get_current_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn current_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_recent_chooser_get_current_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn filter(&self) -> Option<RecentFilter> {
        unsafe {
            from_glib_none(ffi::gtk_recent_chooser_get_filter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_recent_chooser_get_items(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn limit(&self) -> i32 {
        unsafe { ffi::gtk_recent_chooser_get_limit(self.as_ref().to_glib_none().0) }
    }

    fn is_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_local_only(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn selects_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_select_multiple(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_icons(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_icons(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_not_found(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_not_found(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_private(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_private(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_tips(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_tips(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn sort_type(&self) -> RecentSortType {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_sort_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uris(&self) -> Vec<glib::GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::gtk_recent_chooser_get_uris(
                    self.as_ref().to_glib_none().0,
                    length.as_mut_ptr(),
                ),
                length.assume_init() as _,
            );
            ret
        }
    }

    fn list_filters(&self) -> Vec<RecentFilter> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_recent_chooser_list_filters(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_remove_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_recent_chooser_select_all(self.as_ref().to_glib_none().0);
        }
    }

    fn select_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gtk_recent_chooser_select_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_current_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gtk_recent_chooser_set_current_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_filter(&self, filter: Option<&RecentFilter>) {
        unsafe {
            ffi::gtk_recent_chooser_set_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn set_limit(&self, limit: i32) {
        unsafe {
            ffi::gtk_recent_chooser_set_limit(self.as_ref().to_glib_none().0, limit);
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_local_only(
                self.as_ref().to_glib_none().0,
                local_only.into_glib(),
            );
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_select_multiple(
                self.as_ref().to_glib_none().0,
                select_multiple.into_glib(),
            );
        }
    }

    fn set_show_icons(&self, show_icons: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_icons(
                self.as_ref().to_glib_none().0,
                show_icons.into_glib(),
            );
        }
    }

    fn set_show_not_found(&self, show_not_found: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_not_found(
                self.as_ref().to_glib_none().0,
                show_not_found.into_glib(),
            );
        }
    }

    fn set_show_private(&self, show_private: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_private(
                self.as_ref().to_glib_none().0,
                show_private.into_glib(),
            );
        }
    }

    fn set_show_tips(&self, show_tips: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_tips(
                self.as_ref().to_glib_none().0,
                show_tips.into_glib(),
            );
        }
    }

    fn set_sort_func<P: Fn(&RecentInfo, &RecentInfo) -> i32 + 'static>(&self, sort_func: P) {
        let sort_func_data: Box_<P> = Box_::new(sort_func);
        unsafe extern "C" fn sort_func_func<P: Fn(&RecentInfo, &RecentInfo) -> i32 + 'static>(
            a: *mut ffi::GtkRecentInfo,
            b: *mut ffi::GtkRecentInfo,
            user_data: glib::ffi::gpointer,
        ) -> libc::c_int {
            let a = from_glib_borrow(a);
            let b = from_glib_borrow(b);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&a, &b);
            res
        }
        let sort_func = Some(sort_func_func::<P> as _);
        unsafe extern "C" fn data_destroy_func<P: Fn(&RecentInfo, &RecentInfo) -> i32 + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(data_destroy_func::<P> as _);
        let super_callback0: Box_<P> = sort_func_data;
        unsafe {
            ffi::gtk_recent_chooser_set_sort_func(
                self.as_ref().to_glib_none().0,
                sort_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_sort_type(&self, sort_type: RecentSortType) {
        unsafe {
            ffi::gtk_recent_chooser_set_sort_type(
                self.as_ref().to_glib_none().0,
                sort_type.into_glib(),
            );
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    fn connect_item_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn item_activated_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"item-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    item_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_changed_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    selection_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_limit_trampoline<P: IsA<RecentChooser>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_limit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_only_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_only_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_multiple_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::select-multiple\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_select_multiple_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_icons_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-icons\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_icons_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_not_found_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_not_found_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-not-found\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_not_found_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_private_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_private_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-private\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_private_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_tips_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_tips_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-tips\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_tips_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_sort_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sort_type_trampoline<
            P: IsA<RecentChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRecentChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sort-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sort_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RecentChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RecentChooser")
    }
}
