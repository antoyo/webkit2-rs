// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

#[cfg(feature = "v2_6")]
use NavigationAction;
use NavigationType;
use PolicyDecision;
use URIRequest;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct NavigationPolicyDecision(Object<ffi::WebKitNavigationPolicyDecision>): PolicyDecision;

    match fn {
        get_type => || ffi::webkit_navigation_policy_decision_get_type(),
    }
}

impl NavigationPolicyDecision {
    pub fn get_frame_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_policy_decision_get_frame_name(self.to_glib_none().0))
        }
    }

    pub fn get_modifiers(&self) -> u32 {
        unsafe {
            ffi::webkit_navigation_policy_decision_get_modifiers(self.to_glib_none().0)
        }
    }

    pub fn get_mouse_button(&self) -> u32 {
        unsafe {
            ffi::webkit_navigation_policy_decision_get_mouse_button(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_6")]
    pub fn get_navigation_action(&self) -> Option<NavigationAction> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_policy_decision_get_navigation_action(self.to_glib_none().0))
        }
    }

    pub fn get_navigation_type(&self) -> NavigationType {
        unsafe {
            from_glib(ffi::webkit_navigation_policy_decision_get_navigation_type(self.to_glib_none().0))
        }
    }

    pub fn get_request(&self) -> Option<URIRequest> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_policy_decision_get_request(self.to_glib_none().0))
        }
    }
}
