use std::panic::UnwindSafe;

pub fn catch_string_panic<R, F>(f: F) -> String
where
    F: FnOnce() -> R + UnwindSafe,
{
    *std::panic::catch_unwind(f)
        .err()
        .unwrap()
        .downcast::<String>()
        .unwrap()
}
