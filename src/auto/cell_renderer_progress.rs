// This file was generated by gir (94e079d) from gir-files (469db10)
// DO NOT EDIT

use CellRenderer;
use Orientable;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CellRendererProgress(Object<ffi::GtkCellRendererProgress>): CellRenderer, Orientable;

    match fn {
        get_type => || ffi::gtk_cell_renderer_progress_get_type(),
    }
}

impl CellRendererProgress {
    pub fn new() -> CellRendererProgress {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_progress_new()).downcast_unchecked()
        }
    }
}

impl Default for CellRendererProgress {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CellRendererProgressExt {
    fn get_property_inverted(&self) -> bool;

    fn set_property_inverted(&self, inverted: bool);

    fn get_property_pulse(&self) -> i32;

    fn set_property_pulse(&self, pulse: i32);

    fn get_property_text(&self) -> Option<String>;

    fn set_property_text(&self, text: Option<&str>);

    fn get_property_text_xalign(&self) -> f32;

    fn set_property_text_xalign(&self, text_xalign: f32);

    fn get_property_text_yalign(&self) -> f32;

    fn set_property_text_yalign(&self, text_yalign: f32);

    fn get_property_value(&self) -> i32;

    fn set_property_value(&self, value: i32);

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pulse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererProgress> + IsA<glib::object::Object>> CellRendererProgressExt for O {
    fn get_property_inverted(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "inverted".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_inverted(&self, inverted: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "inverted".to_glib_none().0, Value::from(&inverted).to_glib_none().0);
        }
    }

    fn get_property_pulse(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pulse".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_pulse(&self, pulse: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pulse".to_glib_none().0, Value::from(&pulse).to_glib_none().0);
        }
    }

    fn get_property_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    fn get_property_text_xalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text-xalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_text_xalign(&self, text_xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text-xalign".to_glib_none().0, Value::from(&text_xalign).to_glib_none().0);
        }
    }

    fn get_property_text_yalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text-yalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_text_yalign(&self, text_yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text-yalign".to_glib_none().0, Value::from(&text_yalign).to_glib_none().0);
        }
    }

    fn get_property_value(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "value".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_value(&self, value: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "value".to_glib_none().0, Value::from(&value).to_glib_none().0);
        }
    }

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inverted",
                transmute(notify_inverted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pulse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pulse",
                transmute(notify_pulse_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text-xalign",
                transmute(notify_text_xalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text-yalign",
                transmute(notify_text_yalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_inverted_trampoline<P>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererProgress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererProgress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pulse_trampoline<P>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererProgress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererProgress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererProgress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererProgress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_xalign_trampoline<P>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererProgress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererProgress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_yalign_trampoline<P>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererProgress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererProgress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererProgress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererProgress::from_glib_borrow(this).downcast_unchecked())
}
