use crate::game_object::Behaviour;
use crate::transform::Transform;

pub struct RandomMovement{

}

impl Behaviour for RandomMovement{
    fn update(&self, transform: &mut Transform) {

    }
}