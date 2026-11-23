use wasm_bindgen::prelude::wasm_bindgen;

// `clippy::inline_modules` (restriction, canon #445): a module lives in
// its own file, self-named rather than `mod.rs`.
#[cfg(test)]
mod tests;

/// The one thing the package exists to do, callable from JS.
// `missing_const_for_fn` is impossible to satisfy rather than merely
// unwanted: `#[wasm_bindgen]` refuses outright — "can only
// #[wasm_bindgen] non-const functions".
#[expect(
    clippy::missing_const_for_fn,
    reason = "#[wasm_bindgen] rejects const fn"
)]
#[inline]
#[must_use]
#[wasm_bindgen]
pub fn answer() -> u32 {
    lab_core::answer()
}
