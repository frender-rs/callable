//!
//! ```
//! # use frender_events::*;
//! let callback = crate::r#ref(u8::clone).with_input_ref(8);
//! assert_eq!(callback.emit(()), 8u8);
//! ```

mod traits;
pub use traits::*;

pub mod accept_anything;
pub mod chain;

pub mod argument;

mod maybe_handle_event;
pub use maybe_handle_event::*;

mod sealed {
    pub trait Tuple {}
}

#[derive(Debug, Clone, Copy)]
pub struct HkFn<F>(F);

#[cfg(feature = "impl_with_macro_rules")]
mod imp {
    mod macros;
    macros::impl_with_macro_rules!(a1: A1, a2: A2, a3: A3);
}

#[cfg(not(feature = "impl_with_macro_rules"))]
mod imp;

pub use imp::*;

#[macro_export]
macro_rules! ArgumentTypes {
    () => {
        ()
    };
    (@$resolved:tt) => {
        $resolved
    };
    ($(@($($resolved:tt)*))? &mut $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::argument::ByMut<$t>,) $($rest)* }
    };
    ($(@($($resolved:tt)*))? &    $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::argument::ByRef<$t>,) $($rest)* }
    };
    ($(@($($resolved:tt)*))?      $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::argument::Value<$t>,) $($rest)* }
    };
}

#[cfg(test)]
mod tests {
    use super::{Callable, Callback, IsCallable};

    #[test]
    fn test_callback_ref() {
        fn asserts<T>(t: T) -> T
        where
            T: for<'i> Callback<&'i usize, Output = usize>,
        {
            t
        }

        let cbk = asserts(crate::r#ref(Clone::clone));
        assert_eq!(cbk.emit(&0), 0);
        assert_eq!(cbk.provide_last_argument_refed(8).call_fn(()), 8);
    }

    #[test]
    fn mods() {
        use super::callback;
        let _: fn() = callback(|| {});

        let _: fn(()) = crate::value(|()| {});
        let _: crate::HkFn<fn(&())> = crate::r#ref(|&()| {});
        let _: crate::HkFn<fn(&mut ())> = crate::r#mut(|&mut ()| {});

        let _: fn((), ()) = crate::value::value(|(), ()| {});
        let _: crate::value::HkFn<fn((), &())> = crate::value::r#ref(|(), &()| {});
    }
}

pub fn callback<Out>(f: fn() -> Out) -> fn() -> Out {
    f
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_or {
    ([         ] $($b:tt)*) => { $($b)* };
    ([$($a:tt)+] $($b:tt)*) => { $($a)+ };
}

macro_rules! impl_callback_parse_input {
    ($dollar:tt $([$($match:tt)*][$($clone:tt)*] => $output_path:ident $output_input:tt )*) => {
        #[macro_export]
        macro_rules! __callback_parse_input {
            // end input
            ([| $dollar($other:tt)*] $_clone:tt $path:tt $input:tt ) => {
                $crate::__callback_input_resolved! { [$path $input] $dollar($other)*  }
            };
            $(
                (   [$($match)* , $dollar($_rest:tt)*]
                    [$($clone)* , $dollar($ rest:tt)*]
                    {$dollar($paths:ident)*}
                    {$dollar($inputs:tt  )*}
                ) => {
                    $crate::__callback_parse_input! {
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
                    $crate::__callback_input_resolved! { [
                        {$dollar($paths )* $output_path }
                        {$dollar($inputs)* $output_input}
                    ] $dollar($rest)* }
                };
            )*
        }
    };
}

impl_callback_parse_input! { $
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

#[macro_export]
macro_rules! callback {
    (|| $($rest:tt)*) => {
        $crate::__callback_input_resolved! { [{} {}] $($rest)*  }
    };
    (|  $($rest:tt)* ) => {
        $crate::__callback_parse_input! { [$($rest)*][$($rest)*]{}{} }
    };
    ($e:expr , fn($($args:tt)*) $(-> $output:ty)?) => {
        $crate::__callable_wrap_fn! { [$($args)*]{}{}($e){$($output)?} }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __callable_wrap_fn {
    ([                             ]{$($path:ident)*}{$($generics:tt)*}($e:expr){$output_ty:ty} ) => {
        $crate::callback $(:: $path)* ::<$($generics)* $output_ty> ($e)
    };
    ([                             ]{$($path:ident)*}{$($generics:tt)*}($e:expr){             } ) => {
        $crate::callback $(:: $path)* ::<$($generics)* _         > ($e)
    };
    ([&mut $t:ty $(, $($rest:tt)*)?]{$($path:ident)*}{$($generics:tt)*} $e:tt    $output_ty:tt  ) => {
        $crate::__callable_wrap_fn! { [$($($rest)*)?]{$($path)* r#mut}{$t,} $e $output_ty }
    };
    ([&    $t:ty $(, $($rest:tt)*)?]{$($path:ident)*}{$($generics:tt)*} $e:tt    $output_ty:tt  ) => {
        $crate::__callable_wrap_fn! { [$($($rest)*)?]{$($path)* r#ref}{$t,} $e $output_ty }
    };
    ([     $t:ty $(, $($rest:tt)*)?]{$($path:ident)*}{$($generics:tt)*} $e:tt    $output_ty:tt  ) => {
        $crate::__callable_wrap_fn! { [$($($rest)*)?]{$($path)* value}{$t,} $e $output_ty }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __callback_input_resolved {
    // --- no explicit output type
    ($input:tt                   $body:expr   $(, $($rest:tt)*)?) => {
        $crate::__callback_all_resolved! { $input [       ] [   $body   ] $($($rest)*)? }
    };
    // This branch is for dev experience
    ($input:tt               { $($body:tt)* } $(, $($rest:tt)*)?) => {
        $crate::__callback_all_resolved! { $input [       ] [{$($body)*}] $($($rest)*)? }
    };
    // --- explicit output
    ($input:tt -> $output:ty { $($body:tt)* } $(, $($rest:tt)*)?) => {
        $crate::__callback_all_resolved! { $input [$output] [{$($body)*}] $($($rest)*)? }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __callback_all_resolved {
    // no   state
    ([{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*]) => {
        $crate::callback $(::$method_path)*                                (
            |$($($input)*),*               | $(-> $output)? $($body)*
        )
    };
    // one  state
    ([{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*]   $state:ident $(= $state_expr:expr)?    $(,)?) => {
        $crate::callback $(::$method_path)* ::r#ref::provide_last_argument (
            |$($($input)* ,)* $state       | $(-> $output)? $($body)* ,
            $crate::__expand_or!([$($state_expr)?] $state),
        )
    };
    // many states
    ([{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*] $($state:ident $(= $state_expr:expr)?),+ $(,)?) => {
        $crate::callback $(::$method_path)* ::r#ref::provide_last_argument (
            |$($($input)* ,)* ($($state,)+)| $(-> $output)? $($body)* ,
            ($( $crate::__expand_or!([$($state_expr)?] $state) ,)+),
        )
    };
}