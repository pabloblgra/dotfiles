// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{translate::*};
use std::{fmt};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GtkLayerShellEdge")]
pub enum Edge {
    /// The left edge of the screen.
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_LEFT")]
    Left,
    /// The right edge of the screen.
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_RIGHT")]
    Right,
    /// The top edge of the screen.
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_TOP")]
    Top,
    /// The bottom edge of the screen.
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_BOTTOM")]
    Bottom,
    /// Should not be used except to get the number of entries. (NOTE: may change in
    /// future releases as more entries are added)
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_ENTRY_NUMBER")]
    EntryNumber,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge::{}", match *self {
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Top => "Top",
            Self::Bottom => "Bottom",
            Self::EntryNumber => "EntryNumber",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for Edge {
    type GlibType = ffi::GtkLayerShellEdge;

    #[inline]
fn into_glib(self) -> ffi::GtkLayerShellEdge {
        match self {
            Self::Left => ffi::GTK_LAYER_SHELL_EDGE_LEFT,
            Self::Right => ffi::GTK_LAYER_SHELL_EDGE_RIGHT,
            Self::Top => ffi::GTK_LAYER_SHELL_EDGE_TOP,
            Self::Bottom => ffi::GTK_LAYER_SHELL_EDGE_BOTTOM,
            Self::EntryNumber => ffi::GTK_LAYER_SHELL_EDGE_ENTRY_NUMBER,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkLayerShellEdge> for Edge {
    #[inline]
unsafe fn from_glib(value: ffi::GtkLayerShellEdge) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GTK_LAYER_SHELL_EDGE_LEFT => Self::Left,
            ffi::GTK_LAYER_SHELL_EDGE_RIGHT => Self::Right,
            ffi::GTK_LAYER_SHELL_EDGE_TOP => Self::Top,
            ffi::GTK_LAYER_SHELL_EDGE_BOTTOM => Self::Bottom,
            ffi::GTK_LAYER_SHELL_EDGE_ENTRY_NUMBER => Self::EntryNumber,
            value => Self::__Unknown(value),
}
    }
}

/// GTK_LAYER_SHELL_KEYBOARD_MODE_NONE: This window should not receive keyboard events.
/// GTK_LAYER_SHELL_KEYBOARD_MODE_EXCLUSIVE: This window should have exclusive focus if it is on the top or overlay layer.
/// GTK_LAYER_SHELL_KEYBOARD_MODE_ON_DEMAND: The user should be able to focus and unfocues this window in an implementation
/// defined way. Not supported for protocol version < 4.
/// GTK_LAYER_SHELL_KEYBOARD_MODE_ENTRY_NUMBER: Should not be used except to get the number of entries. (NOTE: may change in
/// future releases as more entries are added)
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GtkLayerShellKeyboardMode")]
pub enum KeyboardMode {
    #[doc(alias = "GTK_LAYER_SHELL_KEYBOARD_MODE_NONE")]
    None,
    #[doc(alias = "GTK_LAYER_SHELL_KEYBOARD_MODE_EXCLUSIVE")]
    Exclusive,
    #[doc(alias = "GTK_LAYER_SHELL_KEYBOARD_MODE_ON_DEMAND")]
    OnDemand,
    #[doc(alias = "GTK_LAYER_SHELL_KEYBOARD_MODE_ENTRY_NUMBER")]
    EntryNumber,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for KeyboardMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyboardMode::{}", match *self {
            Self::None => "None",
            Self::Exclusive => "Exclusive",
            Self::OnDemand => "OnDemand",
            Self::EntryNumber => "EntryNumber",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for KeyboardMode {
    type GlibType = ffi::GtkLayerShellKeyboardMode;

    #[inline]
fn into_glib(self) -> ffi::GtkLayerShellKeyboardMode {
        match self {
            Self::None => ffi::GTK_LAYER_SHELL_KEYBOARD_MODE_NONE,
            Self::Exclusive => ffi::GTK_LAYER_SHELL_KEYBOARD_MODE_EXCLUSIVE,
            Self::OnDemand => ffi::GTK_LAYER_SHELL_KEYBOARD_MODE_ON_DEMAND,
            Self::EntryNumber => ffi::GTK_LAYER_SHELL_KEYBOARD_MODE_ENTRY_NUMBER,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkLayerShellKeyboardMode> for KeyboardMode {
    #[inline]
unsafe fn from_glib(value: ffi::GtkLayerShellKeyboardMode) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GTK_LAYER_SHELL_KEYBOARD_MODE_NONE => Self::None,
            ffi::GTK_LAYER_SHELL_KEYBOARD_MODE_EXCLUSIVE => Self::Exclusive,
            ffi::GTK_LAYER_SHELL_KEYBOARD_MODE_ON_DEMAND => Self::OnDemand,
            ffi::GTK_LAYER_SHELL_KEYBOARD_MODE_ENTRY_NUMBER => Self::EntryNumber,
            value => Self::__Unknown(value),
}
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GtkLayerShellLayer")]
pub enum Layer {
    /// The background layer.
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_BACKGROUND")]
    Background,
    /// The bottom layer.
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_BOTTOM")]
    Bottom,
    /// The top layer.
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_TOP")]
    Top,
    /// The overlay layer.
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_OVERLAY")]
    Overlay,
    /// Should not be used except to get the number of entries. (NOTE: may change in
    /// future releases as more entries are added)
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_ENTRY_NUMBER")]
    EntryNumber,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layer::{}", match *self {
            Self::Background => "Background",
            Self::Bottom => "Bottom",
            Self::Top => "Top",
            Self::Overlay => "Overlay",
            Self::EntryNumber => "EntryNumber",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for Layer {
    type GlibType = ffi::GtkLayerShellLayer;

    #[inline]
fn into_glib(self) -> ffi::GtkLayerShellLayer {
        match self {
            Self::Background => ffi::GTK_LAYER_SHELL_LAYER_BACKGROUND,
            Self::Bottom => ffi::GTK_LAYER_SHELL_LAYER_BOTTOM,
            Self::Top => ffi::GTK_LAYER_SHELL_LAYER_TOP,
            Self::Overlay => ffi::GTK_LAYER_SHELL_LAYER_OVERLAY,
            Self::EntryNumber => ffi::GTK_LAYER_SHELL_LAYER_ENTRY_NUMBER,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkLayerShellLayer> for Layer {
    #[inline]
unsafe fn from_glib(value: ffi::GtkLayerShellLayer) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GTK_LAYER_SHELL_LAYER_BACKGROUND => Self::Background,
            ffi::GTK_LAYER_SHELL_LAYER_BOTTOM => Self::Bottom,
            ffi::GTK_LAYER_SHELL_LAYER_TOP => Self::Top,
            ffi::GTK_LAYER_SHELL_LAYER_OVERLAY => Self::Overlay,
            ffi::GTK_LAYER_SHELL_LAYER_ENTRY_NUMBER => Self::EntryNumber,
            value => Self::__Unknown(value),
}
    }
}

