#![no_std]

//!
//! ```
//! # use callable::prelude::*;
//! let callable = callable![fn(&_) -> _](u8::clone);
//! assert_eq!(callable.call_fn((&1,)), 1);
//! assert_eq!(callable.emit(&2), 2);
//!
//! let callable = callable.provide_first_argument_refed(3);
//! assert_eq!(callable.call_fn(()), 3);
//! ```

mod traits;
pub use traits::*;

pub mod accept_anything;
pub mod chain;

pub mod argument;
pub use argument::{ArgumentOfType, ArgumentType, ArgumentTypes, ArgumentsOfTypes};

mod maybe_handle_event;
pub use maybe_handle_event::*;

mod sealed {
    pub trait Sealed {}
}

mod private {
    #[derive(Debug, Clone, Copy)]
    pub struct HkFn<F>(pub(crate) F);
}

#[cfg(feature = "impl_with_macro_rules")]
mod imp {
    mod macros;
    macros::impl_with_macro_rules!(a1: A1, a2: A2, a3: A3, a4: A4);
}

#[cfg(not(feature = "impl_with_macro_rules"))]
mod imp;

pub use imp::*;

/// ```
/// # use callable::{ArgumentType, ArgumentTypes};
/// # fn __(v:
/// (ArgumentType![u8], ArgumentType![&str], ArgumentType![&mut Vec<u8>])
/// # ) ->
/// // is exactly the same as
/// ArgumentTypes!(u8, &str, &mut Vec<u8>)
/// # { v }
/// ```
#[macro_export]
macro_rules! ArgumentType {
    (&mut $t:ty) => { $crate::argument::ByMut<$t> };
    (&    $t:ty) => { $crate::argument::ByRef<$t> };
    (     $t:ty) => { $crate::argument::Value<$t> };
}

#[macro_export]
macro_rules! ArgumentTypes {
    () => {
        ()
    };
    (@$resolved:tt) => {
        $resolved
    };
    ($(@($($resolved:tt)*))? &mut $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::argument::ByMut<$t>,) $($($rest)*)? }
    };
    ($(@($($resolved:tt)*))? &    $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::argument::ByRef<$t>,) $($($rest)*)? }
    };
    ($(@($($resolved:tt)*))?      $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::argument::Value<$t>,) $($($rest)*)? }
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_callable_ref() {
        fn asserts<T>(t: T) -> T
        where
            T: for<'i> CallableOne<&'i usize, Output = usize>,
        {
            t
        }

        let cbk = asserts(crate::r#ref(Clone::clone));
        assert_eq!(cbk.emit(&0), 0);
        assert_eq!(cbk.provide_last_argument_refed(8).call_fn(()), 8);
    }

    #[test]
    fn mods() {
        let _: fn() = callable(|| {});

        let _: fn(()) = crate::value(|()| {});
        let _: callable![fn(&())] = crate::r#ref(|&()| {});
        let _: callable![fn(&mut ())] = crate::r#mut(|&mut ()| {});

        let _: fn((), ()) = crate::value::value(|(), ()| {});
        let _: crate::value::HkFn<fn((), &())> = crate::value::r#ref(|(), &()| {});
    }
}

mod fn_pointer_fn {
    #[allow(non_snake_case)]
    pub fn FnPointer<Out>(f: fn() -> Out) -> fn() -> Out {
        f
    }
}
pub use fn_pointer_fn::FnPointer;
pub type FnPointer<Out> = fn() -> Out;

pub use fn_pointer_fn::FnPointer as callable;

pub mod prelude {
    pub use crate::{ArgumentTypes, Callable, CallableOne, CallableWithFixedArguments, IsCallable};

