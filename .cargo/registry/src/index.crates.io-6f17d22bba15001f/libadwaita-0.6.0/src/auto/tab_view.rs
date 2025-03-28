// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TabPage;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
use crate::TabViewShortcuts;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwTabView")]
    pub struct TabView(Object<ffi::AdwTabView, ffi::AdwTabViewClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_tab_view_get_type(),
    }
}

impl TabView {
    #[doc(alias = "adw_tab_view_new")]
    pub fn new() -> TabView {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::adw_tab_view_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`TabView`] objects.
    ///
    /// This method returns an instance of [`TabViewBuilder`](crate::builders::TabViewBuilder) which can be used to create [`TabView`] objects.
    pub fn builder() -> TabViewBuilder {
        TabViewBuilder::new()
    }

    #[doc(alias = "adw_tab_view_add_page")]
    pub fn add_page(&self, child: &impl IsA<gtk::Widget>, parent: Option<&TabPage>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_add_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                parent.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_tab_view_add_shortcuts")]
    pub fn add_shortcuts(&self, shortcuts: TabViewShortcuts) {
        unsafe {
            ffi::adw_tab_view_add_shortcuts(self.to_glib_none().0, shortcuts.into_glib());
        }
    }

    #[doc(alias = "adw_tab_view_append")]
    pub fn append(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_append(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_append_pinned")]
    pub fn append_pinned(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_append_pinned(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_close_other_pages")]
    pub fn close_other_pages(&self, page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_close_other_pages(self.to_glib_none().0, page.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_close_page")]
    pub fn close_page(&self, page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_close_page(self.to_glib_none().0, page.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_close_page_finish")]
    pub fn close_page_finish(&self, page: &TabPage, confirm: bool) {
        unsafe {
            ffi::adw_tab_view_close_page_finish(
                self.to_glib_none().0,
                page.to_glib_none().0,
                confirm.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_view_close_pages_after")]
    pub fn close_pages_after(&self, page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_close_pages_after(self.to_glib_none().0, page.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_close_pages_before")]
    pub fn close_pages_before(&self, page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_close_pages_before(self.to_glib_none().0, page.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_get_default_icon")]
    #[doc(alias = "get_default_icon")]
    pub fn default_icon(&self) -> gio::Icon {
        unsafe { from_glib_none(ffi::adw_tab_view_get_default_icon(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_get_is_transferring_page")]
    #[doc(alias = "get_is_transferring_page")]
    pub fn is_transferring_page(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_get_is_transferring_page(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_get_menu_model")]
    #[doc(alias = "get_menu_model")]
    pub fn menu_model(&self) -> Option<gio::MenuModel> {
        unsafe { from_glib_none(ffi::adw_tab_view_get_menu_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    pub fn n_pages(&self) -> i32 {
        unsafe { ffi::adw_tab_view_get_n_pages(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_tab_view_get_n_pinned_pages")]
    #[doc(alias = "get_n_pinned_pages")]
    pub fn n_pinned_pages(&self) -> i32 {
        unsafe { ffi::adw_tab_view_get_n_pinned_pages(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_tab_view_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_get_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_get_page_position")]
    #[doc(alias = "get_page_position")]
    pub fn page_position(&self, page: &TabPage) -> i32 {
        unsafe { ffi::adw_tab_view_get_page_position(self.to_glib_none().0, page.to_glib_none().0) }
    }

    #[doc(alias = "adw_tab_view_get_pages")]
    #[doc(alias = "get_pages")]
    pub fn pages(&self) -> gtk::SelectionModel {
        unsafe { from_glib_full(ffi::adw_tab_view_get_pages(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_get_selected_page")]
    #[doc(alias = "get_selected_page")]
    pub fn selected_page(&self) -> Option<TabPage> {
        unsafe { from_glib_none(ffi::adw_tab_view_get_selected_page(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_tab_view_get_shortcuts")]
    #[doc(alias = "get_shortcuts")]
    pub fn shortcuts(&self) -> TabViewShortcuts {
        unsafe { from_glib(ffi::adw_tab_view_get_shortcuts(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_insert")]
    pub fn insert(&self, child: &impl IsA<gtk::Widget>, position: i32) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_insert(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_insert_pinned")]
    pub fn insert_pinned(&self, child: &impl IsA<gtk::Widget>, position: i32) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_insert_pinned(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "adw_tab_view_invalidate_thumbnails")]
    pub fn invalidate_thumbnails(&self) {
        unsafe {
            ffi::adw_tab_view_invalidate_thumbnails(self.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_prepend")]
    pub fn prepend(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_prepend(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_prepend_pinned")]
    pub fn prepend_pinned(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_prepend_pinned(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_tab_view_remove_shortcuts")]
    pub fn remove_shortcuts(&self, shortcuts: TabViewShortcuts) {
        unsafe {
            ffi::adw_tab_view_remove_shortcuts(self.to_glib_none().0, shortcuts.into_glib());
        }
    }

    #[doc(alias = "adw_tab_view_reorder_backward")]
    pub fn reorder_backward(&self, page: &TabPage) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_backward(
                self.to_glib_none().0,
                page.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_reorder_first")]
    pub fn reorder_first(&self, page: &TabPage) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_first(
                self.to_glib_none().0,
                page.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_reorder_forward")]
    pub fn reorder_forward(&self, page: &TabPage) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_forward(
                self.to_glib_none().0,
                page.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_reorder_last")]
    pub fn reorder_last(&self, page: &TabPage) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_last(
                self.to_glib_none().0,
                page.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_reorder_page")]
    pub fn reorder_page(&self, page: &TabPage, position: i32) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_page(
                self.to_glib_none().0,
                page.to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_select_next_page")]
    pub fn select_next_page(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_view_select_next_page(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_select_previous_page")]
    pub fn select_previous_page(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_select_previous_page(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_set_default_icon")]
    pub fn set_default_icon(&self, default_icon: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::adw_tab_view_set_default_icon(
                self.to_glib_none().0,
                default_icon.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_view_set_menu_model")]
    pub fn set_menu_model(&self, menu_model: Option<&impl IsA<gio::MenuModel>>) {
        unsafe {
            ffi::adw_tab_view_set_menu_model(
                self.to_glib_none().0,
                menu_model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_view_set_page_pinned")]
    pub fn set_page_pinned(&self, page: &TabPage, pinned: bool) {
        unsafe {
            ffi::adw_tab_view_set_page_pinned(
                self.to_glib_none().0,
                page.to_glib_none().0,
                pinned.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_view_set_selected_page")]
    pub fn set_selected_page(&self, selected_page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_set_selected_page(
                self.to_glib_none().0,
                selected_page.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_tab_view_set_shortcuts")]
    pub fn set_shortcuts(&self, shortcuts: TabViewShortcuts) {
        unsafe {
            ffi::adw_tab_view_set_shortcuts(self.to_glib_none().0, shortcuts.into_glib());
        }
    }

    #[doc(alias = "adw_tab_view_transfer_page")]
    pub fn transfer_page(&self, page: &TabPage, other_view: &TabView, position: i32) {
        unsafe {
            ffi::adw_tab_view_transfer_page(
                self.to_glib_none().0,
                page.to_glib_none().0,
                other_view.to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "close-page")]
    pub fn connect_close_page<F: Fn(&Self, &TabPage) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn close_page_trampoline<F: Fn(&TabView, &TabPage) -> bool + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close-page\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    close_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "create-window")]
    pub fn connect_create_window<F: Fn(&Self) -> Option<TabView> + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_window_trampoline<
            F: Fn(&TabView) -> Option<TabView> + 'static,
        >(
            this: *mut ffi::AdwTabView,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::AdwTabView {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)) /*Not checked*/
                .to_glib_none()
                .0
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-window\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    create_window_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "indicator-activated")]
    pub fn connect_indicator_activated<F: Fn(&Self, &TabPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn indicator_activated_trampoline<F: Fn(&TabView, &TabPage) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"indicator-activated\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    indicator_activated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-attached")]
    pub fn connect_page_attached<F: Fn(&Self, &TabPage, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_attached_trampoline<F: Fn(&TabView, &TabPage, i32) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            position: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page), position)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-attached\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    page_attached_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-detached")]
    pub fn connect_page_detached<F: Fn(&Self, &TabPage, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_detached_trampoline<F: Fn(&TabView, &TabPage, i32) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            position: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page), position)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-detached\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    page_detached_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-reordered")]
    pub fn connect_page_reordered<F: Fn(&Self, &TabPage, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_reordered_trampoline<F: Fn(&TabView, &TabPage, i32) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            position: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page), position)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-reordered\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    page_reordered_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "setup-menu")]
    pub fn connect_setup_menu<F: Fn(&Self, Option<&TabPage>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn setup_menu_trampoline<F: Fn(&TabView, Option<&TabPage>) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<TabPage>::from_glib_borrow(page).as_ref().as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"setup-menu\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    setup_menu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "default-icon")]
    pub fn connect_default_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_icon_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::default-icon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_default_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-transferring-page")]
    pub fn connect_is_transferring_page_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_transferring_page_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::is-transferring-page\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_is_transferring_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu-model")]
    pub fn connect_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_model_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::menu-model\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_menu_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "n-pages")]
    pub fn connect_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_pages_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::n-pages\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_n_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "n-pinned-pages")]
    pub fn connect_n_pinned_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_pinned_pages_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::n-pinned-pages\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_n_pinned_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pages")]
    pub fn connect_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::pages\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected-page")]
    pub fn connect_selected_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_page_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::selected-page\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "shortcuts")]
    pub fn connect_shortcuts_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcuts_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::shortcuts\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_shortcuts_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TabView {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`TabView`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TabViewBuilder {
    builder: glib::object::ObjectBuilder<'static, TabView>,
}

impl TabViewBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn default_icon(self, default_icon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-icon", default_icon.clone().upcast()),
        }
    }

    pub fn menu_model(self, menu_model: &impl IsA<gio::MenuModel>) -> Self {
        Self {
            builder: self
                .builder
                .property("menu-model", menu_model.clone().upcast()),
        }
    }

    pub fn selected_page(self, selected_page: &TabPage) -> Self {
        Self {
            builder: self
                .builder
                .property("selected-page", selected_page.clone()),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub fn shortcuts(self, shortcuts: TabViewShortcuts) -> Self {
        Self {
            builder: self.builder.property("shortcuts", shortcuts),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TabView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TabView {
        self.builder.build()
    }
}
