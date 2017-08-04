// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

#[cfg(feature = "v2_2")]
use AuthenticationScheme;
use ffi;
#[cfg(feature = "v2_2")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v2_2")]
use glib_ffi;
#[cfg(feature = "v2_2")]
use std::boxed::Box as Box_;
#[cfg(feature = "v2_2")]
use std::mem::transmute;

glib_wrapper! {
    pub struct AuthenticationRequest(Object<ffi::WebKitAuthenticationRequest>);

    match fn {
        get_type => || ffi::webkit_authentication_request_get_type(),
    }
}

impl AuthenticationRequest {
    //#[cfg(feature = "v2_2")]
    //pub fn authenticate<'a, P: Into<Option<&'a /*Ignored*/Credential>>>(&self, credential: P) {
    //    unsafe { TODO: call ffi::webkit_authentication_request_authenticate() }
    //}

    #[cfg(feature = "v2_2")]
    pub fn can_save_credentials(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_can_save_credentials(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn cancel(&self) {
        unsafe {
            ffi::webkit_authentication_request_cancel(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn get_host(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_host(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn get_port(&self) -> u32 {
        unsafe {
            ffi::webkit_authentication_request_get_port(self.to_glib_none().0)
        }
    }

    //#[cfg(feature = "v2_2")]
    //pub fn get_proposed_credential(&self) -> /*Ignored*/Option<Credential> {
    //    unsafe { TODO: call ffi::webkit_authentication_request_get_proposed_credential() }
    //}

    #[cfg(feature = "v2_2")]
    pub fn get_realm(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_realm(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn get_scheme(&self) -> AuthenticationScheme {
        unsafe {
            from_glib(ffi::webkit_authentication_request_get_scheme(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn is_for_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_for_proxy(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn is_retry(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_retry(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn connect_cancelled<F: Fn(&AuthenticationRequest) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&AuthenticationRequest) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancelled",
                transmute(cancelled_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_2")]
unsafe extern "C" fn cancelled_trampoline(this: *mut ffi::WebKitAuthenticationRequest, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&AuthenticationRequest) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
