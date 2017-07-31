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
    pub struct WindowProperties(Object<ffi::WebKitWindowProperties>);

    match fn {
        get_type => || ffi::webkit_window_properties_get_type(),
    }
}

pub trait WindowPropertiesExt {
    fn get_fullscreen(&self) -> bool;

    //fn get_geometry(&self, geometry: /*Ignored*/gdk::Rectangle);

    fn get_locationbar_visible(&self) -> bool;

    fn get_menubar_visible(&self) -> bool;

    fn get_resizable(&self) -> bool;

    fn get_scrollbars_visible(&self) -> bool;

    fn get_statusbar_visible(&self) -> bool;

    fn get_toolbar_visible(&self) -> bool;
}

impl<O: IsA<WindowProperties>> WindowPropertiesExt for O {
    fn get_fullscreen(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_fullscreen(self.to_glib_none().0))
        }
    }

    //fn get_geometry(&self, geometry: /*Ignored*/gdk::Rectangle) {
    //    unsafe { TODO: call ffi::webkit_window_properties_get_geometry() }
    //}

    fn get_locationbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_locationbar_visible(self.to_glib_none().0))
        }
    }

    fn get_menubar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_menubar_visible(self.to_glib_none().0))
        }
    }

    fn get_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_resizable(self.to_glib_none().0))
        }
    }

    fn get_scrollbars_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_scrollbars_visible(self.to_glib_none().0))
        }
    }

    fn get_statusbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_statusbar_visible(self.to_glib_none().0))
        }
    }

    fn get_toolbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_toolbar_visible(self.to_glib_none().0))
        }
    }
}
