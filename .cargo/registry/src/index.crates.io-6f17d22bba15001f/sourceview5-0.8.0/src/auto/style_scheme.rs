// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Style;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSourceStyleScheme")]
    pub struct StyleScheme(Object<ffi::GtkSourceStyleScheme, ffi::GtkSourceStyleSchemeClass>);

    match fn {
        type_ => || ffi::gtk_source_style_scheme_get_type(),
    }
}

impl StyleScheme {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`StyleScheme`] objects.
    ///
    /// This method returns an instance of [`StyleSchemeBuilder`](crate::builders::StyleSchemeBuilder) which can be used to create [`StyleScheme`] objects.
    pub fn builder() -> StyleSchemeBuilder {
        StyleSchemeBuilder::new()
    }

    #[doc(alias = "gtk_source_style_scheme_get_authors")]
    #[doc(alias = "get_authors")]
    pub fn authors(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_get_authors(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_style_scheme_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_description(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_style_scheme_get_filename")]
    #[doc(alias = "get_filename")]
    pub fn filename(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_filename(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_style_scheme_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_source_style_scheme_get_id(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v5_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v5_4")))]
    #[doc(alias = "gtk_source_style_scheme_get_metadata")]
    #[doc(alias = "get_metadata")]
    pub fn metadata(&self, name: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_metadata(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_style_scheme_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_source_style_scheme_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_source_style_scheme_get_style")]
    #[doc(alias = "get_style")]
    pub fn style(&self, style_id: &str) -> Option<Style> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_style(
                self.to_glib_none().0,
                style_id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "description")]
    pub fn connect_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<F: Fn(&StyleScheme) + 'static>(
            this: *mut ffi::GtkSourceStyleScheme,
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
                b"notify::description\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_description_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "filename")]
    pub fn connect_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filename_trampoline<F: Fn(&StyleScheme) + 'static>(
            this: *mut ffi::GtkSourceStyleScheme,
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
                b"notify::filename\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_filename_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&StyleScheme) + 'static>(
            this: *mut ffi::GtkSourceStyleScheme,
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
                b"notify::name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl std::fmt::Display for StyleScheme {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`StyleScheme`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StyleSchemeBuilder {
    builder: glib::object::ObjectBuilder<'static, StyleScheme>,
}

impl StyleSchemeBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`StyleScheme`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> StyleScheme {
        self.builder.build()
    }
}
