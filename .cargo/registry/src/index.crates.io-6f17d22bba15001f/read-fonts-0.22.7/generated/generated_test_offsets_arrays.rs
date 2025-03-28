// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct KindsOfOffsetsMarker {
    versioned_nullable_record_array_offset_byte_start: Option<usize>,
    versioned_nonnullable_offset_byte_start: Option<usize>,
    versioned_nullable_offset_byte_start: Option<usize>,
}

impl KindsOfOffsetsMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn nonnullable_offset_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn nullable_offset_byte_range(&self) -> Range<usize> {
        let start = self.nonnullable_offset_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn array_offset_count_byte_range(&self) -> Range<usize> {
        let start = self.nullable_offset_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn array_offset_byte_range(&self) -> Range<usize> {
        let start = self.array_offset_count_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn record_array_offset_byte_range(&self) -> Range<usize> {
        let start = self.array_offset_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn versioned_nullable_record_array_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nullable_record_array_offset_byte_start?;
        Some(start..start + Offset16::RAW_BYTE_LEN)
    }
    fn versioned_nonnullable_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nonnullable_offset_byte_start?;
        Some(start..start + Offset16::RAW_BYTE_LEN)
    }
    fn versioned_nullable_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nullable_offset_byte_start?;
        Some(start..start + Offset32::RAW_BYTE_LEN)
    }
}

impl<'a> FontRead<'a> for KindsOfOffsets<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let version: MajorMinor = cursor.read()?;
        cursor.advance::<Offset16>();
        cursor.advance::<Offset16>();
        cursor.advance::<u16>();
        cursor.advance::<Offset16>();
        cursor.advance::<Offset16>();
        let versioned_nullable_record_array_offset_byte_start = version
            .compatible((1u16, 1u16))
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible((1u16, 1u16))
            .then(|| cursor.advance::<Offset16>());
        let versioned_nonnullable_offset_byte_start = version
            .compatible((1u16, 1u16))
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible((1u16, 1u16))
            .then(|| cursor.advance::<Offset16>());
        let versioned_nullable_offset_byte_start = version
            .compatible((1u16, 1u16))
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible((1u16, 1u16))
            .then(|| cursor.advance::<Offset32>());
        cursor.finish(KindsOfOffsetsMarker {
            versioned_nullable_record_array_offset_byte_start,
            versioned_nonnullable_offset_byte_start,
            versioned_nullable_offset_byte_start,
        })
    }
}

pub type KindsOfOffsets<'a> = TableRef<'a, KindsOfOffsetsMarker>;

