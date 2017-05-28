// This file was generated by gir (5c017c9) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use ReliefStyle;
use ToolItem;
use ToolShell;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use pango;

glib_wrapper! {
    pub struct ToolItemGroup(Object<ffi::GtkToolItemGroup>): Container, Widget, ToolShell;

    match fn {
        get_type => || ffi::gtk_tool_item_group_get_type(),
    }
}

impl ToolItemGroup {
    pub fn new(label: &str) -> ToolItemGroup {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_tool_item_group_new(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ToolItemGroupExt {
    fn get_collapsed(&self) -> bool;

    fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem>;

    fn get_ellipsize(&self) -> pango::EllipsizeMode;

    fn get_header_relief(&self) -> ReliefStyle;

    fn get_item_position<P: IsA<ToolItem>>(&self, item: &P) -> i32;

    fn get_label(&self) -> Option<String>;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_n_items(&self) -> u32;

    fn get_nth_item(&self, index: u32) -> Option<ToolItem>;

    fn insert<P: IsA<ToolItem>>(&self, item: &P, position: i32);

    fn set_collapsed(&self, collapsed: bool);

    fn set_ellipsize(&self, ellipsize: pango::EllipsizeMode);

    fn set_header_relief(&self, style: ReliefStyle);

    fn set_item_position<P: IsA<ToolItem>>(&self, item: &P, position: i32);

    fn set_label(&self, label: &str);

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: &P);

    fn get_item_expand<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_expand<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, expand: bool);

    fn get_item_fill<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_fill<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, fill: bool);

    fn get_item_homogeneous<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_homogeneous<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, homogeneous: bool);

    fn get_item_new_row<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_new_row<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, new_row: bool);
}

impl<O: IsA<ToolItemGroup> + IsA<Container>> ToolItemGroupExt for O {
    fn get_collapsed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_group_get_collapsed(self.to_glib_none().0))
        }
    }

    fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_drop_item(self.to_glib_none().0, x, y))
        }
    }

    fn get_ellipsize(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_tool_item_group_get_ellipsize(self.to_glib_none().0))
        }
    }

    fn get_header_relief(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_group_get_header_relief(self.to_glib_none().0))
        }
    }

    fn get_item_position<P: IsA<ToolItem>>(&self, item: &P) -> i32 {
        unsafe {
            ffi::gtk_tool_item_group_get_item_position(self.to_glib_none().0, item.to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_label(self.to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_label_widget(self.to_glib_none().0))
        }
    }

    fn get_n_items(&self) -> u32 {
        unsafe {
            ffi::gtk_tool_item_group_get_n_items(self.to_glib_none().0)
        }
    }

    fn get_nth_item(&self, index: u32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_nth_item(self.to_glib_none().0, index))
        }
    }

    fn insert<P: IsA<ToolItem>>(&self, item: &P, position: i32) {
        unsafe {
            ffi::gtk_tool_item_group_insert(self.to_glib_none().0, item.to_glib_none().0, position);
        }
    }

    fn set_collapsed(&self, collapsed: bool) {
        unsafe {
            ffi::gtk_tool_item_group_set_collapsed(self.to_glib_none().0, collapsed.to_glib());
        }
    }

    fn set_ellipsize(&self, ellipsize: pango::EllipsizeMode) {
        unsafe {
            ffi::gtk_tool_item_group_set_ellipsize(self.to_glib_none().0, ellipsize.to_glib());
        }
    }

    fn set_header_relief(&self, style: ReliefStyle) {
        unsafe {
            ffi::gtk_tool_item_group_set_header_relief(self.to_glib_none().0, style.to_glib());
        }
    }

    fn set_item_position<P: IsA<ToolItem>>(&self, item: &P, position: i32) {
        unsafe {
            ffi::gtk_tool_item_group_set_item_position(self.to_glib_none().0, item.to_glib_none().0, position);
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_tool_item_group_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: &P) {
        unsafe {
            ffi::gtk_tool_item_group_set_label_widget(self.to_glib_none().0, label_widget.to_glib_none().0);
        }
    }

    fn get_item_expand<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_item_expand<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, expand: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, Value::from(&expand).to_glib_none().0);
        }
    }

    fn get_item_fill<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "fill".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_item_fill<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, fill: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "fill".to_glib_none().0, Value::from(&fill).to_glib_none().0);
        }
    }

    fn get_item_homogeneous<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "homogeneous".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_item_homogeneous<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, homogeneous: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "homogeneous".to_glib_none().0, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn get_item_new_row<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "new-row".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_item_new_row<T: IsA<ToolItem> + IsA<Widget>>(&self, item: &T, new_row: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "new-row".to_glib_none().0, Value::from(&new_row).to_glib_none().0);
        }
    }
}
