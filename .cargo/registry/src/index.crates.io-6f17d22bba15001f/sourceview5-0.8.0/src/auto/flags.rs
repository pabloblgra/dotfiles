// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::{bitflags::bitflags, prelude::*, translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GtkSourceFileSaverFlags")]
    pub struct FileSaverFlags: u32 {
        #[doc(alias = "GTK_SOURCE_FILE_SAVER_FLAGS_NONE")]
        const NONE = ffi::GTK_SOURCE_FILE_SAVER_FLAGS_NONE as _;
        #[doc(alias = "GTK_SOURCE_FILE_SAVER_FLAGS_IGNORE_INVALID_CHARS")]
        const IGNORE_INVALID_CHARS = ffi::GTK_SOURCE_FILE_SAVER_FLAGS_IGNORE_INVALID_CHARS as _;
        #[doc(alias = "GTK_SOURCE_FILE_SAVER_FLAGS_IGNORE_MODIFICATION_TIME")]
        const IGNORE_MODIFICATION_TIME = ffi::GTK_SOURCE_FILE_SAVER_FLAGS_IGNORE_MODIFICATION_TIME as _;
        #[doc(alias = "GTK_SOURCE_FILE_SAVER_FLAGS_CREATE_BACKUP")]
        const CREATE_BACKUP = ffi::GTK_SOURCE_FILE_SAVER_FLAGS_CREATE_BACKUP as _;
    }
}

#[doc(hidden)]
impl IntoGlib for FileSaverFlags {
    type GlibType = ffi::GtkSourceFileSaverFlags;

    #[inline]
    fn into_glib(self) -> ffi::GtkSourceFileSaverFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceFileSaverFlags> for FileSaverFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GtkSourceFileSaverFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for FileSaverFlags {
    #[inline]
    #[doc(alias = "gtk_source_file_saver_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_source_file_saver_flags_get_type()) }
    }
}

impl glib::HasParamSpec for FileSaverFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for FileSaverFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for FileSaverFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FileSaverFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FileSaverFlags> for glib::Value {
    #[inline]
    fn from(v: FileSaverFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GtkSourceSortFlags")]
    pub struct SortFlags: u32 {
        #[doc(alias = "GTK_SOURCE_SORT_FLAGS_NONE")]
        const NONE = ffi::GTK_SOURCE_SORT_FLAGS_NONE as _;
        #[doc(alias = "GTK_SOURCE_SORT_FLAGS_CASE_SENSITIVE")]
        const CASE_SENSITIVE = ffi::GTK_SOURCE_SORT_FLAGS_CASE_SENSITIVE as _;
        #[doc(alias = "GTK_SOURCE_SORT_FLAGS_REVERSE_ORDER")]
        const REVERSE_ORDER = ffi::GTK_SOURCE_SORT_FLAGS_REVERSE_ORDER as _;
        #[doc(alias = "GTK_SOURCE_SORT_FLAGS_REMOVE_DUPLICATES")]
        const REMOVE_DUPLICATES = ffi::GTK_SOURCE_SORT_FLAGS_REMOVE_DUPLICATES as _;
    }
}

#[doc(hidden)]
impl IntoGlib for SortFlags {
    type GlibType = ffi::GtkSourceSortFlags;

    #[inline]
    fn into_glib(self) -> ffi::GtkSourceSortFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceSortFlags> for SortFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GtkSourceSortFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for SortFlags {
    #[inline]
    #[doc(alias = "gtk_source_sort_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_source_sort_flags_get_type()) }
    }
}

impl glib::HasParamSpec for SortFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for SortFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for SortFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for SortFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SortFlags> for glib::Value {
    #[inline]
    fn from(v: SortFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GtkSourceSpaceLocationFlags")]
    pub struct SpaceLocationFlags: u32 {
        #[doc(alias = "GTK_SOURCE_SPACE_LOCATION_NONE")]
        const NONE = ffi::GTK_SOURCE_SPACE_LOCATION_NONE as _;
        #[doc(alias = "GTK_SOURCE_SPACE_LOCATION_LEADING")]
        const LEADING = ffi::GTK_SOURCE_SPACE_LOCATION_LEADING as _;
        #[doc(alias = "GTK_SOURCE_SPACE_LOCATION_INSIDE_TEXT")]
        const INSIDE_TEXT = ffi::GTK_SOURCE_SPACE_LOCATION_INSIDE_TEXT as _;
        #[doc(alias = "GTK_SOURCE_SPACE_LOCATION_TRAILING")]
        const TRAILING = ffi::GTK_SOURCE_SPACE_LOCATION_TRAILING as _;
        #[doc(alias = "GTK_SOURCE_SPACE_LOCATION_ALL")]
        const ALL = ffi::GTK_SOURCE_SPACE_LOCATION_ALL as _;
    }
}

#[doc(hidden)]
impl IntoGlib for SpaceLocationFlags {
    type GlibType = ffi::GtkSourceSpaceLocationFlags;

    #[inline]
    fn into_glib(self) -> ffi::GtkSourceSpaceLocationFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceSpaceLocationFlags> for SpaceLocationFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GtkSourceSpaceLocationFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for SpaceLocationFlags {
    #[inline]
    #[doc(alias = "gtk_source_space_location_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_source_space_location_flags_get_type()) }
    }
}

impl glib::HasParamSpec for SpaceLocationFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for SpaceLocationFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for SpaceLocationFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for SpaceLocationFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SpaceLocationFlags> for glib::Value {
    #[inline]
    fn from(v: SpaceLocationFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GtkSourceSpaceTypeFlags")]
    pub struct SpaceTypeFlags: u32 {
        #[doc(alias = "GTK_SOURCE_SPACE_TYPE_NONE")]
        const NONE = ffi::GTK_SOURCE_SPACE_TYPE_NONE as _;
        #[doc(alias = "GTK_SOURCE_SPACE_TYPE_SPACE")]
        const SPACE = ffi::GTK_SOURCE_SPACE_TYPE_SPACE as _;
        #[doc(alias = "GTK_SOURCE_SPACE_TYPE_TAB")]
        const TAB = ffi::GTK_SOURCE_SPACE_TYPE_TAB as _;
        #[doc(alias = "GTK_SOURCE_SPACE_TYPE_NEWLINE")]
        const NEWLINE = ffi::GTK_SOURCE_SPACE_TYPE_NEWLINE as _;
        #[doc(alias = "GTK_SOURCE_SPACE_TYPE_NBSP")]
        const NBSP = ffi::GTK_SOURCE_SPACE_TYPE_NBSP as _;
        #[doc(alias = "GTK_SOURCE_SPACE_TYPE_ALL")]
        const ALL = ffi::GTK_SOURCE_SPACE_TYPE_ALL as _;
    }
}

#[doc(hidden)]
impl IntoGlib for SpaceTypeFlags {
    type GlibType = ffi::GtkSourceSpaceTypeFlags;

    #[inline]
    fn into_glib(self) -> ffi::GtkSourceSpaceTypeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceSpaceTypeFlags> for SpaceTypeFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GtkSourceSpaceTypeFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for SpaceTypeFlags {
    #[inline]
    #[doc(alias = "gtk_source_space_type_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_source_space_type_flags_get_type()) }
    }
}

impl glib::HasParamSpec for SpaceTypeFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for SpaceTypeFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for SpaceTypeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for SpaceTypeFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SpaceTypeFlags> for glib::Value {
    #[inline]
    fn from(v: SpaceTypeFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
