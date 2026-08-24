::pgrx::pg_module_magic!();

// The harness `cargo pgrx test` reaches for by name. pgrx requires the
// module and the two function names; where the module LIVES is ours, and
// `inline_modules` says beside this file rather than inside it — the
// self-named layout `mod_module_files` picks, as in lab-core.
#[cfg(test)]
pub mod pg_test;

/// The one thing the extension exists to do, callable from SQL.
// Every path into this crate is a SQL statement, and no lint can see one.
// The single caller clippy does see is the wrapper `#[pg_extern]`
// generates, which is the extension's whole point rather than a helper
// worth inlining into its user.
// Conditioned on the build the lint actually fires in, the way
// lab-core conditions its own pair: an `#[expect]` must hold in every
// configuration compiled, and wherever the test module is compiled the
// test below is a second caller and the lint goes quiet.
#[cfg_attr(
    not(any(test, feature = "pg_test")),
    expect(
        clippy::single_call_fn,
        reason = "the one in-crate caller is the SQL wrapper #[pg_extern] generates"
    )
)]
#[::pgrx::pg_extern]
const fn lab_answer() -> i32 {
    // `cast_signed` rather than `as`: the org ladder refuses a silent
    // conversion and `i32::try_from` is not const. Nothing is lost —
    // lab-core answers a literal 42, three orders inside `i32::MAX` —
    // and the method name says the reinterpretation out loud.
    lab_core::answer().cast_signed()
}

/// Names the version, so an upgraded installation can say what it runs.
#[expect(
    clippy::single_call_fn,
    reason = "the one in-crate caller is the SQL wrapper #[pg_extern] generates"
)]
#[::pgrx::pg_extern]
fn lab_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

// `cfg(any(test, feature = "pg_test"))` is pgrx's requirement, not a
// choice: `cargo pgrx test` compiles these INTO the extension under that
// feature. `tests_outside_test_module` wants a bare `cfg(test)` and is
// the belt's eleventh named contradiction for exactly this reason.
//
// Every suppression below sits on its ITEM. An inner `#![expect(...)]`
// here would not survive `#[pg_schema]`: syn re-emits it as an outer
// attribute and the build dies, which is what burned edtf v1.3.0
// (edtf#185) while plain rustc accepted the same source.
#[cfg(any(test, feature = "pg_test"))]
#[::pgrx::pg_schema]
mod tests {
    // `missing_panics_doc` reaches private items because the canon sets
    // `check-private-items = true` — right for a helper, wrong for a
    // test: asserting IS the contract and there is no caller to warn.
    // The same expect, for the same reason, sits on lab-core's tests.
    #[expect(
        clippy::missing_panics_doc,
        reason = "a test panics by design; there is no caller to warn"
    )]
    #[expect(
        clippy::single_call_fn,
        reason = "the one caller is the runner #[pg_test] generates"
    )]
    #[::pgrx::pg_test]
    fn answers() {
        assert_eq!(
            crate::lab_answer(),
            42_i32,
            "the extension must answer through SQL what lab-core answers in Rust"
        );
    }
}
