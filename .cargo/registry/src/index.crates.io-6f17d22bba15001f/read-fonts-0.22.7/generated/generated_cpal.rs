// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [CPAL (Color Palette Table)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-table-header) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct CpalMarker {
    color_record_indices_byte_len: usize,
    palette_types_array_offset_byte_start: Option<usize>,
    palette_labels_array_offset_byte_start: Option<usize>,
    palette_entry_labels_array_offset_byte_start: Option<usize>,
}

impl CpalMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn num_palette_entries_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn num_palettes_byte_range(&self) -> Range<usize> {
        let start = self.num_palette_entries_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn num_color_records_byte_range(&self) -> Range<usize> {
        let start = self.num_palettes_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn color_records_array_offset_byte_range(&self) -> Range<usize> {
        let start = self.num_color_records_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn color_record_indices_byte_range(&self) -> Range<usize> {
        let start = self.color_records_array_offset_byte_range().end;
        start..start + self.color_record_indices_byte_len
    }
    fn palette_types_array_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.palette_types_array_offset_byte_start?;
        Some(start..start + Offset32::RAW_BYTE_LEN)
    }
    fn palette_labels_array_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.palette_labels_array_offset_byte_start?;
        Some(start..start + Offset32::RAW_BYTE_LEN)
    }
    fn palette_entry_labels_array_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.palette_entry_labels_array_offset_byte_start?;
        Some(start..start + Offset32::RAW_BYTE_LEN)
    }
}

impl TopLevelTable for Cpal<'_> {
    /// `CPAL`
    const TAG: Tag = Tag::new(b"CPAL");
}

impl<'a> FontRead<'a> for Cpal<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let version: u16 = cursor.read()?;
        cursor.advance::<u16>();
        let num_palettes: u16 = cursor.read()?;
        cursor.advance::<u16>();
        cursor.advance::<Offset32>();
        let color_record_indices_byte_len = (num_palettes as usize)
            .checked_mul(u16::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(color_record_indices_byte_len);
        let palette_types_array_offset_byte_start = version
            .compatible(1u16)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(1u16)
            .then(|| cursor.advance::<Offset32>());
        let palette_labels_array_offset_byte_start = version
            .compatible(1u16)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(1u16)
            .then(|| cursor.advance::<Offset32>());
        let palette_entry_labels_array_offset_byte_start = version
            .compatible(1u16)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(1u16)
            .then(|| cursor.advance::<Offset32>());
        cursor.finish(CpalMarker {
            color_record_indices_byte_len,
            palette_types_array_offset_byte_start,
            palette_labels_array_offset_byte_start,
            palette_entry_labels_array_offset_byte_start,
        })
    }
}

/// [CPAL (Color Palette Table)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-table-header) table
pub type Cpal<'a> = TableRef<'a, CpalMarker>;

impl<'a> Cpal<'a> {
    /// Table version number (=0).
    pub fn version(&self) -> u16 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of palette entries in each palette.
    pub fn num_palette_entries(&self) -> u16 {
        let range = self.shape.num_palette_entries_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of palettes in the table.
    pub fn num_palettes(&self) -> u16 {
        let range = self.shape.num_palettes_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Total number of color records, combined for all palettes.
    pub fn num_color_records(&self) -> u16 {
        let range = self.shape.num_color_records_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset from the beginning of CPAL table to the first
    /// ColorRecord.
    pub fn color_records_array_offset(&self) -> Nullable<Offset32> {
        let range = self.shape.color_records_array_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`color_records_array_offset`][Self::color_records_array_offset].
    pub fn color_records_array(&self) -> Option<Result<&'a [ColorRecord], ReadError>> {
        let data = self.data;
        let args = self.num_color_records();
        self.color_records_array_offset()
            .resolve_with_args(data, &args)
    }

    /// Index of each palette’s first color record in the combined
    /// color record array.
    pub fn color_record_indices(&self) -> &'a [BigEndian<u16>] {
        let range = self.shape.color_record_indices_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Offset from the beginning of CPAL table to the [Palette Types Array][].
    ///
    /// This is an array of 32-bit flag fields that describe properties of each palette.
    ///
    /// [Palette Types Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-type-array
    pub fn palette_types_array_offset(&self) -> Option<Nullable<Offset32>> {
        let range = self.shape.palette_types_array_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`palette_types_array_offset`][Self::palette_types_array_offset].
    pub fn palette_types_array(&self) -> Option<Result<&'a [BigEndian<PaletteType>], ReadError>> {
        let data = self.data;
        let args = self.num_palettes();
        self.palette_types_array_offset()
            .map(|x| x.resolve_with_args(data, &args))?
    }

    /// Offset from the beginning of CPAL table to the [Palette Labels Array][].
    ///
    /// This is an array of 'name' table IDs (typically in the font-specific name
    /// ID range) that specify user interface strings associated with  each palette.
    /// Use 0xFFFF if no name ID is provided for a palette.
    ///
    /// [Palette Labels Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-labels-array
    pub fn palette_labels_array_offset(&self) -> Option<Nullable<Offset32>> {
        let range = self.shape.palette_labels_array_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`palette_labels_array_offset`][Self::palette_labels_array_offset].
    pub fn palette_labels_array(&self) -> Option<Result<&'a [BigEndian<u16>], ReadError>> {
        let data = self.data;
        let args = self.num_palettes();
        self.palette_labels_array_offset()
            .map(|x| x.resolve_with_args(data, &args))?
    }

