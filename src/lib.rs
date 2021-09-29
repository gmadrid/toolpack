mod lineproc;
pub use lineproc::{process_bufread, process_stdin};

mod spew;
pub use spew::{set_level, spew_at_level, SpewLevel};

mod tperr;
pub use tperr::{TPError, TPResult};

type Result<T> = TPResult<T>;
