#[macro_export]
macro_rules! pipe {
    // If no pipes were found, throw a comptime error and stop the compilation process, invalid usage was found.
    (@accumulate_expression [$($expr:tt)*]) => (::std::compile_error!("failed to accumulate pipeline, missing pipes."));

    // Found a pipe operator, the expression should now have been accumulated into
    // an expression. Call the internal @accumulate_pipes rule.
    (@accumulate_expression [$expr:expr] |> $($tail:tt)+) => ($crate::pipe!(
        @accumulate_pipes [$expr] [] |> $($tail)+
    ));

    // Accumulate the next token into the expression buffer and recurse.
    (@accumulate_expression [$($expr:tt)*] $token:tt $($tail:tt)*) => ($crate::pipe!(
        @accumulate_expression [$($expr)* $token] $($tail)*
    ));

    // This arm matches a partial invocation of a pipe where `@` will be replaced by the
    // value being passed through the pipeline.
    (@accumulate_pipes [$expr:expr] [$($pipes:tt)*] |> $pipe:ident($($l_arg:expr,)* $(@ $(@$($_:tt)* $value:tt)? $(, $r_arg:expr)*)?) $($tail:tt)*) => ($crate::pipe!(
        @accumulate_pipes [$expr] [$($pipes)* [
            (|value| $pipe($($l_arg,)* $($($value)? value, $($r_arg),*)?))
        ]] $($tail)*
    ));

    // This arm matches a pipe that consists of only an identifier, this assumes the identifier is callable.
    (@accumulate_pipes [$expr:tt] [$($pipes:tt)*] |> $pipe:ident $($tail:tt)*) => ($crate::pipe!(
        @accumulate_pipes [$expr] [$($pipes)* [$pipe]] $($tail)*
    ));

    // No more pipes were found, execute all the pipes in order with the result of the previous,
    // or the expression buffer if no previous piped-value exists.
    (@accumulate_pipes [$expr:expr] [$([$($pipe:tt)+])+]) => ({
        let current = $expr;

        $(
            let current = $($pipe)*(current);
        )+

        current
    });

    // Accepts any tokens and attempts to parse them as a pipeline.
    ($($tokens:tt)+) => ($crate::pipe!(
        @accumulate_expression [] $($tokens)*
    ));

    // An empty pipeline results in a unit type.
    () => (());
}