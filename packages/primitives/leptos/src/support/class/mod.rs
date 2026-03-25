//! Typed `class` prop for Pith UI components.
//!
//! Provides [`ClassProp`], a component prop type that accepts class strings,
//! custom style types (via [`IntoPithClass`]), and reactive wrappers
//! (`Signal`, `Memo`, `ReadSignal`, `RwSignal`) around any of those types.
//!
//! # Usage in components
//!
//! ```ignore
//! #[component]
//! pub fn MyComponent(
//!     #[prop(into, optional)] class: ClassProp,
//! ) -> impl IntoView { /* ... */ }
//! ```
//!
//! # Supported call-site forms
//!
//! ```ignore
//! // Static strings
//! <MyComponent class="flex p-4" />
//! <MyComponent class=my_string />
//!
//! // Custom types implementing IntoPithClass
//! <MyComponent class=BtnSize::Sm />
//!
//! // Reactive
//! <MyComponent class=my_signal />       // Signal<String>, Signal<T: IntoPithClass>, etc.
//! <MyComponent class=my_memo />         // Memo<String>, Memo<T: IntoPithClass>, etc.
//! ```

use leptos::prelude::*;

/// Resolved class value wrapping a class string.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PithClass(pub String);

impl PithClass {
    /// Create a new class value from anything that converts to `String`.
    pub fn new(s: impl Into<String>) -> Self {
        PithClass(s.into())
    }

    /// View the class as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Implement this trait on a custom type to make it usable as a `class` prop
/// value. Any type implementing `IntoPithClass` can be passed directly to
/// a component's `class` prop (both statically and inside reactive wrappers).
///
/// The method takes `&self` so that reactive signal wrappers (which call
/// `sig.get()` and then convert the cloned value) can invoke it without
/// consuming the original.
///
/// # Example
///
/// ```ignore
/// struct MyTheme { dark: bool }
///
/// impl IntoPithClass for MyTheme {
///     fn to_pith_class(&self) -> PithClass {
///         PithClass::new(if self.dark { "theme-dark" } else { "theme-light" })
///     }
/// }
/// ```
pub trait IntoPithClass {
    /// Convert this value into a [`PithClass`].
    fn to_pith_class(&self) -> PithClass;
}

/// Component prop type for the `class` attribute.
///
/// Wraps a [`MaybeProp<PithClass>`] so that it works with both static values
/// and reactive signals. Use with `#[prop(into, optional)]`.
#[derive(Clone)]
pub struct ClassProp(MaybeProp<PithClass>);

impl Default for ClassProp {
    fn default() -> Self {
        ClassProp(None::<PithClass>.into())
    }
}

impl ClassProp {
    /// Read the current class value, if any.
    pub fn get(&self) -> Option<PithClass> {
        self.0.get()
    }

    /// Read the current class as a `String`, defaulting to `""` when unset.
    pub fn get_string(&self) -> String {
        self.get().map(|c| c.0).unwrap_or_default()
    }
}

// ── Static From impls ──────────────────────────────────────────

impl From<&str> for ClassProp {
    fn from(s: &str) -> Self {
        ClassProp(MaybeProp::from(PithClass(s.to_string())))
    }
}

impl From<String> for ClassProp {
    fn from(s: String) -> Self {
        ClassProp(MaybeProp::from(PithClass(s)))
    }
}

impl From<PithClass> for ClassProp {
    fn from(c: PithClass) -> Self {
        ClassProp(MaybeProp::from(c))
    }
}

/// Blanket impl: any `T: IntoPithClass` converts to `ClassProp` in one hop.
impl<T: IntoPithClass> From<T> for ClassProp {
    fn from(value: T) -> Self {
        ClassProp(MaybeProp::from(value.to_pith_class()))
    }
}

impl From<MaybeProp<PithClass>> for ClassProp {
    fn from(mp: MaybeProp<PithClass>) -> Self {
        ClassProp(mp)
    }
}

// ── Reactive From impls ────────────────────────────────────────

macro_rules! impl_from_reactive {
    ($($sig:ident),*) => {$(
        impl From<$sig<String>> for ClassProp {
            fn from(sig: $sig<String>) -> Self {
                let sig = Signal::from(sig);
                ClassProp(MaybeProp::from(Signal::derive(move || {
                    Some(PithClass(sig.get()))
                })))
            }
        }

        impl From<$sig<PithClass>> for ClassProp {
            fn from(sig: $sig<PithClass>) -> Self {
                let sig = Signal::from(sig);
                ClassProp(MaybeProp::from(Signal::derive(move || {
                    Some(sig.get())
                })))
            }
        }

        impl<T> From<$sig<T>> for ClassProp
        where
            T: IntoPithClass + Clone + Send + Sync + 'static,
        {
            fn from(sig: $sig<T>) -> Self {
                let sig: Signal<T> = Signal::from(sig);
                ClassProp(MaybeProp::from(Signal::derive(move || {
                    Some(sig.get().to_pith_class())
                })))
            }
        }
    )*}
}

impl_from_reactive!(Signal, ReadSignal, Memo, RwSignal);

// ── Tests ──────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    use any_spawner::Executor;
    use leptos::reactive::owner::Owner;

