use std::sync::Arc;

use vulkano::{
    image::{Image, ImageUsage},
    swapchain::{
        CompositeAlpha, PresentMode, Swapchain,
        SwapchainCreateInfo,
    },
};

use crate::VulkanContext;

pub struct SwapchainState {
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<Image>>,
    pub image_extent: [u32; 2],
}

impl SwapchainState {
    pub fn new(
        context: &VulkanContext<'_>,
        window: &sdl3::video::Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let device = context.device();
        let surface = context.surface();

        // Ask the GPU what this window surface supports.
        let capabilities = device
            .physical_device()
            .surface_capabilities(surface, Default::default())?;

        // We need to be allowed to render into swapchain images.
        if !capabilities
            .supported_usage_flags
            .contains(ImageUsage::COLOR_ATTACHMENT)
        {
            return Err(std::io::Error::other(
                "Surface does not support color attachments",
            ).into());
        }

        // Choose the image dimensions.
        // Prefer the dimensions required by the surface.
        // Otherwise, use SDL's actual pixel dimensions.
        let (width, height) = window.size_in_pixels();

        let mut image_extent = capabilities
            .current_extent
            .unwrap_or([width, height]);
        // Clamp dimensions to the supported range.
        image_extent[0] = image_extent[0].clamp(
            capabilities.min_image_extent[0],
            capabilities.max_image_extent[0],
        );

        image_extent[1] = image_extent[1].clamp(
            capabilities.min_image_extent[1],
            capabilities.max_image_extent[1],
        );

        if image_extent[0] == 0 || image_extent[1] == 0 {
            return Err(std::io::Error::other(
                "Cannot create a swapchain for a zero-sized window",
            ).into());
        }

        // Choose the number of images.
        let mut image_count = capabilities.min_image_count.max(2);

        if let Some(maximum) = capabilities.max_image_count {
            image_count = image_count.min(maximum);
        }

        // Choose a supported image format and color space.
        let formats = device
            .physical_device()
            .surface_formats(surface, Default::default())?;

        let (image_format, image_color_space) = formats
            .first()
            .copied()
            .ok_or_else(|| {
                std::io::Error::other("No surface formats available")
            })?;

        // Choose a supported transparency mode.
        let supported_alpha = capabilities.supported_composite_alpha;

        let composite_alpha = [
            CompositeAlpha::Opaque,
            CompositeAlpha::PreMultiplied,
            CompositeAlpha::PostMultiplied,
            CompositeAlpha::Inherit,
        ]
        .into_iter()
        .find(|alpha| supported_alpha.contains_enum(*alpha))
        .ok_or_else(|| {
            std::io::Error::other("No composite alpha mode available")
        })?;

        // Create the swapchain.
        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: image_count,
                image_format,
                image_color_space,
                image_extent,
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                pre_transform: capabilities.current_transform,
                composite_alpha,
                present_mode: PresentMode::Fifo,
                ..Default::default()
            },
        )?;

        println!("Swapchain created!");
        println!(
            "Swapchain dimensions: {}x{}",
            image_extent[0],
            image_extent[1]
        );
        println!("Swapchain images: {}", images.len());

        Ok(Self {
            swapchain,
            images,
            image_extent,
        })
    }
}