use super::{argument::ProvideArgument, *};

/// This trait is [sealed]
/// and only implemented for tuples (with max 4 elements).
///
/// ```
/// # use callable::Tuple;
/// # type A = u8; type B = String; type C = &'static str; type D = Vec<u8>;
/// # fn assert() where
/// (A, B, C, D): Tuple
/// # {} assert()
/// ```
///
/// Tuple with more than 4 elements doesn't implement [`Tuple`].
///
/// ```compile_fail
/// # use callable::Tuple;
/// # type A = u8; type B = String; type C = &'static str; type D = Vec<u8>;
/// # type E = i8;
/// # fn assert() where
/// (A, B, C, D, E): Tuple
/// # {}
/// ```
/// [sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
pub trait Tuple: sealed::Sealed {}

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

    fn provide_last_argument_refed<T>(
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

    fn provide_first_argument_refed<T>(
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

    fn provide_first_argument_copied<T: Copy>(
        self,
        first_argument: T,
    ) -> super::argument::FirstArgumentProvided<Self, argument::Copied<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<First = argument::Value<T>>,
    {
        self.provide_first_argument(argument::Copied(first_argument))
    }

    fn provide_last_argument_copied<T: Copy>(
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

    fn provide_last_argument_cloned<T: Clone>(
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

    fn provide_first_argument_cloned<T: Clone>(
        self,
        first_argument: T,
    ) -> super::argument::FirstArgumentProvided<Self, argument::Cloned<T>>
    where
        Self: Sized + CallableWithFixedArguments,
        Self::FixedArgumentTypes: argument::ArgumentTypes<First = argument::Value<T>>,
    {
        super::argument::FirstArgumentProvided {
            f: self,
            first_argument: argument::Cloned(first_argument),
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
/// # use callable::CallableOne;
/// # fn assert<IN, Out>() where
/// fn(IN) -> Out : CallableOne<IN, Output = Out>
/// # {} assert::<String, usize>()
/// ```
///
/// ```
/// # use callable::{CallableOne, callable};
/// # fn assert<IN, Out>() where
/// callable![fn(&IN) -> Out] : for<'input> CallableOne<&'input IN, Output = Out>
/// # {} assert::<String, usize>()
/// ```
///
/// Note that `fn(&IN) -> Out` doesn't implement `for<'input> CallableOne<&'input IN, Output = Out>`
/// due to limitations of higher kind types in rust.
pub trait Callable<Args: Tuple>: crate::IsCallable {
    type Output;

    fn call_fn(&self, args: Args) -> Self::Output;
}

/// A *trait alias* for [`Callable`] with exactly one argument.
pub trait CallableOne<IN>: Callable<(IN,)> {
    /// A shortcut for [`Callable::call_fn`] with exactly one argument.
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
    /// # use callable::prelude::*;
    /// let plus_1 = callable!(|v: i32| v + 1);
    /// let plus_2 = plus_1.reform(plus_1);
    /// let plus_4 = plus_2.reform(callable!(|v| v + 2));
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

    fn reform_mut<NewInput, F: for<'input> CallableOne<&'input mut NewInput, Output = IN>>(
        self,
        f: F,
    ) -> chain::Chain<F, Self>
    where
        Self: Sized,
    {
        chain::Chain(f, self)
    }
}

impl<A1, F: Callable<(A1,)>> crate::CallableOne<A1> for F {}

/// A [`Callable`] which has no overloading.
///
/// For example:
///
/// ```
/// # use callable::prelude::*;
/// # macro_rules! assert_impl { ($($where_clause:tt)+) => { const _: fn() = {
/// #    fn assert() where $($where_clause)+ {} || assert()
/// # }; } }
/// # assert_impl! {
/// fn() -> usize : CallableWithFixedArguments<FixedArgumentTypes = ArgumentTypes!(), Output = usize>,
/// fn(u8) -> i16 : CallableWithFixedArguments<FixedArgumentTypes = ArgumentTypes!(u8), Output = i16>,
/// callable![fn(&str) -> String] : CallableWithFixedArguments<FixedArgumentTypes = ArgumentTypes!(&str), Output = String>,
/// callable![fn(&str, &str) -> String] : CallableWithFixedArguments<FixedArgumentTypes = ArgumentTypes!(&str, &str), Output = String>,
/// # }
/// ```
pub trait CallableWithFixedArguments:
    IsCallable
    + for<'a> crate::Callable<super::argument::ArgumentsOfTypes<'a, Self::FixedArgumentTypes>>
{
    type FixedArgumentTypes: argument::ArgumentTypes;

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
