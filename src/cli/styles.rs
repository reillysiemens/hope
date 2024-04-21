//! Clap color and text styling.

use clap::builder::{styling::AnsiColor, Styles};

// Clap v4 defaults to a colorless style. This emulates the colored v3 style.
pub(super) const STYLES: Styles = Styles::styled()
    .usage(AnsiColor::Yellow.on_default().underline())
    .header(AnsiColor::Yellow.on_default().underline())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::White.on_default());
