use anyhow::{anyhow, Context, Error, Result};
use std::fmt;
use std::io;

// A simple custom error type for benchmarking.
#[derive(Debug)]
struct CustomError {
    code: u32,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "custom error (code {})", self.code)
    }
}

impl std::error::Error for CustomError {}

#[divan::bench]
fn error_from_msg_static() -> Error {
    Error::msg("something went wrong")
}

#[divan::bench]
fn error_from_msg_string() -> Error {
    Error::msg(format!("error code: {}", 42))
}

#[divan::bench]
fn error_from_anyhow_macro_static() -> Error {
    anyhow!("something went wrong")
}

#[divan::bench]
fn error_from_anyhow_macro_interpolated() -> Error {
    anyhow!("error code: {}", 42)
}

#[divan::bench]
fn error_from_std_error() -> Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Error::new(io_err)
}

#[divan::bench]
fn error_from_custom_error() -> Error {
    Error::new(CustomError { code: 404 })
}

#[divan::bench]
fn error_context_static() -> Error {
    let err: Result<()> = Err(anyhow!("root cause"));
    err.context("additional context").unwrap_err()
}

#[divan::bench]
fn error_context_lazy() -> Error {
    let err: Result<()> = Err(anyhow!("root cause"));
    err.with_context(|| format!("context for code {}", 42))
        .unwrap_err()
}

#[divan::bench]
fn error_context_deep_3_levels() -> Error {
    let err: Result<()> = Err(anyhow!("root cause"));
    err.context("level 1")
        .context("level 2")
        .context("level 3")
        .unwrap_err()
}

#[divan::bench]
fn error_downcast_ref_success() -> bool {
    let err = anyhow!(CustomError { code: 404 });
    err.downcast_ref::<CustomError>().is_some()
}

#[divan::bench]
fn error_downcast_ref_failure() -> bool {
    let err = anyhow!("a string error");
    err.downcast_ref::<CustomError>().is_some()
}

#[divan::bench]
fn error_downcast_through_context() -> bool {
    let err: Result<()> = Err(Error::new(CustomError { code: 500 }));
    let err = err.context("wrapping context").unwrap_err();
    err.downcast_ref::<CustomError>().is_some()
}

#[divan::bench]
fn error_display(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| anyhow!("something went wrong"))
        .bench_local_refs(|err| fmt::format(format_args!("{}", err)))
}

#[divan::bench]
fn error_debug(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| anyhow!("something went wrong"))
        .bench_local_refs(|err| fmt::format(format_args!("{:?}", err)))
}

#[divan::bench]
fn error_alternate_display(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let err: Result<()> = Err(anyhow!("root cause"));
            err.context("higher level context").unwrap_err()
        })
        .bench_local_refs(|err| fmt::format(format_args!("{:#}", err)))
}

#[divan::bench]
fn error_chain_length(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let err: Result<()> = Err(anyhow!("root cause"));
            err.context("level 1")
                .context("level 2")
                .context("level 3")
                .unwrap_err()
        })
        .bench_local_refs(|err| err.chain().count())
}

#[divan::bench]
fn error_root_cause(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let err: Result<()> = Err(anyhow!("root cause"));
            err.context("level 1")
                .context("level 2")
                .context("level 3")
                .unwrap_err()
        })
        .bench_local_refs(|err| err.root_cause().to_string())
}

#[divan::bench]
fn error_into_boxed_dyn() -> Box<dyn std::error::Error + Send + Sync + 'static> {
    let err = anyhow!("something went wrong");
    err.into_boxed_dyn_error()
}

#[divan::bench]
fn error_from_boxed_dyn() -> Error {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let boxed: Box<dyn std::error::Error + Send + Sync + 'static> = Box::new(io_err);
    Error::from_boxed(boxed)
}

fn main() {
    divan::main();
}
