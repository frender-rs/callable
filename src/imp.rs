// Recursive expansion of impl_with_macro_rules! macro
// ====================================================

use super::private::HkFn;
impl crate::sealed::Sealed for () {}

impl crate::Tuple for () {}

impl<'arg> crate::argument::Arguments<'arg> for () {
    type Arguments = ();
}
impl crate::argument::ArgumentTypes for () {
    type First = crate::argument::Invalid;
    type FirstTrimmed = crate::argument::InvalidTuple;
    type Last = crate::argument::Invalid;
    type LastTrimmed = crate::argument::InvalidTuple;
    fn re_borrow<'a: 'b, 'b>(
        (): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'b, Self> {
        ()
    }
}
impl<Arg: crate::argument::ArgumentType> crate::argument::PrependArgument<Arg> for () {
    type Prepended = (Arg,);
    fn prepend_argument<'a>(
        first: crate::argument::ArgumentOfType<'a, Arg>,
        (): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'a, Self::Prepended> {
        (first,)
    }
}
impl<Arg: crate::argument::ArgumentType> crate::argument::AppendArgument<Arg> for () {
    type Appended = (Arg,);
    fn append_argument<'a>(
        (): crate::argument::ArgumentsOfTypes<'a, Self>,
        last: crate::argument::ArgumentOfType<'a, Arg>,
    ) -> crate::argument::ArgumentsOfTypes<'a, Self::Appended> {
        (last,)
    }
}
impl<Out> crate::IsCallable for fn() -> Out {}

impl<Out> crate::Callable<()> for fn() -> Out {
    type Output = Out;
    fn call_fn(&self, (): ()) -> Self::Output {
        self()
    }
}
impl<Out> crate::CallableWithFixedArguments for fn() -> Out {
    type FixedArgumentTypes = ();
}
impl<A1> crate::sealed::Sealed for (A1,) {}

impl<A1> crate::Tuple for (A1,) {}

impl<'arg, A1: crate::argument::ArgumentType> crate::argument::Arguments<'arg> for (A1,) {
    type Arguments = (crate::argument::ArgumentOfType<'arg, A1>,);
}
impl<A1: crate::argument::ArgumentType> crate::argument::ArgumentTypes for (A1,) {
    type First = A1;
    type FirstTrimmed = ();
    type Last = A1;
    type LastTrimmed = ();
    fn re_borrow<'a: 'b, 'b>(
        (a1,): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'b, Self> {
        (A1::re_borrow(a1),)
    }
}
impl<Arg: crate::argument::ArgumentType, A1: crate::argument::ArgumentType>
    crate::argument::PrependArgument<Arg> for (A1,)
{
    type Prepended = (Arg, A1);
    fn prepend_argument<'a>(
        first: crate::argument::ArgumentOfType<'a, Arg>,
        (a1,): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'a, Self::Prepended> {
        (first, a1)
    }
}
impl<Arg: crate::argument::ArgumentType, A1: crate::argument::ArgumentType>
    crate::argument::AppendArgument<Arg> for (A1,)
{
    type Appended = (A1, Arg);
    fn append_argument<'a>(
        (a1,): crate::argument::ArgumentsOfTypes<'a, Self>,
        last: crate::argument::ArgumentOfType<'a, Arg>,
    ) -> crate::argument::ArgumentsOfTypes<'a, Self::Appended> {
        (a1, last)
    }
}
impl<A1, Out> crate::IsCallable for fn(A1) -> Out {}

impl<A1, Out> crate::Callable<(A1,)> for fn(A1) -> Out {
    type Output = Out;
    fn call_fn(&self, (a1,): (A1,)) -> Self::Output {
        self(a1)
    }
}
impl<A1, Out> crate::CallableWithFixedArguments for fn(A1) -> Out {
    type FixedArgumentTypes = (crate::argument::Value<A1>,);
}
impl<A1, A2> crate::sealed::Sealed for (A1, A2) {}

impl<A1, A2> crate::Tuple for (A1, A2) {}

impl<'arg, A1: crate::argument::ArgumentType, A2: crate::argument::ArgumentType>
    crate::argument::Arguments<'arg> for (A1, A2)
{
    type Arguments = (
        crate::argument::ArgumentOfType<'arg, A1>,
        crate::argument::ArgumentOfType<'arg, A2>,
    );
}
impl<A1: crate::argument::ArgumentType, A2: crate::argument::ArgumentType>
    crate::argument::ArgumentTypes for (A1, A2)
{
    type First = A1;
    type FirstTrimmed = (A2,);
    type Last = A2;
    type LastTrimmed = (A1,);
    fn re_borrow<'a: 'b, 'b>(
        (a1, a2): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'b, Self> {
        (A1::re_borrow(a1), A2::re_borrow(a2))
    }
}
impl<
        Arg: crate::argument::ArgumentType,
        A1: crate::argument::ArgumentType,
        A2: crate::argument::ArgumentType,
    > crate::argument::PrependArgument<Arg> for (A1, A2)
{
    type Prepended = (Arg, A1, A2);
    fn prepend_argument<'a>(
        first: crate::argument::ArgumentOfType<'a, Arg>,
        (a1, a2): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'a, Self::Prepended> {
        (first, a1, a2)
    }
}
impl<
        Arg: crate::argument::ArgumentType,
        A1: crate::argument::ArgumentType,
        A2: crate::argument::ArgumentType,
    > crate::argument::AppendArgument<Arg> for (A1, A2)
{
    type Appended = (A1, A2, Arg);
    fn append_argument<'a>(
        (a1, a2): crate::argument::ArgumentsOfTypes<'a, Self>,
        last: crate::argument::ArgumentOfType<'a, Arg>,
    ) -> crate::argument::ArgumentsOfTypes<'a, Self::Appended> {
        (a1, a2, last)
    }
}
impl<A1, A2, Out> crate::IsCallable for fn(A1, A2) -> Out {}

impl<A1, A2, Out> crate::Callable<(A1, A2)> for fn(A1, A2) -> Out {
    type Output = Out;
    fn call_fn(&self, (a1, a2): (A1, A2)) -> Self::Output {
        self(a1, a2)
    }
}
impl<A1, A2, Out> crate::CallableWithFixedArguments for fn(A1, A2) -> Out {
    type FixedArgumentTypes = (crate::argument::Value<A1>, crate::argument::Value<A2>);
}
impl<A1, A2, A3> crate::sealed::Sealed for (A1, A2, A3) {}

impl<A1, A2, A3> crate::Tuple for (A1, A2, A3) {}

impl<
        'arg,
        A1: crate::argument::ArgumentType,
        A2: crate::argument::ArgumentType,
        A3: crate::argument::ArgumentType,
    > crate::argument::Arguments<'arg> for (A1, A2, A3)
{
    type Arguments = (
        crate::argument::ArgumentOfType<'arg, A1>,
        crate::argument::ArgumentOfType<'arg, A2>,
        crate::argument::ArgumentOfType<'arg, A3>,
    );
}
impl<
        A1: crate::argument::ArgumentType,
        A2: crate::argument::ArgumentType,
        A3: crate::argument::ArgumentType,
    > crate::argument::ArgumentTypes for (A1, A2, A3)
{
    type First = A1;
    type FirstTrimmed = (A2, A3);
    type Last = A3;
    type LastTrimmed = (A1, A2);
    fn re_borrow<'a: 'b, 'b>(
        (a1, a2, a3): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'b, Self> {
        (A1::re_borrow(a1), A2::re_borrow(a2), A3::re_borrow(a3))
    }
}
impl<
        Arg: crate::argument::ArgumentType,
        A1: crate::argument::ArgumentType,
        A2: crate::argument::ArgumentType,
        A3: crate::argument::ArgumentType,
    > crate::argument::PrependArgument<Arg> for (A1, A2, A3)
{
    type Prepended = (Arg, A1, A2, A3);
    fn prepend_argument<'a>(
        first: crate::argument::ArgumentOfType<'a, Arg>,
        (a1, a2, a3): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'a, Self::Prepended> {
        (first, a1, a2, a3)
    }
}
impl<
        Arg: crate::argument::ArgumentType,
        A1: crate::argument::ArgumentType,
        A2: crate::argument::ArgumentType,
        A3: crate::argument::ArgumentType,
    > crate::argument::AppendArgument<Arg> for (A1, A2, A3)
{
    type Appended = (A1, A2, A3, Arg);
    fn append_argument<'a>(
        (a1, a2, a3): crate::argument::ArgumentsOfTypes<'a, Self>,
        last: crate::argument::ArgumentOfType<'a, Arg>,
    ) -> crate::argument::ArgumentsOfTypes<'a, Self::Appended> {
        (a1, a2, a3, last)
    }
}
impl<A1, A2, A3, Out> crate::IsCallable for fn(A1, A2, A3) -> Out {}

