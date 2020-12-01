use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct GeometryType {
    pub name:   Ident,
    pub fields: Vec<Ident>,
}

impl GeometryType {
    pub fn new(name: &str, fields: &[&Ident]) -> Self {
        Self {
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
    pub x:      GeometryType,
    pub y:      GeometryType,
    pub width:  GeometryType,
    pub height: GeometryType,
    pub point:  GeometryType,
    pub rect:   GeometryType,
    pub size:   GeometryType,
    pub all:    Vec<GeometryType>,
}

impl Geometry {
    pub fn new() -> Self {
        let usize = Ident {
            snake:  Str::new("0"),
            pascal: Str::new("usize"),
        };
        let usize = &[&usize];

        let x = GeometryType::new("X", usize);
        let y = GeometryType::new("Y", usize);
        let width = GeometryType::new("Width", usize);
        let height = GeometryType::new("Height", usize);

        let point = GeometryType::new("Point", &[&x, &y]);
        let size = GeometryType::new("Size", &[&width, &height]);

        let rect = GeometryType::new("Rect", &[&point, &size]);

        let all = vec![
            x.clone(),
            y.clone(),
            width.clone(),
            height.clone(),
            point.clone(),
            size.clone(),
            rect.clone(),
        ];

        Self {
            x,
            y,
            width,
            height,
            point,
            size,
            rect,
            all,
        }
    }
}
