// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{Breakpoint, DialogPresentationMode};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwDialog")]
    pub struct Dialog(Object<ffi::AdwDialog, ffi::AdwDialogClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_dialog_get_type(),
    }
}

impl Dialog {
    pub const NONE: Option<&'static Dialog> = None;

    #[doc(alias = "adw_dialog_new")]
    pub fn new() -> Dialog {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::adw_dialog_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Dialog`] objects.
    ///
    /// This method returns an instance of [`DialogBuilder`](crate::builders::DialogBuilder) which can be used to create [`Dialog`] objects.
    pub fn builder() -> DialogBuilder {
        DialogBuilder::new()
    }
}

#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Dialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DialogBuilder {
    builder: glib::object::ObjectBuilder<'static, Dialog>,
}

impl DialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn can_close(self, can_close: bool) -> Self {
        Self {
            builder: self.builder.property("can-close", can_close),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn content_height(self, content_height: i32) -> Self {
        Self {
            builder: self.builder.property("content-height", content_height),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn content_width(self, content_width: i32) -> Self {
        Self {
            builder: self.builder.property("content-width", content_width),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn default_widget(self, default_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-widget", default_widget.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn focus_widget(self, focus_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("focus-widget", focus_widget.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn follows_content_size(self, follows_content_size: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("follows-content-size", follows_content_size),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn presentation_mode(self, presentation_mode: DialogPresentationMode) -> Self {
        Self {
            builder: self
                .builder
                .property("presentation-mode", presentation_mode),
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Dialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Dialog {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Dialog>> Sealed for T {}
}

pub trait AdwDialogExt: IsA<Dialog> + sealed::Sealed + 'static {
    #[doc(alias = "adw_dialog_add_breakpoint")]
    fn add_breakpoint(&self, breakpoint: Breakpoint) {
        unsafe {
            ffi::adw_dialog_add_breakpoint(
                self.as_ref().to_glib_none().0,
                breakpoint.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "adw_dialog_close")]
    fn close(&self) -> bool {
        unsafe { from_glib(ffi::adw_dialog_close(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "adw_dialog_force_close")]
    fn force_close(&self) {
        unsafe {
            ffi::adw_dialog_force_close(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_dialog_get_can_close")]
    #[doc(alias = "get_can_close")]
    fn can_close(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_dialog_get_can_close(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_dialog_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_dialog_get_child(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "adw_dialog_get_content_height")]
    #[doc(alias = "get_content_height")]
    fn content_height(&self) -> i32 {
        unsafe { ffi::adw_dialog_get_content_height(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "adw_dialog_get_content_width")]
    #[doc(alias = "get_content_width")]
    fn content_width(&self) -> i32 {
        unsafe { ffi::adw_dialog_get_content_width(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "adw_dialog_get_current_breakpoint")]
    #[doc(alias = "get_current_breakpoint")]
    fn current_breakpoint(&self) -> Option<Breakpoint> {
        unsafe {
            from_glib_none(ffi::adw_dialog_get_current_breakpoint(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_dialog_get_default_widget")]
    #[doc(alias = "get_default_widget")]
    fn default_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::adw_dialog_get_default_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_dialog_get_focus")]
    #[doc(alias = "get_focus")]
    fn focus(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_dialog_get_focus(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "adw_dialog_get_follows_content_size")]
    #[doc(alias = "get_follows_content_size")]
    fn follows_content_size(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_dialog_get_follows_content_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_dialog_get_presentation_mode")]
    #[doc(alias = "get_presentation_mode")]
    fn presentation_mode(&self) -> DialogPresentationMode {
        unsafe {
            from_glib(ffi::adw_dialog_get_presentation_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_dialog_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::adw_dialog_get_title(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "adw_dialog_present")]
    fn present(&self, parent: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_dialog_present(
                self.as_ref().to_glib_none().0,
                parent.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_dialog_set_can_close")]
    fn set_can_close(&self, can_close: bool) {
        unsafe {
            ffi::adw_dialog_set_can_close(self.as_ref().to_glib_none().0, can_close.into_glib());
        }
    }

    #[doc(alias = "adw_dialog_set_child")]
    fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_dialog_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_dialog_set_content_height")]
    fn set_content_height(&self, content_height: i32) {
        unsafe {
            ffi::adw_dialog_set_content_height(self.as_ref().to_glib_none().0, content_height);
        }
    }

    #[doc(alias = "adw_dialog_set_content_width")]
    fn set_content_width(&self, content_width: i32) {
        unsafe {
            ffi::adw_dialog_set_content_width(self.as_ref().to_glib_none().0, content_width);
        }
    }

    #[doc(alias = "adw_dialog_set_default_widget")]
    fn set_default_widget(&self, default_widget: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_dialog_set_default_widget(
                self.as_ref().to_glib_none().0,
                default_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_dialog_set_focus")]
    fn set_focus(&self, focus: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_dialog_set_focus(
                self.as_ref().to_glib_none().0,
                focus.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_dialog_set_follows_content_size")]
    fn set_follows_content_size(&self, follows_content_size: bool) {
        unsafe {
            ffi::adw_dialog_set_follows_content_size(
                self.as_ref().to_glib_none().0,
                follows_content_size.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_dialog_set_presentation_mode")]
    fn set_presentation_mode(&self, presentation_mode: DialogPresentationMode) {
        unsafe {
            ffi::adw_dialog_set_presentation_mode(
                self.as_ref().to_glib_none().0,
                presentation_mode.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_dialog_set_title")]
    fn set_title(&self, title: &str) {
        unsafe {
            ffi::adw_dialog_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "focus-widget")]
    fn focus_widget(&self) -> Option<gtk::Widget> {
        ObjectExt::property(self.as_ref(), "focus-widget")
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "focus-widget")]
    fn set_focus_widget<P: IsA<gtk::Widget>>(&self, focus_widget: Option<&P>) {
        ObjectExt::set_property(self.as_ref(), "focus-widget", focus_widget)
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "close-attempt")]
    fn connect_close_attempt<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_attempt_trampoline<P: IsA<Dialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwDialog,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close-attempt\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    close_attempt_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "closed")]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P: IsA<Dialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwDialog,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "can-close")]
    fn connect_can_close_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_close_trampoline<P: IsA<Dialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-close\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_can_close_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P: IsA<Dialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "content-height")]
    fn connect_content_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_height_trampoline<
            P: IsA<Dialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content-height\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_content_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "content-width")]
    fn connect_content_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_width_trampoline<
            P: IsA<Dialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content-width\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_content_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "current-breakpoint")]
    fn connect_current_breakpoint_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_breakpoint_trampoline<
            P: IsA<Dialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-breakpoint\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_current_breakpoint_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "default-widget")]
    fn connect_default_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_widget_trampoline<
            P: IsA<Dialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_default_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "focus-widget")]
    fn connect_focus_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_widget_trampoline<P: IsA<Dialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::focus-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_focus_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "follows-content-size")]
    fn connect_follows_content_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_follows_content_size_trampoline<
            P: IsA<Dialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::follows-content-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_follows_content_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "presentation-mode")]
    fn connect_presentation_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_presentation_mode_trampoline<
            P: IsA<Dialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::presentation-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_presentation_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<Dialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Dialog>> AdwDialogExt for O {}
