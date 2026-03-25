//! Leptos port of [Radix Themes](https://www.radix-ui.com/).
//!
//! Radix Themes is a pre-styled component library that is designed to work out
//! of the box with minimal configuration.
//!
//! See [the Pith UI book](https://pith-ui.dev/themes/index.html) for more
//! documentation.
//!
//! See [`@radix-ui/themes`](https://www.npmjs.com/package/@radix-ui/themes)
//! for the original package.
//!
//! # tailwind_fuse bridge
//!
//! This crate provides [`impl_pith_tw_class!`] and [`impl_pith_tw_variant!`]
//! macros that bridge `tailwind_fuse` derive types (`TwClass`, `TwVariant`) to
//! [`pith_ui::class::IntoPithClass`], so they can be used directly as
//! `class` prop values on any Pith UI component.
//!
//! ```ignore
//! use tailwind_fuse::*;
//! use pith_ui_themes::*;
//!
//! #[derive(TwVariant)]
//! enum BtnSize {
//!     #[tw(default, class = "h-9 px-4 py-2")]
//!     Default,
//!     #[tw(class = "h-8 px-3")]
//!     Sm,
//! }
//! impl_pith_tw_variant!(BtnSize);
//!
//! // Now usable: <Button class=BtnSize::Sm />
//! ```

/// Bridge one or more `TwClass` structs (which implement
/// [`tailwind_fuse::IntoTailwindClass`]) to
/// [`pith_ui::class::IntoPithClass`], making them usable as `class` prop
/// values on Pith UI components.
///
/// ```ignore
/// #[derive(TwClass)]
/// #[tw(class = "flex")]
/// struct Btn { size: BtnSize }
///
/// impl_pith_tw_class!(Btn);
/// ```
#[macro_export]
macro_rules! impl_pith_tw_class {
    ($($ty:ty),* $(,)?) => {
        $(impl ::pith_ui::class::IntoPithClass for $ty {
            fn to_pith_class(&self) -> ::pith_ui::class::PithClass {
                ::pith_ui::class::PithClass::new(
                    ::tailwind_fuse::IntoTailwindClass::to_class(self)
                )
            }
        })*
    }
}

/// Bridge one or more `TwVariant` enums (which implement
/// [`tailwind_fuse::AsTailwindClass`]) to
/// [`pith_ui::class::IntoPithClass`], making them usable as `class` prop
/// values on Pith UI components.
///
/// ```ignore
/// #[derive(TwVariant)]
/// enum BtnSize {
///     #[tw(default, class = "h-9 px-4 py-2")]
///     Default,
///     #[tw(class = "h-8 px-3")]
///     Sm,
/// }
///
/// impl_pith_tw_variant!(BtnSize);
/// ```
#[macro_export]
macro_rules! impl_pith_tw_variant {
    ($($ty:ty),* $(,)?) => {
        $(impl ::pith_ui::class::IntoPithClass for $ty {
            fn to_pith_class(&self) -> ::pith_ui::class::PithClass {
                ::pith_ui::class::PithClass::new(
                    ::tailwind_fuse::AsTailwindClass::as_class(self)
                )
            }
        })*
    }
}

#[cfg(test)]
mod tests {
    use pith_ui::class::*;
    use leptos::prelude::*;
    use leptos::reactive::owner::Owner;
    use tailwind_fuse::*;

    #[derive(TwVariant)]
    enum BtnSize {
        #[tw(default, class = "h-9 px-4 py-2")]
        Default,
        #[tw(class = "h-8 px-3")]
        Sm,
    }
    impl_pith_tw_variant!(BtnSize);

    #[derive(TwClass)]
    #[tw(class = "flex")]
    struct Btn {
        size: BtnSize,
    }
    impl_pith_tw_class!(Btn);

    struct NoopExecutor;

    impl any_spawner::CustomExecutor for NoopExecutor {
        fn spawn(&self, _fut: any_spawner::PinnedFuture<()>) {}
        fn spawn_local(&self, _fut: any_spawner::PinnedLocalFuture<()>) {}
        fn poll_local(&self) {}
    }

    fn with_owner<T>(f: impl FnOnce() -> T) -> T {
        let _ = any_spawner::Executor::init_custom_executor(NoopExecutor);
        let owner = Owner::new_root(None);
        owner.with(f)
    }

    fn component(class: impl Into<ClassProp>) -> String {
        let class: ClassProp = class.into();
        class.get_string()
    }

    // ── Static ─────────────────────────────────────────────────

    #[test]
    fn static_tw_variant() {
        assert_eq!(component(BtnSize::Sm), "h-8 px-3");
    }

    #[test]
    fn static_tw_class() {
        assert_eq!(component(Btn { size: BtnSize::Sm }), "flex h-8 px-3");
    }

    // ── Reactive: Signal<TwVariant> ────────────────────────────

    #[test]
    fn signal_tw_variant() {
        with_owner(|| {
            assert_eq!(component(Signal::stored(BtnSize::Sm)), "h-8 px-3");
        });
    }

    // ── Reactive: updates propagate ────────────────────────────

    #[test]
    fn signal_tw_variant_updates() {
        with_owner(|| {
            let (read, write) = signal(BtnSize::Default);
            let class: ClassProp = read.into();

            assert_eq!(class.get_string(), "h-9 px-4 py-2");
            write.set(BtnSize::Sm);
            assert_eq!(class.get_string(), "h-8 px-3");
        });
    }
}
