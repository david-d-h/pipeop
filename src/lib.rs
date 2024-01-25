#![no_std]
#[macro_export]
macro_rules! pipe {
    // If no pipes were found, throw a comptime error and stop the compilation process, invalid usage was found.
    (@accumulate_expression [$($expr:tt)*]) => (::std::compile_error!("failed to accumulate pipeline, missing pipes."));

    // Found a pipe operator, the expression should now have been accumulated into
    // an expression. Call the internal @accumulate_pipes rule.
    (@accumulate_expression [$($expr:tt)+] |> $($tail:tt)+) => ($crate::pipe!(
        @accumulate_pipes [$($expr)+] [] |> $($tail)+
    ));

    // Accumulate the next token into the expression buffer and recurse.
    (@accumulate_expression [$($expr:tt)*] $token:tt $($tail:tt)*) => ($crate::pipe!(
        @accumulate_expression [$($expr)* $token] $($tail)*
    ));

    // This arm matches a partial invocation of a pipe where `@` will be replaced by the
    // value being passed through the pipeline.
    (
        @accumulate_pipes [$($expr:tt)+] [$($pipes:tt)*]
        |> $(:: $(@$($_:tt)* $prefixed:tt)?)? $ident:ident $(:: $path:ident)*
        ($($l_arg:expr,)* $(ref $(@@$($__:tt)* $borrow:tt)?)? @ $(, $r_arg:expr)* $(,)?)
        $($tail:tt)*
    ) => ($crate::pipe!(
        @accumulate_pipes [$($expr)+] [$($pipes)*
            [|$($($borrow)? ref)? value| $($($prefixed)? ::)? $ident $(:: $path)* ($($l_arg,)* value, $($r_arg),*)]
        ] $($tail)*
    ));

    // This arm matches a pipe that consists of only an identifier, this assumes the identifier is callable.
    (
        @accumulate_pipes [$($expr:tt)+] [$($pipes:tt)*]
        |> $(:: $(@$($_:tt)* $prefixed:tt)?)? $ident:ident $(:: $path:ident)*
        $(|> $($tail:tt)*)?
    ) => ($crate::pipe!(
        @accumulate_pipes [$($expr)+]
            [$($pipes)* [$($($prefixed)? ::)? $ident $(:: $path)*]
        ] $(|> $($tail)*)?
    ));

    // This arm matches a method invocation on the value currently going through the pipeline.
    (@accumulate_pipes [$($expr:tt)+] [$($pipes:tt)*] |> . $pipe:ident($($arg:expr),*) $($tail:tt)*) => ($crate::pipe!(
        @accumulate_pipes [$($expr)+] [$($pipes)* [|value| value.$pipe($($($arg),*)?)]] $($tail)*
    ));

    // This arm matches a method invocation without parentheses, and thus also without arguments.
    (@accumulate_pipes [$($expr:tt)+] [$($pipes:tt)*] |> . $pipe:ident $($tail:tt)*) => ($crate::pipe!(
        @accumulate_pipes [$($expr)+] [$($pipes)* [|value| value.$pipe()]] $($tail)*
    ));

    // This arm matches a closure with a block.
    (@accumulate_pipes [$($expr:tt)+] [$($pipes:tt)*] |> |$ident:ident| $block:block $($tail:tt)*) => ($crate::pipe!(
        @accumulate_pipes [$($expr)+] [$($pipes)* [|$ident| $block]] $($tail)*
    ));

    // This arm matches a closure that evaluates an expression.
    (@accumulate_pipes [$($expr:tt)+] [$($pipes:tt)*] |> |$ident:ident| $($residual:tt)+) => ($crate::pipe!(
        @accumulate_expr_closure_pipe [[$($expr)+]] [$($pipes)*] [$ident] [] $($residual)*
    ));

    (@accumulate_pipes [$($expr:tt)+] [$($pipes:tt)*] |> |_| $($residual:tt)+) => ($crate::pipe!(
        @accumulate_expr_closure_pipe [[$($expr)+]] [$($pipes)*] [_] [] $($residual)*
    ));

    (@accumulate_expr_closure_pipe [$($carry:tt)*] [$($pipes:tt)*] [$($args:tt)*] [$expression:expr] $(|> $($tail:tt)+)?) => ($crate::pipe!(
        @accumulate_pipes $($carry)* [$($pipes)* [|$($args)*| $expression]] $(|> $($tail)+)?
    ));

    (@accumulate_expr_closure_pipe [$($carry:tt)*] [$($pipes:tt)*] [$($args:tt)*] [$($expression:tt)*] $token:tt $($tail:tt)*) => ($crate::pipe!(
        @accumulate_expr_closure_pipe [$($carry)*] [$($pipes)*] [$($args)*] [$($expression)* $token] $($tail)*
    ));

    // Create a closure that encapsulates the pipeline, so the pipeline can be reused.
    (@accumulate_pipes [... $(as $(@$($_:tt)* $explicit:tt)? $ty:ty)?] [$($pipes:tt)+]) => ((
        |item $($($explicit)? : $ty)?| $crate::pipe!(@accumulate_pipes [item] [$($pipes)+])
    ));

    // No more pipes were found, execute all the pipes in order with the result of the previous,
    // or the expression buffer if no previous piped-value exists and return the result.
    (@accumulate_pipes [$expr:expr] [$([$($pipe:tt)+])+]) => ({
        let current = $expr;
        $(let current = $crate::call_with($($pipe)+, current);)+
        current
    });

    (@accumulate_pipes [$($expr:tt)*] [$($pipes:tt)*] $($tail:tt)*) => (::std::compile_error!("found invalid pipe syntax"));

    // Accepts any tokens and attempts to parse them as a pipeline.
    ($($tokens:tt)+) => ($crate::pipe!(
        @accumulate_expression [] $($tokens)*
    ));

    // An empty pipeline results in a unit type.
    () => (());
}

pub fn call_with<T, R, F: FnOnce(T) -> R>(f: F, t: T) -> R {
    f(t)
}