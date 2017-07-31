// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use Error;
use URIRequest;
use URIResponse;
use WebView;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Download(Object<ffi::WebKitDownload>);

    match fn {
        get_type => || ffi::webkit_download_get_type(),
    }
}

pub trait DownloadExt {
    fn cancel(&self);

    #[cfg(feature = "v2_6")]
    fn get_allow_overwrite(&self) -> bool;

    fn get_destination(&self) -> Option<String>;

    fn get_elapsed_time(&self) -> f64;

    fn get_estimated_progress(&self) -> f64;

    fn get_received_data_length(&self) -> u64;

    fn get_request(&self) -> Option<URIRequest>;

    fn get_response(&self) -> Option<URIResponse>;

    fn get_web_view(&self) -> Option<WebView>;

    #[cfg(feature = "v2_6")]
    fn set_allow_overwrite(&self, allowed: bool);

    fn set_destination(&self, uri: &str);

    fn connect_created_destination<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64;

    fn connect_decide_destination<F: Fn(&Self, &str) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_failed<F: Fn(&Self, &Error) + 'static>(&self, f: F) -> u64;

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Download> + IsA<glib::object::Object>> DownloadExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::webkit_download_cancel(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_6")]
    fn get_allow_overwrite(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_download_get_allow_overwrite(self.to_glib_none().0))
        }
    }

    fn get_destination(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_download_get_destination(self.to_glib_none().0))
        }
    }

    fn get_elapsed_time(&self) -> f64 {
        unsafe {
            ffi::webkit_download_get_elapsed_time(self.to_glib_none().0)
        }
    }

    fn get_estimated_progress(&self) -> f64 {
        unsafe {
            ffi::webkit_download_get_estimated_progress(self.to_glib_none().0)
        }
    }

    fn get_received_data_length(&self) -> u64 {
        unsafe {
            ffi::webkit_download_get_received_data_length(self.to_glib_none().0)
        }
    }

    fn get_request(&self) -> Option<URIRequest> {
        unsafe {
            from_glib_none(ffi::webkit_download_get_request(self.to_glib_none().0))
        }
    }

    fn get_response(&self) -> Option<URIResponse> {
        unsafe {
            from_glib_none(ffi::webkit_download_get_response(self.to_glib_none().0))
        }
    }

    fn get_web_view(&self) -> Option<WebView> {
        unsafe {
            from_glib_none(ffi::webkit_download_get_web_view(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_6")]
    fn set_allow_overwrite(&self, allowed: bool) {
        unsafe {
            ffi::webkit_download_set_allow_overwrite(self.to_glib_none().0, allowed.to_glib());
        }
    }

    fn set_destination(&self, uri: &str) {
        unsafe {
            ffi::webkit_download_set_destination(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn connect_created_destination<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "created-destination",
                transmute(created_destination_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_decide_destination<F: Fn(&Self, &str) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "decide-destination",
                transmute(decide_destination_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_failed<F: Fn(&Self, &Error) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Error) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "failed",
                transmute(failed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "finished",
                transmute(finished_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "received-data",
                transmute(received_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn created_destination_trampoline<P>(this: *mut ffi::WebKitDownload, destination: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Download> {
    callback_guard!();
    let f: &Box_<Fn(&P, &str) + 'static> = transmute(f);
    f(&Download::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(destination))
}

unsafe extern "C" fn decide_destination_trampoline<P>(this: *mut ffi::WebKitDownload, suggested_filename: *mut libc::c_char, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Download> {
    callback_guard!();
    let f: &Box_<Fn(&P, &str) -> bool + 'static> = transmute(f);
    f(&Download::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(suggested_filename)).to_glib()
}

unsafe extern "C" fn failed_trampoline<P>(this: *mut ffi::WebKitDownload, error: *mut glib_ffi::GError, f: glib_ffi::gpointer)
where P: IsA<Download> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Error) + 'static> = transmute(f);
    f(&Download::from_glib_none(this).downcast_unchecked(), &from_glib_none(error))
}

unsafe extern "C" fn finished_trampoline<P>(this: *mut ffi::WebKitDownload, f: glib_ffi::gpointer)
where P: IsA<Download> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Download::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn received_data_trampoline<P>(this: *mut ffi::WebKitDownload, data_length: u64, f: glib_ffi::gpointer)
where P: IsA<Download> {
    callback_guard!();
    let f: &Box_<Fn(&P, u64) + 'static> = transmute(f);
    f(&Download::from_glib_none(this).downcast_unchecked(), data_length)
}
