// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Snippet;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSourceSnippetManager")]
    pub struct SnippetManager(Object<ffi::GtkSourceSnippetManager, ffi::GtkSourceSnippetManagerClass>);

    match fn {
        type_ => || ffi::gtk_source_snippet_manager_get_type(),
    }
}

impl SnippetManager {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SnippetManager`] objects.
    ///
    /// This method returns an instance of [`SnippetManagerBuilder`](crate::builders::SnippetManagerBuilder) which can be used to create [`SnippetManager`] objects.
    pub fn builder() -> SnippetManagerBuilder {
        SnippetManagerBuilder::new()
    }

    #[doc(alias = "gtk_source_snippet_manager_get_search_path")]
    #[doc(alias = "get_search_path")]
    pub fn search_path(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_snippet_manager_get_search_path(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_snippet_manager_get_snippet")]
    #[doc(alias = "get_snippet")]
    pub fn snippet(
        &self,
        group: Option<&str>,
        language_id: Option<&str>,
        trigger: &str,
    ) -> Option<Snippet> {
        unsafe {
            from_glib_full(ffi::gtk_source_snippet_manager_get_snippet(
                self.to_glib_none().0,
                group.to_glib_none().0,
                language_id.to_glib_none().0,
                trigger.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v5_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v5_6")))]
    #[doc(alias = "gtk_source_snippet_manager_list_all")]
    pub fn list_all(&self) -> gio::ListModel {
        unsafe {
            from_glib_none(ffi::gtk_source_snippet_manager_list_all(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_snippet_manager_list_groups")]
    pub fn list_groups(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_source_snippet_manager_list_groups(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_snippet_manager_list_matching")]
    pub fn list_matching(
        &self,
        group: Option<&str>,
        language_id: Option<&str>,
        trigger_prefix: Option<&str>,
    ) -> gio::ListModel {
        unsafe {
            from_glib_full(ffi::gtk_source_snippet_manager_list_matching(
                self.to_glib_none().0,
                group.to_glib_none().0,
                language_id.to_glib_none().0,
                trigger_prefix.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_snippet_manager_set_search_path")]
    pub fn set_search_path(&self, dirs: &[&str]) {
        unsafe {
            ffi::gtk_source_snippet_manager_set_search_path(
                self.to_glib_none().0,
                dirs.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_snippet_manager_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> SnippetManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_source_snippet_manager_get_default()) }
    }

    #[doc(alias = "search-path")]
    pub fn connect_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_path_trampoline<F: Fn(&SnippetManager) + 'static>(
            this: *mut ffi::GtkSourceSnippetManager,
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
                b"notify::search-path\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_search_path_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SnippetManager`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SnippetManagerBuilder {
    builder: glib::object::ObjectBuilder<'static, SnippetManager>,
}

impl SnippetManagerBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn search_path(self, search_path: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("search-path", search_path.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SnippetManager`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SnippetManager {
        self.builder.build()
    }
}