    pub use crate as callable;
    pub use crate::callable;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_or {
    ([         ] $($b:tt)*) => { $($b)* };
    ([$($a:tt)+] $($b:tt)*) => { $($a)+ };
}

macro_rules! impl_callable_parse_input {
    ($dollar:tt $([$($match:tt)*][$($clone:tt)*] => $output_path:ident $output_input:tt )*) => {
        #[macro_export]
        macro_rules! __callable_parse_input {
            // end input
            ([| $dollar($other:tt)*] $_clone:tt $path:tt $input:tt ) => {
                $crate::__callable_input_resolved! { [$path $input] $dollar($other)*  }
            };
            $(
                (   [$($match)* , $dollar($_rest:tt)*]
                    [$($clone)* , $dollar($ rest:tt)*]
                    {$dollar($paths:ident)*}
                    {$dollar($inputs:tt  )*}
                ) => {
                    $crate::__callable_parse_input! {
                        [$dollar($_rest)*]
                        [$dollar($ rest)*]
                        {$dollar($paths )* $output_path }
                        {$dollar($inputs)* $output_input}
                    }
                };
                (   [$($match)* | $dollar($_rest:tt)*]
                    [$($clone)* | $dollar($ rest:tt)*]
                    {$dollar($paths:ident)*}
                    {$dollar($inputs:tt  )*}
                ) => {
                    $crate::__callable_input_resolved! { [
                        {$dollar($paths )* $output_path }
                        {$dollar($inputs)* $output_input}
                    ] $dollar($rest)* }
                };
            )*
        }
    };
}

impl_callable_parse_input! { $
    // &mut _
    [&     mut   $_input:pat_param][$r:tt $m:tt $ input:pat_param] => r#mut[$r $m $input]
    // &mut v: _
    [&       mut     $($_input:ident)+ :                $_input_ty:ty][$r:tt $m:tt $($ input:ident)+ :                $ input_ty:ty] => r#mut[$r $m $($input)+ : $input_ty]
    // &mut _: _
    [&       mut       $_input:tt      :      $_input_ty:ty][$r:tt $m:tt   $ input:tt      :      $ input_ty:ty]=> r#mut [$r $m   $input   : $input_ty]
    // v: &mut _
    [                $($_input:ident)+ : &mut $_input_ty:ty][                $($ input:ident)+ : $r:tt $m:tt $ input_ty:ty]=> r#mut [$($input)+   : $r $m $input_ty]
    // _: &mut _
    [                  $_input:tt      : &mut $_input_ty:ty , $($_rest:tt)*][                  $ input:tt      : &mut $ input_ty:ty , $($ rest:tt)*]=> r#mut[$input : &mut $input_ty]

    // &_
    [&    $_input:pat_param][$r:tt $input:pat_param] => r#ref[$r $input]
    // &v: _
    [&    $($_input:ident)+ : $_input_ty:ty][$r:tt    $($input:ident)+ : $input_ty:ty] => r#ref[$r $($input)+ : $input_ty]
    // &_: _
    [&    $_input:tt : $_input_ty:ty][$r:tt    $input:tt : $input_ty:ty] => r#ref[$r $input : $input_ty]
    // v: &_
    [$($_input:ident)+ : & $_input_ty:ty ][$($input:ident)+ : $r:tt $input_ty:ty ] => r#ref[$($input)+ : $r $input_ty]
    // _: &_
    [$_input:tt : & $_input_ty:ty ][$input:tt : $r:tt $input_ty:ty ] => r#ref[$input : $r $input_ty]

    // |_| {}
    [$_input:pat_param][$input:pat_param] => value[$input]
    // |v: _| {}
    [$($_input:ident)+ :   $_input_ty:ty][$($input:ident)+ :   $input_ty:ty] => value[$($input)+ : $input_ty]
    // |_: _| {}
    [$_input:tt :   $_input_ty:ty][$input:tt :   $input_ty:ty] => value[$input : $input_ty]
}

/// ```
/// use callable::callable;
/// let _: callable![fn() -> u8] = || 1;
/// let _: callable![fn(&mut _)] = callable![fn(&mut _)](|_: &mut u8| {});
/// ```
#[macro_export]
macro_rules! callable {
    (|| $($rest:tt)*) => {
        $crate::__callable_input_resolved! { [{} {}] $($rest)*  }
    };
    (|  $($rest:tt)* ) => {
        $crate::__callable_parse_input! { [$($rest)*][$($rest)*]{}{} }
    };
    (fn($($args:tt)*) $(-> $output:ty)?) => {
        $crate::__fn_pointer! { [$($args)*]{}{}{$($output)?} }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fn_pointer {
    ([                             ]{$($path:ident)*}{$($generics:tt)*}{$output_ty:ty} ) => {
        $crate $(:: $path)* ::FnPointer::<$($generics)* $output_ty>
    };
    ([                             ]{$($path:ident)*}{$($generics:tt)*}{             } ) => {
        $crate $(:: $path)* ::FnPointer::<$($generics)* ()        >
    };
    ([&mut $t:ty $(, $($rest:tt)*)?]{$($path:ident)*}{$($generics:tt)*} $output_ty:tt  ) => {
        $crate::__fn_pointer! { [$($($rest)*)?]{$($path)* r#mut}{$($generics)* $t,} $output_ty }
    };
    ([&    $t:ty $(, $($rest:tt)*)?]{$($path:ident)*}{$($generics:tt)*} $output_ty:tt  ) => {
        $crate::__fn_pointer! { [$($($rest)*)?]{$($path)* r#ref}{$($generics)* $t,} $output_ty }
    };
    ([     $t:ty $(, $($rest:tt)*)?]{$($path:ident)*}{$($generics:tt)*} $output_ty:tt  ) => {
        $crate::__fn_pointer! { [$($($rest)*)?]{$($path)* value}{$($generics)* $t,} $output_ty }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __callable_input_resolved {
    // --- no explicit output type
    ($input:tt                   $body:expr   $(, $($rest:tt)*)?) => {
        $crate::__callable_all_resolved! { $input [       ] [   $body   ] $($($rest)*)? }
    };
    // This branch is for dev experience
    ($input:tt               { $($body:tt)* } $(, $($rest:tt)*)?) => {
        $crate::__callable_all_resolved! { $input [       ] [{$($body)*}] $($($rest)*)? }
    };
    // --- explicit output
    ($input:tt -> $output:ty { $($body:tt)* } $(, $($rest:tt)*)?) => {
        $crate::__callable_all_resolved! { $input [$output] [{$($body)*}] $($($rest)*)? }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __callable_all_resolved {
    // no   state
    ([{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*]) => {
        $crate $(::$method_path)* ::FnPointer                    (
            |$($($input)*),*               | $(-> $output)? $($body)*
        )
    };
    // one  state
    ([{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*]   $state:ident $(= $state_expr:expr)?    $(,)?) => {
        $crate $(::$method_path)* ::r#ref::provide_last_argument (
            |$($($input)* ,)* $state       | $(-> $output)? $($body)* ,
            $crate::__expand_or!([$($state_expr)?] $state),
        )
    };
    // many states
    ([{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*] $($state:ident $(= $state_expr:expr)?),+ $(,)?) => {
        $crate $(::$method_path)* ::r#ref::provide_last_argument (
            |$($($input)* ,)* ($($state,)+)| $(-> $output)? $($body)* ,
            ($( $crate::__expand_or!([$($state_expr)?] $state) ,)+),
        )
    };
}
