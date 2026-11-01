
use sdl3::event::Event;
use sdl3::keyboard::Keycode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize SDL
    let sdl = sdl3::init()?;

    // Initialize video subsystem
    let video = sdl.video()?;

    // Create window
    let _window = video
        .window("The Nine Worlds", 1280, 720)
        .position_centered()
        .resizable()
        .vulkan()
        .build()?;

    // Event handling
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

        // Rendering will go here later.
    }

    Ok(())
}
