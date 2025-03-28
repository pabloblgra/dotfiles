// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Mark;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSourceMarkAttributes")]
    pub struct MarkAttributes(Object<ffi::GtkSourceMarkAttributes, ffi::GtkSourceMarkAttributesClass>);

    match fn {
        type_ => || ffi::gtk_source_mark_attributes_get_type(),
    }
}

impl MarkAttributes {
    #[doc(alias = "gtk_source_mark_attributes_new")]
    pub fn new() -> MarkAttributes {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_source_mark_attributes_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`MarkAttributes`] objects.
    ///
    /// This method returns an instance of [`MarkAttributesBuilder`](crate::builders::MarkAttributesBuilder) which can be used to create [`MarkAttributes`] objects.
    pub fn builder() -> MarkAttributesBuilder {
        MarkAttributesBuilder::new()
    }

    #[doc(alias = "gtk_source_mark_attributes_get_background")]
    #[doc(alias = "get_background")]
    pub fn background(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut background = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_source_mark_attributes_get_background(
                self.to_glib_none().0,
                background.to_glib_none_mut().0,
            ));
            if ret {
                Some(background)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_gicon")]
    #[doc(alias = "get_gicon")]
    pub fn gicon(&self) -> gio::Icon {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_get_gicon(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    pub fn icon_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_get_icon_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_pixbuf")]
    #[doc(alias = "get_pixbuf")]
    pub fn pixbuf(&self) -> gdk_pixbuf::Pixbuf {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_get_pixbuf(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_tooltip_markup")]
    #[doc(alias = "get_tooltip_markup")]
    pub fn tooltip_markup(&self, mark: &impl IsA<Mark>) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_attributes_get_tooltip_markup(
                self.to_glib_none().0,
                mark.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_tooltip_text")]
    #[doc(alias = "get_tooltip_text")]
    pub fn tooltip_text(&self, mark: &impl IsA<Mark>) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_attributes_get_tooltip_text(
                self.to_glib_none().0,
                mark.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_render_icon")]
    pub fn render_icon(&self, widget: &impl IsA<gtk::Widget>, size: i32) -> gdk::Paintable {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_render_icon(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                size,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_set_background")]
    pub fn set_background(&self, background: &gdk::RGBA) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_background(
                self.to_glib_none().0,
                background.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_set_gicon")]
    pub fn set_gicon(&self, gicon: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_gicon(
                self.to_glib_none().0,
                gicon.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_set_icon_name")]
    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_set_pixbuf")]
    pub fn set_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_pixbuf(
                self.to_glib_none().0,
                pixbuf.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "query-tooltip-markup")]
    pub fn connect_query_tooltip_markup<F: Fn(&Self, &Mark) -> String + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_tooltip_markup_trampoline<
            F: Fn(&MarkAttributes, &Mark) -> String + 'static,
        >(
            this: *mut ffi::GtkSourceMarkAttributes,
            mark: *mut ffi::GtkSourceMark,
            f: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(mark)).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-tooltip-markup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    query_tooltip_markup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "query-tooltip-text")]
    pub fn connect_query_tooltip_text<F: Fn(&Self, &Mark) -> String + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_tooltip_text_trampoline<
            F: Fn(&MarkAttributes, &Mark) -> String + 'static,
        >(
            this: *mut ffi::GtkSourceMarkAttributes,
            mark: *mut ffi::GtkSourceMark,
            f: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(mark)).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-tooltip-text\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    query_tooltip_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "background")]
    pub fn connect_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_trampoline<F: Fn(&MarkAttributes) + 'static>(
            this: *mut ffi::GtkSourceMarkAttributes,
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
                b"notify::background\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_background_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "gicon")]
    pub fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<F: Fn(&MarkAttributes) + 'static>(
            this: *mut ffi::GtkSourceMarkAttributes,
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
                b"notify::gicon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&MarkAttributes) + 'static>(
            this: *mut ffi::GtkSourceMarkAttributes,
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
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixbuf")]
    pub fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<F: Fn(&MarkAttributes) + 'static>(
            this: *mut ffi::GtkSourceMarkAttributes,
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
                b"notify::pixbuf\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for MarkAttributes {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`MarkAttributes`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MarkAttributesBuilder {
    builder: glib::object::ObjectBuilder<'static, MarkAttributes>,
}

impl MarkAttributesBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn background(self, background: &gdk::RGBA) -> Self {
        Self {
            builder: self.builder.property("background", background),
        }
    }

    pub fn gicon(self, gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property("gicon", gicon.clone().upcast()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn pixbuf(self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self.builder.property("pixbuf", pixbuf.clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`MarkAttributes`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MarkAttributes {
        self.builder.build()
    }
}
