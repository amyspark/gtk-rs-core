// This file was generated by gir (5c017c9) from gir-files (71d73f0)
// DO NOT EDIT

use AccelGroup;
use Application;
use Bin;
use Container;
use Error;
use Widget;
use WindowGroup;
use WindowPosition;
use WindowType;
use ffi;
use gdk;
use gdk_pixbuf;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Window(Object<ffi::GtkWindow>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_window_get_type(),
    }
}

impl Window {
    pub fn new(type_: WindowType) -> Window {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_window_new(type_.to_glib())).downcast_unchecked()
        }
    }

    pub fn get_default_icon_list() -> Vec<gdk_pixbuf::Pixbuf> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_get_default_icon_list())
        }
    }

    pub fn get_default_icon_name() -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_window_get_default_icon_name())
        }
    }

    pub fn list_toplevels() -> Vec<Widget> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_list_toplevels())
        }
    }

    pub fn set_auto_startup_notification(setting: bool) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_auto_startup_notification(setting.to_glib());
        }
    }

    pub fn set_default_icon(icon: &gdk_pixbuf::Pixbuf) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_default_icon(icon.to_glib_none().0);
        }
    }

    pub fn set_default_icon_from_file<P: AsRef<std::path::Path>>(filename: P) -> Result<(), Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_window_set_default_icon_from_file(filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_default_icon_list(list: &[gdk_pixbuf::Pixbuf]) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_default_icon_list(list.to_glib_container().0);
        }
    }

    pub fn set_default_icon_name(name: &str) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_default_icon_name(name.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_interactive_debugging(enable: bool) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_interactive_debugging(enable.to_glib());
        }
    }
}

pub trait WindowExt {
    fn activate_default(&self) -> bool;

    fn activate_focus(&self) -> bool;

    fn activate_key(&self, event: &gdk::EventKey) -> bool;

    fn add_accel_group(&self, accel_group: &AccelGroup);

    fn add_mnemonic<P: IsA<Widget>>(&self, keyval: u32, target: &P);

    fn begin_move_drag(&self, button: i32, root_x: i32, root_y: i32, timestamp: u32);

    fn begin_resize_drag(&self, edge: gdk::WindowEdge, button: i32, root_x: i32, root_y: i32, timestamp: u32);

    #[cfg(feature = "v3_10")]
    fn close(&self);

    fn deiconify(&self);

    fn fullscreen(&self);

    #[cfg(feature = "v3_18")]
    fn fullscreen_on_monitor(&self, screen: &gdk::Screen, monitor: i32);

    fn get_accept_focus(&self) -> bool;

    fn get_application(&self) -> Option<Application>;

    fn get_attached_to(&self) -> Option<Widget>;

    fn get_decorated(&self) -> bool;

    fn get_default_size(&self) -> (i32, i32);

    fn get_default_widget(&self) -> Option<Widget>;

    fn get_deletable(&self) -> bool;

    fn get_destroy_with_parent(&self) -> bool;

    fn get_focus(&self) -> Option<Widget>;

    fn get_focus_on_map(&self) -> bool;

    fn get_focus_visible(&self) -> bool;

    fn get_gravity(&self) -> gdk::Gravity;

    fn get_group(&self) -> Option<WindowGroup>;

    fn get_has_resize_grip(&self) -> bool;

    fn get_hide_titlebar_when_maximized(&self) -> bool;

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_icon_list(&self) -> Vec<gdk_pixbuf::Pixbuf>;

    fn get_icon_name(&self) -> Option<String>;

    fn get_mnemonic_modifier(&self) -> gdk::ModifierType;

    fn get_mnemonics_visible(&self) -> bool;

    fn get_modal(&self) -> bool;

    #[cfg(not(feature = "v3_8"))]
    fn get_opacity(&self) -> f64;

    fn get_position(&self) -> (i32, i32);

    fn get_resizable(&self) -> bool;

    fn get_resize_grip_area(&self) -> Option<gdk::Rectangle>;

    fn get_role(&self) -> Option<String>;

    fn get_size(&self) -> (i32, i32);

