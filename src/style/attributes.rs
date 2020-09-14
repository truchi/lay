attribute!(
    /// `Weighted` text.
    Weighted: Bold(Bold) Light(Dim) + ResetWeight(NormalIntensity)
);

attribute!(
    /// `Slanted` text.
    Slanted: Italic(Italic) + ResetSlant(NoItalic)
);

attribute!(
    /// `Blinking` text.
    Blinking: Slow(SlowBlink) Fast(RapidBlink) + ResetBlink(NoBlink)
);

attribute!(
    /// `Inverted` text.
    Inverted: Invert(Reverse) + ResetInvert(NoReverse)
);

attribute!(
    /// `Striked` text.
    Striked: Strike(CrossedOut) + ResetStrike(NotCrossedOut)
);

attribute!(
    /// `Underlined` text.
    Underlined: Underline(Underlined) + ResetUnderline(NoUnderline)
);

attribute!(
    /// `Overlined` text.
    Overlined: Overline(OverLined) + ResetOverline(NotOverLined)
);

attribute!(
    /// `Bordered` text.
    Bordered: Frame(Framed) Circle(Encircled) + ResetBorder(NotFramedOrEncircled)
);
