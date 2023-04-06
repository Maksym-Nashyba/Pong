use std::sync::Arc;
use vulkano::shader::ShaderModule;
use crate::renderer::model::Model;

pub struct Material{
    pub shader:Arc<ShaderModule>,
    pub model:Model
}