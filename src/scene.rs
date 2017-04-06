use primitive::Primitive;

pub trait Scene {
    fn primitive(&self) -> Primitive;
    // TODO: fn lights(&self) -> XXX;
}

pub struct SceneBase {
    // TODO: lights
    _primitive: Primitive,
}

// impl Scene {
//     // pub fn new(aggregate: &Primitive) {
//     //     self._primitive
//     // }
// }