    /// Offset from the beginning of CPAL table to the [Palette Entry Labels Array][].
    ///
    /// This is an array of 'name' table IDs (typically in the font-specific name
    /// ID range) that specify user interface strings associated with  each palette
    /// entry, e.g. “Outline”, “Fill”. This set of palette entry labels applies
    /// to all palettes in the font. Use  0xFFFF if no name ID is provided for a
    /// palette entry.
    ///
    /// [Palette Entry Labels Array]: https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entry-label-array
    pub fn palette_entry_labels_array_offset(&self) -> Option<Nullable<Offset32>> {
        let range = self.shape.palette_entry_labels_array_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`palette_entry_labels_array_offset`][Self::palette_entry_labels_array_offset].
    pub fn palette_entry_labels_array(&self) -> Option<Result<&'a [BigEndian<NameId>], ReadError>> {
        let data = self.data;
        let args = self.num_palette_entries();
        self.palette_entry_labels_array_offset()
            .map(|x| x.resolve_with_args(data, &args))?
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Cpal<'a> {
    fn type_name(&self) -> &str {
        "Cpal"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new(
                "num_palette_entries",
                self.num_palette_entries(),
            )),
            2usize => Some(Field::new("num_palettes", self.num_palettes())),
            3usize => Some(Field::new("num_color_records", self.num_color_records())),
            4usize => Some(Field::new(
                "color_records_array_offset",
                traversal::FieldType::offset_to_array_of_records(
                    self.color_records_array_offset(),
                    self.color_records_array(),
                    stringify!(ColorRecord),
                    self.offset_data(),
                ),
            )),
            5usize => Some(Field::new(
                "color_record_indices",
                self.color_record_indices(),
            )),
            6usize if version.compatible(1u16) => Some(Field::new(
                "palette_types_array_offset",
                FieldType::offset_to_array_of_scalars(
                    self.palette_types_array_offset().unwrap(),
                    self.palette_types_array(),
                ),
            )),
            7usize if version.compatible(1u16) => Some(Field::new(
                "palette_labels_array_offset",
                FieldType::offset_to_array_of_scalars(
                    self.palette_labels_array_offset().unwrap(),
                    self.palette_labels_array(),
                ),
            )),
            8usize if version.compatible(1u16) => Some(Field::new(
                "palette_entry_labels_array_offset",
                FieldType::offset_to_array_of_scalars(
                    self.palette_entry_labels_array_offset().unwrap(),
                    self.palette_entry_labels_array(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for Cpal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// The [PaletteType](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-type-array) flags.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck :: AnyBitPattern)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct PaletteType {
    bits: u32,
}

impl PaletteType {
    /// Bit 0: palette is appropriate to use when displaying the font on a light background such as white.
    pub const USABLE_WITH_LIGHT_BACKGROUND: Self = Self { bits: 0x0001 };

    /// Bit 1: palette is appropriate to use when displaying the font on a dark background such as black.
    pub const USABLE_WITH_DARK_BACKGROUND: Self = Self { bits: 0x0002 };
}

impl PaletteType {
    ///  Returns an empty set of flags.
    #[inline]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Returns the set containing all flags.
    #[inline]
    pub const fn all() -> Self {
        Self {
            bits: Self::USABLE_WITH_LIGHT_BACKGROUND.bits | Self::USABLE_WITH_DARK_BACKGROUND.bits,
        }
    }

    /// Returns the raw value of the flags currently stored.
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self { bits })
        } else {
            None
        }
    }

    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self {
            bits: bits & Self::all().bits,
        }
    }

    /// Returns `true` if no flags are currently stored.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }

