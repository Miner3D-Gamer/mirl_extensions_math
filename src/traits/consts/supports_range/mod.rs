// TODO: Add something to our proc macro mirl derive to resolve all these duplicate lines

mod range_128;
pub use range_128::*;
mod range_256;
pub use range_256::*;
mod range_512;
pub use range_512::*;

mod range_1024;
pub use range_1024::*;

mod range_2048;
pub use range_2048::*;

mod range_4096;
pub use range_4096::*;
