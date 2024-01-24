#[macro_export]
macro_rules! pipe {
    (@accumulate_expression [$($expr:tt)*]) => (::std::compile_error!("failed to accumulate pipeline, missing pipes."));
    (@accumulate_expression [$($expr:tt)*] |> $($tail:tt)+) => ($crate::pipe!(@accumulate_pipes [$($expr)*] [] |> $($tail)+));
    (@accumulate_expression [$($expr:tt)*] $token:tt $($tail:tt)*) => ($crate::pipe!(@accumulate_expression [$($expr)* $token] $($tail)*));
    (@accumulate_pipes [$($expr:tt)*] [$($pipes:tt)*] |> $pipe:ident $($tail:tt)*) => ($crate::pipe!(@accumulate_pipes [$($expr)*] [$($pipes)* [$pipe]] $($tail)*));
    (@accumulate_pipes [$expr:expr] [$([$($pipe:tt)+])+]) => ({
        let current = $expr;

        $(
            let current = $($pipe)*(current);
        )+

        current
    });
    ($($tokens:tt)*) => ($crate::pipe!(@accumulate_expression [] $($tokens)*));
}