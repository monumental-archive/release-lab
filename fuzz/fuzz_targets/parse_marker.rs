//! Round-trip invariant: any input parse_version_marker accepts must
//! re-render to a string it accepts again with the same value — and no
//! input may panic it.
//!
//! Built-value oracle: three `u32`s read little-endian from the input's
//! first twelve bytes (a missing byte counts as zero) render to
//! `v{a}.{b}.{c}`, which must parse to exactly `Some((a, b, c))`.
#![no_main]
use libfuzzer_sys::fuzz_target;

fn word(data: &[u8], index: usize) -> u32 {
    let mut bytes = [0u8; 4];
    for (slot, byte) in bytes.iter_mut().zip(data.iter().skip(index * 4)) {
        *slot = *byte;
    }
    u32::from_le_bytes(bytes)
}

fuzz_target!(|data: &[u8]| {
    let (a, b, c) = (word(data, 0), word(data, 1), word(data, 2));
    assert_eq!(
        lab_core::parse_version_marker(&format!("v{a}.{b}.{c}")),
        Some((a, b, c))
    );

    if let Ok(s) = std::str::from_utf8(data) {
        if let Some((maj, min, pat)) = lab_core::parse_version_marker(s) {
            let rendered = format!("v{maj}.{min}.{pat}");
            assert_eq!(
                lab_core::parse_version_marker(&rendered),
                Some((maj, min, pat))
            );
        }
    }
});
