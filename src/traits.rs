use super::{argument::ProvideArgument, *};

pub trait IsCallable {
    fn accept_anything(self) -> accept_anything::AcceptAnything<Self>
    where
        Self: Sized + Callable<()>,
    {
        accept_anything::AcceptAnything(self)
    }

    fn provide_last_argument<
        A: ProvideArgument<
            ProvideArgumentType = <Self::FixedArgumentTypes as argument::ArgumentTypes>::Last,
        >,
    >(
        self,
        last_argument: A,
    ) -> super::argument::LastArgumentProvided<Self, A>
    where
        Self: Sized + CallableWithFixedArguments,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument,
        }
    }

    fn provide_first_argument<
        A: ProvideArgument<
            ProvideArgumentType = <Self::FixedArgumentTypes as argument::ArgumentTypes>::First,
        >,
    >(
        self,
        first_argument: A,
    ) -> super::argument::FirstArgumentProvided<Self, A>
    where
        Self: Sized + CallableWithFixedArguments,
    {
        super::argument::FirstArgumentProvided {
            f: self,
            first_argument,
        }
    }

    fn provide_last_argument_refed<T: 'static>(
        self,
        last_argument: T,
    ) -> super::argument::LastArgumentProvided<Self, argument::Refed<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<Last = argument::ByRef<T>>,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument: argument::Refed(last_argument),
        }
    }

    fn provide_first_argument_refed<T: 'static>(
        self,
        first_argument: T,
    ) -> super::argument::FirstArgumentProvided<Self, argument::Refed<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<First = argument::ByRef<T>>,
    {
        super::argument::FirstArgumentProvided {
            f: self,
            first_argument: argument::Refed(first_argument),
        }
    }

    fn provide_first_argument_copied<T: Copy + 'static>(
        self,
        first_argument: T,
    ) -> super::argument::FirstArgumentProvided<Self, argument::Copied<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<First = argument::Value<T>>,
    {
        self.provide_first_argument(argument::Copied(first_argument))
    }

    fn provide_last_argument_copied<T: Copy + 'static>(
        self,
        last_argument: T,
    ) -> super::argument::LastArgumentProvided<Self, argument::Copied<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<Last = argument::Value<T>>,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument: argument::Copied(last_argument),
        }
    }

    fn provide_last_argument_cloned<T: Clone + 'static>(
        self,
        last_argument: T,
    ) -> super::argument::LastArgumentProvided<Self, argument::Cloned<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<Last = argument::Value<T>>,
    {
        super::argument::LastArgumentProvided {
            f: self,
            last_argument: argument::Cloned(last_argument),
        }
    }
}

/// Anything implementing CallableOne has the following traits:
///
/// - Clone-able: impl [`Clone`]
/// - Comparable: impl [`PartialEq`]
/// - Marked as callable: impl [`IsCallableOne`]
/// - Callable: impl [`CallableOne<IN>`],
///   which has associated type [`Output`](CallableOne::Output)
///   and method [`emit()`](CallableOne::emit).
///
/// ## Why not closures?
///
/// Rust has support for closures. They automatically impl [`Fn`] traits.
/// And closures also automatically impl [`Clone`]
/// [if it only captured Clone-able data](https://doc.rust-lang.org/reference/types/closure.html#other-traits).
///
/// However, closure doesn't automatically impl [`PartialEq`].
///
/// ## Notable implementors:
///
/// ```
/// # use frender_events::CallableOne;
/// # fn assert<IN, Out>() where
/// fn(IN) -> Out : CallableOne<IN, Output = Out>
/// # {} assert::<String, usize>()
/// ```
///
/// ```
/// # use frender_events::{CallableOne, HkFn};
/// # fn assert<IN, Out>() where
/// HkFn<fn(&IN) -> Out> : for<'input> CallableOne<&'input IN, Output = Out>
/// # {} assert::<String, usize>()
/// ```
///
/// Note that `fn(&IN) -> Out` doesn't implement `for<'input> CallableOne<&'input IN, Output = Out>`
/// due to limitations of higher kind types in rust.
pub trait CallableOne<IN>: Callable<(IN,)> {
    // type Output;