impl<A1, A2, A3, Out> crate::Callable<(A1, A2, A3)> for fn(A1, A2, A3) -> Out {
    type Output = Out;
    fn call_fn(&self, (a1, a2, a3): (A1, A2, A3)) -> Self::Output {
        self(a1, a2, a3)
    }
}
impl<A1, A2, A3, Out> crate::CallableWithFixedArguments for fn(A1, A2, A3) -> Out {
    type FixedArgumentTypes = (
        crate::argument::Value<A1>,
        crate::argument::Value<A2>,
        crate::argument::Value<A3>,
    );
}
impl<A1, A2, A3, A4> crate::sealed::Sealed for (A1, A2, A3, A4) {}

impl<A1, A2, A3, A4> crate::Tuple for (A1, A2, A3, A4) {}

impl<
        'arg,
        A1: crate::argument::ArgumentType,
        A2: crate::argument::ArgumentType,
        A3: crate::argument::ArgumentType,
        A4: crate::argument::ArgumentType,
    > crate::argument::Arguments<'arg> for (A1, A2, A3, A4)
{
    type Arguments = (
        crate::argument::ArgumentOfType<'arg, A1>,
        crate::argument::ArgumentOfType<'arg, A2>,
        crate::argument::ArgumentOfType<'arg, A3>,
        crate::argument::ArgumentOfType<'arg, A4>,
    );
}
impl<
        A1: crate::argument::ArgumentType,
        A2: crate::argument::ArgumentType,
        A3: crate::argument::ArgumentType,
        A4: crate::argument::ArgumentType,
    > crate::argument::ArgumentTypes for (A1, A2, A3, A4)
{
    type First = A1;
    type FirstTrimmed = (A2, A3, A4);
    type Last = A4;
    type LastTrimmed = (A1, A2, A3);
    fn re_borrow<'a: 'b, 'b>(
        (a1, a2, a3, a4): crate::argument::ArgumentsOfTypes<'a, Self>,
    ) -> crate::argument::ArgumentsOfTypes<'b, Self> {
        (
            A1::re_borrow(a1),
            A2::re_borrow(a2),
            A3::re_borrow(a3),
            A4::re_borrow(a4),
        )
    }
}
impl<A1, A2, A3, A4, Out> crate::IsCallable for fn(A1, A2, A3, A4) -> Out {}

impl<A1, A2, A3, A4, Out> crate::Callable<(A1, A2, A3, A4)> for fn(A1, A2, A3, A4) -> Out {
    type Output = Out;
    fn call_fn(&self, (a1, a2, a3, a4): (A1, A2, A3, A4)) -> Self::Output {
        self(a1, a2, a3, a4)
    }
}
impl<A1, A2, A3, A4, Out> crate::CallableWithFixedArguments for fn(A1, A2, A3, A4) -> Out {
    type FixedArgumentTypes = (
        crate::argument::Value<A1>,
        crate::argument::Value<A2>,
        crate::argument::Value<A3>,
        crate::argument::Value<A4>,
    );
}
pub use value::fn_pointer::FnPointer as value;
pub mod value {
    pub(super) mod fn_pointer {
        #[allow(unused_imports)]
        use super::super::*;
        #[allow(non_snake_case)]
        pub fn FnPointer<A1, Out>(f: fn(A1) -> Out) -> fn(A1) -> Out {
            f
        }
    }
    mod fn_pointer_type {
        #[allow(unused_imports)]
        use super::super::*;
        pub type FnPointer<A1, Out> = fn(A1) -> Out;
    }
    pub use fn_pointer::FnPointer;
    pub use fn_pointer_type::FnPointer;
    #[derive(Debug, Clone, Copy)]
    pub struct HkFn<F>(pub(super) F);

    pub use value::fn_pointer::FnPointer as value;
    pub mod value {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1, A2, Out>(f: fn(A1, A2) -> Out) -> fn(A1, A2) -> Out {
                f
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = fn(A1, A2) -> Out;
        }
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        #[derive(Debug, Clone, Copy)]
        pub struct HkFn<F>(pub(super) F);

