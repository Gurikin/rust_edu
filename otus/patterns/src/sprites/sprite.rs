use crate::visitors::{dyn_visitor::DynVisitor, static_visitor::StaticVisitor};

#[derive(Debug, Clone)]
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
    pub fn accept(&mut self, visitor: &impl StaticVisitor) {
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
    pub fn accept(&mut self, visitor: &impl StaticVisitor) {
        visitor.visit_animated(self);
    }
}

pub trait Sprite {
    fn set_position(&mut self, pos: &Point);
    fn get_position(&mut self) -> &mut Point;
    fn set_size(&mut self, size: &Size);
    fn get_size(&mut self) -> &mut Size;
    fn name(&mut self) -> &str;
    fn velocity(&mut self) -> Option<&mut Velocity>;
    fn accept(&mut self, v: &impl DynVisitor);
}

impl Sprite for StaticSprite {
    fn set_position(&mut self, pos: &Point) {
        self.base.pos.x = pos.x;
        self.base.pos.y = pos.y;
    }

    fn get_position(&mut self) -> &mut Point {
        &mut self.base.pos
    }

    fn set_size(&mut self, size: &Size) {
        self._size._length = size._length;
        self._size._width = size._width;
    }

    fn get_size(&mut self) -> &mut Size {
        &mut self._size
    }

    fn accept(&mut self, v: &impl DynVisitor) {
        v.visit_static(self);
    }

    fn name(&mut self) -> &str {
        &self.base.name
    }

    fn velocity(&mut self) -> Option<&mut Velocity> {
        None
    }
}

impl Sprite for AnimatedSprite {
    fn set_position(&mut self, pos: &Point) {
        self.base.pos.x = pos.x;
        self.base.pos.y = pos.y;
    }

    fn get_position(&mut self) -> &mut Point {
        &mut self.base.pos
    }

    fn set_size(&mut self, size: &Size) {
        self._size._length = size._length;
        self._size._width = size._width;
    }

    fn get_size(&mut self) -> &mut Size {
        &mut self._size
    }

    fn accept(&mut self, v: &impl DynVisitor) {
        v.visit_animated(self);
    }

    fn name(&mut self) -> &str {
        &self.base.name
    }

    fn velocity(&mut self) -> Option<&mut Velocity> {
        Some(&mut self.velocity)
    }
}
