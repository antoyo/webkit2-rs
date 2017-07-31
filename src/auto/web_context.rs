// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use CacheModel;
use CookieManager;
use Download;
use FaviconDatabase;
#[cfg(feature = "v2_4")]
use ProcessModel;
use SecurityManager;
use TLSErrorsPolicy;
#[cfg(feature = "v2_10")]
use WebsiteDataManager;
use ffi;
use glib;
#[cfg(feature = "v2_8")]
use glib::Value;
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
    pub struct WebContext(Object<ffi::WebKitWebContext>);

    match fn {
        get_type => || ffi::webkit_web_context_get_type(),
    }
}

impl WebContext {
    #[cfg(feature = "v2_8")]
    pub fn new() -> WebContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new())
        }
    }

    #[cfg(feature = "v2_16")]
    pub fn new_ephemeral() -> WebContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new_ephemeral())
        }
    }

    #[cfg(feature = "v2_10")]
    pub fn new_with_website_data_manager(manager: &WebsiteDataManager) -> WebContext {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_web_context_new_with_website_data_manager(manager.to_glib_none().0))
        }
    }

    pub fn get_default() -> Option<WebContext> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_default())
        }
    }
}

pub trait WebContextExt {
    //#[cfg(feature = "v2_6")]
    //fn allow_tls_certificate_for_host(&self, certificate: /*Ignored*/&gio::TlsCertificate, host: &str);

    fn clear_cache(&self);

    fn download_uri(&self, uri: &str) -> Option<Download>;

    fn get_cache_model(&self) -> CacheModel;

    fn get_cookie_manager(&self) -> Option<CookieManager>;

    fn get_favicon_database(&self) -> Option<FaviconDatabase>;

    fn get_favicon_database_directory(&self) -> Option<String>;

    //fn get_plugins<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    //fn get_plugins_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<Vec<Plugin>, Error>;

    #[cfg(feature = "v2_4")]
    fn get_process_model(&self) -> ProcessModel;

    fn get_security_manager(&self) -> Option<SecurityManager>;

    fn get_spell_checking_enabled(&self) -> bool;

    fn get_spell_checking_languages(&self) -> Vec<String>;

    fn get_tls_errors_policy(&self) -> TLSErrorsPolicy;

    #[cfg(feature = "v2_10")]
    fn get_web_process_count_limit(&self) -> u32;

    #[cfg(feature = "v2_10")]
    fn get_website_data_manager(&self) -> Option<WebsiteDataManager>;

    //#[cfg(feature = "v2_16")]
    //fn initialize_notification_permissions(&self, allowed_origins: /*Ignored*/&[&SecurityOrigin], disallowed_origins: /*Ignored*/&[&SecurityOrigin]);

    #[cfg(feature = "v2_16")]
    fn is_ephemeral(&self) -> bool;

    fn prefetch_dns(&self, hostname: &str);

    //fn register_uri_scheme<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, scheme: &str, callback: /*Unknown conversion*//*Unimplemented*/URISchemeRequestCallback, user_data: P, user_data_destroy_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_additional_plugins_directory(&self, directory: &str);

    fn set_cache_model(&self, cache_model: CacheModel);

    fn set_disk_cache_directory(&self, directory: &str);

    fn set_favicon_database_directory<'a, P: Into<Option<&'a str>>>(&self, path: P);

    //#[cfg(feature = "v2_16")]
    //fn set_network_proxy_settings<'a, P: Into<Option<&'a /*Ignored*/NetworkProxySettings>>>(&self, proxy_mode: /*Ignored*/NetworkProxyMode, proxy_settings: P);

    fn set_preferred_languages(&self, languages: &[&str]);

    #[cfg(feature = "v2_4")]
    fn set_process_model(&self, process_model: ProcessModel);

    fn set_spell_checking_enabled(&self, enabled: bool);

    fn set_spell_checking_languages(&self, languages: &[&str]);

    fn set_tls_errors_policy(&self, policy: TLSErrorsPolicy);

