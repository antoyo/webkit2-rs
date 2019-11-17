// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CookieAcceptPolicy;
use CookiePersistentStorage;
use gio;
use gio_sys;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;
use webkit2_sys;

glib_wrapper! {
    pub struct CookieManager(Object<webkit2_sys::WebKitCookieManager, webkit2_sys::WebKitCookieManagerClass, CookieManagerClass>);

    match fn {
        get_type => || webkit2_sys::webkit_cookie_manager_get_type(),
    }
}

pub const NONE_COOKIE_MANAGER: Option<&CookieManager> = None;

pub trait CookieManagerExt: 'static {
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn add_cookie<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, cookie: /*Ignored*/&mut soup::Cookie, cancellable: Option<&P>, callback: Q);

    //
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn add_cookie_future(&self, cookie: /*Ignored*/&mut soup::Cookie) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[cfg_attr(feature = "v2_16", deprecated)]
    fn delete_all_cookies(&self);

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn delete_cookie<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, cookie: /*Ignored*/&mut soup::Cookie, cancellable: Option<&P>, callback: Q);

    //
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn delete_cookie_future(&self, cookie: /*Ignored*/&mut soup::Cookie) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[cfg_attr(feature = "v2_16", deprecated)]
    fn delete_cookies_for_domain(&self, domain: &str);

    fn get_accept_policy<P: IsA<gio::Cancellable>, Q: FnOnce(Result<CookieAcceptPolicy, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    
    fn get_accept_policy_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<CookieAcceptPolicy, glib::Error>> + 'static>>;

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn get_cookies<P: IsA<gio::Cancellable>, Q: FnOnce(Result</*Ignored*/Vec<soup::Cookie>, glib::Error>) + Send + 'static>(&self, uri: &str, cancellable: Option<&P>, callback: Q);

    //
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn get_cookies_future(&self, uri: &str) -> Pin<Box_<dyn std::future::Future<Output = Result</*Ignored*/Vec<soup::Cookie>, glib::Error>> + 'static>>;

    #[cfg_attr(feature = "v2_16", deprecated)]
    fn get_domains_with_cookies<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<GString>, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    #[cfg_attr(feature = "v2_16", deprecated)]
    
    fn get_domains_with_cookies_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<GString>, glib::Error>> + 'static>>;

    fn set_accept_policy(&self, policy: CookieAcceptPolicy);

    fn set_persistent_storage(&self, filename: &str, storage: CookiePersistentStorage);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CookieManager>> CookieManagerExt for O {
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn add_cookie<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, cookie: /*Ignored*/&mut soup::Cookie, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call webkit2_sys:webkit_cookie_manager_add_cookie() }
    //}

    //
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn add_cookie_future(&self, cookie: /*Ignored*/&mut soup::Cookie) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let cookie = cookie.clone();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.add_cookie(
        //        &cookie,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    fn delete_all_cookies(&self) {
        unsafe {
            webkit2_sys::webkit_cookie_manager_delete_all_cookies(self.as_ref().to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn delete_cookie<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, cookie: /*Ignored*/&mut soup::Cookie, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call webkit2_sys:webkit_cookie_manager_delete_cookie() }
    //}

    //
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn delete_cookie_future(&self, cookie: /*Ignored*/&mut soup::Cookie) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let cookie = cookie.clone();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.delete_cookie(
        //        &cookie,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    fn delete_cookies_for_domain(&self, domain: &str) {
        unsafe {
            webkit2_sys::webkit_cookie_manager_delete_cookies_for_domain(self.as_ref().to_glib_none().0, domain.to_glib_none().0);
        }
    }

    fn get_accept_policy<P: IsA<gio::Cancellable>, Q: FnOnce(Result<CookieAcceptPolicy, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn get_accept_policy_trampoline<Q: FnOnce(Result<CookieAcceptPolicy, glib::Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let ret = webkit2_sys::webkit_cookie_manager_get_accept_policy_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_accept_policy_trampoline::<Q>;
        unsafe {
            webkit2_sys::webkit_cookie_manager_get_accept_policy(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn get_accept_policy_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<CookieAcceptPolicy, glib::Error>> + 'static>> {
        use gio::GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.get_accept_policy(
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn get_cookies<P: IsA<gio::Cancellable>, Q: FnOnce(Result</*Ignored*/Vec<soup::Cookie>, glib::Error>) + Send + 'static>(&self, uri: &str, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call webkit2_sys:webkit_cookie_manager_get_cookies() }
    //}

    //
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn get_cookies_future(&self, uri: &str) -> Pin<Box_<dyn std::future::Future<Output = Result</*Ignored*/Vec<soup::Cookie>, glib::Error>> + 'static>> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let uri = String::from(uri);
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.get_cookies(
        //        &uri,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    fn get_domains_with_cookies<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<GString>, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn get_domains_with_cookies_trampoline<Q: FnOnce(Result<Vec<GString>, glib::Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let ret = webkit2_sys::webkit_cookie_manager_get_domains_with_cookies_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_domains_with_cookies_trampoline::<Q>;
        unsafe {
            webkit2_sys::webkit_cookie_manager_get_domains_with_cookies(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn get_domains_with_cookies_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<GString>, glib::Error>> + 'static>> {
        use gio::GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.get_domains_with_cookies(
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn set_accept_policy(&self, policy: CookieAcceptPolicy) {
        unsafe {
            webkit2_sys::webkit_cookie_manager_set_accept_policy(self.as_ref().to_glib_none().0, policy.to_glib());
        }
    }

    fn set_persistent_storage(&self, filename: &str, storage: CookiePersistentStorage) {
        unsafe {
            webkit2_sys::webkit_cookie_manager_set_persistent_storage(self.as_ref().to_glib_none().0, filename.to_glib_none().0, storage.to_glib());
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitCookieManager, f: glib_sys::gpointer)
            where P: IsA<CookieManager>
        {
            let f: &F = &*(f as *const F);
            f(&CookieManager::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for CookieManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CookieManager")
    }
}
