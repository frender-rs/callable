macro_rules! expand_first_or {
    ([                         ]$($or:tt)*) => { $($or)* };
    ([{$($expand:tt)*}$($t:tt)*]$($or:tt)*) => { $($expand)* };
}

pub(super) use expand_first_or;

macro_rules! expand_last_or {
    ([                         ]$($or:tt)*) => { $($or)* };
    ([{$($expand:tt)*}]$($or:tt)*) => { $($expand)* };
    ([$t:tt $($more:tt)+]$($or:tt)*) => {
        crate::imp::macros::expand_last_or! {
            [$($more)+]
        }
    };
}

pub(super) use expand_last_or;

macro_rules! expand_first_trimmed {
    ((,) $({$($first:tt)*}$({$($expand:tt)*})*)?) => { ($($($($expand)*,)*)?) };
}

pub(super) use expand_first_trimmed;

macro_rules! argument_type {
    (&mut $t:ty) => { crate::argument::ByMut<$t> };
    (&    $t:ty) => { crate::argument::ByRef<$t> };
    (     $t:ty) => { crate::argument::Value<$t> };
}

pub(super) use argument_type;

macro_rules! expand_last_trimmed {
    ($(@impl[$($prepend:tt)*])? (,) $last:tt) => {
        ($($($prepend)*)?)
    };
    ($(@impl[$($prepend:tt)*])? (,) {$($not_last:tt)*}  $($more:tt)+) => {
        crate::imp::macros::expand_last_trimmed! {
            @impl[$($($prepend)*)? $($not_last)*,]
            (,)
            $($more)+
        }
    };
}

pub(super) use expand_last_trimmed;

macro_rules! impl_fn {
    ( [$($item:tt)*]$more:tt) => {
        crate::imp::macros::impl_fn! { ![$($item)*] $more }
        crate::imp::macros::impl_fn! { *[$($item)*] $more }
    };
    (*[$($item:tt)*][$next:tt $($more:tt)*]) => {
        crate::imp::macros::impl_fn! { [$($item)* $next][$($more)*] }
    };
    (*[$($item:tt)*][]) => {};
    (![$({$v:ident $tp:ident})*] $more:tt) => {
        impl<$($tp,)*> crate::sealed::Sealed for ($($tp,)*) {}
        impl<$($tp,)*> crate::Tuple for ($($tp,)*) {}
        impl<'arg, $($tp: crate::argument::ArgumentType,)*> crate::argument::Arguments<'arg> for ($($tp,)*) {
            type Arguments = ($(crate::argument::ArgumentOfType<'arg, $tp>,)*);
        }
        impl<      $($tp: crate::argument::ArgumentType,)*> crate::argument::ArgumentTypes   for ($($tp,)*) {
            type First = crate::imp::macros::expand_first_or![
                [$({$tp})*]
                crate::argument::Invalid
            ];
            type FirstTrimmed = crate::imp::macros::expand_if_else![ [$($tp)*][
                crate::imp::macros::expand_first_trimmed![
                    (,)
                    $({$tp})*
                ]
            ][
                crate::argument::InvalidTuple
            ]];
            type Last = crate::imp::macros::expand_last_or![
                [$({$tp})*]
                crate::argument::Invalid
            ];
            type LastTrimmed = crate::imp::macros::expand_if_else![[$($tp)*][
                crate::imp::macros::expand_last_trimmed![ (,) $({$tp})* ]
            ][
                crate::argument::InvalidTuple
            ]];

            fn re_borrow<'a: 'b, 'b>(($($v,)*): crate::argument::ArgumentsOfTypes<'a, Self>) -> crate::argument::ArgumentsOfTypes<'b, Self> {
                ($($tp::re_borrow($v),)*)
            }
        }

        $crate::imp::macros::expand_if_else! { $more [
        impl<
            Arg: crate::argument::ArgumentType,
            $($tp: crate::argument::ArgumentType,)*
        > crate::argument::PrependArgument<Arg> for ($($tp,)*) {
            type Prepended = (Arg, $($tp,)*);

            fn prepend_argument<'a>(
                first: crate::argument::ArgumentOfType<'a, Arg>,
                ($($v,)*): crate::argument::ArgumentsOfTypes<'a, Self>,
            ) -> crate::argument::ArgumentsOfTypes<'a, Self::Prepended> {
                (first, $($v,)*)
            }
        }
        impl<
            Arg: crate::argument::ArgumentType,
            $($tp: crate::argument::ArgumentType,)*
        > crate::argument::AppendArgument<Arg> for ($($tp,)*) {
            type Appended = ($($tp,)* Arg,);

            fn append_argument<'a>(
                ($($v,)*): crate::argument::ArgumentsOfTypes<'a, Self>,
                last: crate::argument::ArgumentOfType<'a, Arg>,
            ) -> crate::argument::ArgumentsOfTypes<'a, Self::Appended> {
                ($($v,)* last,)
            }
        }
        ][] }

        impl<$($tp,)* Out> crate::IsCallable                 for fn($($tp),*) -> Out {
        }
        impl<$($tp,)* Out> crate::Callable<($($tp,)*)>       for fn($($tp),*) -> Out {
            type Output = Out;

            fn call_fn(&self, ($($v,)*): ($($tp,)*)) -> Self::Output {
                self($($v),*)
            }
        }
        impl<$($tp,)* Out> crate::CallableWithFixedArguments for fn($($tp),*) -> Out {
            type FixedArgumentTypes = ($(crate::argument::Value<$tp>,)*);
        }
    };
}

