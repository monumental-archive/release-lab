//! The binary's one observable act, run the way a user runs it.
//!
//! Nothing ran the binary before the first mutation run (canon #1355):
//! replacing `main` with `()` survived, because the banner is built and
//! tested in lab-core while the line the CLI exists to print was
//! checked by no one.

/// Runs the built binary and holds it to the banner lab-core renders.
#[cfg(test)]
mod tests {
    use std::env::consts::ARCH;
    use std::process::Command;

    #[expect(
        clippy::missing_panics_doc,
        reason = "a test panics by design; there is no caller to warn"
    )]
    #[test]
    fn prints_the_banner() {
        let output = Command::new(env!("CARGO_BIN_EXE_lab-cli")).output();
        assert!(
            output.as_ref().is_ok_and(|run| run.status.success()),
            "the binary must run and exit 0: {output:?}"
        );
        let expected = format!("{}\n", lab_core::banner(env!("CARGO_PKG_VERSION"), ARCH));
        assert_eq!(
            output.ok().map(|run| run.stdout),
            Some(expected.into_bytes()),
            "the binary must print exactly the banner"
        );
    }
}
