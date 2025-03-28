// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtkEditableText")]
    pub struct EditableText(Interface<ffi::AtkEditableText, ffi::AtkEditableTextIface>);

    match fn {
        type_ => || ffi::atk_editable_text_get_type(),
    }
}

impl EditableText {
    pub const NONE: Option<&'static EditableText> = None;
}

pub trait EditableTextExt: 'static {
    #[doc(alias = "atk_editable_text_copy_text")]
    fn copy_text(&self, start_pos: i32, end_pos: i32);

    #[doc(alias = "atk_editable_text_cut_text")]
    fn cut_text(&self, start_pos: i32, end_pos: i32);

    #[doc(alias = "atk_editable_text_delete_text")]
    fn delete_text(&self, start_pos: i32, end_pos: i32);

    #[doc(alias = "atk_editable_text_paste_text")]
    fn paste_text(&self, position: i32);

    //#[doc(alias = "atk_editable_text_set_run_attributes")]
    //fn set_run_attributes(&self, attrib_set: /*Ignored*/&mut AttributeSet, start_offset: i32, end_offset: i32) -> bool;

    #[doc(alias = "atk_editable_text_set_text_contents")]
    fn set_text_contents(&self, string: &str);
}

impl<O: IsA<EditableText>> EditableTextExt for O {
    fn copy_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::atk_editable_text_copy_text(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn cut_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::atk_editable_text_cut_text(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::atk_editable_text_delete_text(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn paste_text(&self, position: i32) {
        unsafe {
            ffi::atk_editable_text_paste_text(self.as_ref().to_glib_none().0, position);
        }
    }

    //fn set_run_attributes(&self, attrib_set: /*Ignored*/&mut AttributeSet, start_offset: i32, end_offset: i32) -> bool {
    //    unsafe { TODO: call ffi:atk_editable_text_set_run_attributes() }
    //}

    fn set_text_contents(&self, string: &str) {
        unsafe {
            ffi::atk_editable_text_set_text_contents(
                self.as_ref().to_glib_none().0,
                string.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for EditableText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EditableText")
    }
}