pub(super) use impl_fn;

macro_rules! expand_if_else {
    ([]$if:tt[$($else:tt)*]) => {
        $($else)*
    };
    ([$($cond:tt)+][$($if:tt)*]$else:tt) => {
        $($if)*
    };
}

pub(super) use expand_if_else;

macro_rules! expand_if_ident_is_else {
    (value           value       [$($if:tt)*]   $else:tt   ) => {
        $($if)*
    };
    (r#ref           r#ref       [$($if:tt)*]   $else:tt   ) => {
        $($if)*
    };
    ($expected:ident $name:ident    $if:tt   [$($else:tt)*]) => {
        $($else)*
    };
}

pub(super) use expand_if_ident_is_else;

macro_rules! impl_one_resolved {
    (
        name!       { $fn_name:ident }
        fn_type!    { $fn_type:ty }
        fn_wrap!    { $($fn_wrap:tt)* }
        fn_wrapped! { $fn_wrapped:ty }
        fn_trait!   { $($should_impl:ident)? }
        all_tps!    { $({[$($all_t:tt)*] $all_tp:ident {$($($all_tp_bounds:tt)+)?}})* }
        before_tps! { $({[$($bef_t:tt)*] $bef_tp:ident {$($($bef_tp_bounds:tt)+)?}})* }
        cur_tp!     {   {[$($cur_t:tt)*] $cur_tp:ident {$($($cur_tp_bounds:tt)+)?}}   }
        more_tps!   { $more:tt }
        fn_expr!    { |$self_ident:ident| $fn_expr:expr }
    ) => {
        pub use $fn_name::fn_pointer::FnPointer as $fn_name;

        $crate::imp::macros::expand_if_else! { [$($should_impl)?][
        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> crate::IsCallable for $fn_wrapped {
        }

        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> crate::Callable<($($($all_t)* $all_tp,)*)> for $fn_wrapped {
            type Output = Out;

            fn call_fn(&self, #[allow(non_snake_case)] ($($all_tp,)*): ($($($all_t)* $all_tp,)*)) -> Self::Output {
                self.0($($all_tp),*)
            }
        }

        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> crate::CallableWithFixedArguments for $fn_wrapped {
            type FixedArgumentTypes   = ($(crate::imp::macros::argument_type![$($all_t)* $all_tp],)*);
        }

        impl<$($all_tp $(: $($all_tp_bounds)+)?,)* Out> PartialEq for $fn_wrapped {
            fn eq(&self, other: &Self) -> bool {
                self.0 as usize == other.0 as usize
            }
        }
        ][] }

        pub mod $fn_name {
            pub(super) mod fn_pointer {
                #[allow(unused_imports)]
                use super::super::*;
                #[allow(non_snake_case)]
                pub fn FnPointer<$($all_tp $(: $($all_tp_bounds)+)?,)* Out>(f: $fn_type) -> $fn_wrapped {
                    $crate::imp::macros::expand_if_else![[$($fn_wrap)*][
                        $($fn_wrap)* (f)
                    ][
                        f
                    ]]
                }
            }

            mod fn_pointer_type {
                #[allow(unused_imports)]
                use super::super::*;
                pub type FnPointer<$($all_tp,)* Out> = $fn_wrapped;
            }

            pub use fn_pointer::FnPointer;
            pub use fn_pointer_type::FnPointer;

            crate::imp::macros::expand_if_ident_is_else! { r#ref $fn_name [
            mod provide_last_argument {
                use super::super::HkFn;
                pub fn provide_last_argument<$($bef_tp $(: $($bef_tp_bounds)+)?,)* $cur_tp, Out>(
                    f: $fn_type,
                    arg: $cur_tp,
                ) -> crate::argument::LastArgumentProvided<
                    $fn_wrapped,
                    crate::argument::Refed<$cur_tp>,
                > {
                    use crate::IsCallable;
                    super::super::$fn_name(f).provide_last_argument_refed(arg)
                }
            }

            pub use provide_last_argument::provide_last_argument;
            ][] }

            crate::imp::macros::expand_if_ident_is_else! { value $fn_name [
            #[derive(Debug, Clone, Copy)]
            pub struct HkFn<F>(pub(super) F);
            ][
            crate::imp::macros::expand_if_else! {$more[
                use super::HkFn;
            ][]}
            ]}

            crate::imp::macros::expand_if_else! {$more[
            crate::imp::macros::impl_all! {
                ($({[$($all_t)*] $all_tp {$($($all_tp_bounds)+)?}})*) @$more
            }
            ][]}
        }
    };
}

