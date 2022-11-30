//! This module contains the backend of the Papyri compiler; it is responsible
//! for compiling an abstract syntax tree into HTML (or plain text).

mod compiler;
mod frame;
mod func;
mod highlight;
mod highlight_papyri;
mod html;
mod loader;
mod matcher;
mod native;
mod render;
mod sequence;
mod tag;
mod types;
mod value;

pub use compiler::{compile, CompileResult};
pub use html::HTML;
pub use loader::ModuleLoader;
pub use render::Renderer;
pub use types::Type;
pub use value::Value;
