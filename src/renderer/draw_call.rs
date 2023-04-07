use std::sync::Arc;
use vulkano::shader::ShaderModule;
use crate::material::Material;
use crate::renderer::model::Model;
use crate::transform::Transform;

pub struct DrawCall{
    pub transform:Transform,
    pub model:Model,
    pub material:Material
}