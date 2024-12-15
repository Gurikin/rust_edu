use crate::sprites::sprite::{Sprite, Vector3D};

pub trait DynVisitor {
    fn visit_static<T: Sprite>(&self, static_sprite: &mut T);
    fn visit_animated<T: Sprite>(&self, animated_sprite: &mut T);
}

pub struct StepRightPositionVisitor {}

impl DynVisitor for StepRightPositionVisitor {
    fn visit_static<T: Sprite>(&self, static_sprite: &mut T) {
        println!("Move `{}` to the right one px", static_sprite.name());
        let mut pos = static_sprite.get_position().clone();
        pos.x += 1.0;
        static_sprite.set_position(&pos);
        println!("New position: {:?}", pos)
    }

    fn visit_animated<T: Sprite>(&self, animated_sprite: &mut T) {
        println!("Move `{}` to the right one px", animated_sprite.name());
        let mut pos = animated_sprite.get_position().clone();
        pos.x += 1.0;
        animated_sprite.set_position(&pos);
        println!("New position: {:?}", pos)
    }
}

pub struct ResetVelocityVisitor {}

impl DynVisitor for ResetVelocityVisitor {
    fn visit_static<T: Sprite>(&self, _: &mut T) {
        println!("`Change velocity` is unsupported operation");
    }

    fn visit_animated<T: Sprite>(&self, animated_sprite: &mut T) {
        println!("Reset velocity for the `{}` sprite", animated_sprite.name());
        let velocity = match animated_sprite.velocity() {
            Some(v) => {
                v.direction = Vector3D {
                    _x: 0.0,
                    _y: 0.0,
                    _z: 0.0,
                };
                v.speed = 0.0;
                Ok(v)
            }
            None => Err(format!(
                "Animated sprinte {} have not setted velocity",
                animated_sprite.name()
            )),
        };
        println!("New velocity: {:?}", velocity)
    }
}