    /// Returns `true` if there are flags common to both `self` and `other`.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !(Self {
            bits: self.bits & other.bits,
        })
        .is_empty()
    }

    /// Returns `true` if all of the flags in `other` are contained within `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    /// Inserts the specified flags in-place.
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }

    /// Removes the specified flags in-place.
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.bits &= !other.bits;
    }

    /// Toggles the specified flags in-place.
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.bits ^= other.bits;
    }

    /// Returns the intersection between the flags in `self` and
    /// `other`.
    ///
    /// Specifically, the returned set contains only the flags which are
    /// present in *both* `self` *and* `other`.
    ///
    /// This is equivalent to using the `&` operator (e.g.
    /// [`ops::BitAnd`]), as in `flags & other`.
    ///
    /// [`ops::BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }

    /// Returns the union of between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags which are
    /// present in *either* `self` *or* `other`, including any which are
    /// present in both.
    ///
    /// This is equivalent to using the `|` operator (e.g.
    /// [`ops::BitOr`]), as in `flags | other`.
    ///
    /// [`ops::BitOr`]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Returns the difference between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags present in
    /// `self`, except for the ones present in `other`.
    ///
    /// It is also conceptually equivalent to the "bit-clear" operation:
    /// `flags & !other` (and this syntax is also supported).
    ///
    /// This is equivalent to using the `-` operator (e.g.
    /// [`ops::Sub`]), as in `flags - other`.
    ///
    /// [`ops::Sub`]: https://doc.rust-lang.org/std/ops/trait.Sub.html
    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl std::ops::BitOr for PaletteType {
    type Output = Self;

    /// Returns the union of the two sets of flags.
    #[inline]
    fn bitor(self, other: PaletteType) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}

impl std::ops::BitOrAssign for PaletteType {
    /// Adds the set of flags.
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.bits |= other.bits;
    }
}

impl std::ops::BitXor for PaletteType {
    type Output = Self;

    /// Returns the left flags, but with all the right flags toggled.
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self {
            bits: self.bits ^ other.bits,
        }
    }
}

impl std::ops::BitXorAssign for PaletteType {
    /// Toggles the set of flags.
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.bits ^= other.bits;
    }
}

impl std::ops::BitAnd for PaletteType {
    type Output = Self;

    /// Returns the intersection between the two sets of flags.
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }
}

impl std::ops::BitAndAssign for PaletteType {
    /// Disables all flags disabled in the set.
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.bits &= other.bits;
    }
}

impl std::ops::Sub for PaletteType {
    type Output = Self;

    /// Returns the set difference of the two sets of flags.
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl std::ops::SubAssign for PaletteType {
    /// Disables all flags enabled in the set.
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.bits &= !other.bits;
    }
}

impl std::ops::Not for PaletteType {
    type Output = Self;

    /// Returns the complement of this set of flags.
    #[inline]
    fn not(self) -> Self {
        Self { bits: !self.bits } & Self::all()
    }
}

impl std::fmt::Debug for PaletteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let members: &[(&str, Self)] = &[
            (
                "USABLE_WITH_LIGHT_BACKGROUND",
                Self::USABLE_WITH_LIGHT_BACKGROUND,
            ),
            (
                "USABLE_WITH_DARK_BACKGROUND",
                Self::USABLE_WITH_DARK_BACKGROUND,
            ),
        ];
        let mut first = true;
        for (name, value) in members {
            if self.contains(*value) {
                if !first {
                    f.write_str(" | ")?;
                }
                first = false;
                f.write_str(name)?;
            }
        }
        if first {
            f.write_str("(empty)")?;
        }
        Ok(())
    }
}

impl std::fmt::Binary for PaletteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.bits, f)
    }
}

impl std::fmt::Octal for PaletteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.bits, f)
    }
}

impl std::fmt::LowerHex for PaletteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.bits, f)
    }
}

impl std::fmt::UpperHex for PaletteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.bits, f)
    }
}

impl font_types::Scalar for PaletteType {
    type Raw = <u32 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.bits().to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u32>::from_raw(raw);
        Self::from_bits_truncate(t)
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> From<PaletteType> for FieldType<'a> {
    fn from(src: PaletteType) -> FieldType<'a> {
        src.bits().into()
    }
}

/// [CPAL (Color Record)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entries-and-color-records) record
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct ColorRecord {
    /// Blue value (B0).
    pub blue: u8,
    /// Green value (B1).
    pub green: u8,
    ///     Red value (B2).
    pub red: u8,
    /// Alpha value (B3).
    pub alpha: u8,
}

impl ColorRecord {
    /// Blue value (B0).
    pub fn blue(&self) -> u8 {
        self.blue
    }

    /// Green value (B1).
    pub fn green(&self) -> u8 {
        self.green
    }

    ///     Red value (B2).
    pub fn red(&self) -> u8 {
        self.red
    }

    /// Alpha value (B3).
    pub fn alpha(&self) -> u8 {
        self.alpha
    }
}

impl FixedSize for ColorRecord {
    const RAW_BYTE_LEN: usize =
        u8::RAW_BYTE_LEN + u8::RAW_BYTE_LEN + u8::RAW_BYTE_LEN + u8::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for ColorRecord {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "ColorRecord",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("blue", self.blue())),
                1usize => Some(Field::new("green", self.green())),
                2usize => Some(Field::new("red", self.red())),
                3usize => Some(Field::new("alpha", self.alpha())),
                _ => None,
            }),
            data,
        }
    }
}
