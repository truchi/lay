////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;

/// A trait for getting `Option`al attributes on mutable styled types.
pub trait StylerIndexMut {
    /// Gets `&mut Option<Foreground>`.
    fn get_foreground_mut(&mut self) -> &mut Option<Foreground>;

    /// Gets `&mut Option<Background>`.
    fn get_background_mut(&mut self) -> &mut Option<Background>;

    /// Gets `&mut Option<Weight>`.
    fn get_weight_mut(&mut self) -> &mut Option<Weight>;

    /// Gets `&mut Option<Slant>`.
    fn get_slant_mut(&mut self) -> &mut Option<Slant>;

    /// Gets `&mut Option<Underline>`.
    fn get_underline_mut(&mut self) -> &mut Option<Underline>;

    /// Gets `&mut Option<Strike>`.
    fn get_strike_mut(&mut self) -> &mut Option<Strike>;

    /// Gets `&mut Option<Overline>`.
    fn get_overline_mut(&mut self) -> &mut Option<Overline>;

    /// Gets `&mut Option<Invert>`.
    fn get_invert_mut(&mut self) -> &mut Option<Invert>;

    /// Gets `&mut Option<Blink>`.
    fn get_blink_mut(&mut self) -> &mut Option<Blink>;

    /// Gets `&mut Option<Border>`.
    fn get_border_mut(&mut self) -> &mut Option<Border>;
}
