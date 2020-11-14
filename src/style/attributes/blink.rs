////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

pub use Blink::*;

/// `Blink` (`Slow`, `Fast`, `ResetBlink`).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.  
/// `Default`s to `Blink::ResetBlink`, the unsetting CSI.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Blink {
    Slow,
    Fast,
    ResetBlink,
}

/// Returns `Blink::ResetBlink`.
impl Default for Blink {
    /// Returns `Blink::ResetBlink`.
    fn default() -> Self {
        Blink::ResetBlink
    }
}
