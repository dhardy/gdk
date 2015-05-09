// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkDisplay — Controls a set of GdkScreens and their associated input devices

use ffi;
use glib::translate::*;
use glib::types::{StaticType, Type};
use atom::Atom;
use app_launch_context::AppLaunchContext;
use device::Device;
use device_manager::DeviceManager;
use object::Object;
use screen::Screen;
use window::Window;

pub type Display = Object<ffi::C_GdkDisplay>;

impl StaticType for Display {
    fn static_type() -> Type { unsafe { from_glib(ffi::gdk_display_get_type()) } }
}

impl Display {
    pub fn open(display_name: &str) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_display_open(display_name.to_glib_none().0)) }
    }

    pub fn get_default() -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_display_get_default()) }
    }

    pub fn get_name(&self) -> String {
        unsafe { from_glib_none(ffi::gdk_display_get_name(self.to_glib_none().0)) }
    }

    pub fn get_screen(&self, screen_num: i32) -> Screen {
        unsafe { from_glib_none(ffi::gdk_display_get_screen(self.to_glib_none().0, screen_num)) }
    }

    pub fn get_default_screen(&self) -> Screen {
        unsafe {
            from_glib_none(ffi::gdk_display_get_default_screen(self.to_glib_none().0))
        }
    }

    pub fn get_device_manager(&self) -> Option<DeviceManager> {
        unsafe { from_glib_none(ffi::gdk_display_get_device_manager(self.to_glib_none().0)) }
    }

    pub fn device_is_grabbed(&self, device: &Device) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_device_is_grabbed(self.to_glib_none().0,
                      device.to_glib_none().0))
        }
    }

    pub fn beep(&self) {
        unsafe { ffi::gdk_display_beep(self.to_glib_none().0) }
    }

    pub fn sync(&self) {
        unsafe { ffi::gdk_display_sync(self.to_glib_none().0) }
    }

    pub fn flush(&self) {
        unsafe { ffi::gdk_display_flush(self.to_glib_none().0) }
    }

    pub fn close(&self) {
        unsafe { ffi::gdk_display_close(self.to_glib_none().0) }
    }

    pub fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_is_closed(self.to_glib_none().0)) }
    }

    /*pub fn get_event(&self) -> Option<::Event> {
        unsafe { ffi::gdk_display_get_event(self.to_glib_none().0) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Event::wrap(tmp)) }
        }
    }

    pub fn peek_event(&self) -> Option<::Event> {
        unsafe { ffi::gdk_display_peek_event(self.to_glib_none().0) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Event::wrap(tmp)) }
        }
    }

    pub fn put_event(&self, event: &::Event) {
        unsafe { ffi::gdk_display_put_event(self.to_glib_none().0, event.to_glib_none().0 as *const ffi::C_GdkEvent) }
    }*/

    pub fn has_pending(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_has_pending(self.to_glib_none().0)) }
    }

    pub fn set_double_click_time(&self, msec: u32) {
        unsafe { ffi::gdk_display_set_double_click_time(self.to_glib_none().0, msec) }
    }

    pub fn set_double_click_distance(&self, msec: u32) {
        unsafe { ffi::gdk_display_set_double_click_distance(self.to_glib_none().0, msec) }
    }

    pub fn supports_cursor_color(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_cursor_color(self.to_glib_none().0)) }
    }

    pub fn supports_cursor_alpha(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_cursor_alpha(self.to_glib_none().0)) }
    }

    pub fn get_default_cursor_size(&self) -> u32 {
        unsafe { ffi::gdk_display_get_default_cursor_size(self.to_glib_none().0) }
    }

    pub fn get_maximal_cursor_size(&self, width: &mut u32, height: &mut u32) {
        unsafe { ffi::gdk_display_get_maximal_cursor_size(self.to_glib_none().0, width, height) }
    }

    pub fn get_default_group(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_display_get_default_group(self.to_glib_none().0)) }
    }

    pub fn supports_selection_notification(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_selection_notification(self.to_glib_none().0)) }
    }

    pub fn request_selection_notification(&self, selection: &Atom) -> bool {
        unsafe {
            from_glib(
                ffi::gdk_display_request_selection_notification(self.to_glib_none().0,
                    selection.to_glib_none().0))
        }
    }

    pub fn supports_clipboard_persistence(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_clipboard_persistence(self.to_glib_none().0)) }
    }

    /*pub fn store_clipboard(&self, clipboard_window: &::Window, time_: u32, targets: Vec<Atom>) {
        unsafe { ffi::gdk_display_store_clipboard(self.to_glib_none().0, clipboard_window.to_glib_none().0, time_, targets.as_mut_pointer(),
            targets.len() as c_int) }
    }*/

    pub fn supports_shapes(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_shapes(self.to_glib_none().0)) }
    }

    pub fn supports_input_shapes(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_input_shapes(self.to_glib_none().0)) }
    }

    pub fn supports_composite(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_composite(self.to_glib_none().0)) }
    }

    pub fn get_app_launch_context(&self) -> AppLaunchContext {
        unsafe { from_glib_full(ffi::gdk_display_get_app_launch_context(self.to_glib_none().0)) }
    }

    pub fn notify_startup_complete(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_display_notify_startup_complete(self.to_glib_none().0,
                                                     startup_id.to_glib_none().0)
        }
    }
}