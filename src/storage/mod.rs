mod codec;
mod store;

pub use codec::decode;
pub use store::Storage;

#[cfg(test)]
mod tests;
