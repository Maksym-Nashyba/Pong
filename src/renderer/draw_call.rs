use std::sync::Arc;
use vulkano::shader::ShaderModule;
use crate::renderer::model::Model;
use crate::transform::Transform;

pub struct DrawCall{
    pub transform:Transform,
    pub model:Model,
    pub vertex_shader:Arc<ShaderModule>,
    pub fragment_shader:Arc<ShaderModule>
}