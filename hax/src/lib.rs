#![feature(const_trait_impl)]

pub use feature::*;
pub use hax_macros::*;
pub use mem::*;

#[doc(hidden)]
pub use linkme::distributed_slice as init_fn;

mod feature;
mod mem;

#[cfg(feature = "external")]
pub type Memory = mem::ExternalMemory;

#[cfg(feature = "internal")]
pub type Memory = mem::InternalMemory;

#[cfg(all(feature = "external", feature = "internal"))]
compile_error!(
    "Only one of the features `external` and `internal` can be enabled at the same time."
);
