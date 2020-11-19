use crate::*;

#[derive(Clone, Debug)]
pub struct Lay {
    pub index:      Ident,
    pub reset:      Ident,
    pub none:       Ident,
    pub color:      Color,
    pub foreground: Attr,
    pub background: Attr,
    pub attributes: Vec<Attr>,
    pub grounds:    Vec<Attr>,
    pub all:        Vec<Attr>,
    pub styler:     Styler,
}

impl Lay {
    pub fn new() -> Self {
        let colors: Vec<(Ident, Vec<&str>)> = Self::COLORS
            .iter()
            .map(|(color, fields)| (Ident::new(color), fields.to_vec()))
            .chain(vec![(Ident::new(&[Self::RESET, Self::COLOR]), vec![])])
            .collect::<Vec<_>>();

        let attributes: Vec<(Ident, Vec<(Ident, Vec<&str>)>)> = Self::ATTRIBUTES
            .iter()
            .map(|(name, variants)| {
                let name = Ident::new(&[name]);

                let variants = variants
                    .iter()
                    .map(|variant| (Ident::new(&[variant]), vec![]))
                    .chain(vec![(Ident::new(&[Self::RESET, &name]), vec![])])
                    .collect();

                (name, variants)
            })
            .collect::<Vec<_>>();

        let (foreground, background) = Attr::grounds(colors.clone());
        let grounds = vec![foreground.clone(), background.clone()];
        let attributes = Attr::attributes(attributes);
        let all = [grounds.clone(), attributes.clone()].concat();

        let styler = Styler::new(&all);

        Self {
            index: Ident::new(&[Self::INDEX]),
            reset: Ident::new(&[Self::RESET]),
            none: Ident::new(&[Self::NONE]),
            color: Color::new(colors),
            foreground,
            background,
            attributes,
            grounds,
            all,
            styler,
        }
    }
}
