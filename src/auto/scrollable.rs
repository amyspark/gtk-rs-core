// This file was generated by gir (5c017c9) from gir-files (71d73f0)
// DO NOT EDIT

use Adjustment;
use ScrollablePolicy;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Scrollable(Object<ffi::GtkScrollable>);

    match fn {
        get_type => || ffi::gtk_scrollable_get_type(),
    }
}

pub trait ScrollableExt {
    //#[cfg(feature = "v3_16")]
    //fn get_border(&self, border: /*Ignored*/Border) -> bool;

    fn get_hadjustment(&self) -> Option<Adjustment>;

    fn get_hscroll_policy(&self) -> ScrollablePolicy;

    fn get_vadjustment(&self) -> Option<Adjustment>;

    fn get_vscroll_policy(&self) -> ScrollablePolicy;

    fn set_hadjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, hadjustment: P);

    fn set_hscroll_policy(&self, policy: ScrollablePolicy);

    fn set_vadjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, vadjustment: P);

    fn set_vscroll_policy(&self, policy: ScrollablePolicy);
}

impl<O: IsA<Scrollable>> ScrollableExt for O {
    //#[cfg(feature = "v3_16")]
    //fn get_border(&self, border: /*Ignored*/Border) -> bool {
    //    unsafe { TODO: call ffi::gtk_scrollable_get_border() }
    //}

    fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_hadjustment(self.to_glib_none().0))
        }
    }

    fn get_hscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(ffi::gtk_scrollable_get_hscroll_policy(self.to_glib_none().0))
        }
    }

    fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_vadjustment(self.to_glib_none().0))
        }
    }

    fn get_vscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(ffi::gtk_scrollable_get_vscroll_policy(self.to_glib_none().0))
        }
    }

    fn set_hadjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, hadjustment: P) {
        let hadjustment = hadjustment.into();
        let hadjustment = hadjustment.to_glib_none();
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(self.to_glib_none().0, hadjustment.0);
        }
    }

    fn set_hscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    fn set_vadjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, vadjustment: P) {
        let vadjustment = vadjustment.into();
        let vadjustment = vadjustment.to_glib_none();
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(self.to_glib_none().0, vadjustment.0);
        }
    }

    fn set_vscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_vscroll_policy(self.to_glib_none().0, policy.to_glib());
        }
    }
}