    fn emit(&self, input: IN) -> Self::Output {
        self.call_fn((input,))
    }

    fn chain<F: CallableOne<Self::Output>>(self, f: F) -> chain::Chain<Self, F>
    where
        Self: Sized,
    {
        chain::Chain(self, f)
    }

    /// Provide input with another callable.
    ///
    /// ```
    /// # use frender_events::{callable, CallableOne, CallableOneExt};
    /// let plus_1 = callable(|v: i32| v + 1);
    /// let plus_2 = plus_1.reform(plus_1);
    /// let plus_4 = plus_2.reform(callable(|v| v + 2));
    ///
    /// assert_eq!(plus_1.emit(1), 2);
    /// assert_eq!(plus_2.emit(1), 3);
    /// assert_eq!(plus_4.emit(1), 5);
    /// ```
    fn reform<NewInput, F: CallableOne<NewInput, Output = IN>>(self, f: F) -> chain::Chain<F, Self>
    where
        Self: Sized,
    {
        f.chain(self)
    }

    fn reform_ref<NewInput, F: for<'i> CallableOne<&'i NewInput, Output = IN>>(
        self,
        f: F,
    ) -> chain::Chain<F, Self>
    where
        Self: Sized,
    {
        chain::Chain(f, self)
    }

    // fn reform_mut<NewInput, F: for<'input> CallableOne<&'input mut NewInput, Output = IN>>(
    //     self,
    //     f: F,
    // ) -> Chain<F, Self> {
    //     f.chain(self)
    // }
}

impl<A1, F: Callable<(A1,)>> crate::CallableOne<A1> for F {}

pub trait Callable<Args: sealed::Tuple>: crate::IsCallable {
    type Output;

    fn call_fn(&self, args: Args) -> Self::Output;
}

pub trait CallableWithFixedArguments:
    IsCallable
    + for<'a> crate::Callable<super::argument::ArgumentsOfTypes<'a, Self::FixedArgumentTypes>>
{
    type FixedArgumentTypes: argument::ArgumentTypes;
    // type LastArgumentProvided: CallableWithFixedArguments<
    //     FixedArgumentTypes = <Self::FixedArgumentTypes as argument::ArgumentTypes>::LastTrimmed,
    // >;

    fn call_with_prepended_args<'first: 'out, 'args: 'out, 'out>(
        &self,
        first: super::argument::ArgumentOfType<
            'first,
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::First,
        >,
        args: super::argument::ArgumentsOfTypes<
            'args,
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::FirstTrimmed,
        >,
    ) -> <Self as Callable<super::argument::ArgumentsOfTypes<'out, Self::FixedArgumentTypes>>>::Output
    {
        let first = <<Self::FixedArgumentTypes as argument::ArgumentTypes>::First as argument::ArgumentType>::re_borrow(first);
        let args = <<Self::FixedArgumentTypes as argument::ArgumentTypes>::FirstTrimmed as argument::ArgumentTypes>::re_borrow(args);
        self.call_fn(
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::from_prepended(first, args),
        )
    }

    fn call_with_appended_args<'last: 'out, 'args: 'out, 'out>(
        &self,
        args: super::argument::ArgumentsOfTypes<
            'args,
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::LastTrimmed,
        >,
        last: super::argument::ArgumentOfType<
            'last,
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::Last,
        >,
    ) -> <Self as Callable<super::argument::ArgumentsOfTypes<'out, Self::FixedArgumentTypes>>>::Output
    {
        let args = <<Self::FixedArgumentTypes as argument::ArgumentTypes>::LastTrimmed as argument::ArgumentTypes>::re_borrow(args);
        let last = <<Self::FixedArgumentTypes as argument::ArgumentTypes>::Last as argument::ArgumentType>::re_borrow(last);
        self.call_fn(
            <Self::FixedArgumentTypes as argument::ArgumentTypes>::from_appended(args, last),
        )
    }
}

impl<Out> IsCallable for fn() -> Out {}
impl<Out> CallableWithFixedArguments for fn() -> Out {
    type FixedArgumentTypes = ();
}
