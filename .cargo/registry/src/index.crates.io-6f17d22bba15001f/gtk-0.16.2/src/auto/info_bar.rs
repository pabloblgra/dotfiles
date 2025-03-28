// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::BaselinePosition;
use crate::Box;
use crate::Buildable;
use crate::Button;
use crate::Container;
use crate::MessageType;
use crate::Orientable;
use crate::Orientation;
use crate::ResizeMode;
use crate::ResponseType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkInfoBar")]
    pub struct InfoBar(Object<ffi::GtkInfoBar, ffi::GtkInfoBarClass>) @extends Box, Container, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_info_bar_get_type(),
    }
}

impl InfoBar {
    pub const NONE: Option<&'static InfoBar> = None;

    #[doc(alias = "gtk_info_bar_new")]
    pub fn new() -> InfoBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_info_bar_new()).unsafe_cast() }
    }

    //#[doc(alias = "gtk_info_bar_new_with_buttons")]
    //#[doc(alias = "new_with_buttons")]
    //pub fn with_buttons(first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> InfoBar {
    //    unsafe { TODO: call ffi:gtk_info_bar_new_with_buttons() }
    //}

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`InfoBar`] objects.
    ///
    /// This method returns an instance of [`InfoBarBuilder`](crate::builders::InfoBarBuilder) which can be used to create [`InfoBar`] objects.
    pub fn builder() -> InfoBarBuilder {
        InfoBarBuilder::default()
    }
}

