
use std::sync::Arc;

use vulkano::{
    device::Device,
    pipeline::{
        GraphicsPipeline,
        PipelineShaderStageCreateInfo,
        layout::{
            PipelineLayout,
            PipelineLayoutCreateInfo,
        },
        graphics::{
            GraphicsPipelineCreateInfo,
            input_assembly::InputAssemblyState,
            vertex_input::VertexInputState,
            viewport::{Viewport, ViewportState},
            rasterization::RasterizationState,
            multisample::MultisampleState,
            color_blend::{
                ColorBlendState,
                ColorBlendAttachmentState,
            },
        },
    },
    render_pass::{RenderPass, Subpass},
    shader::{ShaderModule, ShaderModuleCreateInfo},
};

use crate::shaders::{FRAGMENT_SHADER, VERTEX_SHADER};

pub fn create_pipeline(
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    extent: [u32; 2],
) -> Result<Arc<GraphicsPipeline>, Box<dyn std::error::Error>> {

    // Compile GLSL shaders into SPIR-V.
    let compiler = shaderc::Compiler::new()?;

    let vertex_spirv = compiler.compile_into_spirv(
        VERTEX_SHADER,
        shaderc::ShaderKind::Vertex,
        "triangle.vert",
        "main",
        None,
    )?;

    let fragment_spirv = compiler.compile_into_spirv(
        FRAGMENT_SHADER,
        shaderc::ShaderKind::Fragment,
        "triangle.frag",
        "main",
        None,
    )?;

    // Create Vulkan shader modules.
    // SAFETY: shaderc compiled both shaders into SPIR-V.
    let vertex_shader = unsafe {
        ShaderModule::new(
            device.clone(),
            ShaderModuleCreateInfo::new(vertex_spirv.as_binary()),
        )?
    };

    let fragment_shader = unsafe {
        ShaderModule::new(
            device.clone(),
            ShaderModuleCreateInfo::new(fragment_spirv.as_binary()),
        )?
    };

    let vertex_entry = vertex_shader.entry_point("main").unwrap();
    let fragment_entry = fragment_shader.entry_point("main").unwrap();

    let stages = [
        PipelineShaderStageCreateInfo::new(vertex_entry),
        PipelineShaderStageCreateInfo::new(fragment_entry),
    ];

    // No descriptor sets or push constants are needed yet.
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineLayoutCreateInfo::default(),
    )?;

    let subpass = Subpass::from(render_pass, 0).unwrap();

    // Define the area where the triangle will be drawn.
    let viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [extent[0] as f32, extent[1] as f32],
        depth_range: 0.0..=1.0,
    };

    // Create the graphics pipeline.
    let pipeline = GraphicsPipeline::new(
        device,
        None,
        GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),

            vertex_input_state: Some(
                VertexInputState::new(),
            ),

            input_assembly_state: Some(
                InputAssemblyState::default(),
            ),

            viewport_state: Some(ViewportState {
                viewports: [viewport].into_iter().collect(),
                ..Default::default()
            }),

            rasterization_state: Some(
                RasterizationState::default(),
            ),

            multisample_state: Some(
                MultisampleState::default(),
            ),

            color_blend_state: Some(
                ColorBlendState::with_attachment_states(
                    1,
                    ColorBlendAttachmentState::default(),
                ),
            ),

            subpass: Some(subpass.into()),

            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    )?;

    Ok(pipeline)
}