    fn set_web_extensions_directory(&self, directory: &str);

    #[cfg(feature = "v2_4")]
    fn set_web_extensions_initialization_user_data(&self, user_data: &glib::Variant);

    #[cfg(feature = "v2_10")]
    fn set_web_process_count_limit(&self, limit: u32);

    #[cfg(feature = "v2_8")]
    fn get_property_local_storage_directory(&self) -> Option<String>;

    fn connect_download_started<F: Fn(&Self, &Download) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v2_16")]
    fn connect_initialize_notification_permissions<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v2_4")]
    fn connect_initialize_web_extensions<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<WebContext> + IsA<glib::object::Object>> WebContextExt for O {
    //#[cfg(feature = "v2_6")]
    //fn allow_tls_certificate_for_host(&self, certificate: /*Ignored*/&gio::TlsCertificate, host: &str) {
    //    unsafe { TODO: call ffi::webkit_web_context_allow_tls_certificate_for_host() }
    //}

    fn clear_cache(&self) {
        unsafe {
            ffi::webkit_web_context_clear_cache(self.to_glib_none().0);
        }
    }

    fn download_uri(&self, uri: &str) -> Option<Download> {
        unsafe {
            from_glib_full(ffi::webkit_web_context_download_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn get_cache_model(&self) -> CacheModel {
        unsafe {
            from_glib(ffi::webkit_web_context_get_cache_model(self.to_glib_none().0))
        }
    }

    fn get_cookie_manager(&self) -> Option<CookieManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_cookie_manager(self.to_glib_none().0))
        }
    }

    fn get_favicon_database(&self) -> Option<FaviconDatabase> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_favicon_database(self.to_glib_none().0))
        }
    }

    fn get_favicon_database_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_favicon_database_directory(self.to_glib_none().0))
        }
    }

    //fn get_plugins<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_web_context_get_plugins() }
    //}

    //fn get_plugins_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<Vec<Plugin>, Error> {
    //    unsafe { TODO: call ffi::webkit_web_context_get_plugins_finish() }
    //}

    #[cfg(feature = "v2_4")]
    fn get_process_model(&self) -> ProcessModel {
        unsafe {
            from_glib(ffi::webkit_web_context_get_process_model(self.to_glib_none().0))
        }
    }

    fn get_security_manager(&self) -> Option<SecurityManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_security_manager(self.to_glib_none().0))
        }
    }

    fn get_spell_checking_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_get_spell_checking_enabled(self.to_glib_none().0))
        }
    }

    fn get_spell_checking_languages(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_web_context_get_spell_checking_languages(self.to_glib_none().0))
        }
    }

    fn get_tls_errors_policy(&self) -> TLSErrorsPolicy {
        unsafe {
            from_glib(ffi::webkit_web_context_get_tls_errors_policy(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_web_process_count_limit(&self) -> u32 {
        unsafe {
            ffi::webkit_web_context_get_web_process_count_limit(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_website_data_manager(&self) -> Option<WebsiteDataManager> {
        unsafe {
            from_glib_none(ffi::webkit_web_context_get_website_data_manager(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v2_16")]
    //fn initialize_notification_permissions(&self, allowed_origins: /*Ignored*/&[&SecurityOrigin], disallowed_origins: /*Ignored*/&[&SecurityOrigin]) {
    //    unsafe { TODO: call ffi::webkit_web_context_initialize_notification_permissions() }
    //}

    #[cfg(feature = "v2_16")]
    fn is_ephemeral(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_context_is_ephemeral(self.to_glib_none().0))
        }
    }

    fn prefetch_dns(&self, hostname: &str) {
        unsafe {
            ffi::webkit_web_context_prefetch_dns(self.to_glib_none().0, hostname.to_glib_none().0);
        }
    }

    //fn register_uri_scheme<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, scheme: &str, callback: /*Unknown conversion*//*Unimplemented*/URISchemeRequestCallback, user_data: P, user_data_destroy_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::webkit_web_context_register_uri_scheme() }
    //}

    fn set_additional_plugins_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_additional_plugins_directory(self.to_glib_none().0, directory.to_glib_none().0);
        }
    }

    fn set_cache_model(&self, cache_model: CacheModel) {
        unsafe {
            ffi::webkit_web_context_set_cache_model(self.to_glib_none().0, cache_model.to_glib());
        }
    }

    fn set_disk_cache_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_disk_cache_directory(self.to_glib_none().0, directory.to_glib_none().0);
        }
    }

    fn set_favicon_database_directory<'a, P: Into<Option<&'a str>>>(&self, path: P) {
        let path = path.into();
        let path = path.to_glib_none();
        unsafe {
            ffi::webkit_web_context_set_favicon_database_directory(self.to_glib_none().0, path.0);
        }
    }

    //#[cfg(feature = "v2_16")]
    //fn set_network_proxy_settings<'a, P: Into<Option<&'a /*Ignored*/NetworkProxySettings>>>(&self, proxy_mode: /*Ignored*/NetworkProxyMode, proxy_settings: P) {
    //    unsafe { TODO: call ffi::webkit_web_context_set_network_proxy_settings() }
    //}

    fn set_preferred_languages(&self, languages: &[&str]) {
        unsafe {
            ffi::webkit_web_context_set_preferred_languages(self.to_glib_none().0, languages.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_4")]
    fn set_process_model(&self, process_model: ProcessModel) {
        unsafe {
            ffi::webkit_web_context_set_process_model(self.to_glib_none().0, process_model.to_glib());
        }
    }

    fn set_spell_checking_enabled(&self, enabled: bool) {
        unsafe {
            ffi::webkit_web_context_set_spell_checking_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_spell_checking_languages(&self, languages: &[&str]) {
        unsafe {
            ffi::webkit_web_context_set_spell_checking_languages(self.to_glib_none().0, languages.to_glib_none().0);
        }
    }

    fn set_tls_errors_policy(&self, policy: TLSErrorsPolicy) {
        unsafe {
            ffi::webkit_web_context_set_tls_errors_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    fn set_web_extensions_directory(&self, directory: &str) {
        unsafe {
            ffi::webkit_web_context_set_web_extensions_directory(self.to_glib_none().0, directory.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_4")]
    fn set_web_extensions_initialization_user_data(&self, user_data: &glib::Variant) {
        unsafe {
            ffi::webkit_web_context_set_web_extensions_initialization_user_data(self.to_glib_none().0, user_data.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_10")]
    fn set_web_process_count_limit(&self, limit: u32) {
        unsafe {
            ffi::webkit_web_context_set_web_process_count_limit(self.to_glib_none().0, limit);
        }
    }

    #[cfg(feature = "v2_8")]
    fn get_property_local_storage_directory(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "local-storage-directory".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_download_started<F: Fn(&Self, &Download) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Download) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "download-started",
                transmute(download_started_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v2_16")]
    fn connect_initialize_notification_permissions<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "initialize-notification-permissions",
                transmute(initialize_notification_permissions_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v2_4")]
    fn connect_initialize_web_extensions<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "initialize-web-extensions",
                transmute(initialize_web_extensions_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn download_started_trampoline<P>(this: *mut ffi::WebKitWebContext, download: *mut ffi::WebKitDownload, f: glib_ffi::gpointer)
where P: IsA<WebContext> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Download) + 'static> = transmute(f);
    f(&WebContext::from_glib_none(this).downcast_unchecked(), &from_glib_none(download))
}

#[cfg(feature = "v2_16")]
unsafe extern "C" fn initialize_notification_permissions_trampoline<P>(this: *mut ffi::WebKitWebContext, f: glib_ffi::gpointer)
where P: IsA<WebContext> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&WebContext::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v2_4")]
unsafe extern "C" fn initialize_web_extensions_trampoline<P>(this: *mut ffi::WebKitWebContext, f: glib_ffi::gpointer)
where P: IsA<WebContext> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&WebContext::from_glib_none(this).downcast_unchecked())
}
