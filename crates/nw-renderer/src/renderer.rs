use std::sync::Arc;

use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder,
        CommandBufferUsage,
        RenderPassBeginInfo,
        SubpassBeginInfo,
        SubpassContents,
        SubpassEndInfo,
        allocator::StandardCommandBufferAllocator,
    },
    device::{Device, Queue},
    image::view::ImageView,
    pipeline::GraphicsPipeline,
    render_pass::{
        Framebuffer,
        FramebufferCreateInfo,
    },
    swapchain::{
        self,
        Swapchain, SwapchainCreateInfo,
        SwapchainPresentInfo,
    },
    sync::GpuFuture,
    Validated,
    VulkanError,
};

use crate::{
    VulkanContext,
    SwapchainState,
    pipeline::create_pipeline,
};

pub struct Renderer {
    device: Arc<Device>,
    queue: Arc<Queue>,

    swapchain: Arc<Swapchain>,
    extent: [u32; 2],
    render_pass: Arc<vulkano::render_pass::RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    pipeline: Arc<GraphicsPipeline>,

    command_allocator: Arc<StandardCommandBufferAllocator>,
    swapchain_needs_recreation: bool,
}

impl Renderer {
    pub fn new(
        context: &VulkanContext<'_>,
        swapchain_state: SwapchainState,
    ) -> Result<Self, Box<dyn std::error::Error>> {

        let device = context.device().clone();
        let queue = context.queue().clone();

        let extent = swapchain_state.image_extent;
        let image_format = swapchain_state.swapchain.image_format();

        let swapchain = swapchain_state.swapchain;
        let images = swapchain_state.images;

        // Create a render pass for the swapchain's image format.
        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    format: image_format,
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )?;
        println!("Renderer::new() created render pass");

        // Create one framebuffer for each swapchain image.
        let mut framebuffers = Vec::new();

        for image in &images {
            let view = ImageView::new_default(image.clone())?;

            let framebuffer = Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )?;

