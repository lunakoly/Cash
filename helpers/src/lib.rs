/// Manual implementation of
/// Kotlin's elsvis operator `?:`.
///
/// Example:
/// ```
///     let that = elvis! { it, Some(it) = wrapped => return };
/// ```
#[macro_export]
macro_rules! elvis {
    ( $it:ident, $pattern:pat = $target:expr => $otherwise:expr ) => {
        if let $pattern = $target {
            $it
        } else {
          $otherwise
        };
    }
}

/// Shortcut for applying `elvis!`
/// for `Option<T>`.
///
/// Example:
/// ```
///     let that = some_or! { wrapped => return };
/// ```
#[macro_export]
macro_rules! some_or {
    ( $target:expr => $otherwise:expr ) => {
        elvis! { it, Some(it) = $target => $otherwise }
    }
}

/// Shortcut for applying `elvis!`
/// for `Result<T, E>`.
///
/// Example:
/// ```
///     let that = result_or! { wrapped => return };
/// ```
#[macro_export]
macro_rules! result_or {
    ( $target:expr => $otherwise:expr ) => {
        elvis! { it, Ok(it) = $target => $otherwise }
    }
}
