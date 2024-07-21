#![cfg_attr(feature = "no_std", no_std)]
#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro)]

extern crate alloc;
extern crate wain_ast;

pub mod trap;

mod cast;
mod globals;
mod import;
mod memory;
mod runtime;
mod stack;
mod table;
mod value;

#[cfg(not(feature = "no_std"))]
pub use import::DefaultImporter;

pub use import::{check_func_signature, ImportInvalidError, ImportInvokeError, Importer};
pub use memory::Memory;
pub use runtime::Runtime;
pub use stack::Stack;
pub use value::Value;

#[cfg(not(feature = "no_std"))]
use std::io;

#[cfg(not(feature = "no_std"))]
use trap::Result;

#[cfg(not(feature = "no_std"))]
use wain_ast::Module;

/// A convenient function to execute a WebAssembly module.
///
/// This function takes parsed and validated WebAssembly module and it invokes a start function if
/// presents. Otherwise it invokes a function exported as '_start' with no argument.
///
/// For standard I/O speed, this function locks io::Stdin and io::Stdout objects because currently
/// getchar() and putchar() don't buffer its input/output. This behavior may change in the future.
///
/// If the behavior is not acceptable, please make an abstract machine runtime with
/// Runtime::instantiate.
///
/// You will need importer for initializing Runtime struct. Please use DefaultImporter::with_stdio()
/// or make your own importer struct which implements Importer trait.
///
/// Please note that this function is not available when `no_std` feature is enabled. You need to create your own `DefaultImporter` and `Runtime` instances using the avaliable STDIO streams on your device in this case.
#[cfg(not(feature = "no_std"))]
pub fn execute(module: &Module<'_>) -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let importer = DefaultImporter::with_stdio(stdin.lock(), stdout.lock());
    let mut runtime = Runtime::instantiate(module, importer)?;
    if runtime.module().entrypoint.is_none() {
        runtime.invoke("_start", &[])?;
    }
    Ok(())
}