    fn get_skip_pager_hint(&self) -> bool;

    fn get_skip_taskbar_hint(&self) -> bool;

    fn get_title(&self) -> Option<String>;

    #[cfg(feature = "v3_16")]
    fn get_titlebar(&self) -> Option<Widget>;

    fn get_transient_for(&self) -> Option<Window>;

    fn get_type_hint(&self) -> gdk::WindowTypeHint;

    fn get_urgency_hint(&self) -> bool;

    fn get_window_type(&self) -> WindowType;

    fn has_group(&self) -> bool;

    fn has_toplevel_focus(&self) -> bool;

    fn iconify(&self);

    fn is_active(&self) -> bool;

    #[cfg(feature = "v3_12")]
    fn is_maximized(&self) -> bool;

    fn maximize(&self);

    fn mnemonic_activate(&self, keyval: u32, modifier: gdk::ModifierType) -> bool;

    fn move_(&self, x: i32, y: i32);

    fn parse_geometry(&self, geometry: &str) -> bool;

    fn present(&self);

    fn present_with_time(&self, timestamp: u32);

    fn propagate_key_event(&self, event: &gdk::EventKey) -> bool;

    fn remove_accel_group(&self, accel_group: &AccelGroup);

    fn remove_mnemonic<P: IsA<Widget>>(&self, keyval: u32, target: &P);

    fn reshow_with_initial_size(&self);

    fn resize(&self, width: i32, height: i32);

    fn resize_grip_is_visible(&self) -> bool;

    fn resize_to_geometry(&self, width: i32, height: i32);

    fn set_accept_focus(&self, setting: bool);

    fn set_application<'a, P: Into<Option<&'a Application>>>(&self, application: P);

