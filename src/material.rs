use std::sync::Arc;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::shader::ShaderModule;
use crate::renderer::model::Model;
use crate::renderer::Renderer;

#[derive(Clone)]
pub struct Material{
    vertex_shader:Arc<ShaderModule>,
    fragment_shader:Arc<ShaderModule>,
    pipeline:Arc<GraphicsPipeline>
}

impl Material {
    pub fn new(renderer:&Renderer, vertex_shader:Arc<ShaderModule>, fragment_shader:Arc<ShaderModule>) -> Self{
        let pipeline:Arc<GraphicsPipeline>
            = renderer.build_pipeline(vertex_shader.clone(), fragment_shader.clone());
        return Self{
            vertex_shader:vertex_shader,
            fragment_shader:fragment_shader,
            pipeline:pipeline
        };
    }

    pub fn pipeline(&self) -> Arc<GraphicsPipeline>{
        return self.pipeline.clone();
    }
}