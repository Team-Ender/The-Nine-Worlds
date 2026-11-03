use std::marker::PhantomData;
use std::sync::Arc;

use vulkano::device::{
    Device,
    DeviceCreateInfo,
    DeviceExtensions,
    Queue,
    QueueCreateInfo,
    QueueFlags,
};

use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    swapchain::Surface,
    Version,
    VulkanLibrary,
};

pub struct VulkanContext<'window> {
    _surface: Arc<Surface>,
    _instance: Arc<Instance>,
    _window: PhantomData<&'window sdl3::video::Window>,
    _device: Arc<Device>,
    _queue: Arc<Queue>,
}


impl<'window> VulkanContext<'window> {
    pub fn device(&self) -> &Arc<Device> {
        &self._device
    }

    pub fn queue(&self) -> &Arc<Queue> {
        &self._queue
    }

    pub fn surface(&self) -> &Arc<Surface> {
        &self._surface
    }
    pub fn new(
        window: &'window sdl3::video::Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {

        println!("VulkanContext::new() started");
        // 1. Load the Vulkan library.
        let library = VulkanLibrary::new()?;

        // 2. Get the extensions required by SDL.
        let enabled_extensions = Surface::required_extensions(window)?;

        // 3. Create the Vulkan instance.
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                application_name: Some("The Nine Worlds".to_owned()),
                application_version: Version::V1_0,
                engine_name: Some("The Nine Worlds Engine".to_owned()),
                engine_version: Version::V1_0,
                enabled_extensions,
                ..Default::default()
            },
        )?;
        println!("Vulkan instance created");

        // 4. Create the window surface.
        // SAFETY: PhantomData ties VulkanContext's lifetime
        // to the window, ensuring the window outlives the surface.
        let surface = unsafe {
            Surface::from_window_ref(instance.clone(), window)?
        };
        println!("Enumerating GPUs...");
        // 5. Find available GPUs.
        // 5. Find a GPU with a graphics + presentation queue.
let physical_devices = instance.enumerate_physical_devices()?;

let mut selected_gpu = None;

for physical_device in physical_devices {
    println!(
        "Checking GPU: {}",
        physical_device.properties().device_name
    );

    for (index, queue_family) in
        physical_device.queue_family_properties().iter().enumerate()
    {
        let supports_graphics =
            queue_family.queue_flags.intersects(QueueFlags::GRAPHICS);

        let supports_present =
            physical_device.surface_support(index as u32, &surface)?;

        if supports_graphics && supports_present {
            selected_gpu = Some((physical_device.clone(), index as u32));
            break;
        }
    }

    if selected_gpu.is_some() {
        break;
    }
}

// Make sure we actually found a suitable GPU.
let (physical_device, queue_family_index) = selected_gpu
    .ok_or_else(|| std::io::Error::other("No suitable GPU found"))?;

println!("Selected GPU: {}", physical_device.properties().device_name);
println!("Selected queue family: {queue_family_index}");

// 6. Create the logical device and queue.
let (device, mut queues) = Device::new(
    physical_device,
    DeviceCreateInfo {
        enabled_extensions: DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        },
        queue_create_infos: vec![
            QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }
        ],
        ..Default::default()
    },
)?;
// Get the queue created with our device.
let queue = queues
    .next()
    .ok_or_else(|| std::io::Error::other("Failed to get GPU queue"))?;

println!("Logical device created!");
println!("Graphics queue acquired!");

// 7. Store everything in VulkanContext.
Ok(Self {
    _surface: surface,
    _instance: instance,
    _device: device,
    _queue: queue,
    _window: PhantomData,
})

}
}