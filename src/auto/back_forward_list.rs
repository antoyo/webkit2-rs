// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::BackForwardListItem;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct BackForwardList(Object<ffi::WebKitBackForwardList, ffi::WebKitBackForwardListClass>);

    match fn {
        get_type => || ffi::webkit_back_forward_list_get_type(),
    }
}

pub const NONE_BACK_FORWARD_LIST: Option<&BackForwardList> = None;

pub trait BackForwardListExt: 'static {
    #[doc(alias = "webkit_back_forward_list_get_back_item")]
    fn get_back_item(&self) -> Option<BackForwardListItem>;

    #[doc(alias = "webkit_back_forward_list_get_back_list")]
    fn get_back_list(&self) -> Vec<BackForwardListItem>;

    #[doc(alias = "webkit_back_forward_list_get_back_list_with_limit")]
    fn get_back_list_with_limit(&self, limit: u32) -> Vec<BackForwardListItem>;

    #[doc(alias = "webkit_back_forward_list_get_current_item")]
    fn get_current_item(&self) -> Option<BackForwardListItem>;

    #[doc(alias = "webkit_back_forward_list_get_forward_item")]
    fn get_forward_item(&self) -> Option<BackForwardListItem>;

    #[doc(alias = "webkit_back_forward_list_get_forward_list")]
    fn get_forward_list(&self) -> Vec<BackForwardListItem>;

    #[doc(alias = "webkit_back_forward_list_get_forward_list_with_limit")]
    fn get_forward_list_with_limit(&self, limit: u32) -> Vec<BackForwardListItem>;

    #[doc(alias = "webkit_back_forward_list_get_length")]
    fn get_length(&self) -> u32;

    #[doc(alias = "webkit_back_forward_list_get_nth_item")]
    fn get_nth_item(&self, index: i32) -> Option<BackForwardListItem>;

    //fn connect_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BackForwardList>> BackForwardListExt for O {
    fn get_back_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_back_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_back_list(&self) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::webkit_back_forward_list_get_back_list(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_back_list_with_limit(&self, limit: u32) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::webkit_back_forward_list_get_back_list_with_limit(
                    self.as_ref().to_glib_none().0,
                    limit,
                ),
            )
        }
    }

    fn get_current_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_current_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_forward_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_forward_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_forward_list(&self) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::webkit_back_forward_list_get_forward_list(self.as_ref().to_glib_none().0),
            )
        }
    }

    fn get_forward_list_with_limit(&self, limit: u32) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::webkit_back_forward_list_get_forward_list_with_limit(
                    self.as_ref().to_glib_none().0,
                    limit,
                ),
            )
        }
    }

    fn get_length(&self) -> u32 {
        unsafe { ffi::webkit_back_forward_list_get_length(self.as_ref().to_glib_none().0) }
    }

    fn get_nth_item(&self, index: i32) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_nth_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    //fn connect_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented items_removed: *.Pointer
    //}
}

impl fmt::Display for BackForwardList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BackForwardList")
    }
}