        pub use value::fn_pointer::FnPointer as value;
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2, A3, Out>(
                    f: fn(A1, A2, A3) -> Out,
                ) -> fn(A1, A2, A3) -> Out {
                    f
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = fn(A1, A2, A3) -> Out;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3, A4, Out>(
                        f: fn(A1, A2, A3, A4) -> Out,
                    ) -> fn(A1, A2, A3, A4) -> Out {
                        f
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = fn(A1, A2, A3, A4) -> Out;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2, A3, A4: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, A2, A3, &A4) -> Out> {}

            impl<A1, A2, A3, A4: ?Sized, Out> crate::Callable<(A1, A2, A3, &A4)>
                for HkFn<fn(A1, A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3, A4: ?Sized, Out>(
                        f: fn(A1, A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2, A3, A4, Out>(
                        f: fn(A1, A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2, A3, A4: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, A2, A3, &mut A4) -> Out> {}

            impl<A1, A2, A3, A4: ?Sized, Out> crate::Callable<(A1, A2, A3, &mut A4)>
                for HkFn<fn(A1, A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, A3, &mut A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3, A4: ?Sized, Out>(
                        f: fn(A1, A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1, A2, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, A2, &A3) -> Out> {}

        impl<A1, A2, A3: ?Sized, Out> crate::Callable<(A1, A2, &A3)> for HkFn<fn(A1, A2, &A3) -> Out> {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (A1, A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2, A3: ?Sized, Out> crate::CallableWithFixedArguments for HkFn<fn(A1, A2, &A3) -> Out> {
            type FixedArgumentTypes = (
                crate::argument::Value<A1>,
                crate::argument::Value<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2, A3: ?Sized, Out>(
                    f: fn(A1, A2, &A3) -> Out,
                ) -> HkFn<fn(A1, A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(A1, A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1, A2, A3, Out>(
                    f: fn(A1, A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(A1, A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1, A2, A3: ?Sized, A4, Out> crate::IsCallable for value::HkFn<fn(A1, A2, &A3, A4) -> Out> {}

            impl<A1, A2, A3: ?Sized, A4, Out> crate::Callable<(A1, A2, &A3, A4)>
                for value::HkFn<fn(A1, A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(A1, A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1, A2, A3: ?Sized, A4, Out> PartialEq for value::HkFn<fn(A1, A2, &A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3: ?Sized, A4, Out>(
                        f: fn(A1, A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(A1, A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(A1, A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, A2, &A3, &A4) -> Out> {}

            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::Callable<(A1, A2, &A3, &A4)>
                for HkFn<fn(A1, A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, &A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2, A3: ?Sized, A4, Out>(
                        f: fn(A1, A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::Callable<(A1, A2, &A3, &mut A4)>
                for HkFn<fn(A1, A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, &A3, &mut A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1, A2, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, A2, &mut A3) -> Out> {}

        impl<A1, A2, A3: ?Sized, Out> crate::Callable<(A1, A2, &mut A3)>
            for HkFn<fn(A1, A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (A1, A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(A1, A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::Value<A1>,
                crate::argument::Value<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2, A3: ?Sized, Out>(
                    f: fn(A1, A2, &mut A3) -> Out,
                ) -> HkFn<fn(A1, A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(A1, A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1, A2, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(A1, A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1, A2, A3: ?Sized, A4, Out> crate::Callable<(A1, A2, &mut A3, A4)>
                for value::HkFn<fn(A1, A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(A1, A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1, A2, A3: ?Sized, A4, Out> PartialEq for value::HkFn<fn(A1, A2, &mut A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3: ?Sized, A4, Out>(
                        f: fn(A1, A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(A1, A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(A1, A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::Callable<(A1, A2, &mut A3, &A4)>
                for HkFn<fn(A1, A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, &mut A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2, A3: ?Sized, A4, Out>(
                        f: fn(A1, A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::Callable<(A1, A2, &mut A3, &mut A4)>
                for HkFn<fn(A1, A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, A2, &mut A3, &mut A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
    pub use r#ref::fn_pointer::FnPointer as r#ref;
    impl<A1, A2: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, &A2) -> Out> {}

    impl<A1, A2: ?Sized, Out> crate::Callable<(A1, &A2)> for HkFn<fn(A1, &A2) -> Out> {
        type Output = Out;
        fn call_fn(&self, #[allow(non_snake_case)] (A1, A2): (A1, &A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1, A2: ?Sized, Out> crate::CallableWithFixedArguments for HkFn<fn(A1, &A2) -> Out> {
        type FixedArgumentTypes = (crate::argument::Value<A1>, crate::argument::ByRef<A2>);
    }
    impl<A1, A2: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    pub mod r#ref {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1, A2: ?Sized, Out>(
                f: fn(A1, &A2) -> Out,
            ) -> HkFn<fn(A1, &A2) -> Out> {
                HkFn(f)
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = HkFn<fn(A1, &A2) -> Out>;
        }
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        mod provide_last_argument {
            use super::super::HkFn;
            pub fn provide_last_argument<A1, A2, Out>(
                f: fn(A1, &A2) -> Out,
                arg: A2,
            ) -> crate::argument::LastArgumentProvided<
                HkFn<fn(A1, &A2) -> Out>,
                crate::argument::Refed<A2>,
            > {
                use crate::IsCallable;
                super::super::r#ref(f).provide_last_argument_refed(arg)
            }
        }
        use super::HkFn;
        pub use provide_last_argument::provide_last_argument;
        pub use value::fn_pointer::FnPointer as value;
        impl<A1, A2: ?Sized, A3, Out> crate::IsCallable for value::HkFn<fn(A1, &A2, A3) -> Out> {}

        impl<A1, A2: ?Sized, A3, Out> crate::Callable<(A1, &A2, A3)>
            for value::HkFn<fn(A1, &A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (A1, &A2, A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3, Out> crate::CallableWithFixedArguments
            for value::HkFn<fn(A1, &A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::Value<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::Value<A3>,
            );
        }
        impl<A1, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(A1, &A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2: ?Sized, A3, Out>(
                    f: fn(A1, &A2, A3) -> Out,
                ) -> value::HkFn<fn(A1, &A2, A3) -> Out> {
                    value::HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = value::HkFn<fn(A1, &A2, A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            impl<A1, A2: ?Sized, A3, A4, Out> crate::IsCallable for value::HkFn<fn(A1, &A2, A3, A4) -> Out> {}

            impl<A1, A2: ?Sized, A3, A4, Out> crate::Callable<(A1, &A2, A3, A4)>
                for value::HkFn<fn(A1, &A2, A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(A1, &A2, A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3, A4, Out> PartialEq for value::HkFn<fn(A1, &A2, A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3, A4, Out>(
                        f: fn(A1, &A2, A3, A4) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(A1, &A2, A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, &A2, A3, &A4) -> Out> {}

            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::Callable<(A1, &A2, A3, &A4)>
                for HkFn<fn(A1, &A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2, A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(A1, &A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, &A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, &A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2: ?Sized, A3, A4, Out>(
                        f: fn(A1, &A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, &A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &A2, A3, &mut A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::Callable<(A1, &A2, A3, &mut A4)>
                for HkFn<fn(A1, &A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2, A3, &mut A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(A1, &A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, &A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, &A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, &A2, &A3) -> Out> {}

        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(A1, &A2, &A3)>
            for HkFn<fn(A1, &A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (A1, &A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(A1, &A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::Value<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(A1, &A2, &A3) -> Out,
                ) -> HkFn<fn(A1, &A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(A1, &A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1, A2: ?Sized, A3, Out>(
                    f: fn(A1, &A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(A1, &A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(A1, &A2, &A3, A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::Callable<(A1, &A2, &A3, A4)>
                for value::HkFn<fn(A1, &A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(A1, &A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq for value::HkFn<fn(A1, &A2, &A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(A1, &A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(A1, &A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &A2, &A3, &A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::Callable<(A1, &A2, &A3, &A4)>
                for HkFn<fn(A1, &A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2, &A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, &A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, &A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, &A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(A1, &A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, &A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(A1, &A2, &A3, &mut A4)>
                for HkFn<fn(A1, &A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(A1, &A2, &A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, &A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, &A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, &A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, &A2, &mut A3) -> Out> {}

        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(A1, &A2, &mut A3)>
            for HkFn<fn(A1, &A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (A1, &A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(A1, &A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::Value<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, &A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(A1, &A2, &mut A3) -> Out,
                ) -> HkFn<fn(A1, &A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(A1, &A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(A1, &A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::Callable<(A1, &A2, &mut A3, A4)>
                for value::HkFn<fn(A1, &A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(A1, &A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(A1, &A2, &mut A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(A1, &A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(A1, &A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(A1, &A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(A1, &A2, &mut A3, &A4)>
                for HkFn<fn(A1, &A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(A1, &A2, &mut A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, &A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, &A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, &A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(A1, &A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, &A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(A1, &A2, &mut A3, &mut A4)>
                for HkFn<fn(A1, &A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(A1, &A2, &mut A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, &A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, &A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, &A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
    pub use r#mut::fn_pointer::FnPointer as r#mut;
    impl<A1, A2: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, &mut A2) -> Out> {}

    impl<A1, A2: ?Sized, Out> crate::Callable<(A1, &mut A2)> for HkFn<fn(A1, &mut A2) -> Out> {
        type Output = Out;
        fn call_fn(&self, #[allow(non_snake_case)] (A1, A2): (A1, &mut A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1, A2: ?Sized, Out> crate::CallableWithFixedArguments for HkFn<fn(A1, &mut A2) -> Out> {
        type FixedArgumentTypes = (crate::argument::Value<A1>, crate::argument::ByMut<A2>);
    }
    impl<A1, A2: ?Sized, Out> PartialEq for HkFn<fn(A1, &mut A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    pub mod r#mut {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1, A2: ?Sized, Out>(
                f: fn(A1, &mut A2) -> Out,
            ) -> HkFn<fn(A1, &mut A2) -> Out> {
                HkFn(f)
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = HkFn<fn(A1, &mut A2) -> Out>;
        }
        use super::HkFn;
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        pub use value::fn_pointer::FnPointer as value;
        impl<A1, A2: ?Sized, A3, Out> crate::IsCallable for value::HkFn<fn(A1, &mut A2, A3) -> Out> {}

        impl<A1, A2: ?Sized, A3, Out> crate::Callable<(A1, &mut A2, A3)>
            for value::HkFn<fn(A1, &mut A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (A1, &mut A2, A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3, Out> crate::CallableWithFixedArguments
            for value::HkFn<fn(A1, &mut A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::Value<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::Value<A3>,
            );
        }
        impl<A1, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(A1, &mut A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2: ?Sized, A3, Out>(
                    f: fn(A1, &mut A2, A3) -> Out,
                ) -> value::HkFn<fn(A1, &mut A2, A3) -> Out> {
                    value::HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = value::HkFn<fn(A1, &mut A2, A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            impl<A1, A2: ?Sized, A3, A4, Out> crate::IsCallable
                for value::HkFn<fn(A1, &mut A2, A3, A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3, A4, Out> crate::Callable<(A1, &mut A2, A3, A4)>
                for value::HkFn<fn(A1, &mut A2, A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(A1, &mut A2, A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3, A4, Out> PartialEq for value::HkFn<fn(A1, &mut A2, A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3, A4, Out>(
                        f: fn(A1, &mut A2, A3, A4) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(A1, &mut A2, A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &mut A2, A3, &A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::Callable<(A1, &mut A2, A3, &A4)>
                for HkFn<fn(A1, &mut A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &mut A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, &mut A2, A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(A1, &mut A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(A1, &mut A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2: ?Sized, A3, A4, Out>(
                        f: fn(A1, &mut A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, &mut A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &mut A2, A3, &mut A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::Callable<(A1, &mut A2, A3, &mut A4)>
                for HkFn<fn(A1, &mut A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &mut A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(A1, &mut A2, A3, &mut A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(A1, &mut A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, &mut A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, &mut A2, &A3) -> Out> {}

        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(A1, &mut A2, &A3)>
            for HkFn<fn(A1, &mut A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (A1, &mut A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(A1, &mut A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::Value<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, &mut A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(A1, &mut A2, &A3) -> Out,
                ) -> HkFn<fn(A1, &mut A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(A1, &mut A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1, A2: ?Sized, A3, Out>(
                    f: fn(A1, &mut A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(A1, &mut A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(A1, &mut A2, &A3, A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::Callable<(A1, &mut A2, &A3, A4)>
                for value::HkFn<fn(A1, &mut A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(A1, &mut A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(A1, &mut A2, &A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(A1, &mut A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(A1, &mut A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &mut A2, &A3, &A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(A1, &mut A2, &A3, &A4)>
                for HkFn<fn(A1, &mut A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &mut A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(A1, &mut A2, &A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, &mut A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, &mut A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(A1, &mut A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, &mut A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &mut A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(A1, &mut A2, &A3, &mut A4)>
                for HkFn<fn(A1, &mut A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &mut A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(A1, &mut A2, &A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, &mut A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, &mut A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(A1, &mut A2, &mut A3) -> Out> {}

        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(A1, &mut A2, &mut A3)>
            for HkFn<fn(A1, &mut A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (A1, &mut A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(A1, &mut A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::Value<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(A1, &mut A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(A1, &mut A2, &mut A3) -> Out,
                ) -> HkFn<fn(A1, &mut A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(A1, &mut A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(A1, &mut A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::Callable<(A1, &mut A2, &mut A3, A4)>
                for value::HkFn<fn(A1, &mut A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(A1, &mut A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(A1, &mut A2, &mut A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(A1, &mut A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(A1, &mut A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(A1, &mut A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(A1, &mut A2, &mut A3, &A4)>
                for HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, &mut A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(A1, &mut A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(A1, &mut A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(A1, &mut A2, &mut A3, &mut A4)>
                for HkFn<fn(A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (A1, &mut A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::Value<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(A1, &mut A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(A1, &mut A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(A1, &mut A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
}
pub use r#ref::fn_pointer::FnPointer as r#ref;
impl<A1: ?Sized, Out> crate::IsCallable for HkFn<fn(&A1) -> Out> {}

impl<A1: ?Sized, Out> crate::Callable<(&A1,)> for HkFn<fn(&A1) -> Out> {
    type Output = Out;
    fn call_fn(&self, #[allow(non_snake_case)] (A1,): (&A1,)) -> Self::Output {
        self.0(A1)
    }
}
impl<A1: ?Sized, Out> crate::CallableWithFixedArguments for HkFn<fn(&A1) -> Out> {
    type FixedArgumentTypes = (crate::argument::ByRef<A1>,);
}
impl<A1: ?Sized, Out> PartialEq for HkFn<fn(&A1) -> Out> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}
pub mod r#ref {
    pub(super) mod fn_pointer {
        #[allow(unused_imports)]
        use super::super::*;
        #[allow(non_snake_case)]
        pub fn FnPointer<A1: ?Sized, Out>(f: fn(&A1) -> Out) -> HkFn<fn(&A1) -> Out> {
            HkFn(f)
        }
    }
    mod fn_pointer_type {
        #[allow(unused_imports)]
        use super::super::*;
        pub type FnPointer<A1, Out> = HkFn<fn(&A1) -> Out>;
    }
    pub use fn_pointer::FnPointer;
    pub use fn_pointer_type::FnPointer;
    mod provide_last_argument {
        use super::super::HkFn;
        pub fn provide_last_argument<A1, Out>(
            f: fn(&A1) -> Out,
            arg: A1,
        ) -> crate::argument::LastArgumentProvided<HkFn<fn(&A1) -> Out>, crate::argument::Refed<A1>>
        {
            use crate::IsCallable;
            super::super::r#ref(f).provide_last_argument_refed(arg)
        }
    }
    use super::HkFn;
    pub use provide_last_argument::provide_last_argument;
    pub use value::fn_pointer::FnPointer as value;
    impl<A1: ?Sized, A2, Out> crate::IsCallable for value::HkFn<fn(&A1, A2) -> Out> {}

    impl<A1: ?Sized, A2, Out> crate::Callable<(&A1, A2)> for value::HkFn<fn(&A1, A2) -> Out> {
        type Output = Out;
        fn call_fn(&self, #[allow(non_snake_case)] (A1, A2): (&A1, A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2, Out> crate::CallableWithFixedArguments for value::HkFn<fn(&A1, A2) -> Out> {
        type FixedArgumentTypes = (crate::argument::ByRef<A1>, crate::argument::Value<A2>);
    }
    impl<A1: ?Sized, A2, Out> PartialEq for value::HkFn<fn(&A1, A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    pub mod value {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1: ?Sized, A2, Out>(
                f: fn(&A1, A2) -> Out,
            ) -> value::HkFn<fn(&A1, A2) -> Out> {
                value::HkFn(f)
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = value::HkFn<fn(&A1, A2) -> Out>;
        }
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        #[derive(Debug, Clone, Copy)]
        pub struct HkFn<F>(pub(super) F);

        pub use value::fn_pointer::FnPointer as value;
        impl<A1: ?Sized, A2, A3, Out> crate::IsCallable for value::HkFn<fn(&A1, A2, A3) -> Out> {}

        impl<A1: ?Sized, A2, A3, Out> crate::Callable<(&A1, A2, A3)>
            for value::HkFn<fn(&A1, A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, A2, A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3, Out> crate::CallableWithFixedArguments
            for value::HkFn<fn(&A1, A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::Value<A2>,
                crate::argument::Value<A3>,
            );
        }
        impl<A1: ?Sized, A2, A3, Out> PartialEq for value::HkFn<fn(&A1, A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2, A3, Out>(
                    f: fn(&A1, A2, A3) -> Out,
                ) -> value::HkFn<fn(&A1, A2, A3) -> Out> {
                    value::HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = value::HkFn<fn(&A1, A2, A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2, A3, A4, Out> crate::IsCallable for value::HkFn<fn(&A1, A2, A3, A4) -> Out> {}

            impl<A1: ?Sized, A2, A3, A4, Out> crate::Callable<(&A1, A2, A3, A4)>
                for value::HkFn<fn(&A1, A2, A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, A2, A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3, A4, Out> PartialEq for value::HkFn<fn(&A1, A2, A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3, A4, Out>(
                        f: fn(&A1, A2, A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, A2, A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::IsCallable for HkFn<fn(&A1, A2, A3, &A4) -> Out> {}

            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::Callable<(&A1, A2, A3, &A4)>
                for HkFn<fn(&A1, A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(&A1, A2, A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3, A4: ?Sized, Out>(
                        f: fn(&A1, A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(&A1, A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2, A3, A4, Out>(
                        f: fn(&A1, A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, A2, A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::Callable<(&A1, A2, A3, &mut A4)>
                for HkFn<fn(&A1, A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(&A1, A2, A3, &mut A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3, A4: ?Sized, Out>(
                        f: fn(&A1, A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(&A1, A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(&A1, A2, &A3) -> Out> {}

        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::Callable<(&A1, A2, &A3)>
            for HkFn<fn(&A1, A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&A1, A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::Value<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, Out>(
                    f: fn(&A1, A2, &A3) -> Out,
                ) -> HkFn<fn(&A1, A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&A1, A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2, A3, Out>(
                    f: fn(&A1, A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(&A1, A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&A1, A2, &A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::Callable<(&A1, A2, &A3, A4)>
                for value::HkFn<fn(&A1, A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> PartialEq for value::HkFn<fn(&A1, A2, &A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4, Out>(
                        f: fn(&A1, A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, A2, &A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::Callable<(&A1, A2, &A3, &A4)>
                for HkFn<fn(&A1, A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq for HkFn<fn(&A1, A2, &A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(&A1, A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2, A3: ?Sized, A4, Out>(
                        f: fn(&A1, A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, A2, &A3, &mut A4)>
                for HkFn<fn(&A1, A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, A2, &A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(&A1, A2, &mut A3) -> Out> {}

        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::Callable<(&A1, A2, &mut A3)>
            for HkFn<fn(&A1, A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&A1, A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::Value<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, Out>(
                    f: fn(&A1, A2, &mut A3) -> Out,
                ) -> HkFn<fn(&A1, A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&A1, A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&A1, A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::Callable<(&A1, A2, &mut A3, A4)>
                for value::HkFn<fn(&A1, A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&A1, A2, &mut A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4, Out>(
                        f: fn(&A1, A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, A2, &mut A3, &A4)>
                for HkFn<fn(&A1, A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, A2, &mut A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2, A3: ?Sized, A4, Out>(
                        f: fn(&A1, A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, A2, &mut A3, &mut A4)>
                for HkFn<fn(&A1, A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, A2, &mut A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
    pub use r#ref::fn_pointer::FnPointer as r#ref;
    impl<A1: ?Sized, A2: ?Sized, Out> crate::IsCallable for HkFn<fn(&A1, &A2) -> Out> {}

    impl<A1: ?Sized, A2: ?Sized, Out> crate::Callable<(&A1, &A2)> for HkFn<fn(&A1, &A2) -> Out> {
        type Output = Out;
        fn call_fn(&self, #[allow(non_snake_case)] (A1, A2): (&A1, &A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::CallableWithFixedArguments for HkFn<fn(&A1, &A2) -> Out> {
        type FixedArgumentTypes = (crate::argument::ByRef<A1>, crate::argument::ByRef<A2>);
    }
    impl<A1: ?Sized, A2: ?Sized, Out> PartialEq for HkFn<fn(&A1, &A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    pub mod r#ref {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1: ?Sized, A2: ?Sized, Out>(
                f: fn(&A1, &A2) -> Out,
            ) -> HkFn<fn(&A1, &A2) -> Out> {
                HkFn(f)
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = HkFn<fn(&A1, &A2) -> Out>;
        }
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        mod provide_last_argument {
            use super::super::HkFn;
            pub fn provide_last_argument<A1: ?Sized, A2, Out>(
                f: fn(&A1, &A2) -> Out,
                arg: A2,
            ) -> crate::argument::LastArgumentProvided<
                HkFn<fn(&A1, &A2) -> Out>,
                crate::argument::Refed<A2>,
            > {
                use crate::IsCallable;
                super::super::r#ref(f).provide_last_argument_refed(arg)
            }
        }
        use super::HkFn;
        pub use provide_last_argument::provide_last_argument;
        pub use value::fn_pointer::FnPointer as value;
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::IsCallable for value::HkFn<fn(&A1, &A2, A3) -> Out> {}

        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::Callable<(&A1, &A2, A3)>
            for value::HkFn<fn(&A1, &A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, &A2, A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::CallableWithFixedArguments
            for value::HkFn<fn(&A1, &A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::Value<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(&A1, &A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&A1, &A2, A3) -> Out,
                ) -> value::HkFn<fn(&A1, &A2, A3) -> Out> {
                    value::HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = value::HkFn<fn(&A1, &A2, A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::IsCallable
                for value::HkFn<fn(&A1, &A2, A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::Callable<(&A1, &A2, A3, A4)>
                for value::HkFn<fn(&A1, &A2, A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, &A2, A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> PartialEq for value::HkFn<fn(&A1, &A2, A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4, Out>(
                        f: fn(&A1, &A2, A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, &A2, A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &A2, A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::Callable<(&A1, &A2, A3, &A4)>
                for HkFn<fn(&A1, &A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, &A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(&A1, &A2, A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(&A1, &A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, &A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(&A1, &A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, A4, Out>(
                        f: fn(&A1, &A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, &A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &A2, A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>
                crate::Callable<(&A1, &A2, A3, &mut A4)>
                for HkFn<fn(&A1, &A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, &A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &A2, A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(&A1, &A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, &A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(&A1, &A2, &A3) -> Out> {}

        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(&A1, &A2, &A3)>
            for HkFn<fn(&A1, &A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, &A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&A1, &A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, &A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(&A1, &A2, &A3) -> Out,
                ) -> HkFn<fn(&A1, &A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&A1, &A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&A1, &A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(&A1, &A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&A1, &A2, &A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::Callable<(&A1, &A2, &A3, A4)>
                for value::HkFn<fn(&A1, &A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, &A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&A1, &A2, &A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&A1, &A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, &A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &A2, &A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, &A2, &A3, &A4)> for HkFn<fn(&A1, &A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&A1, &A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &A2, &A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, &A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(&A1, &A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&A1, &A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, &A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, &A2, &A3, &mut A4)>
                for HkFn<fn(&A1, &A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&A1, &A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &A2, &A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, &A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable
            for HkFn<fn(&A1, &A2, &mut A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(&A1, &A2, &mut A3)>
            for HkFn<fn(&A1, &A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, &A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&A1, &A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, &A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(&A1, &A2, &mut A3) -> Out,
                ) -> HkFn<fn(&A1, &A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&A1, &A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&A1, &A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>
                crate::Callable<(&A1, &A2, &mut A3, A4)>
                for value::HkFn<fn(&A1, &A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, &A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&A1, &A2, &mut A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&A1, &A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, &A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, &A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, &A2, &mut A3, &A4)>
                for HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, &A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&A1, &A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, &A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, &A2, &mut A3, &mut A4)>
                for HkFn<fn(&A1, &A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&A1, &A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &A2, &mut A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, &A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, &A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
    pub use r#mut::fn_pointer::FnPointer as r#mut;
    impl<A1: ?Sized, A2: ?Sized, Out> crate::IsCallable for HkFn<fn(&A1, &mut A2) -> Out> {}

    impl<A1: ?Sized, A2: ?Sized, Out> crate::Callable<(&A1, &mut A2)>
        for HkFn<fn(&A1, &mut A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, #[allow(non_snake_case)] (A1, A2): (&A1, &mut A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::CallableWithFixedArguments
        for HkFn<fn(&A1, &mut A2) -> Out>
    {
        type FixedArgumentTypes = (crate::argument::ByRef<A1>, crate::argument::ByMut<A2>);
    }
    impl<A1: ?Sized, A2: ?Sized, Out> PartialEq for HkFn<fn(&A1, &mut A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    pub mod r#mut {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1: ?Sized, A2: ?Sized, Out>(
                f: fn(&A1, &mut A2) -> Out,
            ) -> HkFn<fn(&A1, &mut A2) -> Out> {
                HkFn(f)
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = HkFn<fn(&A1, &mut A2) -> Out>;
        }
        use super::HkFn;
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        pub use value::fn_pointer::FnPointer as value;
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::IsCallable
            for value::HkFn<fn(&A1, &mut A2, A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::Callable<(&A1, &mut A2, A3)>
            for value::HkFn<fn(&A1, &mut A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, &mut A2, A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::CallableWithFixedArguments
            for value::HkFn<fn(&A1, &mut A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::Value<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(&A1, &mut A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&A1, &mut A2, A3) -> Out,
                ) -> value::HkFn<fn(&A1, &mut A2, A3) -> Out> {
                    value::HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = value::HkFn<fn(&A1, &mut A2, A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::IsCallable
                for value::HkFn<fn(&A1, &mut A2, A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::Callable<(&A1, &mut A2, A3, A4)>
                for value::HkFn<fn(&A1, &mut A2, A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, &mut A2, A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> PartialEq
                for value::HkFn<fn(&A1, &mut A2, A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4, Out>(
                        f: fn(&A1, &mut A2, A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, &mut A2, A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &mut A2, A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>
                crate::Callable<(&A1, &mut A2, A3, &A4)>
                for HkFn<fn(&A1, &mut A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, &mut A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &mut A2, A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(&A1, &mut A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &mut A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, A4, Out>(
                        f: fn(&A1, &mut A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, &mut A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &mut A2, A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>
                crate::Callable<(&A1, &mut A2, A3, &mut A4)>
                for HkFn<fn(&A1, &mut A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&A1, &mut A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &mut A2, A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(&A1, &mut A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &mut A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable
            for HkFn<fn(&A1, &mut A2, &A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(&A1, &mut A2, &A3)>
            for HkFn<fn(&A1, &mut A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, &mut A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&A1, &mut A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, &mut A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(&A1, &mut A2, &A3) -> Out,
                ) -> HkFn<fn(&A1, &mut A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&A1, &mut A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&A1, &mut A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(&A1, &mut A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&A1, &mut A2, &A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>
                crate::Callable<(&A1, &mut A2, &A3, A4)>
                for value::HkFn<fn(&A1, &mut A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, &mut A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&A1, &mut A2, &A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&A1, &mut A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, &mut A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, &mut A2, &A3, &A4)>
                for HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, &mut A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&A1, &mut A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, &mut A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &mut A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, &mut A2, &A3, &mut A4)>
                for HkFn<fn(&A1, &mut A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&A1, &mut A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &mut A2, &A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, &mut A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &mut A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable
            for HkFn<fn(&A1, &mut A2, &mut A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(&A1, &mut A2, &mut A3)>
            for HkFn<fn(&A1, &mut A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&A1, &mut A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&A1, &mut A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByRef<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&A1, &mut A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(&A1, &mut A2, &mut A3) -> Out,
                ) -> HkFn<fn(&A1, &mut A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&A1, &mut A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&A1, &mut A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>
                crate::Callable<(&A1, &mut A2, &mut A3, A4)>
                for value::HkFn<fn(&A1, &mut A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&A1, &mut A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&A1, &mut A2, &mut A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&A1, &mut A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(&A1, &mut A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&A1, &mut A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, &mut A2, &mut A3, &A4)>
                for HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&A1, &mut A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&A1, &mut A2, &mut A3, &mut A4)>
                for HkFn<fn(&A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&A1, &mut A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments
                for HkFn<fn(&A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByRef<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&A1, &mut A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&A1, &mut A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&A1, &mut A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
}
pub use r#mut::fn_pointer::FnPointer as r#mut;
impl<A1: ?Sized, Out> crate::IsCallable for HkFn<fn(&mut A1) -> Out> {}

impl<A1: ?Sized, Out> crate::Callable<(&mut A1,)> for HkFn<fn(&mut A1) -> Out> {
    type Output = Out;
    fn call_fn(&self, #[allow(non_snake_case)] (A1,): (&mut A1,)) -> Self::Output {
        self.0(A1)
    }
}
impl<A1: ?Sized, Out> crate::CallableWithFixedArguments for HkFn<fn(&mut A1) -> Out> {
    type FixedArgumentTypes = (crate::argument::ByMut<A1>,);
}
impl<A1: ?Sized, Out> PartialEq for HkFn<fn(&mut A1) -> Out> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}
pub mod r#mut {
    pub(super) mod fn_pointer {
        #[allow(unused_imports)]
        use super::super::*;
        #[allow(non_snake_case)]
        pub fn FnPointer<A1: ?Sized, Out>(f: fn(&mut A1) -> Out) -> HkFn<fn(&mut A1) -> Out> {
            HkFn(f)
        }
    }
    mod fn_pointer_type {
        #[allow(unused_imports)]
        use super::super::*;
        pub type FnPointer<A1, Out> = HkFn<fn(&mut A1) -> Out>;
    }
    use super::HkFn;
    pub use fn_pointer::FnPointer;
    pub use fn_pointer_type::FnPointer;
    pub use value::fn_pointer::FnPointer as value;
    impl<A1: ?Sized, A2, Out> crate::IsCallable for value::HkFn<fn(&mut A1, A2) -> Out> {}

    impl<A1: ?Sized, A2, Out> crate::Callable<(&mut A1, A2)> for value::HkFn<fn(&mut A1, A2) -> Out> {
        type Output = Out;
        fn call_fn(&self, #[allow(non_snake_case)] (A1, A2): (&mut A1, A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2, Out> crate::CallableWithFixedArguments
        for value::HkFn<fn(&mut A1, A2) -> Out>
    {
        type FixedArgumentTypes = (crate::argument::ByMut<A1>, crate::argument::Value<A2>);
    }
    impl<A1: ?Sized, A2, Out> PartialEq for value::HkFn<fn(&mut A1, A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    pub mod value {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1: ?Sized, A2, Out>(
                f: fn(&mut A1, A2) -> Out,
            ) -> value::HkFn<fn(&mut A1, A2) -> Out> {
                value::HkFn(f)
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = value::HkFn<fn(&mut A1, A2) -> Out>;
        }
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        #[derive(Debug, Clone, Copy)]
        pub struct HkFn<F>(pub(super) F);

        pub use value::fn_pointer::FnPointer as value;
        impl<A1: ?Sized, A2, A3, Out> crate::IsCallable for value::HkFn<fn(&mut A1, A2, A3) -> Out> {}

        impl<A1: ?Sized, A2, A3, Out> crate::Callable<(&mut A1, A2, A3)>
            for value::HkFn<fn(&mut A1, A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, A2, A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3, Out> crate::CallableWithFixedArguments
            for value::HkFn<fn(&mut A1, A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::Value<A2>,
                crate::argument::Value<A3>,
            );
        }
        impl<A1: ?Sized, A2, A3, Out> PartialEq for value::HkFn<fn(&mut A1, A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2, A3, Out>(
                    f: fn(&mut A1, A2, A3) -> Out,
                ) -> value::HkFn<fn(&mut A1, A2, A3) -> Out> {
                    value::HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = value::HkFn<fn(&mut A1, A2, A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2, A3, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, A2, A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3, A4, Out> crate::Callable<(&mut A1, A2, A3, A4)>
                for value::HkFn<fn(&mut A1, A2, A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, A2, A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3, A4, Out> PartialEq for value::HkFn<fn(&mut A1, A2, A3, A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3, A4, Out>(
                        f: fn(&mut A1, A2, A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, A2, A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, A2, A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::Callable<(&mut A1, A2, A3, &A4)>
                for HkFn<fn(&mut A1, A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, A2, A3, &A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3, A4: ?Sized, Out>(
                        f: fn(&mut A1, A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> = HkFn<fn(&mut A1, A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2, A3, A4, Out>(
                        f: fn(&mut A1, A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, A2, A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::Callable<(&mut A1, A2, A3, &mut A4)>
                for HkFn<fn(&mut A1, A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3, A4: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, A2, A3, &mut A4) -> Out> {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3, A4: ?Sized, Out>(
                        f: fn(&mut A1, A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(&mut A1, A2, &A3) -> Out> {}

        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::Callable<(&mut A1, A2, &A3)>
            for HkFn<fn(&mut A1, A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&mut A1, A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::Value<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, Out>(
                    f: fn(&mut A1, A2, &A3) -> Out,
                ) -> HkFn<fn(&mut A1, A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&mut A1, A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2, A3, Out>(
                    f: fn(&mut A1, A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(&mut A1, A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, A2, &A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::Callable<(&mut A1, A2, &A3, A4)>
                for value::HkFn<fn(&mut A1, A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&mut A1, A2, &A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, A2, &A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, A2, &A3, &A4)>
                for HkFn<fn(&mut A1, A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, A2, &A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, A2, &A3, &mut A4)>
                for HkFn<fn(&mut A1, A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, A2, &A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::IsCallable for HkFn<fn(&mut A1, A2, &mut A3) -> Out> {}

        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::Callable<(&mut A1, A2, &mut A3)>
            for HkFn<fn(&mut A1, A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&mut A1, A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::Value<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1: ?Sized, A2, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, Out>(
                    f: fn(&mut A1, A2, &mut A3) -> Out,
                ) -> HkFn<fn(&mut A1, A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&mut A1, A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::Callable<(&mut A1, A2, &mut A3, A4)>
                for value::HkFn<fn(&mut A1, A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&mut A1, A2, &mut A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, A2, &mut A3, &A4)>
                for HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, A2, &mut A3, &mut A4)>
                for HkFn<fn(&mut A1, A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::Value<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, A2, &mut A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
    pub use r#ref::fn_pointer::FnPointer as r#ref;
    impl<A1: ?Sized, A2: ?Sized, Out> crate::IsCallable for HkFn<fn(&mut A1, &A2) -> Out> {}

    impl<A1: ?Sized, A2: ?Sized, Out> crate::Callable<(&mut A1, &A2)>
        for HkFn<fn(&mut A1, &A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, #[allow(non_snake_case)] (A1, A2): (&mut A1, &A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::CallableWithFixedArguments
        for HkFn<fn(&mut A1, &A2) -> Out>
    {
        type FixedArgumentTypes = (crate::argument::ByMut<A1>, crate::argument::ByRef<A2>);
    }
    impl<A1: ?Sized, A2: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    pub mod r#ref {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1: ?Sized, A2: ?Sized, Out>(
                f: fn(&mut A1, &A2) -> Out,
            ) -> HkFn<fn(&mut A1, &A2) -> Out> {
                HkFn(f)
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = HkFn<fn(&mut A1, &A2) -> Out>;
        }
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        mod provide_last_argument {
            use super::super::HkFn;
            pub fn provide_last_argument<A1: ?Sized, A2, Out>(
                f: fn(&mut A1, &A2) -> Out,
                arg: A2,
            ) -> crate::argument::LastArgumentProvided<
                HkFn<fn(&mut A1, &A2) -> Out>,
                crate::argument::Refed<A2>,
            > {
                use crate::IsCallable;
                super::super::r#ref(f).provide_last_argument_refed(arg)
            }
        }
        use super::HkFn;
        pub use provide_last_argument::provide_last_argument;
        pub use value::fn_pointer::FnPointer as value;
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::IsCallable
            for value::HkFn<fn(&mut A1, &A2, A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::Callable<(&mut A1, &A2, A3)>
            for value::HkFn<fn(&mut A1, &A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, &A2, A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::CallableWithFixedArguments
            for value::HkFn<fn(&mut A1, &A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::Value<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(&mut A1, &A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&mut A1, &A2, A3) -> Out,
                ) -> value::HkFn<fn(&mut A1, &A2, A3) -> Out> {
                    value::HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = value::HkFn<fn(&mut A1, &A2, A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, &A2, A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::Callable<(&mut A1, &A2, A3, A4)>
                for value::HkFn<fn(&mut A1, &A2, A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, &A2, A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> PartialEq
                for value::HkFn<fn(&mut A1, &A2, A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4, Out>(
                        f: fn(&mut A1, &A2, A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, &A2, A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &A2, A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &A2, A3, &A4)>
                for HkFn<fn(&mut A1, &A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, &A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &A2, A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(&mut A1, &A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, A4, Out>(
                        f: fn(&mut A1, &A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, &A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &A2, A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &A2, A3, &mut A4)>
                for HkFn<fn(&mut A1, &A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, &A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &A2, A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(&mut A1, &A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable
            for HkFn<fn(&mut A1, &A2, &A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(&mut A1, &A2, &A3)>
            for HkFn<fn(&mut A1, &A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, &A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&mut A1, &A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(&mut A1, &A2, &A3) -> Out,
                ) -> HkFn<fn(&mut A1, &A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&mut A1, &A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&mut A1, &A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(&mut A1, &A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, &A2, &A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>
                crate::Callable<(&mut A1, &A2, &A3, A4)>
                for value::HkFn<fn(&mut A1, &A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, &A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&mut A1, &A2, &A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, &A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, &A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &A2, &A3, &A4)>
                for HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, &A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, &A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, &A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &A2, &A3, &mut A4)>
                for HkFn<fn(&mut A1, &A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&mut A1, &A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &A2, &A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, &A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable
            for HkFn<fn(&mut A1, &A2, &mut A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(&mut A1, &A2, &mut A3)>
            for HkFn<fn(&mut A1, &A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, &A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&mut A1, &A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::ByRef<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &A2, &mut A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(&mut A1, &A2, &mut A3) -> Out,
                ) -> HkFn<fn(&mut A1, &A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&mut A1, &A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, &A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>
                crate::Callable<(&mut A1, &A2, &mut A3, A4)>
                for value::HkFn<fn(&mut A1, &A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, &A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&mut A1, &A2, &mut A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, &A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, &A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &A2, &mut A3, &A4)>
                for HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, &A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &A2, &mut A3, &mut A4)>
                for HkFn<fn(&mut A1, &A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, &A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByRef<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &A2, &mut A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, &A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
    pub use r#mut::fn_pointer::FnPointer as r#mut;
    impl<A1: ?Sized, A2: ?Sized, Out> crate::IsCallable for HkFn<fn(&mut A1, &mut A2) -> Out> {}

    impl<A1: ?Sized, A2: ?Sized, Out> crate::Callable<(&mut A1, &mut A2)>
        for HkFn<fn(&mut A1, &mut A2) -> Out>
    {
        type Output = Out;
        fn call_fn(&self, #[allow(non_snake_case)] (A1, A2): (&mut A1, &mut A2)) -> Self::Output {
            self.0(A1, A2)
        }
    }
    impl<A1: ?Sized, A2: ?Sized, Out> crate::CallableWithFixedArguments
        for HkFn<fn(&mut A1, &mut A2) -> Out>
    {
        type FixedArgumentTypes = (crate::argument::ByMut<A1>, crate::argument::ByMut<A2>);
    }
    impl<A1: ?Sized, A2: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &mut A2) -> Out> {
        fn eq(&self, other: &Self) -> bool {
            self.0 as usize == other.0 as usize
        }
    }
    pub mod r#mut {
        pub(super) mod fn_pointer {
            #[allow(unused_imports)]
            use super::super::*;
            #[allow(non_snake_case)]
            pub fn FnPointer<A1: ?Sized, A2: ?Sized, Out>(
                f: fn(&mut A1, &mut A2) -> Out,
            ) -> HkFn<fn(&mut A1, &mut A2) -> Out> {
                HkFn(f)
            }
        }
        mod fn_pointer_type {
            #[allow(unused_imports)]
            use super::super::*;
            pub type FnPointer<A1, A2, Out> = HkFn<fn(&mut A1, &mut A2) -> Out>;
        }
        use super::HkFn;
        pub use fn_pointer::FnPointer;
        pub use fn_pointer_type::FnPointer;
        pub use value::fn_pointer::FnPointer as value;
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::IsCallable
            for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::Callable<(&mut A1, &mut A2, A3)>
            for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, &mut A2, A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> crate::CallableWithFixedArguments
            for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::Value<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3, Out> PartialEq for value::HkFn<fn(&mut A1, &mut A2, A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod value {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&mut A1, &mut A2, A3) -> Out,
                ) -> value::HkFn<fn(&mut A1, &mut A2, A3) -> Out> {
                    value::HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = value::HkFn<fn(&mut A1, &mut A2, A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);

            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, &mut A2, A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::Callable<(&mut A1, &mut A2, A3, A4)>
                for value::HkFn<fn(&mut A1, &mut A2, A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, &mut A2, A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4, Out> PartialEq
                for value::HkFn<fn(&mut A1, &mut A2, A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4, Out>(
                        f: fn(&mut A1, &mut A2, A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, &mut A2, A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &mut A2, A3, &A4)>
                for HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(&mut A1, &mut A2, A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, A4, Out>(
                        f: fn(&mut A1, &mut A2, A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, &mut A2, A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &mut A2, A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &mut A2, A3, &mut A4)>
                for HkFn<fn(&mut A1, &mut A2, A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, &mut A2, A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::Value<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &mut A2, A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3, A4: ?Sized, Out>(
                        f: fn(&mut A1, &mut A2, A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &mut A2, A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#ref::fn_pointer::FnPointer as r#ref;
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable
            for HkFn<fn(&mut A1, &mut A2, &A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(&mut A1, &mut A2, &A3)>
            for HkFn<fn(&mut A1, &mut A2, &A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, &mut A2, &A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&mut A1, &mut A2, &A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::ByRef<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq for HkFn<fn(&mut A1, &mut A2, &A3) -> Out> {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#ref {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(&mut A1, &mut A2, &A3) -> Out,
                ) -> HkFn<fn(&mut A1, &mut A2, &A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&mut A1, &mut A2, &A3) -> Out>;
            }
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3, Out>(
                    f: fn(&mut A1, &mut A2, &A3) -> Out,
                    arg: A3,
                ) -> crate::argument::LastArgumentProvided<
                    HkFn<fn(&mut A1, &mut A2, &A3) -> Out>,
                    crate::argument::Refed<A3>,
                > {
                    use crate::IsCallable;
                    super::super::r#ref(f).provide_last_argument_refed(arg)
                }
            }
            use super::HkFn;
            pub use provide_last_argument::provide_last_argument;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, &mut A2, &A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>
                crate::Callable<(&mut A1, &mut A2, &A3, A4)>
                for value::HkFn<fn(&mut A1, &mut A2, &A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, &A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, &mut A2, &A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&mut A1, &mut A2, &A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, &mut A2, &A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, &A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, &mut A2, &A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &mut A2, &A3, &A4)>
                for HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, &A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments for HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, &mut A2, &A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &mut A2, &A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &mut A2, &A3, &mut A4)>
                for HkFn<fn(&mut A1, &mut A2, &A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, &A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, &mut A2, &A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByRef<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &mut A2, &A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, &mut A2, &A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &mut A2, &A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
        pub use r#mut::fn_pointer::FnPointer as r#mut;
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::IsCallable
            for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
        {
        }

        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::Callable<(&mut A1, &mut A2, &mut A3)>
            for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
        {
            type Output = Out;
            fn call_fn(
                &self,
                #[allow(non_snake_case)] (A1, A2, A3): (&mut A1, &mut A2, &mut A3),
            ) -> Self::Output {
                self.0(A1, A2, A3)
            }
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> crate::CallableWithFixedArguments
            for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
        {
            type FixedArgumentTypes = (
                crate::argument::ByMut<A1>,
                crate::argument::ByMut<A2>,
                crate::argument::ByMut<A3>,
            );
        }
        impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out> PartialEq
            for HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>
        {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        pub mod r#mut {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, Out>(
                    f: fn(&mut A1, &mut A2, &mut A3) -> Out,
                ) -> HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out> {
                    HkFn(f)
                }
            }
            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<A1, A2, A3, Out> = HkFn<fn(&mut A1, &mut A2, &mut A3) -> Out>;
            }
            use super::HkFn;
            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;
            pub use value::fn_pointer::FnPointer as value;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::IsCallable
                for value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>
                crate::Callable<(&mut A1, &mut A2, &mut A3, A4)>
                for value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, &mut A3, A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> crate::CallableWithFixedArguments
                for value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::Value<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out> PartialEq
                for value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod value {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, A4) -> Out,
                    ) -> value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4) -> Out> {
                        value::HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        value::HkFn<fn(&mut A1, &mut A2, &mut A3, A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                #[derive(Debug, Clone, Copy)]
                pub struct HkFn<F>(pub(super) F);
            }
            pub use r#ref::fn_pointer::FnPointer as r#ref;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &mut A2, &mut A3, &A4)>
                for HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, &mut A3, &A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByRef<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#ref {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
                mod provide_last_argument {
                    use super::super::HkFn;
                    pub fn provide_last_argument<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &A4) -> Out,
                        arg: A4,
                    ) -> crate::argument::LastArgumentProvided<
                        HkFn<fn(&mut A1, &mut A2, &mut A3, &A4) -> Out>,
                        crate::argument::Refed<A4>,
                    > {
                        use crate::IsCallable;
                        super::super::r#ref(f).provide_last_argument_refed(arg)
                    }
                }
                pub use provide_last_argument::provide_last_argument;
            }
            pub use r#mut::fn_pointer::FnPointer as r#mut;
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> crate::IsCallable
                for HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
            }

            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::Callable<(&mut A1, &mut A2, &mut A3, &mut A4)>
                for HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                type Output = Out;
                fn call_fn(
                    &self,
                    #[allow(non_snake_case)] (A1, A2, A3, A4): (&mut A1, &mut A2, &mut A3, &mut A4),
                ) -> Self::Output {
                    self.0(A1, A2, A3, A4)
                }
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>
                crate::CallableWithFixedArguments
                for HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                type FixedArgumentTypes = (
                    crate::argument::ByMut<A1>,
                    crate::argument::ByMut<A2>,
                    crate::argument::ByMut<A3>,
                    crate::argument::ByMut<A4>,
                );
            }
            impl<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out> PartialEq
                for HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out>
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 as usize == other.0 as usize
                }
            }
            pub mod r#mut {
                pub(super) mod fn_pointer {
                    #[allow(unused_imports)]
                    use super::super::*;
                    #[allow(non_snake_case)]
                    pub fn FnPointer<A1: ?Sized, A2: ?Sized, A3: ?Sized, A4: ?Sized, Out>(
                        f: fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out,
                    ) -> HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out> {
                        HkFn(f)
                    }
                }
                mod fn_pointer_type {
                    #[allow(unused_imports)]
                    use super::super::*;
                    pub type FnPointer<A1, A2, A3, A4, Out> =
                        HkFn<fn(&mut A1, &mut A2, &mut A3, &mut A4) -> Out>;
                }
                pub use fn_pointer::FnPointer;
                pub use fn_pointer_type::FnPointer;
            }
        }
    }
}