    #[derive(Clone, PartialEq)]
    struct MyTheme {
        dark: bool,
    }

    impl IntoPithClass for MyTheme {
        fn to_pith_class(&self) -> PithClass {
            PithClass::new(if self.dark { "theme-dark" } else { "theme-light" })
        }
    }

    struct NoopExecutor;

    impl any_spawner::CustomExecutor for NoopExecutor {
        fn spawn(&self, _fut: any_spawner::PinnedFuture<()>) {}
        fn spawn_local(&self, _fut: any_spawner::PinnedLocalFuture<()>) {}
        fn poll_local(&self) {}
    }

    fn with_owner<T>(f: impl FnOnce() -> T) -> T {
        let _ = Executor::init_custom_executor(NoopExecutor);
        let owner = Owner::new_root(None);
        owner.with(f)
    }

    /// Simulates `#[prop(into)] class: ClassProp`.
    fn component(class: impl Into<ClassProp>) -> String {
        let class: ClassProp = class.into();
        class.get_string()
    }

    // ── Static ─────────────────────────────────────────────────

    #[test]
    fn static_str() {
        assert_eq!(component("flex p-4"), "flex p-4");
    }

    #[test]
    fn static_string() {
        assert_eq!(component(String::from("flex p-4")), "flex p-4");
    }

    #[test]
    fn static_custom() {
        assert_eq!(component(MyTheme { dark: true }), "theme-dark");
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(ClassProp::default().get_string(), "");
    }

    // ── Reactive: Signal ───────────────────────────────────────

    #[test]
    fn signal_string() {
        with_owner(|| {
            assert_eq!(component(Signal::stored(String::from("flex"))), "flex");
        });
    }

    #[test]
    fn signal_pith_class() {
        with_owner(|| {
            assert_eq!(component(Signal::stored(PithClass::new("flex"))), "flex");
        });
    }

    #[test]
    fn signal_custom() {
        with_owner(|| {
            assert_eq!(component(Signal::stored(MyTheme { dark: true })), "theme-dark");
        });
    }

    // ── Reactive: Memo ─────────────────────────────────────────

    #[test]
    fn memo_string() {
        with_owner(|| {
            assert_eq!(component(Memo::new(|_| String::from("p-4"))), "p-4");
        });
    }

    #[test]
    fn memo_custom() {
        with_owner(|| {
            assert_eq!(
                component(Memo::new(|_| MyTheme { dark: false })),
                "theme-light"
            );
        });
    }

    // ── Reactive: ReadSignal ───────────────────────────────────

    #[test]
    fn read_signal_string() {
        with_owner(|| {
            let (read, _write) = signal(String::from("grid"));
            assert_eq!(component(read), "grid");
        });
    }

    // ── Reactive: RwSignal ─────────────────────────────────────

    #[test]
    fn rw_signal_string() {
        with_owner(|| {
            assert_eq!(component(RwSignal::new(String::from("block"))), "block");
        });
    }

    #[test]
    fn rw_signal_custom() {
        with_owner(|| {
            assert_eq!(component(RwSignal::new(MyTheme { dark: true })), "theme-dark");
        });
    }

    // ── Reactive: updates propagate ────────────────────────────

    #[test]
    fn signal_string_updates() {
        with_owner(|| {
            let (read, write) = signal(String::from("before"));
            let class: ClassProp = read.into();

            assert_eq!(class.get_string(), "before");
            write.set(String::from("after"));
            assert_eq!(class.get_string(), "after");
        });
    }

    #[test]
    fn signal_custom_updates() {
        with_owner(|| {
            let (read, write) = signal(MyTheme { dark: false });
            let class: ClassProp = read.into();

            assert_eq!(class.get_string(), "theme-light");
            write.set(MyTheme { dark: true });
            assert_eq!(class.get_string(), "theme-dark");
        });
    }

    #[test]
    fn rw_signal_updates() {
        with_owner(|| {
            let rw = RwSignal::new(String::from("initial"));
            let class: ClassProp = rw.into();

            assert_eq!(class.get_string(), "initial");
            rw.set(String::from("updated"));
            assert_eq!(class.get_string(), "updated");
        });
    }

    #[test]
    fn memo_recomputes() {
        with_owner(|| {
            let (count, set_count) = signal(0);
            let m = Memo::new(move |_| {
                if count.get() > 0 {
                    String::from("has-items")
                } else {
                    String::from("empty")
                }
            });
            let class: ClassProp = m.into();

            assert_eq!(class.get_string(), "empty");
            set_count.set(5);
            assert_eq!(class.get_string(), "has-items");
        });
    }
}
