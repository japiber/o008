use std::error::Error as StdError;

pub type BoxDynError = Box<dyn StdError + Send + Sync + 'static>;

pub struct ScopeCall<F: FnMut()> {
    pub c: F
}
impl<F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        (self.c)();
    }
}

#[macro_export]
macro_rules! defer {
    ($e:expr) => (
        let _scope_call = ScopeCall { c: || -> () { $e; } };
    )
}
