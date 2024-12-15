use patterns::{
    sprites::sprite::{
        AnimatedSprite, BaseSprite, Point, Size, StaticSprite, Vector3D, Velocity,
    },
    visitors::static_visitor::{ResetVelocityVisitor, StepRightPositionVisitor},
};

fn main() {
    let mut static_sprite = StaticSprite {
        base: BaseSprite {
            _id: 1,
            name: "staticSprite".to_string(),
            pos: Point { x: 0.0, y: 10.0 },
        },
        _size: Size {
            _width: 10.0,
            _length: 10.0,
        },
    };
    let mut animated_sprite = AnimatedSprite {
        base: BaseSprite {
            _id: 2,
            name: "animatedSprite".to_string(),
            pos: Point { x: 0.0, y: 10.0 },
        },
        _size: Size {
            _width: 10.0,
            _length: 10.0,
        },
        velocity: Velocity {
            direction: Vector3D {
                _x: 1.0,
                _y: 0.0,
                _z: 0.0,
            },
            speed: 1.0,
        },
    };

    println!("\n\n\n========================= Base state =========================");
    println!("{:?}", static_sprite);
    println!("{:?}", animated_sprite);
    println!("==============================================================\n\n\n");

    let step_right_visitor = StepRightPositionVisitor {};
    let reset_velocity_visitor = ResetVelocityVisitor {};

    println!("========================= Static sprite operations =========================");
    static_sprite.accept(&step_right_visitor);
    static_sprite.accept(&reset_velocity_visitor);
    println!("============================================================================\n\n\n");

    println!("========================= Animated sprite operations =========================");
    animated_sprite.accept(&step_right_visitor);
    animated_sprite.accept(&reset_velocity_visitor);
    println!(
        "==============================================================================\n\n\n"
    );
}