impl<'a> KindsOfOffsets<'a> {
    /// The major/minor version of the GDEF table
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// A normal offset
    pub fn nonnullable_offset(&self) -> Offset16 {
        let range = self.shape.nonnullable_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`nonnullable_offset`][Self::nonnullable_offset].
    pub fn nonnullable(&self) -> Result<Dummy<'a>, ReadError> {
        let data = self.data;
        self.nonnullable_offset().resolve(data)
    }

    /// An offset that is nullable, but always present
    pub fn nullable_offset(&self) -> Nullable<Offset16> {
        let range = self.shape.nullable_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`nullable_offset`][Self::nullable_offset].
    pub fn nullable(&self) -> Option<Result<Dummy<'a>, ReadError>> {
        let data = self.data;
        self.nullable_offset().resolve(data)
    }

    /// count of the array at array_offset
    pub fn array_offset_count(&self) -> u16 {
        let range = self.shape.array_offset_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// An offset to an array:
    pub fn array_offset(&self) -> Offset16 {
        let range = self.shape.array_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`array_offset`][Self::array_offset].
    pub fn array(&self) -> Result<&'a [BigEndian<u16>], ReadError> {
        let data = self.data;
        let args = self.array_offset_count();
        self.array_offset().resolve_with_args(data, &args)
    }

    /// An offset to an array of records
    pub fn record_array_offset(&self) -> Offset16 {
        let range = self.shape.record_array_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`record_array_offset`][Self::record_array_offset].
    pub fn record_array(&self) -> Result<&'a [Shmecord], ReadError> {
        let data = self.data;
        let args = self.array_offset_count();
        self.record_array_offset().resolve_with_args(data, &args)
    }

    /// A nullable, versioned offset to an array of records
    pub fn versioned_nullable_record_array_offset(&self) -> Option<Nullable<Offset16>> {
        let range = self
            .shape
            .versioned_nullable_record_array_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`versioned_nullable_record_array_offset`][Self::versioned_nullable_record_array_offset].
    pub fn versioned_nullable_record_array(&self) -> Option<Result<&'a [Shmecord], ReadError>> {
        let data = self.data;
        let args = self.array_offset_count();
        self.versioned_nullable_record_array_offset()
            .map(|x| x.resolve_with_args(data, &args))?
    }

    /// A normal offset that is versioned
    pub fn versioned_nonnullable_offset(&self) -> Option<Offset16> {
        let range = self.shape.versioned_nonnullable_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`versioned_nonnullable_offset`][Self::versioned_nonnullable_offset].
    pub fn versioned_nonnullable(&self) -> Option<Result<Dummy<'a>, ReadError>> {
        let data = self.data;
        self.versioned_nonnullable_offset().map(|x| x.resolve(data))
    }

    /// An offset that is nullable and versioned
    pub fn versioned_nullable_offset(&self) -> Option<Nullable<Offset32>> {
        let range = self.shape.versioned_nullable_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`versioned_nullable_offset`][Self::versioned_nullable_offset].
    pub fn versioned_nullable(&self) -> Option<Result<Dummy<'a>, ReadError>> {
        let data = self.data;
        self.versioned_nullable_offset().map(|x| x.resolve(data))?
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for KindsOfOffsets<'a> {
    fn type_name(&self) -> &str {
        "KindsOfOffsets"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new(
                "nonnullable_offset",
                FieldType::offset(self.nonnullable_offset(), self.nonnullable()),
            )),
            2usize => Some(Field::new(
                "nullable_offset",
                FieldType::offset(self.nullable_offset(), self.nullable()),
            )),
            3usize => Some(Field::new("array_offset_count", self.array_offset_count())),
            4usize => Some(Field::new(
                "array_offset",
                FieldType::offset_to_array_of_scalars(self.array_offset(), self.array()),
            )),
            5usize => Some(Field::new(
                "record_array_offset",
                traversal::FieldType::offset_to_array_of_records(
                    self.record_array_offset(),
                    self.record_array(),
                    stringify!(Shmecord),
                    self.offset_data(),
                ),
            )),
            6usize if version.compatible((1u16, 1u16)) => Some(Field::new(
                "versioned_nullable_record_array_offset",
                traversal::FieldType::offset_to_array_of_records(
                    self.versioned_nullable_record_array_offset().unwrap(),
                    self.versioned_nullable_record_array(),
                    stringify!(Shmecord),
                    self.offset_data(),
                ),
            )),
            7usize if version.compatible((1u16, 1u16)) => Some(Field::new(
                "versioned_nonnullable_offset",
                FieldType::offset(
                    self.versioned_nonnullable_offset().unwrap(),
                    self.versioned_nonnullable().unwrap(),
                ),
            )),
            8usize if version.compatible((1u16, 1u16)) => Some(Field::new(
                "versioned_nullable_offset",
                FieldType::offset(
                    self.versioned_nullable_offset().unwrap(),
                    self.versioned_nullable(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for KindsOfOffsets<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct KindsOfArraysOfOffsetsMarker {
    nonnullable_offsets_byte_len: usize,
    nullable_offsets_byte_len: usize,
    versioned_nonnullable_offsets_byte_start: Option<usize>,
    versioned_nonnullable_offsets_byte_len: Option<usize>,
    versioned_nullable_offsets_byte_start: Option<usize>,
    versioned_nullable_offsets_byte_len: Option<usize>,
}

impl KindsOfArraysOfOffsetsMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn count_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn nonnullable_offsets_byte_range(&self) -> Range<usize> {
        let start = self.count_byte_range().end;
        start..start + self.nonnullable_offsets_byte_len
    }
    fn nullable_offsets_byte_range(&self) -> Range<usize> {
        let start = self.nonnullable_offsets_byte_range().end;
        start..start + self.nullable_offsets_byte_len
    }
    fn versioned_nonnullable_offsets_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nonnullable_offsets_byte_start?;
        Some(start..start + self.versioned_nonnullable_offsets_byte_len?)
    }
    fn versioned_nullable_offsets_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_nullable_offsets_byte_start?;
        Some(start..start + self.versioned_nullable_offsets_byte_len?)
    }
}

impl<'a> FontRead<'a> for KindsOfArraysOfOffsets<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let version: MajorMinor = cursor.read()?;
        let count: u16 = cursor.read()?;
        let nonnullable_offsets_byte_len = (count as usize)
            .checked_mul(Offset16::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(nonnullable_offsets_byte_len);
        let nullable_offsets_byte_len = (count as usize)
            .checked_mul(Offset16::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(nullable_offsets_byte_len);
        let versioned_nonnullable_offsets_byte_start = version
            .compatible((1u16, 1u16))
            .then(|| cursor.position())
            .transpose()?;
        let versioned_nonnullable_offsets_byte_len = version.compatible((1u16, 1u16)).then_some(
            (count as usize)
                .checked_mul(Offset16::RAW_BYTE_LEN)
                .ok_or(ReadError::OutOfBounds)?,
        );
        if let Some(value) = versioned_nonnullable_offsets_byte_len {
            cursor.advance_by(value);
        }
        let versioned_nullable_offsets_byte_start = version
            .compatible((1u16, 1u16))
            .then(|| cursor.position())
            .transpose()?;
        let versioned_nullable_offsets_byte_len = version.compatible((1u16, 1u16)).then_some(
            (count as usize)
                .checked_mul(Offset16::RAW_BYTE_LEN)
                .ok_or(ReadError::OutOfBounds)?,
        );
        if let Some(value) = versioned_nullable_offsets_byte_len {
            cursor.advance_by(value);
        }
        cursor.finish(KindsOfArraysOfOffsetsMarker {
            nonnullable_offsets_byte_len,
            nullable_offsets_byte_len,
            versioned_nonnullable_offsets_byte_start,
            versioned_nonnullable_offsets_byte_len,
            versioned_nullable_offsets_byte_start,
            versioned_nullable_offsets_byte_len,
        })
    }
}

pub type KindsOfArraysOfOffsets<'a> = TableRef<'a, KindsOfArraysOfOffsetsMarker>;

impl<'a> KindsOfArraysOfOffsets<'a> {
    /// The version
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of items in each array
    pub fn count(&self) -> u16 {
        let range = self.shape.count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// A normal array offset
    pub fn nonnullable_offsets(&self) -> &'a [BigEndian<Offset16>] {
        let range = self.shape.nonnullable_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// A dynamically resolving wrapper for [`nonnullable_offsets`][Self::nonnullable_offsets].
    pub fn nonnullables(&self) -> ArrayOfOffsets<'a, Dummy<'a>, Offset16> {
        let data = self.data;
        let offsets = self.nonnullable_offsets();
        ArrayOfOffsets::new(offsets, data, ())
    }

    /// An offset that is nullable, but always present
    pub fn nullable_offsets(&self) -> &'a [BigEndian<Nullable<Offset16>>] {
        let range = self.shape.nullable_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// A dynamically resolving wrapper for [`nullable_offsets`][Self::nullable_offsets].
    pub fn nullables(&self) -> ArrayOfNullableOffsets<'a, Dummy<'a>, Offset16> {
        let data = self.data;
        let offsets = self.nullable_offsets();
        ArrayOfNullableOffsets::new(offsets, data, ())
    }

    /// A normal offset that is versioned
    pub fn versioned_nonnullable_offsets(&self) -> Option<&'a [BigEndian<Offset16>]> {
        let range = self.shape.versioned_nonnullable_offsets_byte_range()?;
        Some(self.data.read_array(range).unwrap())
    }

    /// A dynamically resolving wrapper for [`versioned_nonnullable_offsets`][Self::versioned_nonnullable_offsets].
    pub fn versioned_nonnullables(&self) -> Option<ArrayOfOffsets<'a, Dummy<'a>, Offset16>> {
        let data = self.data;
        let offsets = self.versioned_nonnullable_offsets();
        offsets.map(|offsets| ArrayOfOffsets::new(offsets, data, ()))
    }

    /// An offset that is nullable and versioned
    pub fn versioned_nullable_offsets(&self) -> Option<&'a [BigEndian<Nullable<Offset16>>]> {
        let range = self.shape.versioned_nullable_offsets_byte_range()?;
        Some(self.data.read_array(range).unwrap())
    }

    /// A dynamically resolving wrapper for [`versioned_nullable_offsets`][Self::versioned_nullable_offsets].
    pub fn versioned_nullables(&self) -> Option<ArrayOfNullableOffsets<'a, Dummy<'a>, Offset16>> {
        let data = self.data;
        let offsets = self.versioned_nullable_offsets();
        offsets.map(|offsets| ArrayOfNullableOffsets::new(offsets, data, ()))
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for KindsOfArraysOfOffsets<'a> {
    fn type_name(&self) -> &str {
        "KindsOfArraysOfOffsets"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("count", self.count())),
            2usize => Some({
                let data = self.data;
                Field::new(
                    "nonnullable_offsets",
                    FieldType::array_of_offsets(
                        better_type_name::<Dummy>(),
                        self.nonnullable_offsets(),
                        move |off| {
                            let target = off.get().resolve::<Dummy>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            3usize => Some({
                let data = self.data;
                Field::new(
                    "nullable_offsets",
                    FieldType::array_of_offsets(
                        better_type_name::<Dummy>(),
                        self.nullable_offsets(),
                        move |off| {
                            let target = off.get().resolve::<Dummy>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            4usize if version.compatible((1u16, 1u16)) => Some({
                let data = self.data;
                Field::new(
                    "versioned_nonnullable_offsets",
                    FieldType::array_of_offsets(
                        better_type_name::<Dummy>(),
                        self.versioned_nonnullable_offsets().unwrap(),
                        move |off| {
                            let target = off.get().resolve::<Dummy>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            5usize if version.compatible((1u16, 1u16)) => Some({
                let data = self.data;
                Field::new(
                    "versioned_nullable_offsets",
                    FieldType::array_of_offsets(
                        better_type_name::<Dummy>(),
                        self.versioned_nullable_offsets().unwrap(),
                        move |off| {
                            let target = off.get().resolve::<Dummy>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for KindsOfArraysOfOffsets<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct KindsOfArraysMarker {
    scalars_byte_len: usize,
    records_byte_len: usize,
    versioned_scalars_byte_start: Option<usize>,
    versioned_scalars_byte_len: Option<usize>,
    versioned_records_byte_start: Option<usize>,
    versioned_records_byte_len: Option<usize>,
}

impl KindsOfArraysMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn count_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn scalars_byte_range(&self) -> Range<usize> {
        let start = self.count_byte_range().end;
        start..start + self.scalars_byte_len
    }
    fn records_byte_range(&self) -> Range<usize> {
        let start = self.scalars_byte_range().end;
        start..start + self.records_byte_len
    }
    fn versioned_scalars_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_scalars_byte_start?;
        Some(start..start + self.versioned_scalars_byte_len?)
    }
    fn versioned_records_byte_range(&self) -> Option<Range<usize>> {
        let start = self.versioned_records_byte_start?;
        Some(start..start + self.versioned_records_byte_len?)
    }
}

impl<'a> FontRead<'a> for KindsOfArrays<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let version: u16 = cursor.read()?;
        let count: u16 = cursor.read()?;
        let scalars_byte_len = (count as usize)
            .checked_mul(u16::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(scalars_byte_len);
        let records_byte_len = (count as usize)
            .checked_mul(Shmecord::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(records_byte_len);
        let versioned_scalars_byte_start = version
            .compatible(1u16)
            .then(|| cursor.position())
            .transpose()?;
        let versioned_scalars_byte_len = version.compatible(1u16).then_some(
            (count as usize)
                .checked_mul(u16::RAW_BYTE_LEN)
                .ok_or(ReadError::OutOfBounds)?,
        );
        if let Some(value) = versioned_scalars_byte_len {
            cursor.advance_by(value);
        }
        let versioned_records_byte_start = version
            .compatible(1u16)
            .then(|| cursor.position())
            .transpose()?;
        let versioned_records_byte_len = version.compatible(1u16).then_some(
            (count as usize)
                .checked_mul(Shmecord::RAW_BYTE_LEN)
                .ok_or(ReadError::OutOfBounds)?,
        );
        if let Some(value) = versioned_records_byte_len {
            cursor.advance_by(value);
        }
        cursor.finish(KindsOfArraysMarker {
            scalars_byte_len,
            records_byte_len,
            versioned_scalars_byte_start,
            versioned_scalars_byte_len,
            versioned_records_byte_start,
            versioned_records_byte_len,
        })
    }
}

pub type KindsOfArrays<'a> = TableRef<'a, KindsOfArraysMarker>;

impl<'a> KindsOfArrays<'a> {
    pub fn version(&self) -> u16 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// the number of items in each array
    pub fn count(&self) -> u16 {
        let range = self.shape.count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// an array of scalars
    pub fn scalars(&self) -> &'a [BigEndian<u16>] {
        let range = self.shape.scalars_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// an array of records
    pub fn records(&self) -> &'a [Shmecord] {
        let range = self.shape.records_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// a versioned array of scalars
    pub fn versioned_scalars(&self) -> Option<&'a [BigEndian<u16>]> {
        let range = self.shape.versioned_scalars_byte_range()?;
        Some(self.data.read_array(range).unwrap())
    }

    /// a versioned array of scalars
    pub fn versioned_records(&self) -> Option<&'a [Shmecord]> {
        let range = self.shape.versioned_records_byte_range()?;
        Some(self.data.read_array(range).unwrap())
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for KindsOfArrays<'a> {
    fn type_name(&self) -> &str {
        "KindsOfArrays"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        let version = self.version();
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("count", self.count())),
            2usize => Some(Field::new("scalars", self.scalars())),
            3usize => Some(Field::new(
                "records",
                traversal::FieldType::array_of_records(
                    stringify!(Shmecord),
                    self.records(),
                    self.offset_data(),
                ),
            )),
            4usize if version.compatible(1u16) => Some(Field::new(
                "versioned_scalars",
                self.versioned_scalars().unwrap(),
            )),
            5usize if version.compatible(1u16) => Some(Field::new(
                "versioned_records",
                traversal::FieldType::array_of_records(
                    stringify!(Shmecord),
                    self.versioned_records().unwrap(),
                    self.offset_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for KindsOfArrays<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct VarLenHaverMarker {
    var_len_byte_len: usize,
}

impl VarLenHaverMarker {
    fn count_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn var_len_byte_range(&self) -> Range<usize> {
        let start = self.count_byte_range().end;
        start..start + self.var_len_byte_len
    }
    fn other_field_byte_range(&self) -> Range<usize> {
        let start = self.var_len_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }
}

impl<'a> FontRead<'a> for VarLenHaver<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let count: u16 = cursor.read()?;
        let var_len_byte_len = {
            let data = cursor.remaining().ok_or(ReadError::OutOfBounds)?;
            <VarSizeDummy as VarSize>::total_len_for_count(data, count as usize)?
        };
        cursor.advance_by(var_len_byte_len);
        cursor.advance::<u32>();
        cursor.finish(VarLenHaverMarker { var_len_byte_len })
    }
}

pub type VarLenHaver<'a> = TableRef<'a, VarLenHaverMarker>;

impl<'a> VarLenHaver<'a> {
    pub fn count(&self) -> u16 {
        let range = self.shape.count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    pub fn var_len(&self) -> VarLenArray<'a, VarSizeDummy> {
        let range = self.shape.var_len_byte_range();
        VarLenArray::read(self.data.split_off(range.start).unwrap()).unwrap()
    }

    pub fn other_field(&self) -> u32 {
        let range = self.shape.other_field_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for VarLenHaver<'a> {
    fn type_name(&self) -> &str {
        "VarLenHaver"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("count", self.count())),
            1usize => Some(Field::new("var_len", traversal::FieldType::Unknown)),
            2usize => Some(Field::new("other_field", self.other_field())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for VarLenHaver<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct DummyMarker {}

impl DummyMarker {
    fn value_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn _reserved_byte_range(&self) -> Range<usize> {
        let start = self.value_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
}

impl<'a> FontRead<'a> for Dummy<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.finish(DummyMarker {})
    }
}

pub type Dummy<'a> = TableRef<'a, DummyMarker>;

impl<'a> Dummy<'a> {
    pub fn value(&self) -> u16 {
        let range = self.shape.value_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Dummy<'a> {
    fn type_name(&self) -> &str {
        "Dummy"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("value", self.value())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for Dummy<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct Shmecord {
    pub length: BigEndian<u16>,
    pub breadth: BigEndian<u32>,
}

impl Shmecord {
    pub fn length(&self) -> u16 {
        self.length.get()
    }

    pub fn breadth(&self) -> u32 {
        self.breadth.get()
    }
}

impl FixedSize for Shmecord {
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN + u32::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for Shmecord {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "Shmecord",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("length", self.length())),
                1usize => Some(Field::new("breadth", self.breadth())),
                _ => None,
            }),
            data,
        }
    }
}
