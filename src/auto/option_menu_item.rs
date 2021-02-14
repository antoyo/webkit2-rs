// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OptionMenuItem(Boxed<ffi::WebKitOptionMenuItem>);

    match fn {
        copy => |ptr| ffi::webkit_option_menu_item_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_option_menu_item_free(ptr),
        get_type => || ffi::webkit_option_menu_item_get_type(),
    }
}

impl OptionMenuItem {
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_item_get_label")]
    pub fn get_label(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_option_menu_item_get_label(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_item_get_tooltip")]
    pub fn get_tooltip(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_option_menu_item_get_tooltip(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_item_is_enabled")]
    pub fn is_enabled(&mut self) -> bool {
        unsafe {
            from_glib(ffi::webkit_option_menu_item_is_enabled(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_item_is_group_child")]
    pub fn is_group_child(&mut self) -> bool {
        unsafe {
            from_glib(ffi::webkit_option_menu_item_is_group_child(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_item_is_group_label")]
    pub fn is_group_label(&mut self) -> bool {
        unsafe {
            from_glib(ffi::webkit_option_menu_item_is_group_label(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_item_is_selected")]
    pub fn is_selected(&mut self) -> bool {
        unsafe {
            from_glib(ffi::webkit_option_menu_item_is_selected(
                self.to_glib_none_mut().0,
            ))
        }
    }
}