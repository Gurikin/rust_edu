use crate::visitors::static_visitor::Visitor;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Size {
    pub _width: f32,
    pub _length: f32,
}

#[derive(Debug)]
pub struct BaseSprite {
    pub _id: i64,
    pub name: String,
    pub pos: Point,
}

#[derive(Debug)]
pub struct Vector3D {
    pub _x: f32,
    pub _y: f32,
    pub _z: f32,
}

#[derive(Debug)]
pub struct Velocity {
    pub direction: Vector3D,
    pub speed: f32,
}

#[derive(Debug)]
pub struct StaticSprite {
    pub _size: Size,
    pub base: BaseSprite,
}

impl StaticSprite {
    pub fn visit(&mut self, visitor: &impl Visitor) {
        visitor.visit_static(self);
    }
}

#[derive(Debug)]
pub struct AnimatedSprite {
    pub _size: Size,
    pub velocity: Velocity,
    pub base: BaseSprite,
}

impl AnimatedSprite {
    pub fn visit(&mut self, visitor: &impl Visitor) {
        visitor.visit_animated(self);
    }
}