// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::mem;
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RecentInfo(Shared<ffi::GtkRecentInfo>);

    match fn {
        ref => |ptr| ffi::gtk_recent_info_ref(ptr),
        unref => |ptr| ffi::gtk_recent_info_unref(ptr),
        type_ => || ffi::gtk_recent_info_get_type(),
    }
}

impl RecentInfo {
    #[doc(alias = "gtk_recent_info_create_app_info")]
    pub fn create_app_info(
        &self,
        app_name: Option<&str>,
    ) -> Result<Option<gio::AppInfo>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_info_create_app_info(
                self.to_glib_none().0,
                app_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_recent_info_exists")]
    pub fn exists(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_exists(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_added")]
    #[doc(alias = "get_added")]
    pub fn added(&self) -> libc::c_long {
        unsafe { ffi::gtk_recent_info_get_added(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_recent_info_get_age")]
    #[doc(alias = "get_age")]
    pub fn age(&self) -> i32 {
        unsafe { ffi::gtk_recent_info_get_age(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_recent_info_get_application_info")]
    #[doc(alias = "get_application_info")]
    pub fn application_info(&self, app_name: &str) -> Option<(glib::GString, u32, libc::c_long)> {
        unsafe {
            let mut app_exec = ptr::null();
            let mut count = mem::MaybeUninit::uninit();
            let mut time_ = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_recent_info_get_application_info(
                self.to_glib_none().0,
                app_name.to_glib_none().0,
                &mut app_exec,
                count.as_mut_ptr(),
                time_.as_mut_ptr(),
            ));
            if ret {
                Some((
                    from_glib_none(app_exec),
                    count.assume_init(),
                    time_.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_recent_info_get_applications")]
    #[doc(alias = "get_applications")]
    pub fn applications(&self) -> Vec<glib::GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::gtk_recent_info_get_applications(self.to_glib_none().0, length.as_mut_ptr()),
                length.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "gtk_recent_info_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_description(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_display_name")]
    #[doc(alias = "get_display_name")]
    pub fn display_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_display_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_gicon")]
    #[doc(alias = "get_gicon")]
    pub fn gicon(&self) -> Option<gio::Icon> {
        unsafe { from_glib_full(ffi::gtk_recent_info_get_gicon(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_groups")]
    #[doc(alias = "get_groups")]
    pub fn groups(&self) -> Vec<glib::GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::gtk_recent_info_get_groups(self.to_glib_none().0, length.as_mut_ptr()),
                length.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "gtk_recent_info_get_icon")]
    #[doc(alias = "get_icon")]
    pub fn icon(&self, size: i32) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe { from_glib_full(ffi::gtk_recent_info_get_icon(self.to_glib_none().0, size)) }
    }

    #[doc(alias = "gtk_recent_info_get_mime_type")]
    #[doc(alias = "get_mime_type")]
    pub fn mime_type(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_mime_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_modified")]
    #[doc(alias = "get_modified")]
    pub fn modified(&self) -> libc::c_long {
        unsafe { ffi::gtk_recent_info_get_modified(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_recent_info_get_private_hint")]
    #[doc(alias = "get_private_hint")]
    pub fn is_private_hint(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_get_private_hint(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_short_name")]
    #[doc(alias = "get_short_name")]
    pub fn short_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gtk_recent_info_get_short_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_uri")]
    #[doc(alias = "get_uri")]
    pub fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_uri_display")]
    #[doc(alias = "get_uri_display")]
    pub fn uri_display(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gtk_recent_info_get_uri_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_get_visited")]
    #[doc(alias = "get_visited")]
    pub fn visited(&self) -> libc::c_long {
        unsafe { ffi::gtk_recent_info_get_visited(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_recent_info_has_application")]
    pub fn has_application(&self, app_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_has_application(
                self.to_glib_none().0,
                app_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_recent_info_has_group")]
    pub fn has_group(&self, group_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_has_group(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_recent_info_is_local")]
    pub fn is_local(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_is_local(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_last_application")]
    pub fn last_application(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gtk_recent_info_last_application(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_info_match")]
    #[doc(alias = "match")]
    pub fn match_(&self, info_b: &RecentInfo) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_match(
                self.to_glib_none().0,
                info_b.to_glib_none().0,
            ))
        }
    }
}
