//! What became of a patched site in the title's code. Only the shared
//! outcome lives here: the frame-rate switch itself (Belobug's
//! `frame_rate.rs`) was not ported.

/// What became of a site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Patched {
    Done,
    /// Its bytes are not the ones expected: left alone.
    Differs,
    Failed(&'static str),
}
