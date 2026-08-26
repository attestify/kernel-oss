//! Text bounded values used by the kernel.

pub mod block;
pub mod line;
pub(crate) mod nfc_text;

pub use nfc_text::NfcText;
