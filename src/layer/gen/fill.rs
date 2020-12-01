
    ////////////////////////////////////////////////////////////////////////////////
    // 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
    // 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
    ////////////////////////////////////////////////////////////////////////////////




 use crate :: * ; 

 

 impl < > StylerIndex for Fill { /// Gets `Option<Foreground>`.
 fn get_foreground (& self) -> Option < Foreground > { StylerIndex :: get_foreground (& self . cell) } /// Gets `Option<Background>`.
 fn get_background (& self) -> Option < Background > { StylerIndex :: get_background (& self . cell) } /// Gets `Option<Weight>`.
 fn get_weight (& self) -> Option < Weight > { StylerIndex :: get_weight (& self . cell) } /// Gets `Option<Slant>`.
 fn get_slant (& self) -> Option < Slant > { StylerIndex :: get_slant (& self . cell) } /// Gets `Option<Underline>`.
 fn get_underline (& self) -> Option < Underline > { StylerIndex :: get_underline (& self . cell) } /// Gets `Option<Strike>`.
 fn get_strike (& self) -> Option < Strike > { StylerIndex :: get_strike (& self . cell) } /// Gets `Option<Overline>`.
 fn get_overline (& self) -> Option < Overline > { StylerIndex :: get_overline (& self . cell) } /// Gets `Option<Invert>`.
 fn get_invert (& self) -> Option < Invert > { StylerIndex :: get_invert (& self . cell) } /// Gets `Option<Blink>`.
 fn get_blink (& self) -> Option < Blink > { StylerIndex :: get_blink (& self . cell) } /// Gets `Option<Border>`.
 fn get_border (& self) -> Option < Border > { StylerIndex :: get_border (& self . cell) } } 

 impl < > Styler for Fill { /// Sets `Option<Foreground>`.
 fn foreground (self , foreground : impl Into < Option < Foreground >>) -> Self { Styler :: foreground (self . cell , foreground) ; self } /// Sets `Option<Background>`.
 fn background (self , background : impl Into < Option < Background >>) -> Self { Styler :: background (self . cell , background) ; self } /// Sets `Option<Weight>`.
 fn weight (self , weight : impl Into < Option < Weight >>) -> Self { Styler :: weight (self . cell , weight) ; self } /// Sets `Option<Slant>`.
 fn slant (self , slant : impl Into < Option < Slant >>) -> Self { Styler :: slant (self . cell , slant) ; self } /// Sets `Option<Underline>`.
 fn underline (self , underline : impl Into < Option < Underline >>) -> Self { Styler :: underline (self . cell , underline) ; self } /// Sets `Option<Strike>`.
 fn strike (self , strike : impl Into < Option < Strike >>) -> Self { Styler :: strike (self . cell , strike) ; self } /// Sets `Option<Overline>`.
 fn overline (self , overline : impl Into < Option < Overline >>) -> Self { Styler :: overline (self . cell , overline) ; self } /// Sets `Option<Invert>`.
 fn invert (self , invert : impl Into < Option < Invert >>) -> Self { Styler :: invert (self . cell , invert) ; self } /// Sets `Option<Blink>`.
 fn blink (self , blink : impl Into < Option < Blink >>) -> Self { Styler :: blink (self . cell , blink) ; self } /// Sets `Option<Border>`.
 fn border (self , border : impl Into < Option < Border >>) -> Self { Styler :: border (self . cell , border) ; self } } 

 impl < > StylerMut for Fill { /// Sets `Option<Foreground>`, mutably.
 fn foreground_mut (& mut self , foreground : impl Into < Option < Foreground >>) { StylerMut :: foreground_mut (& mut self . cell , foreground) ; } /// Sets `Option<Background>`, mutably.
 fn background_mut (& mut self , background : impl Into < Option < Background >>) { StylerMut :: background_mut (& mut self . cell , background) ; } /// Sets `Option<Weight>`, mutably.
 fn weight_mut (& mut self , weight : impl Into < Option < Weight >>) { StylerMut :: weight_mut (& mut self . cell , weight) ; } /// Sets `Option<Slant>`, mutably.
 fn slant_mut (& mut self , slant : impl Into < Option < Slant >>) { StylerMut :: slant_mut (& mut self . cell , slant) ; } /// Sets `Option<Underline>`, mutably.
 fn underline_mut (& mut self , underline : impl Into < Option < Underline >>) { StylerMut :: underline_mut (& mut self . cell , underline) ; } /// Sets `Option<Strike>`, mutably.
 fn strike_mut (& mut self , strike : impl Into < Option < Strike >>) { StylerMut :: strike_mut (& mut self . cell , strike) ; } /// Sets `Option<Overline>`, mutably.
 fn overline_mut (& mut self , overline : impl Into < Option < Overline >>) { StylerMut :: overline_mut (& mut self . cell , overline) ; } /// Sets `Option<Invert>`, mutably.
 fn invert_mut (& mut self , invert : impl Into < Option < Invert >>) { StylerMut :: invert_mut (& mut self . cell , invert) ; } /// Sets `Option<Blink>`, mutably.
 fn blink_mut (& mut self , blink : impl Into < Option < Blink >>) { StylerMut :: blink_mut (& mut self . cell , blink) ; } /// Sets `Option<Border>`, mutably.
 fn border_mut (& mut self , border : impl Into < Option < Border >>) { StylerMut :: border_mut (& mut self . cell , border) ; } }