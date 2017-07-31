// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct EditorState(Object<ffi::WebKitEditorState>);

    match fn {
        get_type => || ffi::webkit_editor_state_get_type(),
    }
}

pub trait EditorStateExt {
    #[cfg(feature = "v2_10")]
    fn get_typing_attributes(&self) -> u32;
}

impl<O: IsA<EditorState>> EditorStateExt for O {
    #[cfg(feature = "v2_10")]
    fn get_typing_attributes(&self) -> u32 {
        unsafe {
            ffi::webkit_editor_state_get_typing_attributes(self.to_glib_none().0)
        }
    }
}
