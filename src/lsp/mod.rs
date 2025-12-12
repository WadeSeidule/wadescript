/// WadeScript Language Server Protocol implementation
pub mod span;
pub mod server;
pub mod document;
pub mod diagnostics;
pub mod analysis;

pub use server::run_server;