            framebuffers.push(framebuffer);
        }

        // Create our graphics pipeline.
        println!("[Renderer] Framebuffers created");
        println!("[Renderer] Creating graphics pipeline");

        let pipeline = create_pipeline_with_larger_stack(
            device.clone(),
            render_pass.clone(),
            extent,
        )?;

        println!("[Renderer] Graphics pipeline created");

        // Allocate command buffers.
        let command_allocator = Arc::new(
            StandardCommandBufferAllocator::new(
                device.clone(),
                Default::default(),
            ),
        );
        println!("[Renderer] Initialization complete");

        Ok(Self {
            device,
            queue,
            swapchain,
            extent,
            render_pass,
            framebuffers,
            pipeline,
            command_allocator,
            swapchain_needs_recreation: false,
        })
    }

    pub fn draw_frame(
        &mut self,
        context: &VulkanContext<'_>,
        window: &sdl3::video::Window,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let requested_extent = surface_extent(context, window)?;
        if requested_extent[0] == 0 || requested_extent[1] == 0 {
            return Ok(());
        }

        if self.swapchain_needs_recreation || requested_extent != self.extent {
            self.recreate_swapchain(requested_extent)?;
        }

        // 1. Acquire an available swapchain image.
        let (image_index, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(result) => result,
                Err(Validated::Error(VulkanError::OutOfDate)) => {
                    self.swapchain_needs_recreation = true;
                    return Ok(());
                }
                Err(error) => return Err(Box::new(error)),
            };

        let framebuffer =
            self.framebuffers[image_index as usize].clone();

        // 2. Start recording GPU commands.
        let mut builder = AutoCommandBufferBuilder::primary(
            self.command_allocator.clone(),
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        // 3. Begin the render pass and clear the background.
        builder.begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![
                    Some([0.02, 0.02, 0.04, 1.0].into()),
                ],
                ..RenderPassBeginInfo::framebuffer(
                    framebuffer,
                )
            },
            SubpassBeginInfo {
                contents: SubpassContents::Inline,
                ..Default::default()
            },
        )?;

        // 4. Bind the graphics pipeline and draw 3 vertices.
        builder.bind_pipeline_graphics(self.pipeline.clone())?;

        // SAFETY:
        // Our vertex shader uses gl_VertexIndex to access exactly
        // three vertices (indices 0, 1, and 2).
        // Both shader arrays contain exactly three elements.
        unsafe {
            builder.draw(3, 1, 0, 0)?;
        }

        // 5. Finish the render pass.
        builder.end_render_pass(
            SubpassEndInfo::default(),
        )?;

        // 6. Build the command buffer.
        let command_buffer = builder.build()?;

        // 7. Submit the commands and present the image.
        let flush_result = acquire_future
            .then_execute(
                self.queue.clone(),
                command_buffer,
            )?
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    self.swapchain.clone(),
                    image_index,
                ),
            )
            .then_signal_fence_and_flush();

        match flush_result {
            Ok(future) => match future.wait(None) {
                Ok(()) => {}
                Err(Validated::Error(VulkanError::OutOfDate)) => {
                    self.swapchain_needs_recreation = true;
                    return Ok(());
                }
                Err(error) => return Err(Box::new(error)),
            },
            Err(Validated::Error(VulkanError::OutOfDate)) => {
                self.swapchain_needs_recreation = true;
                return Ok(());
            }
            Err(error) => return Err(Box::new(error)),
        }

        self.swapchain_needs_recreation = suboptimal;

        Ok(())
    }

    fn recreate_swapchain(
        &mut self,
        extent: [u32; 2],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (swapchain, images) = self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: extent,
            ..self.swapchain.create_info()
        })?;

        let framebuffers = create_framebuffers(&images, self.render_pass.clone())?;
        let pipeline = create_pipeline_with_larger_stack(
            self.device.clone(),
            self.render_pass.clone(),
            extent,
        )?;

        self.swapchain = swapchain;
        self.extent = extent;
        self.framebuffers = framebuffers;
        self.pipeline = pipeline;
        self.swapchain_needs_recreation = false;

        Ok(())
    }
}

fn create_framebuffers(
    images: &[Arc<vulkano::image::Image>],
    render_pass: Arc<vulkano::render_pass::RenderPass>,
) -> Result<Vec<Arc<Framebuffer>>, Box<dyn std::error::Error>> {
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone())?;
            Ok(Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )?)
        })
        .collect()
}

fn create_pipeline_with_larger_stack(
    device: Arc<Device>,
    render_pass: Arc<vulkano::render_pass::RenderPass>,
    extent: [u32; 2],
) -> Result<Arc<GraphicsPipeline>, Box<dyn std::error::Error>> {
    let result = std::thread::Builder::new()
        .name("graphics-pipeline".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            create_pipeline(device, render_pass, extent)
                .map_err(|error| error.to_string())
        })?
        .join()
        .map_err(|_| {
            std::io::Error::other("Graphics pipeline creation thread panicked")
        })?;

    match result {
        Ok(pipeline) => Ok(pipeline),
        Err(error) => Err(std::io::Error::other(error).into()),
    }
}

fn surface_extent(
    context: &VulkanContext<'_>,
    window: &sdl3::video::Window,
) -> Result<[u32; 2], Box<dyn std::error::Error>> {
    let capabilities = context.device().physical_device().surface_capabilities(
        context.surface(),
        Default::default(),
    )?;
    let (width, height) = window.size_in_pixels();
    let mut extent = capabilities.current_extent.unwrap_or([width, height]);

    extent[0] = extent[0].clamp(
        capabilities.min_image_extent[0],
        capabilities.max_image_extent[0],
    );
    extent[1] = extent[1].clamp(
        capabilities.min_image_extent[1],
        capabilities.max_image_extent[1],
    );

    Ok(extent)
}