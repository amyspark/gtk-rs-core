// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AsyncResult, Cancellable};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GAsyncInitable")]
    pub struct AsyncInitable(Interface<ffi::GAsyncInitable, ffi::GAsyncInitableIface>);

    match fn {
        type_ => || ffi::g_async_initable_get_type(),
    }
}

impl AsyncInitable {
    pub const NONE: Option<&'static AsyncInitable> = None;
}

pub trait AsyncInitableExt: IsA<AsyncInitable> + 'static {
    #[doc(alias = "g_async_initable_init_async")]
    unsafe fn init_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn init_async_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_async_initable_init_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = init_async_trampoline::<P>;
        ffi::g_async_initable_init_async(
            self.as_ref().to_glib_none().0,
            io_priority.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box_::into_raw(user_data) as *mut _,
        );
    }

    unsafe fn init_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.init_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }
}

impl<O: IsA<AsyncInitable>> AsyncInitableExt for O {}

impl fmt::Display for AsyncInitable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AsyncInitable")
    }
}
