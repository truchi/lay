////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

pub use Border::*;

/// `Border` (`Circle`, `Frame`, `ResetBorder`).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.  
/// `Default`s to `Border::ResetBorder`, the unsetting CSI.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Border {
    Circle,
    Frame,
    ResetBorder,
}

/// Returns `Border::ResetBorder`.
impl Default for Border {
    /// Returns `Border::ResetBorder`.
    fn default() -> Self {
        Border::ResetBorder
    }
}