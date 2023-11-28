use crate::{Callable, IsCallable};

#[derive(Clone, Copy)]
pub enum NeverCalled {}

impl IsCallable for NeverCalled {}
impl<Args: crate::Tuple> Callable<Args> for NeverCalled {
    type Output = ();

    fn call_fn(&self, _: Args) -> Self::Output {
        match *self {}
    }
}

pub trait MaybeHandleEvent<E: ?Sized> {
    type State<S>;
    type Callable: for<'e> Callable<(&'e E,)>;

    fn initialize_handle_event_state<S>(
        this: Self,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    ) -> Self::State<S>;

    fn update_handle_event_state<S>(
        this: Self,
        state: &mut Self::State<S>,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    );

    /// When `Self::UpdateWithState<S>` is dropped or replaced with `Default::default`,
    /// the event listener should be unregistered.
    type UpdateWithState<S>: Default;

    fn update_with_state<S>(
        this: Self,
        state: &mut Self::UpdateWithState<S>,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    );
}

impl<E: ?Sized> MaybeHandleEvent<E> for () {
    type State<S> = ();
    type Callable = NeverCalled;

    fn initialize_handle_event_state<S>(
        _: Self,
        _: impl FnOnce(&Self::Callable) -> S,
    ) -> Self::State<S> {
    }

    fn update_handle_event_state<S>(
        _: Self,
        _: &mut Self::State<S>,
        _: impl FnOnce(&Self::Callable) -> S,
    ) {
    }

    type UpdateWithState<S> = ();

    fn update_with_state<S>(
        (): Self,
        (): &mut Self::UpdateWithState<S>,
        _: impl FnOnce(&Self::Callable) -> S,
    ) {
    }
}

impl<E: ?Sized, C: PartialEq + for<'e> Callable<(&'e E,)>> MaybeHandleEvent<E> for C {
    type State<S> = (S, C);
    type Callable = C;

    fn initialize_handle_event_state<S>(
        this: Self,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    ) -> Self::State<S> {
        (new_event_state(&this), this)
    }

    fn update_handle_event_state<S>(
        this: Self,
        state: &mut Self::State<S>,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    ) {
        if this != state.1 {
            *state = Self::initialize_handle_event_state(this, new_event_state)
        }
    }

    type UpdateWithState<S> = Option<(S, C)>;

    fn update_with_state<S>(
        this: Self,
        state: &mut Self::UpdateWithState<S>,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    ) {
        if let Some(state) = state {
            if state.1 == this {
                return;
            }
        }

        *state = Some(Self::initialize_handle_event_state(this, new_event_state));
    }
}

impl<E: ?Sized, M: MaybeHandleEvent<E>> MaybeHandleEvent<E> for Option<M> {
    type State<S> = Option<M::State<S>>;
    type Callable = M::Callable;

    fn initialize_handle_event_state<S>(
        this: Self,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    ) -> Self::State<S> {
        this.map(|this| M::initialize_handle_event_state(this, new_event_state))
    }

    fn update_handle_event_state<S>(
        this: Self,
        state: &mut Self::State<S>,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    ) {
        match (this, state) {
            (None, None) => {}
            (None, state @ Some(_)) => *state = None,
            (Some(this), Some(state)) => M::update_handle_event_state(this, state, new_event_state),
            (Some(this), state @ None) => {
                *state = Some(M::initialize_handle_event_state(this, new_event_state))
            }
        }
    }

    type UpdateWithState<S> = M::UpdateWithState<S>;

    fn update_with_state<S>(
        this: Self,
        state: &mut Self::UpdateWithState<S>,
        new_event_state: impl FnOnce(&Self::Callable) -> S,
    ) {
        if let Some(this) = this {
            M::update_with_state(this, state, new_event_state)
        } else {
            *state = Default::default();
        }
    }
}