    fn set_attached_to<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, attach_widget: Q);

    fn set_decorated(&self, setting: bool);

    fn set_default<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, default_widget: Q);

    fn set_default_geometry(&self, width: i32, height: i32);

    fn set_default_size(&self, width: i32, height: i32);

    fn set_deletable(&self, setting: bool);

    fn set_destroy_with_parent(&self, setting: bool);

    fn set_focus<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, focus: Q);

    fn set_focus_on_map(&self, setting: bool);

    fn set_focus_visible(&self, setting: bool);

    //fn set_geometry_hints<'a, 'b, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b /*Ignored*/gdk::Geometry>>>(&self, geometry_widget: Q, geometry: R, geom_mask: gdk::WindowHints);

    fn set_gravity(&self, gravity: gdk::Gravity);

    fn set_has_resize_grip(&self, value: bool);

    fn set_has_user_ref_count(&self, setting: bool);

    fn set_hide_titlebar_when_maximized(&self, setting: bool);

    fn set_icon<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, icon: P);

    fn set_icon_from_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), Error>;

    fn set_icon_list(&self, list: &[gdk_pixbuf::Pixbuf]);

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, name: P);

    fn set_keep_above(&self, setting: bool);

    fn set_keep_below(&self, setting: bool);

    fn set_mnemonic_modifier(&self, modifier: gdk::ModifierType);

    fn set_mnemonics_visible(&self, setting: bool);

    fn set_modal(&self, modal: bool);

    #[cfg(not(feature = "v3_8"))]
    fn set_opacity(&self, opacity: f64);

    fn set_position(&self, position: WindowPosition);

    fn set_resizable(&self, resizable: bool);

    fn set_role(&self, role: &str);

    fn set_screen(&self, screen: &gdk::Screen);

    fn set_skip_pager_hint(&self, setting: bool);

    fn set_skip_taskbar_hint(&self, setting: bool);

    fn set_startup_id(&self, startup_id: &str);

    fn set_title(&self, title: &str);

    #[cfg(feature = "v3_10")]
    fn set_titlebar<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, titlebar: Q);

    fn set_transient_for<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q);

    fn set_type_hint(&self, hint: gdk::WindowTypeHint);

    fn set_urgency_hint(&self, setting: bool);

    fn set_wmclass(&self, wmclass_name: &str, wmclass_class: &str);

    fn stick(&self);

    fn unfullscreen(&self);

    fn unmaximize(&self);

    fn unstick(&self);

    fn get_property_default_height(&self) -> i32;

    fn set_property_default_height(&self, default_height: i32);

    fn get_property_default_width(&self) -> i32;

    fn set_property_default_width(&self, default_width: i32);

    fn get_property_has_toplevel_focus(&self) -> bool;

    fn get_property_is_active(&self) -> bool;

    fn get_property_is_maximized(&self) -> bool;

    fn get_property_resize_grip_visible(&self) -> bool;

    fn get_property_type(&self) -> WindowType;

    fn get_property_window_position(&self) -> WindowPosition;

    fn set_property_window_position(&self, window_position: WindowPosition);

    fn connect_activate_default<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_activate_focus<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_enable_debugging<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_keys_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_set_focus<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Window> + IsA<glib::object::Object>> WindowExt for O {
    fn activate_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_activate_default(self.to_glib_none().0))
        }
    }

    fn activate_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_activate_focus(self.to_glib_none().0))
        }
    }

    fn activate_key(&self, event: &gdk::EventKey) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_activate_key(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    fn add_accel_group(&self, accel_group: &AccelGroup) {
        unsafe {
            ffi::gtk_window_add_accel_group(self.to_glib_none().0, accel_group.to_glib_none().0);
        }
    }

    fn add_mnemonic<P: IsA<Widget>>(&self, keyval: u32, target: &P) {
        unsafe {
            ffi::gtk_window_add_mnemonic(self.to_glib_none().0, keyval, target.to_glib_none().0);
        }
    }

    fn begin_move_drag(&self, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe {
            ffi::gtk_window_begin_move_drag(self.to_glib_none().0, button, root_x, root_y, timestamp);
        }
    }

    fn begin_resize_drag(&self, edge: gdk::WindowEdge, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe {
            ffi::gtk_window_begin_resize_drag(self.to_glib_none().0, edge.to_glib(), button, root_x, root_y, timestamp);
        }
    }

    #[cfg(feature = "v3_10")]
    fn close(&self) {
        unsafe {
            ffi::gtk_window_close(self.to_glib_none().0);
        }
    }

    fn deiconify(&self) {
        unsafe {
            ffi::gtk_window_deiconify(self.to_glib_none().0);
        }
    }

    fn fullscreen(&self) {
        unsafe {
            ffi::gtk_window_fullscreen(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_18")]
    fn fullscreen_on_monitor(&self, screen: &gdk::Screen, monitor: i32) {
        unsafe {
            ffi::gtk_window_fullscreen_on_monitor(self.to_glib_none().0, screen.to_glib_none().0, monitor);
        }
    }

    fn get_accept_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_accept_focus(self.to_glib_none().0))
        }
    }

    fn get_application(&self) -> Option<Application> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_application(self.to_glib_none().0))
        }
    }

    fn get_attached_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_attached_to(self.to_glib_none().0))
        }
    }

    fn get_decorated(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_decorated(self.to_glib_none().0))
        }
    }

    fn get_default_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_window_get_default_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_default_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_default_widget(self.to_glib_none().0))
        }
    }

    fn get_deletable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_deletable(self.to_glib_none().0))
        }
    }

    fn get_destroy_with_parent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_destroy_with_parent(self.to_glib_none().0))
        }
    }

    fn get_focus(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_focus(self.to_glib_none().0))
        }
    }

    fn get_focus_on_map(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_focus_on_map(self.to_glib_none().0))
        }
    }

    fn get_focus_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_focus_visible(self.to_glib_none().0))
        }
    }

    fn get_gravity(&self) -> gdk::Gravity {
        unsafe {
            from_glib(ffi::gtk_window_get_gravity(self.to_glib_none().0))
        }
    }

    fn get_group(&self) -> Option<WindowGroup> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_group(self.to_glib_none().0))
        }
    }

    fn get_has_resize_grip(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_has_resize_grip(self.to_glib_none().0))
        }
    }

    fn get_hide_titlebar_when_maximized(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_hide_titlebar_when_maximized(self.to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_icon(self.to_glib_none().0))
        }
    }

    fn get_icon_list(&self) -> Vec<gdk_pixbuf::Pixbuf> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_get_icon_list(self.to_glib_none().0))
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_mnemonic_modifier(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(ffi::gtk_window_get_mnemonic_modifier(self.to_glib_none().0))
        }
    }

    fn get_mnemonics_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_mnemonics_visible(self.to_glib_none().0))
        }
    }

    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_modal(self.to_glib_none().0))
        }
    }

    #[cfg(not(feature = "v3_8"))]
    fn get_opacity(&self) -> f64 {
        unsafe {
            ffi::gtk_window_get_opacity(self.to_glib_none().0)
        }
    }

    fn get_position(&self) -> (i32, i32) {
        unsafe {
            let mut root_x = mem::uninitialized();
            let mut root_y = mem::uninitialized();
            ffi::gtk_window_get_position(self.to_glib_none().0, &mut root_x, &mut root_y);
            (root_x, root_y)
        }
    }

    fn get_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_resizable(self.to_glib_none().0))
        }
    }

    fn get_resize_grip_area(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_window_get_resize_grip_area(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    fn get_role(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_role(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_window_get_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_skip_pager_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_skip_pager_hint(self.to_glib_none().0))
        }
    }

    fn get_skip_taskbar_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_skip_taskbar_hint(self.to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_title(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_titlebar(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_titlebar(self.to_glib_none().0))
        }
    }

    fn get_transient_for(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_transient_for(self.to_glib_none().0))
        }
    }

    fn get_type_hint(&self) -> gdk::WindowTypeHint {
        unsafe {
            from_glib(ffi::gtk_window_get_type_hint(self.to_glib_none().0))
        }
    }

    fn get_urgency_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_urgency_hint(self.to_glib_none().0))
        }
    }

    fn get_window_type(&self) -> WindowType {
        unsafe {
            from_glib(ffi::gtk_window_get_window_type(self.to_glib_none().0))
        }
    }

    fn has_group(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_has_group(self.to_glib_none().0))
        }
    }

    fn has_toplevel_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_has_toplevel_focus(self.to_glib_none().0))
        }
    }

    fn iconify(&self) {
        unsafe {
            ffi::gtk_window_iconify(self.to_glib_none().0);
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_is_active(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn is_maximized(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_is_maximized(self.to_glib_none().0))
        }
    }

    fn maximize(&self) {
        unsafe {
            ffi::gtk_window_maximize(self.to_glib_none().0);
        }
    }

    fn mnemonic_activate(&self, keyval: u32, modifier: gdk::ModifierType) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_mnemonic_activate(self.to_glib_none().0, keyval, modifier.to_glib()))
        }
    }

    fn move_(&self, x: i32, y: i32) {
        unsafe {
            ffi::gtk_window_move(self.to_glib_none().0, x, y);
        }
    }

    fn parse_geometry(&self, geometry: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_parse_geometry(self.to_glib_none().0, geometry.to_glib_none().0))
        }
    }

    fn present(&self) {
        unsafe {
            ffi::gtk_window_present(self.to_glib_none().0);
        }
    }

    fn present_with_time(&self, timestamp: u32) {
        unsafe {
            ffi::gtk_window_present_with_time(self.to_glib_none().0, timestamp);
        }
    }

    fn propagate_key_event(&self, event: &gdk::EventKey) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_propagate_key_event(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    fn remove_accel_group(&self, accel_group: &AccelGroup) {
        unsafe {
            ffi::gtk_window_remove_accel_group(self.to_glib_none().0, accel_group.to_glib_none().0);
        }
    }

    fn remove_mnemonic<P: IsA<Widget>>(&self, keyval: u32, target: &P) {
        unsafe {
            ffi::gtk_window_remove_mnemonic(self.to_glib_none().0, keyval, target.to_glib_none().0);
        }
    }

    fn reshow_with_initial_size(&self) {
        unsafe {
            ffi::gtk_window_reshow_with_initial_size(self.to_glib_none().0);
        }
    }

    fn resize(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_resize(self.to_glib_none().0, width, height);
        }
    }

    fn resize_grip_is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_resize_grip_is_visible(self.to_glib_none().0))
        }
    }

    fn resize_to_geometry(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_resize_to_geometry(self.to_glib_none().0, width, height);
        }
    }

    fn set_accept_focus(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_accept_focus(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_application<'a, P: Into<Option<&'a Application>>>(&self, application: P) {
        let application = application.into();
        let application = application.to_glib_none();
        unsafe {
            ffi::gtk_window_set_application(self.to_glib_none().0, application.0);
        }
    }

    fn set_attached_to<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, attach_widget: Q) {
        let attach_widget = attach_widget.into();
        let attach_widget = attach_widget.to_glib_none();
        unsafe {
            ffi::gtk_window_set_attached_to(self.to_glib_none().0, attach_widget.0);
        }
    }

    fn set_decorated(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_decorated(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_default<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, default_widget: Q) {
        let default_widget = default_widget.into();
        let default_widget = default_widget.to_glib_none();
        unsafe {
            ffi::gtk_window_set_default(self.to_glib_none().0, default_widget.0);
        }
    }

    fn set_default_geometry(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_geometry(self.to_glib_none().0, width, height);
        }
    }

    fn set_default_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_size(self.to_glib_none().0, width, height);
        }
    }

    fn set_deletable(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_deletable(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_destroy_with_parent(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_destroy_with_parent(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_focus<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, focus: Q) {
        let focus = focus.into();
        let focus = focus.to_glib_none();
        unsafe {
            ffi::gtk_window_set_focus(self.to_glib_none().0, focus.0);
        }
    }

    fn set_focus_on_map(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_focus_on_map(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_focus_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_focus_visible(self.to_glib_none().0, setting.to_glib());
        }
    }

    //fn set_geometry_hints<'a, 'b, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b /*Ignored*/gdk::Geometry>>>(&self, geometry_widget: Q, geometry: R, geom_mask: gdk::WindowHints) {
    //    unsafe { TODO: call ffi::gtk_window_set_geometry_hints() }
    //}

    fn set_gravity(&self, gravity: gdk::Gravity) {
        unsafe {
            ffi::gtk_window_set_gravity(self.to_glib_none().0, gravity.to_glib());
        }
    }

    fn set_has_resize_grip(&self, value: bool) {
        unsafe {
            ffi::gtk_window_set_has_resize_grip(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_has_user_ref_count(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_has_user_ref_count(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_hide_titlebar_when_maximized(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_hide_titlebar_when_maximized(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_icon<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, icon: P) {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        unsafe {
            ffi::gtk_window_set_icon(self.to_glib_none().0, icon.0);
        }
    }

    fn set_icon_from_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_window_set_icon_from_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_icon_list(&self, list: &[gdk_pixbuf::Pixbuf]) {
        unsafe {
            ffi::gtk_window_set_icon_list(self.to_glib_none().0, list.to_glib_none().0);
        }
    }

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, name: P) {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            ffi::gtk_window_set_icon_name(self.to_glib_none().0, name.0);
        }
    }

    fn set_keep_above(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_keep_above(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_keep_below(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_keep_below(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_mnemonic_modifier(&self, modifier: gdk::ModifierType) {
        unsafe {
            ffi::gtk_window_set_mnemonic_modifier(self.to_glib_none().0, modifier.to_glib());
        }
    }

    fn set_mnemonics_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_mnemonics_visible(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_window_set_modal(self.to_glib_none().0, modal.to_glib());
        }
    }

    #[cfg(not(feature = "v3_8"))]
    fn set_opacity(&self, opacity: f64) {
        unsafe {
            ffi::gtk_window_set_opacity(self.to_glib_none().0, opacity);
        }
    }

    fn set_position(&self, position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(self.to_glib_none().0, position.to_glib());
        }
    }

    fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_window_set_resizable(self.to_glib_none().0, resizable.to_glib());
        }
    }

    fn set_role(&self, role: &str) {
        unsafe {
            ffi::gtk_window_set_role(self.to_glib_none().0, role.to_glib_none().0);
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_window_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn set_skip_pager_hint(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_skip_pager_hint(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_skip_taskbar_hint(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_skip_taskbar_hint(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            ffi::gtk_window_set_startup_id(self.to_glib_none().0, startup_id.to_glib_none().0);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_window_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_titlebar<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, titlebar: Q) {
        let titlebar = titlebar.into();
        let titlebar = titlebar.to_glib_none();
        unsafe {
            ffi::gtk_window_set_titlebar(self.to_glib_none().0, titlebar.0);
        }
    }

    fn set_transient_for<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) {
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            ffi::gtk_window_set_transient_for(self.to_glib_none().0, parent.0);
        }
    }

    fn set_type_hint(&self, hint: gdk::WindowTypeHint) {
        unsafe {
            ffi::gtk_window_set_type_hint(self.to_glib_none().0, hint.to_glib());
        }
    }

    fn set_urgency_hint(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_urgency_hint(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_wmclass(&self, wmclass_name: &str, wmclass_class: &str) {
        unsafe {
            ffi::gtk_window_set_wmclass(self.to_glib_none().0, wmclass_name.to_glib_none().0, wmclass_class.to_glib_none().0);
        }
    }

    fn stick(&self) {
        unsafe {
            ffi::gtk_window_stick(self.to_glib_none().0);
        }
    }

    fn unfullscreen(&self) {
        unsafe {
            ffi::gtk_window_unfullscreen(self.to_glib_none().0);
        }
    }

    fn unmaximize(&self) {
        unsafe {
            ffi::gtk_window_unmaximize(self.to_glib_none().0);
        }
    }

    fn unstick(&self) {
        unsafe {
            ffi::gtk_window_unstick(self.to_glib_none().0);
        }
    }

    fn get_property_default_height(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "default-height".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_default_height(&self, default_height: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "default-height".to_glib_none().0, Value::from(&default_height).to_glib_none().0);
        }
    }

    fn get_property_default_width(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "default-width".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_default_width(&self, default_width: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "default-width".to_glib_none().0, Value::from(&default_width).to_glib_none().0);
        }
    }

    fn get_property_has_toplevel_focus(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "has-toplevel-focus".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_is_active(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-active".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_is_maximized(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-maximized".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_resize_grip_visible(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "resize-grip-visible".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_type(&self) -> WindowType {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn get_property_window_position(&self) -> WindowPosition {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "window-position".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_window_position(&self, window_position: WindowPosition) {
        let window_position = window_position.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "window-position".to_glib_none().0, Value::from(&window_position).to_glib_none().0);
        }
    }

    fn connect_activate_default<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-default",
                transmute(activate_default_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_activate_focus<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-focus",
                transmute(activate_focus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_enable_debugging<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "enable-debugging",
                transmute(enable_debugging_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_keys_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "keys-changed",
                transmute(keys_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_set_focus<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "set-focus",
                transmute(set_focus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_default_trampoline<P>(this: *mut ffi::GtkWindow, f: glib_ffi::gpointer)
where P: IsA<Window> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Window::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn activate_focus_trampoline<P>(this: *mut ffi::GtkWindow, f: glib_ffi::gpointer)
where P: IsA<Window> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Window::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn enable_debugging_trampoline<P>(this: *mut ffi::GtkWindow, toggle: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Window> {
    callback_guard!();
    let f: &Box_<Fn(&P, bool) -> bool + 'static> = transmute(f);
    f(&Window::from_glib_none(this).downcast_unchecked(), from_glib(toggle)).to_glib()
}

unsafe extern "C" fn keys_changed_trampoline<P>(this: *mut ffi::GtkWindow, f: glib_ffi::gpointer)
where P: IsA<Window> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Window::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn set_focus_trampoline<P>(this: *mut ffi::GtkWindow, object: *mut ffi::GtkWidget, f: glib_ffi::gpointer)
where P: IsA<Window> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Widget) + 'static> = transmute(f);
    f(&Window::from_glib_none(this).downcast_unchecked(), &from_glib_none(object))
}
