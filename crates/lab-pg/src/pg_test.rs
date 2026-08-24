//! The harness `cargo pgrx test` generates a runner against.
//!
//! It calls `crate::pg_test::setup` and
//! `crate::pg_test::postgresql_conf_options` before it starts the test
//! database, so both names and the module's name are pgrx's requirement.
//! This fixture needs neither hook to do anything; they exist because
//! the runner will not link without them.

/// Runs once before the test database starts; the fixture needs nothing.
#[expect(
    clippy::single_call_fn,
    reason = "the one caller is the runner cargo pgrx test generates"
)]
#[inline]
pub fn setup(_options: Vec<&str>) {}

/// Extra `postgresql.conf` lines the fixture needs; there are none.
#[expect(
    clippy::single_call_fn,
    reason = "the one caller is the runner cargo pgrx test generates"
)]
#[inline]
#[must_use]
pub fn postgresql_conf_options() -> Vec<&'static str> {
    Vec::new()
}
