#[macro_export]
macro_rules! pipe {
    (@accumulate [$($expr:tt)*]) => (::std::compile_error!("failed to accumulate pipeline, missing pipes."));
    (@accumulate [$($expr:tt)*] |> $($tail:tt)+) => ($crate::pipe!([$($expr)*] |> $($tail)+));
    (@accumulate [$($expr:tt)*] $token:tt $($tail:tt)*) => ($crate::pipe!(@accumulate [$($expr)* $token] $($tail)*));
    ([$expr:expr] $(|> $ident:ident)+) => ({
        let current = $expr;

        $(
            let current = $ident(current);
        )+

        current
    });
    ($($tokens:tt)*) => ($crate::pipe!(@accumulate [] $($tokens)*));
}