use geometry::shape::Shape;


pub struct Primitive {
    _shape: Shape,
    // TODO: _material: Material;
}

impl Primitive {
    #[inline] pub fn shape(&self) -> Shape { self._shape }
    // TODO: #[inline] pub fn material(&self) -> Material { self._shape }
}
