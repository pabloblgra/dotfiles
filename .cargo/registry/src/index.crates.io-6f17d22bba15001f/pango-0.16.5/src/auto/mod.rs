// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod context;
pub use self::context::Context;

mod font;
pub use self::font::Font;

mod font_face;
pub use self::font_face::FontFace;

mod font_family;
pub use self::font_family::FontFamily;

mod font_map;
pub use self::font_map::FontMap;

mod fontset;
pub use self::fontset::Fontset;

mod fontset_simple;
pub use self::fontset_simple::FontsetSimple;

mod layout;
pub use self::layout::Layout;

mod renderer;
pub use self::renderer::Renderer;

mod attr_list;
pub use self::attr_list::AttrList;

mod attribute;
pub use self::attribute::Attribute;

mod color;
pub use self::color::Color;

mod font_description;
pub use self::font_description::FontDescription;

mod font_metrics;
pub use self::font_metrics::FontMetrics;

mod glyph_item;
pub use self::glyph_item::GlyphItem;

mod glyph_string;
pub use self::glyph_string::GlyphString;

mod item;
pub use self::item::Item;

mod language;
pub use self::language::Language;

mod layout_iter;
pub use self::layout_iter::LayoutIter;

mod layout_line;
pub use self::layout_line::LayoutLine;

mod matrix;
pub use self::matrix::Matrix;

mod tab_array;
pub use self::tab_array::TabArray;

mod enums;
pub use self::enums::Alignment;
pub use self::enums::AttrType;
#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
pub use self::enums::BaselineShift;
pub use self::enums::BidiType;
pub use self::enums::CoverageLevel;
pub use self::enums::Direction;
pub use self::enums::EllipsizeMode;
#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
pub use self::enums::FontScale;
pub use self::enums::Gravity;
pub use self::enums::GravityHint;
#[cfg(any(feature = "v1_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
pub use self::enums::Overline;
pub use self::enums::RenderPart;
pub use self::enums::Script;
pub use self::enums::Stretch;
pub use self::enums::Style;
pub use self::enums::TabAlign;
#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
pub use self::enums::TextTransform;
pub use self::enums::Underline;
pub use self::enums::Variant;
pub use self::enums::Weight;
pub use self::enums::WrapMode;

mod flags;
pub use self::flags::FontMask;
#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
pub use self::flags::LayoutDeserializeFlags;
#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
pub use self::flags::LayoutSerializeFlags;
#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
pub use self::flags::ShapeFlags;
#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
pub use self::flags::ShowFlags;

mod alias;
pub use self::alias::Glyph;
pub use self::alias::GlyphUnit;
pub use self::alias::LayoutRun;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::font::FontExt;
    pub use super::font_face::FontFaceExt;
    pub use super::font_family::FontFamilyExt;
    pub use super::font_map::FontMapExt;
    pub use super::fontset::FontsetExt;
    pub use super::renderer::RendererExt;
}