impl Default for InfoBar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`InfoBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct InfoBarBuilder {
    message_type: Option<MessageType>,
    revealed: Option<bool>,
    show_close_button: Option<bool>,
    baseline_position: Option<BaselinePosition>,
    homogeneous: Option<bool>,
    spacing: Option<i32>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
}

impl InfoBarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`InfoBarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`InfoBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> InfoBar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref message_type) = self.message_type {
            properties.push(("message-type", message_type));
        }
        if let Some(ref revealed) = self.revealed {
            properties.push(("revealed", revealed));
        }
        if let Some(ref show_close_button) = self.show_close_button {
            properties.push(("show-close-button", show_close_button));
        }
        if let Some(ref baseline_position) = self.baseline_position {
            properties.push(("baseline-position", baseline_position));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<InfoBar>(&properties)
    }

    pub fn message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = Some(message_type);
        self
    }

    pub fn revealed(mut self, revealed: bool) -> Self {
        self.revealed = Some(revealed);
        self
    }

    pub fn show_close_button(mut self, show_close_button: bool) -> Self {
        self.show_close_button = Some(show_close_button);
        self
    }

    pub fn baseline_position(mut self, baseline_position: BaselinePosition) -> Self {
        self.baseline_position = Some(baseline_position);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &impl IsA<Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &impl IsA<Container>) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub trait InfoBarExt: 'static {
    #[doc(alias = "gtk_info_bar_add_action_widget")]
    fn add_action_widget(&self, child: &impl IsA<Widget>, response_id: ResponseType);

    #[doc(alias = "gtk_info_bar_add_button")]
    fn add_button(&self, button_text: &str, response_id: ResponseType) -> Option<Button>;

    //#[doc(alias = "gtk_info_bar_add_buttons")]
    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs);

    #[doc(alias = "gtk_info_bar_get_action_area")]
    #[doc(alias = "get_action_area")]
    fn action_area(&self) -> Option<Box>;

    #[doc(alias = "gtk_info_bar_get_content_area")]
    #[doc(alias = "get_content_area")]
    fn content_area(&self) -> Box;

    #[doc(alias = "gtk_info_bar_get_message_type")]
    #[doc(alias = "get_message_type")]
    fn message_type(&self) -> MessageType;

    #[doc(alias = "gtk_info_bar_get_revealed")]
    #[doc(alias = "get_revealed")]
    fn is_revealed(&self) -> bool;

    #[doc(alias = "gtk_info_bar_get_show_close_button")]
    #[doc(alias = "get_show_close_button")]
    fn shows_close_button(&self) -> bool;

    #[doc(alias = "gtk_info_bar_response")]
    fn response(&self, response_id: ResponseType);

    #[doc(alias = "gtk_info_bar_set_default_response")]
    fn set_default_response(&self, response_id: ResponseType);

    #[doc(alias = "gtk_info_bar_set_message_type")]
    fn set_message_type(&self, message_type: MessageType);

    #[doc(alias = "gtk_info_bar_set_response_sensitive")]
    fn set_response_sensitive(&self, response_id: ResponseType, setting: bool);

    #[doc(alias = "gtk_info_bar_set_revealed")]
    fn set_revealed(&self, revealed: bool);

    #[doc(alias = "gtk_info_bar_set_show_close_button")]
    fn set_show_close_button(&self, setting: bool);

    #[doc(alias = "close")]
    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_close(&self);

    #[doc(alias = "response")]
    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "message-type")]
    fn connect_message_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "revealed")]
    fn connect_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-close-button")]
    fn connect_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<InfoBar>> InfoBarExt for O {
    fn add_action_widget(&self, child: &impl IsA<Widget>, response_id: ResponseType) {
        unsafe {
            ffi::gtk_info_bar_add_action_widget(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                response_id.into_glib(),
            );
        }
    }

    fn add_button(&self, button_text: &str, response_id: ResponseType) -> Option<Button> {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_add_button(
                self.as_ref().to_glib_none().0,
                button_text.to_glib_none().0,
                response_id.into_glib(),
            ))
        }
    }

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_info_bar_add_buttons() }
    //}

    fn action_area(&self) -> Option<Box> {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_get_action_area(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn content_area(&self) -> Box {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_get_content_area(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn message_type(&self) -> MessageType {
        unsafe {
            from_glib(ffi::gtk_info_bar_get_message_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_revealed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_info_bar_get_revealed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_info_bar_get_show_close_button(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn response(&self, response_id: ResponseType) {
        unsafe {
            ffi::gtk_info_bar_response(self.as_ref().to_glib_none().0, response_id.into_glib());
        }
    }

    fn set_default_response(&self, response_id: ResponseType) {
        unsafe {
            ffi::gtk_info_bar_set_default_response(
                self.as_ref().to_glib_none().0,
                response_id.into_glib(),
            );
        }
    }

    fn set_message_type(&self, message_type: MessageType) {
        unsafe {
            ffi::gtk_info_bar_set_message_type(
                self.as_ref().to_glib_none().0,
                message_type.into_glib(),
            );
        }
    }

    fn set_response_sensitive(&self, response_id: ResponseType, setting: bool) {
        unsafe {
            ffi::gtk_info_bar_set_response_sensitive(
                self.as_ref().to_glib_none().0,
                response_id.into_glib(),
                setting.into_glib(),
            );
        }
    }

    fn set_revealed(&self, revealed: bool) {
        unsafe {
            ffi::gtk_info_bar_set_revealed(self.as_ref().to_glib_none().0, revealed.into_glib());
        }
    }

    fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_info_bar_set_show_close_button(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<P: IsA<InfoBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkInfoBar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InfoBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_close(&self) {
        self.emit_by_name::<()>("close", &[]);
    }

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn response_trampoline<
            P: IsA<InfoBar>,
            F: Fn(&P, ResponseType) + 'static,
        >(
            this: *mut ffi::GtkInfoBar,
            response_id: ffi::GtkResponseType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                InfoBar::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(response_id),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"response\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_message_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_message_type_trampoline<
            P: IsA<InfoBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkInfoBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InfoBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::message-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_message_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_revealed_trampoline<P: IsA<InfoBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkInfoBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InfoBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::revealed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_revealed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_close_button_trampoline<
            P: IsA<InfoBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkInfoBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InfoBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-close-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_close_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for InfoBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InfoBar")
    }
}
