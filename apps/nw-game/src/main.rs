use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Initialize SDL.
    let sdl = sdl3::init()?;
    let video = sdl.video()?;

    // Create the window.
    let window = video
        .window("The Nine Worlds", 1280, 720)
        .position_centered()
        .resizable()
        .vulkan()
        .build()?;

    // Initialize Vulkan.
    let vulkan_context =
        nw_renderer::VulkanContext::new(&window)?;

    // Create the swapchain.
    let swapchain =
        nw_renderer::SwapchainState::new(
            &vulkan_context,
            &window,
        )?;

    // Create the renderer.
    let mut renderer =
        nw_renderer::Renderer::new(
            &vulkan_context,
            swapchain,
        )?;

    // Event handling.
    let mut event_pump = sdl.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        // Draw one frame.
        renderer.draw_frame(&vulkan_context, &window)?;

        // Avoid running the loop unnecessarily fast.
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}