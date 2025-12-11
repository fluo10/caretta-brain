#[cfg(feature = "macros")]
pub use caretta_agent_macros::Mergeable;
pub trait Mergeable<T = Self>: Sized {
    fn merge(&mut self, other: T);
}
