// This is required to be able to use the rustc crates.
#![feature(rustc_private)]

// This is only required for the `rustc_*` crates. Regular dependencies can be used without it.
extern crate rustc_driver;
extern crate rustc_interface;

use rustc_driver::{catch_with_exit_code, Callbacks, Compilation, RunCompiler};
use rustc_interface::{interface::Compiler, Queries};

use std::{env::args, process::exit};

/// Custom Compiler callbacks.s
pub(crate) struct CustomCallbacks;

impl Callbacks for CustomCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        println!("Hello from rustc after analysis");
        Compilation::Continue
    }
}

/// Run the compiler with custom callbacks and return the exit status code.
pub fn run_compiler(args: Vec<String>) -> i32 {
    catch_with_exit_code(move || RunCompiler::new(&args, &mut CustomCallbacks).run())
}

/// Get the path to the sysroot of the current rustup toolchain. Return `None` if the rustup
/// environment variables are not set.
fn sysroot() -> Option<String> {
    let home = option_env!("RUSTUP_HOME")?;
    let toolchain = option_env!("RUSTUP_TOOLCHAIN")?;
    Some(format!("{}/toolchains/{}", home, toolchain))
}

fn main() {
    // Get the arguments from the command line.
    let mut args: Vec<String> = args().collect();
    // Add the sysroot path to the arguments.
    args.push("--sysroot".into());
    args.push(sysroot().expect("rustup is not installed."));
    // Run the rust compiler with the arguments.
    let exit_code = run_compiler(args);
    // Exit with the exit code returned by the compiler.
    exit(exit_code)
}
