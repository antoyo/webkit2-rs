// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use CookieAcceptPolicy;
use CookiePersistentStorage;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CookieManager(Object<ffi::WebKitCookieManager>);

    match fn {
        get_type => || ffi::webkit_cookie_manager_get_type(),
    }
}

pub trait CookieManagerExt {
    fn delete_all_cookies(&self);

    fn delete_cookies_for_domain(&self, domain: &str);

    //fn get_accept_policy<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    //fn get_accept_policy_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<CookieAcceptPolicy, Error>;

    //fn get_domains_with_cookies<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    //fn get_domains_with_cookies_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<Vec<String>, Error>;

    fn set_accept_policy(&self, policy: CookieAcceptPolicy);

    fn set_persistent_storage(&self, filename: &str, storage: CookiePersistentStorage);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<CookieManager> + IsA<glib::object::Object>> CookieManagerExt for O {
    fn delete_all_cookies(&self) {
        unsafe {
            ffi::webkit_cookie_manager_delete_all_cookies(self.to_glib_none().0);
        }
    }

    fn delete_cookies_for_domain(&self, domain: &str) {
        unsafe {
            ffi::webkit_cookie_manager_delete_cookies_for_domain(self.to_glib_none().0, domain.to_glib_none().0);
        }
    }

    //fn get_accept_policy<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_accept_policy() }
    //}

    //fn get_accept_policy_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<CookieAcceptPolicy, Error> {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_accept_policy_finish() }
    //}

    //fn get_domains_with_cookies<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_domains_with_cookies() }
    //}

    //fn get_domains_with_cookies_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<Vec<String>, Error> {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_domains_with_cookies_finish() }
    //}

    fn set_accept_policy(&self, policy: CookieAcceptPolicy) {
        unsafe {
            ffi::webkit_cookie_manager_set_accept_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    fn set_persistent_storage(&self, filename: &str, storage: CookiePersistentStorage) {
        unsafe {
            ffi::webkit_cookie_manager_set_persistent_storage(self.to_glib_none().0, filename.to_glib_none().0, storage.to_glib());
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::WebKitCookieManager, f: glib_ffi::gpointer)
where P: IsA<CookieManager> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&CookieManager::from_glib_none(this).downcast_unchecked())
}
