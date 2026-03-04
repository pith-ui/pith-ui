use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use leptos::prelude::*;

type Machine<S, E> = HashMap<S, HashMap<E, S>>;

pub fn use_state_machine<
    S: Clone + Debug + Eq + Hash + Send + Sync + 'static,
    E: Clone + Debug + Eq + Hash + Send + Sync + 'static,
>(
    initial_state: S,
    machine: Machine<S, E>,
) -> (ReadSignal<S>, Callback<E>) {
    let (state, set_state) = signal(initial_state);

    (
        state,
        Callback::new(move |event| {
            let current_state = state.get_untracked();
            let next_state = machine
                .get(&current_state)
                .and_then(|events| events.get(&event));

            if let Some(next_state) = next_state {
                set_state.set(next_state.clone());
            }
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use any_spawner::Executor;
    use leptos::reactive::owner::Owner;

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

    // A simple A→B→C machine for testing.
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum State {
        A,
        B,
        C,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum Event {
        GoB,
        GoC,
    }

    fn machine() -> HashMap<State, HashMap<Event, State>> {
        HashMap::from([
            (State::A, HashMap::from([(Event::GoB, State::B)])),
            (State::B, HashMap::from([(Event::GoC, State::C)])),
        ])
    }

    #[test]
    fn initial_state_is_returned() {
        with_owner(|| {
            let (state, _send) = use_state_machine(State::A, machine());
            assert_eq!(state.get_untracked(), State::A);
        });
    }

    #[test]
    fn valid_transition_updates_state() {
        with_owner(|| {
            let (state, send) = use_state_machine(State::A, machine());
            send.run(Event::GoB);
            assert_eq!(state.get_untracked(), State::B);
        });
    }

    #[test]
    fn invalid_event_is_noop() {
        with_owner(|| {
            let (state, send) = use_state_machine(State::A, machine());
            // GoC is not valid from State::A
            send.run(Event::GoC);
            assert_eq!(state.get_untracked(), State::A);
        });
    }

    #[test]
    fn multiple_transitions() {
        with_owner(|| {
            let (state, send) = use_state_machine(State::A, machine());
            send.run(Event::GoB);
            assert_eq!(state.get_untracked(), State::B);
            send.run(Event::GoC);
            assert_eq!(state.get_untracked(), State::C);
        });
    }

    #[test]
    fn event_for_different_state_is_noop() {
        with_owner(|| {
            let (state, send) = use_state_machine(State::B, machine());
            // GoB is only valid from State::A, not State::B
            send.run(Event::GoB);
            assert_eq!(state.get_untracked(), State::B);
        });
    }
}
