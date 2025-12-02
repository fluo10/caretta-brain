#[cfg(feature = "macros")]
pub use caretta_brain_macros::Mergeable;
pub trait Mergeable<T = Self>: Sized {
    fn merge(&mut self, other: T);
}
