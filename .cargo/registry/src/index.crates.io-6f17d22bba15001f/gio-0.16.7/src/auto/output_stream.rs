// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::InputStream;
use crate::OutputStreamSpliceFlags;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GOutputStream")]
    pub struct OutputStream(Object<ffi::GOutputStream, ffi::GOutputStreamClass>);

    match fn {
        type_ => || ffi::g_output_stream_get_type(),
    }
}

impl OutputStream {
    pub const NONE: Option<&'static OutputStream> = None;
}

pub trait OutputStreamExt: 'static {
    #[doc(alias = "g_output_stream_clear_pending")]
    fn clear_pending(&self);

    #[doc(alias = "g_output_stream_close")]
    fn close(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error>;

    #[doc(alias = "g_output_stream_close_async")]
    fn close_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn close_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_output_stream_flush")]
    fn flush(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error>;

    #[doc(alias = "g_output_stream_flush_async")]
    fn flush_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn flush_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_output_stream_has_pending")]
    fn has_pending(&self) -> bool;

    #[doc(alias = "g_output_stream_is_closed")]
    fn is_closed(&self) -> bool;

    #[doc(alias = "g_output_stream_is_closing")]
    fn is_closing(&self) -> bool;

    //#[doc(alias = "g_output_stream_printf")]
    //fn printf(&self, cancellable: Option<&impl IsA<Cancellable>>, error: &mut glib::Error, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<usize>;

    #[doc(alias = "g_output_stream_set_pending")]
    fn set_pending(&self) -> Result<(), glib::Error>;

    #[doc(alias = "g_output_stream_splice")]
    fn splice(
        &self,
        source: &impl IsA<InputStream>,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error>;

    #[doc(alias = "g_output_stream_splice_async")]
    fn splice_async<P: FnOnce(Result<isize, glib::Error>) + 'static>(
        &self,
        source: &impl IsA<InputStream>,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn splice_future(
        &self,
        source: &(impl IsA<InputStream> + Clone + 'static),
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>>;

    //#[doc(alias = "g_output_stream_vprintf")]
    //fn vprintf(&self, cancellable: Option<&impl IsA<Cancellable>>, error: &mut glib::Error, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<usize>;

    #[doc(alias = "g_output_stream_write")]
    fn write(
        &self,
        buffer: &[u8],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error>;

    #[doc(alias = "g_output_stream_write_bytes")]
    fn write_bytes(
        &self,
        bytes: &glib::Bytes,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error>;

    #[doc(alias = "g_output_stream_write_bytes_async")]
    fn write_bytes_async<P: FnOnce(Result<isize, glib::Error>) + 'static>(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn write_bytes_future(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_output_stream_writev")]
    //fn writev(&self, vectors: /*Ignored*/&[OutputVector], cancellable: Option<&impl IsA<Cancellable>>) -> Result<usize, glib::Error>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_output_stream_writev_all")]
    //fn writev_all(&self, vectors: /*Ignored*/&[OutputVector], cancellable: Option<&impl IsA<Cancellable>>) -> Result<usize, glib::Error>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_output_stream_writev_all_async")]
    //fn writev_all_async<P: FnOnce(Result<usize, glib::Error>) + 'static>(&self, vectors: /*Ignored*/&[OutputVector], io_priority: glib::Priority, cancellable: Option<&impl IsA<Cancellable>>, callback: P);

    //
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_all_future(&self, vectors: /*Ignored*/&[OutputVector], io_priority: glib::Priority) -> Pin<Box_<dyn std::future::Future<Output = Result<usize, glib::Error>> + 'static>>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_output_stream_writev_async")]
    //fn writev_async<P: FnOnce(Result<usize, glib::Error>) + 'static>(&self, vectors: /*Ignored*/&[OutputVector], io_priority: glib::Priority, cancellable: Option<&impl IsA<Cancellable>>, callback: P);

    //
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_future(&self, vectors: /*Ignored*/&[OutputVector], io_priority: glib::Priority) -> Pin<Box_<dyn std::future::Future<Output = Result<usize, glib::Error>> + 'static>>;
}

impl<O: IsA<OutputStream>> OutputStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_output_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    fn close(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_output_stream_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
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
        unsafe extern "C" fn close_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_close_finish(_source_object as *mut _, res, &mut error);
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
        let callback = close_async_trampoline::<P>;
        unsafe {
            ffi::g_output_stream_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.close_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn flush(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_output_stream_flush(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn flush_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
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
        unsafe extern "C" fn flush_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_flush_finish(_source_object as *mut _, res, &mut error);
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
        let callback = flush_async_trampoline::<P>;
        unsafe {
            ffi::g_output_stream_flush_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn flush_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.flush_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_has_pending(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_is_closed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closing(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_is_closing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn printf(&self, cancellable: Option<&impl IsA<Cancellable>>, error: &mut glib::Error, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<usize> {
    //    unsafe { TODO: call ffi:g_output_stream_printf() }
    //}

    fn set_pending(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok =
                ffi::g_output_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn splice(
        &self,
        source: &impl IsA<InputStream>,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_splice(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
                flags.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn splice_async<P: FnOnce(Result<isize, glib::Error>) + 'static>(
        &self,
        source: &impl IsA<InputStream>,
        flags: OutputStreamSpliceFlags,
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
        unsafe extern "C" fn splice_async_trampoline<
            P: FnOnce(Result<isize, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_splice_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = splice_async_trampoline::<P>;
        unsafe {
            ffi::g_output_stream_splice_async(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
                flags.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn splice_future(
        &self,
        source: &(impl IsA<InputStream> + Clone + 'static),
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        let source = source.clone();
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.splice_async(&source, flags, io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    //fn vprintf(&self, cancellable: Option<&impl IsA<Cancellable>>, error: &mut glib::Error, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<usize> {
    //    unsafe { TODO: call ffi:g_output_stream_vprintf() }
    //}

    fn write(
        &self,
        buffer: &[u8],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error> {
        let count = buffer.len() as _;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_write(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn write_bytes(
        &self,
        bytes: &glib::Bytes,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_write_bytes(
                self.as_ref().to_glib_none().0,
                bytes.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn write_bytes_async<P: FnOnce(Result<isize, glib::Error>) + 'static>(
        &self,
        bytes: &glib::Bytes,
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
        unsafe extern "C" fn write_bytes_async_trampoline<
            P: FnOnce(Result<isize, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_output_stream_write_bytes_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = write_bytes_async_trampoline::<P>;
        unsafe {
            ffi::g_output_stream_write_bytes_async(
                self.as_ref().to_glib_none().0,
                bytes.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn write_bytes_future(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        let bytes = bytes.clone();
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.write_bytes_async(&bytes, io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev(&self, vectors: /*Ignored*/&[OutputVector], cancellable: Option<&impl IsA<Cancellable>>) -> Result<usize, glib::Error> {
    //    unsafe { TODO: call ffi:g_output_stream_writev() }
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_all(&self, vectors: /*Ignored*/&[OutputVector], cancellable: Option<&impl IsA<Cancellable>>) -> Result<usize, glib::Error> {
    //    unsafe { TODO: call ffi:g_output_stream_writev_all() }
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_all_async<P: FnOnce(Result<usize, glib::Error>) + 'static>(&self, vectors: /*Ignored*/&[OutputVector], io_priority: glib::Priority, cancellable: Option<&impl IsA<Cancellable>>, callback: P) {
    //    unsafe { TODO: call ffi:g_output_stream_writev_all_async() }
    //}

    //
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_all_future(&self, vectors: /*Ignored*/&[OutputVector], io_priority: glib::Priority) -> Pin<Box_<dyn std::future::Future<Output = Result<usize, glib::Error>> + 'static>> {

    //let vectors = vectors.clone();
    //Box_::pin(crate::GioFuture::new(self, move |obj, cancellable, send| {
    //    obj.writev_all_async(
    //        &vectors,
    //        io_priority,
    //        Some(cancellable),
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );
    //}))
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_async<P: FnOnce(Result<usize, glib::Error>) + 'static>(&self, vectors: /*Ignored*/&[OutputVector], io_priority: glib::Priority, cancellable: Option<&impl IsA<Cancellable>>, callback: P) {
    //    unsafe { TODO: call ffi:g_output_stream_writev_async() }
    //}

    //
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_future(&self, vectors: /*Ignored*/&[OutputVector], io_priority: glib::Priority) -> Pin<Box_<dyn std::future::Future<Output = Result<usize, glib::Error>> + 'static>> {

    //let vectors = vectors.clone();
    //Box_::pin(crate::GioFuture::new(self, move |obj, cancellable, send| {
    //    obj.writev_async(
    //        &vectors,
    //        io_priority,
    //        Some(cancellable),
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );
    //}))
    //}
}

impl fmt::Display for OutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OutputStream")
    }
}
