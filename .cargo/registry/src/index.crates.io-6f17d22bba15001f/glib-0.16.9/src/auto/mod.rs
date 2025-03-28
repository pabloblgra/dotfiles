// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod checksum;
pub use self::checksum::Checksum;

mod date_time;
pub use self::date_time::DateTime;

mod key_file;
pub use self::key_file::KeyFile;

mod main_context;
pub use self::main_context::MainContext;

mod main_loop;
pub use self::main_loop::MainLoop;

mod markup_parse_context;
pub use self::markup_parse_context::MarkupParseContext;

mod source;
pub use self::source::Source;

mod time_zone;
pub use self::time_zone::TimeZone;

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
mod uri;
#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
pub use self::uri::Uri;

mod enums;
pub use self::enums::ChecksumType;
pub use self::enums::ConvertError;
pub use self::enums::DateMonth;
pub use self::enums::DateWeekday;
pub use self::enums::FileError;
pub use self::enums::KeyFileError;
pub use self::enums::LogWriterOutput;
pub use self::enums::MarkupError;
pub use self::enums::OptionArg;
pub use self::enums::SeekType;
pub use self::enums::TimeType;
pub use self::enums::UnicodeScript;
#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
pub use self::enums::UriError;
pub use self::enums::VariantClass;

mod flags;
#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
pub use self::flags::FileSetContentsFlags;
pub(crate) use self::flags::FileTest;
pub use self::flags::FormatSizeFlags;
pub use self::flags::IOCondition;
pub use self::flags::KeyFileFlags;
pub use self::flags::LogLevelFlags;
#[cfg(any(feature = "v2_72", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
pub use self::flags::MainContextFlags;
pub use self::flags::OptionFlags;
pub use self::flags::SpawnFlags;
#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
pub use self::flags::UriFlags;
#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
pub use self::flags::UriHideFlags;
#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
pub use self::flags::UriParamsFlags;

mod alias;
pub use self::alias::DateDay;
pub use self::alias::DateYear;
pub use self::alias::Time;

pub mod functions;

mod constants;
pub use self::constants::CSET_a_2_z;
pub use self::constants::CSET_A_2_Z;
pub use self::constants::CSET_DIGITS;
pub use self::constants::KEY_FILE_DESKTOP_GROUP;
pub use self::constants::KEY_FILE_DESKTOP_KEY_ACTIONS;
pub use self::constants::KEY_FILE_DESKTOP_KEY_CATEGORIES;
pub use self::constants::KEY_FILE_DESKTOP_KEY_COMMENT;
pub use self::constants::KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE;
pub use self::constants::KEY_FILE_DESKTOP_KEY_EXEC;
pub use self::constants::KEY_FILE_DESKTOP_KEY_GENERIC_NAME;
pub use self::constants::KEY_FILE_DESKTOP_KEY_HIDDEN;
pub use self::constants::KEY_FILE_DESKTOP_KEY_ICON;
pub use self::constants::KEY_FILE_DESKTOP_KEY_MIME_TYPE;
pub use self::constants::KEY_FILE_DESKTOP_KEY_NAME;
pub use self::constants::KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN;
pub use self::constants::KEY_FILE_DESKTOP_KEY_NO_DISPLAY;
pub use self::constants::KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN;
pub use self::constants::KEY_FILE_DESKTOP_KEY_PATH;
pub use self::constants::KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY;
pub use self::constants::KEY_FILE_DESKTOP_KEY_STARTUP_WM_CLASS;
pub use self::constants::KEY_FILE_DESKTOP_KEY_TERMINAL;
pub use self::constants::KEY_FILE_DESKTOP_KEY_TRY_EXEC;
pub use self::constants::KEY_FILE_DESKTOP_KEY_TYPE;
pub use self::constants::KEY_FILE_DESKTOP_KEY_URL;
pub use self::constants::KEY_FILE_DESKTOP_KEY_VERSION;
pub use self::constants::KEY_FILE_DESKTOP_TYPE_APPLICATION;
pub use self::constants::KEY_FILE_DESKTOP_TYPE_DIRECTORY;
pub use self::constants::KEY_FILE_DESKTOP_TYPE_LINK;
pub use self::constants::OPTION_REMAINING;
pub use self::constants::STR_DELIMITERS;
#[cfg(any(feature = "v2_60", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
pub use self::constants::TEST_OPTION_ISOLATE_DIRS;
pub use self::constants::URI_RESERVED_CHARS_GENERIC_DELIMITERS;
pub use self::constants::URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS;
