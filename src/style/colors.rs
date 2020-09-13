color!(
    /// A `Foreground` `Color`.
    Foreground,
    SetForegroundColor
);

color!(
    /// A `Background` `Color`.
    Background,
    SetBackgroundColor
);

/// Sets `Option<Foreground>` & `Option<Background>` fields to `None`.
pub struct NoColor;
