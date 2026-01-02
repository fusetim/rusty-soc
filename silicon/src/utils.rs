/// This trait provides a method to unwrap a Result to void, ignoring the error.
/// 
/// This is useful when you can be sure that the result will be Ok, and you don't care about the error case.
/// Or when you want to explicitly ignore the error, unwrapping it, and discarding any error information (
/// as it would not be useful to you -- and take a lot of space in the final binary).
/// 
/// Basically do not use this unless you really are missing space in the final binary, and are sure that
/// the error case will not happen, or you do not care about it.
pub trait VoidUnwrap<T> {
    /// Unwrap to void, ignoring the error.
    /// 
    /// It is essentially a shortcut for `.map_err(|_| ()).unwrap()`.
    fn void_unwrap(self) -> T;
}

impl<T, E> VoidUnwrap<T> for Result<T, E> {
    #[inline(always)]
    fn void_unwrap(self) -> T {
        self.or(Err(())).unwrap()
    }
}