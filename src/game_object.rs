use vulkanus_3d::material::Material;
use vulkanus_3d::renderer::model::Model;
use crate::transform::Transform;

pub struct GameObject{
    pub transform: Transform,
    pub model: Option<Model>,
    pub material: Option<Material>,
    components: Vec<Box<dyn Behaviour>>
}

impl GameObject{
    pub fn new() -> Self{
        return Self{
            transform: Transform::identity(),
            model: None,
            material: None,
            components: vec![],
        }
    }

    pub fn add_behaviour(&mut self, behaviour: Box<dyn Behaviour>){
        self.components.push(behaviour);
    }

    pub fn update_components(&mut self){
        for component in &self.components {
            component.as_ref().update(&mut self.transform);
        }
    }
}

pub trait Behaviour{
    fn update(&self, transform: &mut Transform);
}