pub(super) use impl_one_resolved;

macro_rules! impl_one {
    // all arguments are by value
    ( @$fn_name:ident $(#$prefix_path:ident)? ($({[] $tp:ident {}})*) {[] $cur_tp:ident {}} @$more:tt) => {
        crate::imp::macros::impl_one_resolved! {
            name!       { $fn_name }
            fn_type!    { fn($($tp,)* $cur_tp) -> Out }
            fn_wrap!    {}
            fn_wrapped! { fn($($tp,)* $cur_tp) -> Out }
            fn_trait!   {}
            all_tps!    { $({[] $tp {}})* {[] $cur_tp {}} }
            before_tps! { $({[] $tp {}})*                 }
            cur_tp!     {                 {[] $cur_tp {}} }
            more_tps!   { $more }
            fn_expr!    { |self| self }
        }
    };
    // some arguments are higher kinded
    ( @$fn_name:ident $(#$prefix_path:ident)? ($({[$($t:tt)*] $tp:ident $tp_bounds:tt})*) {[$($cur_t:tt)*] $cur_tp:ident $cur_tp_bounds:tt} @$more:tt) => {
        crate::imp::macros::impl_one_resolved! {
            name!       { $fn_name }
            fn_type!    {                          fn($( $($t)* $tp,)* $($cur_t)* $cur_tp) -> Out  }
            fn_wrap!    { $($prefix_path::)? HkFn                                                  }
            fn_wrapped! { $($prefix_path::)? HkFn <fn($( $($t)* $tp,)* $($cur_t)* $cur_tp) -> Out> }
            fn_trait!   { impl }
            all_tps!    { $({[$($t)*] $tp $tp_bounds})* {[$($cur_t)*] $cur_tp $cur_tp_bounds} }
            before_tps! { $({[$($t)*] $tp $tp_bounds})*                                       }
            cur_tp!     {                               {[$($cur_t)*] $cur_tp $cur_tp_bounds} }
            more_tps!   { $more }
            fn_expr!    { |self| self.0 }
        }
    };
}

pub(super) use impl_one;

macro_rules! impl_all {
    ( $before:tt @[$next:ident $($more:ident)*] ) => {
        crate::imp::macros::impl_one! { @value #value $before {[    ]$next{      }} @[$($more)*] }
        crate::imp::macros::impl_one! { @r#ref        $before {[&   ]$next{?Sized}} @[$($more)*] }
        crate::imp::macros::impl_one! { @r#mut        $before {[&mut]$next{?Sized}} @[$($more)*] }
    };
}

pub(super) use impl_all;

macro_rules! impl_with_macro_rules {
    ($($v:ident : $tp:ident),* $(,)?) => {
        use super::private::HkFn;
        crate::imp::macros::impl_fn!([][$({$v $tp})*]);
        crate::imp::macros::impl_all! {()@[$($tp)*]}
    };
}

pub(super) use impl_with_macro_rules;
