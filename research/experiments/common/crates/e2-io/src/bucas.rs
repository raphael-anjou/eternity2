//! The legacy `e2.bucas.name` viewer URL, kept as a derived converter for
//! community interop. It is no longer any algorithm's default output — the
//! canonical link is the eternity2.dev URL from [`crate::format::viewer_url`].
//! This module is a thin compatibility shim over the core-agnostic builders in
//! [`crate::format`].

/// Render a grid of per-cell URDL edge colors as a legacy bucas viewer URL.
/// Delegates to [`crate::format::bucas_url`]; identical output to the previous
/// hand-rolled encoder, so existing round-trip tests still hold.
#[must_use]
pub fn board_to_bucas_url(name: &str, width: u8, height: u8, cells: &[[u8; 4]]) -> String {
    crate::format::bucas_url(name, width, height, cells)
}
