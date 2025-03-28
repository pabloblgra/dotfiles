// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::translate::*;
#[cfg(any(feature = "v2_72", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
use crate::MainContextFlags;

crate::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MainContext(Shared<ffi::GMainContext>);

    match fn {
        ref => |ptr| ffi::g_main_context_ref(ptr),
        unref => |ptr| ffi::g_main_context_unref(ptr),
        type_ => || ffi::g_main_context_get_type(),
    }
}

impl MainContext {
    #[doc(alias = "g_main_context_new")]
    pub fn new() -> MainContext {
        unsafe { from_glib_full(ffi::g_main_context_new()) }
    }

    #[cfg(any(feature = "v2_72", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
    #[doc(alias = "g_main_context_new_with_flags")]
    #[doc(alias = "new_with_flags")]
    pub fn with_flags(flags: MainContextFlags) -> MainContext {
        unsafe { from_glib_full(ffi::g_main_context_new_with_flags(flags.into_glib())) }
    }

    //#[doc(alias = "g_main_context_add_poll")]
    //pub fn add_poll(&self, fd: /*Ignored*/&mut PollFD, priority: i32) {
    //    unsafe { TODO: call ffi:g_main_context_add_poll() }
    //}

    //#[doc(alias = "g_main_context_check")]
    //pub fn check(&self, max_priority: i32, fds: /*Ignored*/&[PollFD]) -> bool {
    //    unsafe { TODO: call ffi:g_main_context_check() }
    //}

    #[doc(alias = "g_main_context_dispatch")]
    pub fn dispatch(&self) {
        unsafe {
            ffi::g_main_context_dispatch(self.to_glib_none().0);
        }
    }

    //#[doc(alias = "g_main_context_find_source_by_funcs_user_data")]
    //pub fn find_source_by_funcs_user_data(&self, funcs: /*Ignored*/&mut SourceFuncs, user_data: /*Unimplemented*/Option<Basic: Pointer>) -> Source {
    //    unsafe { TODO: call ffi:g_main_context_find_source_by_funcs_user_data() }
    //}

    //#[doc(alias = "g_main_context_find_source_by_user_data")]
    //pub fn find_source_by_user_data(&self, user_data: /*Unimplemented*/Option<Basic: Pointer>) -> Source {
    //    unsafe { TODO: call ffi:g_main_context_find_source_by_user_data() }
    //}

    //#[doc(alias = "g_main_context_get_poll_func")]
    //#[doc(alias = "get_poll_func")]
    //pub fn poll_func(&self) -> /*Unimplemented*/Fn(/*Ignored*/PollFD, u32, i32) -> i32 {
    //    unsafe { TODO: call ffi:g_main_context_get_poll_func() }
    //}

    #[doc(alias = "g_main_context_is_owner")]
    pub fn is_owner(&self) -> bool {
        unsafe { from_glib(ffi::g_main_context_is_owner(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_main_context_iteration")]
    pub fn iteration(&self, may_block: bool) -> bool {
        unsafe {
            from_glib(ffi::g_main_context_iteration(
                self.to_glib_none().0,
                may_block.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_main_context_pending")]
    pub fn pending(&self) -> bool {
        unsafe { from_glib(ffi::g_main_context_pending(self.to_glib_none().0)) }
    }

    //#[doc(alias = "g_main_context_query")]
    //pub fn query(&self, max_priority: i32, fds: /*Ignored*/Vec<PollFD>) -> (i32, i32) {
    //    unsafe { TODO: call ffi:g_main_context_query() }
    //}

    //#[doc(alias = "g_main_context_remove_poll")]
    //pub fn remove_poll(&self, fd: /*Ignored*/&mut PollFD) {
    //    unsafe { TODO: call ffi:g_main_context_remove_poll() }
    //}

    //#[doc(alias = "g_main_context_set_poll_func")]
    //pub fn set_poll_func(&self, func: /*Unimplemented*/Fn(/*Ignored*/PollFD, u32, i32) -> i32) {
    //    unsafe { TODO: call ffi:g_main_context_set_poll_func() }
    //}

    //#[cfg_attr(feature = "v2_58", deprecated = "Since 2.58")]
    //#[doc(alias = "g_main_context_wait")]
    //pub fn wait(&self, cond: /*Ignored*/&mut Cond, mutex: /*Ignored*/&mut Mutex) -> bool {
    //    unsafe { TODO: call ffi:g_main_context_wait() }
    //}

    #[doc(alias = "g_main_context_wakeup")]
    pub fn wakeup(&self) {
        unsafe {
            ffi::g_main_context_wakeup(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_main_context_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> MainContext {
        unsafe { from_glib_none(ffi::g_main_context_default()) }
    }

    #[doc(alias = "g_main_context_get_thread_default")]
    #[doc(alias = "get_thread_default")]
    pub fn thread_default() -> Option<MainContext> {
        unsafe { from_glib_none(ffi::g_main_context_get_thread_default()) }
    }

    #[doc(alias = "g_main_context_ref_thread_default")]
    pub fn ref_thread_default() -> MainContext {
        unsafe { from_glib_full(ffi::g_main_context_ref_thread_default()) }
    }
}

impl Default for MainContext {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for MainContext {}
unsafe impl Sync for MainContext {}
