attribute!(
    /// `Weighted` text.
    Weighted: Bold(Bold) Light(Dim) + ResetWeight(NormalIntensity),
    NoWeight
);

attribute!(
    /// `Slanted` text.
    Slanted: Italic(Italic) + ResetSlant(NoItalic),
    NoSlant
);

attribute!(
    /// `Blinking` text.
    Blinking: Slow(SlowBlink) Fast(RapidBlink) + ResetBlink(NoBlink),
    NoBlink
);

attribute!(
    /// `Inverted` text.
    Inverted: Invert(Reverse) + ResetInvert(NoReverse),
    NoInvert
);

attribute!(
    /// `Striked` text.
    Striked: Strike(CrossedOut) + ResetStrike(NotCrossedOut),
    NoStrike
);

attribute!(
    /// `Underlined` text.
    Underlined: Underline(Underlined) + ResetUnderline(NoUnderline),
    NoUnderline
);

attribute!(
    /// `Overlined` text.
    Overlined: Overline(OverLined) + ResetOverline(NotOverLined),
    NoOverline
);

attribute!(
    /// `Bordered` text.
    Bordered: Frame(Framed) Circle(Encircled) + ResetBorder(NotFramedOrEncircled),
    NoBorder
);
