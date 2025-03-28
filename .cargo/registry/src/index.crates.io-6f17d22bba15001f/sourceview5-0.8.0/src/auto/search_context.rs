// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{Buffer, SearchSettings, Style};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GtkSourceSearchContext")]
    pub struct SearchContext(Object<ffi::GtkSourceSearchContext, ffi::GtkSourceSearchContextClass>);

    match fn {
        type_ => || ffi::gtk_source_search_context_get_type(),
    }
}

impl SearchContext {
    #[doc(alias = "gtk_source_search_context_new")]
    pub fn new(
        buffer: &impl IsA<Buffer>,
        settings: Option<&impl IsA<SearchSettings>>,
    ) -> SearchContext {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_search_context_new(
                buffer.as_ref().to_glib_none().0,
                settings.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SearchContext`] objects.
    ///
    /// This method returns an instance of [`SearchContextBuilder`](crate::builders::SearchContextBuilder) which can be used to create [`SearchContext`] objects.
    pub fn builder() -> SearchContextBuilder {
        SearchContextBuilder::new()
    }

    #[doc(alias = "gtk_source_search_context_backward")]
    pub fn backward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_source_search_context_backward(
                self.to_glib_none().0,
                iter.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
            ));
            if ret {
                Some((
                    match_start,
                    match_end,
                    from_glib(has_wrapped_around.assume_init()),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_source_search_context_backward_async")]
    pub fn backward_async<
        P: FnOnce(Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>) + 'static,
    >(
        &self,
        iter: &gtk::TextIter,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
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
        unsafe extern "C" fn backward_async_trampoline<
            P: FnOnce(Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = std::mem::MaybeUninit::uninit();
            let _ = ffi::gtk_source_search_context_backward_finish(
                _source_object as *mut _,
                res,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
                &mut error,
            );
            let result = if error.is_null() {
                Ok((
                    match_start,
                    match_end,
                    from_glib(has_wrapped_around.assume_init()),
                ))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = backward_async_trampoline::<P>;
        unsafe {
            ffi::gtk_source_search_context_backward_async(
                self.to_glib_none().0,
                iter.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn backward_future(
        &self,
        iter: &gtk::TextIter,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>,
                > + 'static,
        >,
    > {
        let iter = iter.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.backward_async(&iter, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "gtk_source_search_context_forward")]
    pub fn forward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_source_search_context_forward(
                self.to_glib_none().0,
                iter.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
            ));
            if ret {
                Some((
                    match_start,
                    match_end,
                    from_glib(has_wrapped_around.assume_init()),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_source_search_context_forward_async")]
    pub fn forward_async<
        P: FnOnce(Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>) + 'static,
    >(
        &self,
        iter: &gtk::TextIter,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
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
        unsafe extern "C" fn forward_async_trampoline<
            P: FnOnce(Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = std::mem::MaybeUninit::uninit();
            let _ = ffi::gtk_source_search_context_forward_finish(
                _source_object as *mut _,
                res,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
                &mut error,
            );
            let result = if error.is_null() {
                Ok((
                    match_start,
                    match_end,
                    from_glib(has_wrapped_around.assume_init()),
                ))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = forward_async_trampoline::<P>;
        unsafe {
            ffi::gtk_source_search_context_forward_async(
                self.to_glib_none().0,
                iter.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn forward_future(
        &self,
        iter: &gtk::TextIter,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>,
                > + 'static,
        >,
    > {
        let iter = iter.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.forward_async(&iter, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "gtk_source_search_context_get_buffer")]
    #[doc(alias = "get_buffer")]
    pub fn buffer(&self) -> Buffer {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_buffer(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_get_highlight")]
    #[doc(alias = "get_highlight")]
    pub fn is_highlight(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_context_get_highlight(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_get_match_style")]
    #[doc(alias = "get_match_style")]
    pub fn match_style(&self) -> Style {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_match_style(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_get_occurrence_position")]
    #[doc(alias = "get_occurrence_position")]
    pub fn occurrence_position(
        &self,
        match_start: &gtk::TextIter,
        match_end: &gtk::TextIter,
    ) -> i32 {
        unsafe {
            ffi::gtk_source_search_context_get_occurrence_position(
                self.to_glib_none().0,
                match_start.to_glib_none().0,
                match_end.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gtk_source_search_context_get_occurrences_count")]
    #[doc(alias = "get_occurrences_count")]
    pub fn occurrences_count(&self) -> i32 {
        unsafe { ffi::gtk_source_search_context_get_occurrences_count(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_source_search_context_get_regex_error")]
    #[doc(alias = "get_regex_error")]
    pub fn regex_error(&self) -> Option<glib::Error> {
        unsafe {
            from_glib_full(ffi::gtk_source_search_context_get_regex_error(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_get_settings")]
    #[doc(alias = "get_settings")]
    pub fn settings(&self) -> SearchSettings {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_settings(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_replace")]
    pub fn replace(
        &self,
        match_start: &mut gtk::TextIter,
        match_end: &mut gtk::TextIter,
        replace: &str,
    ) -> Result<(), glib::Error> {
        let replace_length = replace.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gtk_source_search_context_replace(
                self.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                replace.to_glib_none().0,
                replace_length,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_source_search_context_set_highlight")]
    pub fn set_highlight(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_search_context_set_highlight(
                self.to_glib_none().0,
                highlight.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_source_search_context_set_match_style")]
    pub fn set_match_style(&self, match_style: Option<&Style>) {
        unsafe {
            ffi::gtk_source_search_context_set_match_style(
                self.to_glib_none().0,
                match_style.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "highlight")]
    pub fn connect_highlight_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_trampoline<F: Fn(&SearchContext) + 'static>(
            this: *mut ffi::GtkSourceSearchContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::highlight\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_highlight_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "match-style")]
    pub fn connect_match_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_match_style_trampoline<F: Fn(&SearchContext) + 'static>(
            this: *mut ffi::GtkSourceSearchContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::match-style\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_match_style_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "occurrences-count")]
    pub fn connect_occurrences_count_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_occurrences_count_trampoline<
            F: Fn(&SearchContext) + 'static,
        >(
            this: *mut ffi::GtkSourceSearchContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::occurrences-count\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_occurrences_count_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "regex-error")]
    pub fn connect_regex_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_regex_error_trampoline<F: Fn(&SearchContext) + 'static>(
            this: *mut ffi::GtkSourceSearchContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::regex-error\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_regex_error_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SearchContext {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SearchContext`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SearchContextBuilder {
    builder: glib::object::ObjectBuilder<'static, SearchContext>,
}

impl SearchContextBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn buffer(self, buffer: &impl IsA<Buffer>) -> Self {
        Self {
            builder: self.builder.property("buffer", buffer.clone().upcast()),
        }
    }

    pub fn highlight(self, highlight: bool) -> Self {
        Self {
            builder: self.builder.property("highlight", highlight),
        }
    }

    pub fn match_style(self, match_style: &Style) -> Self {
        Self {
            builder: self.builder.property("match-style", match_style.clone()),
        }
    }

    pub fn settings(self, settings: &impl IsA<SearchSettings>) -> Self {
        Self {
            builder: self.builder.property("settings", settings.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SearchContext`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SearchContext {
        self.builder.build()
    }
}
