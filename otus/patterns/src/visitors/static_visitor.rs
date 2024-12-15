use crate::sprites::sprite::{AnimatedSprite, StaticSprite, Vector3D};

pub trait StaticVisitor {
    fn visit_static(&self, static_sprite: &mut StaticSprite);
    fn visit_animated(&self, animated_sprite: &mut AnimatedSprite);
}

pub struct StepRightPositionVisitor {}

impl StaticVisitor for StepRightPositionVisitor {
    fn visit_static(&self, static_sprite: &mut StaticSprite) {
        println!("Move {} to the right one px", static_sprite.base.name);
        static_sprite.base.pos.x += 1.0;
        println!("{:?}", static_sprite)
    }

    fn visit_animated(&self, animated_sprite: &mut AnimatedSprite) {
        println!("Move {} to the right one px", animated_sprite.base.name);
        animated_sprite.base.pos.x += 1.0;
        println!("{:?}", animated_sprite)
    }
}

pub struct ResetVelocityVisitor {}

impl StaticVisitor for ResetVelocityVisitor {
    fn visit_static(&self, static_sprite: &mut StaticSprite) {
        println!("Unsupported operation");
        println!("{:?}", static_sprite)
    }

    fn visit_animated(&self, animated_sprite: &mut AnimatedSprite) {
        println!(
            "Reset velocity for the {} sprite",
            animated_sprite.base.name
        );
        animated_sprite.velocity.direction = Vector3D {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
        };
        animated_sprite.velocity.speed = 0.0;
        println!("{:?}", animated_sprite)
    }
}
