use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct GeometryType {
    pub doc:    Doc,
    pub name:   Ident,
    pub fields: Vec<Ident>,
}

impl GeometryType {
    pub fn new(doc: &str, name: &str, fields: &[&Ident]) -> Self {
        Self {
            doc:    doc!("{}", doc),
            name:   Ident::new(&[name]),
            fields: fields.iter().map(|t| (*t).clone()).collect(),
        }
    }
}

derefs!(self GeometryType {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

#[derive(Clone, Debug)]
pub struct Geometry {
    pub x:        GeometryType,
    pub y:        GeometryType,
    pub width:    GeometryType,
    pub height:   GeometryType,
    pub position: GeometryType,
    pub size:     GeometryType,
    pub rect:     GeometryType,
    pub ones:     Vec<GeometryType>,
    pub twos:     Vec<GeometryType>,
}

impl Geometry {
    pub fn new() -> Self {
        let usize = Ident {
            snake:  Str::new("0"),
            pascal: Str::new("usize"),
        };
        let usize = &[&usize];

        let x = GeometryType::new("An [`X`](crate::X) coordinate.", "X", usize);
        let y = GeometryType::new("An [`Y`](crate::Y) coordinate.", "Y", usize);
        let width = GeometryType::new("A [`Width`](crate::Width) distance.", "Width", usize);
        let height = GeometryType::new("An [`Height`](crate::Height) distance.", "Height", usize);

        let position =
            GeometryType::new("A `(x, y)` [`Position`](crate::Position).", "Position", &[
                &x, &y,
            ]);
        let size = GeometryType::new("A `(width, height)` [`Size`](crate::Size).", "Size", &[
            &width, &height,
        ]);
        let rect = GeometryType::new("A `(position, size)` [`Size`](crate::Size).", "Rect", &[
            &position, &size,
        ]);

        let ones = vec![x.clone(), y.clone(), width.clone(), height.clone()];
        let twos = vec![position.clone(), size.clone()];

        Self {
            x,
            y,
            width,
            height,
            position,
            size,
            rect,
            ones,
            twos,
        }
    }

    pub fn rect_fields(&self) -> Vec<(&Ident, Vec<Ident>)> {
        self.rect
            .fields
            .iter()
            .map(|field| {
                (
                    field,
                    self.twos
                        .iter()
                        .find(|two| two.snake == field.snake)
                        .unwrap()
                        .fields
                        .clone(),
                )
            })
            .collect()
    }
}
