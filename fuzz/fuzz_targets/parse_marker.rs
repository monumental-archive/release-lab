//! Round-trip invariant: any input `parse_version_marker` accepts must
//! re-render to a string it accepts again with the same value — and no
//! input may panic it.
//!
//! Built-value oracle: three `u32`s read little-endian from the input's
//! first twelve bytes (a missing byte counts as zero) render to
//! `v{a}.{b}.{c}`, which must parse to exactly `Some((a, b, c))`.
#![no_main]
use core::str;

use lab_core::parse_version_marker;
use libfuzzer_sys::fuzz_target;

/// The `index`th little-endian `u32` of `data`, its missing bytes zero.
fn word(data: &[u8], index: usize) -> u32 {
    let mut bytes = [0_u8; 4];
    if let Some(chunk) = data.chunks(4).nth(index) {
        for (slot, byte) in bytes.iter_mut().zip(chunk) {
            *slot = *byte;
        }
    }
    u32::from_le_bytes(bytes)
}

fuzz_target!(|data: &[u8]| {
    let built = (word(data, 0), word(data, 1), word(data, 2));
    let (major, minor, patch) = built;
    assert_eq!(
        parse_version_marker(&format!("v{major}.{minor}.{patch}")),
        Some(built),
        "a rendered marker must parse to the words it was built from"
    );

    if let Ok(text) = str::from_utf8(data) {
        if let Some(parsed) = parse_version_marker(text) {
            let (maj, min, pat) = parsed;
            assert_eq!(
                parse_version_marker(&format!("v{maj}.{min}.{pat}")),
                Some(parsed),
                "an accepted marker must re-render to the same value"
            );
        }
    }
});
