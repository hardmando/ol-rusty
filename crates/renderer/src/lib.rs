use ash::khr::swapchain::Device as SwapchainLoader;
use ash::{vk, Entry, Instance};
use std::error::Error;
use std::ffi::CString;
use std::marker::PhantomData;
use winit::raw_window_handle::{HasRawDisplayHandle as _, HasRawWindowHandle as _};
use winit::window::Window;

pub struct Renderer {
    _entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
    surface_loader: ash::khr::surface::Instance,
    _physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
    queue_family_index: u32,
    swapchain_loader: ash::khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain_image_views: Vec<vk::ImageView>,
}

#[allow(deprecated)]
impl Renderer {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let entry = unsafe { Entry::load()? };
        let instance = Self::create_instance(&entry, window)?;

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.raw_display_handle().unwrap(),
                window.raw_window_handle().unwrap(),
                None,
            )?
        };
        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

        let physical_devices = unsafe { instance.enumerate_physical_devices()? };
        let physical_device = physical_devices[0];

        // Find suitable queue family
        let queue_family_index =
            Self::find_queue_family(&instance, physical_device, &surface_loader, surface)?;

        // Get surface properties
        let surface_capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(physical_device, surface)?
        };

        let surface_formats = unsafe {
            surface_loader.get_physical_device_surface_formats(physical_device, surface)?
        };

        let surface_format = surface_formats
            .iter()
            .find(|&&format| format.format == vk::Format::B8G8R8A8_SRGB)
            .unwrap_or(&surface_formats[0]);

        let present_modes = unsafe {
            surface_loader.get_physical_device_surface_present_modes(physical_device, surface)?
        };

        let present_mode = present_modes
            .iter()
            .find(|&&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(&vk::PresentModeKHR::FIFO); // FIFO is always supported

        // Create logical device
        let device = Self::create_device(&instance, physical_device, queue_family_index)?;
        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        // Create swapchain
        let swapchain_extent = if surface_capabilities.current_extent.width != u32::MAX {
            surface_capabilities.current_extent
        } else {
            vk::Extent2D {
                width: 800
                    .min(surface_capabilities.max_image_extent.width)
                    .max(surface_capabilities.min_image_extent.width),
                height: 600
                    .min(surface_capabilities.max_image_extent.height)
                    .max(surface_capabilities.min_image_extent.height),
            }
        };

        let image_count = (surface_capabilities.min_image_count + 1).min(
            if surface_capabilities.max_image_count > 0 {
                surface_capabilities.max_image_count
            } else {
                u32::MAX
            },
        );

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface,
            min_image_count: image_count,
            image_format: surface_format.format,
            image_color_space: surface_format.color_space,
            image_extent: swapchain_extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: std::ptr::null(),
            pre_transform: surface_capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: *present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
            _marker: PhantomData,
        };

        let swapchain_loader = SwapchainLoader::new(&instance, &device);
        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None)? };
        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };

        // Create image views
        let swapchain_image_views: Vec<vk::ImageView> = swapchain_images
            .iter()
            .map(|&image| {
                let image_view_create_info = vk::ImageViewCreateInfo {
                    s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                    p_next: std::ptr::null(),
                    flags: vk::ImageViewCreateFlags::empty(),
                    image,
                    view_type: vk::ImageViewType::TYPE_2D,
                    format: surface_format.format,
                    components: vk::ComponentMapping {
                        r: vk::ComponentSwizzle::IDENTITY,
                        g: vk::ComponentSwizzle::IDENTITY,
                        b: vk::ComponentSwizzle::IDENTITY,
                        a: vk::ComponentSwizzle::IDENTITY,
                    },
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    _marker: PhantomData,
                };
                unsafe { device.create_image_view(&image_view_create_info, None) }
                    .expect("Image View Creation Failed")
            })
            .collect();

        Ok(Renderer {
            _entry: entry,
            instance,
            surface,
            surface_loader,
            _physical_device: physical_device,
            device,
            queue,
            queue_family_index,
            swapchain_loader,
            swapchain,
            swapchain_format: surface_format.format,
            swapchain_extent,
            swapchain_image_views,
        })
    }

    fn find_queue_family(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        surface_loader: &ash::khr::surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> Result<u32, Box<dyn Error>> {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        for (index, queue_family) in queue_families.iter().enumerate() {
            let supports_graphics = queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS);
            let supports_present = unsafe {
                surface_loader.get_physical_device_surface_support(
                    physical_device,
                    index as u32,
                    surface,
                )?
            };

            if supports_graphics && supports_present {
                return Ok(index as u32);
            }
        }

        Err("No suitable queue family found".into())
    }

    fn create_device(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> Result<ash::Device, Box<dyn Error>> {
        let queue_priorities = [1.0f32];
        let queue_create_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index,
            queue_count: 1,
            p_queue_priorities: queue_priorities.as_ptr(),
            _marker: PhantomData,
        };

        // Required extensions for swapchain
        let device_extensions = [ash::khr::swapchain::NAME.as_ptr()];

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_create_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: device_extensions.len() as u32,
            pp_enabled_extension_names: device_extensions.as_ptr(),
            p_enabled_features: std::ptr::null(),
            _marker: PhantomData,
        };

        let device = unsafe { instance.create_device(physical_device, &device_create_info, None)? };
        Ok(device)
    }

    fn create_instance(entry: &Entry, window: &Window) -> Result<Instance, Box<dyn Error>> {
        let app_name = CString::new("ol-rusty")?;
        let engine_name = CString::new("ol_engine")?;

        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 0, 1, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 0, 1, 0),
            api_version: vk::API_VERSION_1_3,
            _marker: PhantomData,
        };

        let extensions =
            ash_window::enumerate_required_extensions(window.raw_display_handle().unwrap())?;

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            _marker: PhantomData,
        };

        let instance = unsafe { entry.create_instance(&create_info, None)? };
        Ok(instance)
    }

    pub fn clear_color(&self, _color: [f32; 4]) {
        // For now, just print that we would clear to this color
        //println!("Would clear to color: {:?}", _color);
        // TODO: Implement actual Vulkan rendering
    }

    // Getters for other components that might need these
    pub fn device(&self) -> &ash::Device {
        &self.device
    }

    pub fn swapchain_format(&self) -> vk::Format {
        self.swapchain_format
    }

    pub fn swapchain_extent(&self) -> vk::Extent2D {
        self.swapchain_extent
    }

    pub fn queue(&self) -> vk::Queue {
        self.queue
    }

    pub fn swapchain_loader(&self) -> &ash::khr::swapchain::Device {
        &self.swapchain_loader
    }

    pub fn swapchain(&self) -> vk::SwapchainKHR {
        self.swapchain
    }

    // TODO: These methods will need to be implemented for actual rendering
    // pub fn acquire_next_image(&self) -> Result<(u32, bool), vk::Result> { ... }
    // pub fn present_image(&self, image_index: u32) -> Result<bool, vk::Result> { ... }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            // Clean up in reverse order of creation
            for &image_view in &self.swapchain_image_views {
                self.device.destroy_image_view(image_view, None);
            }

            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}
