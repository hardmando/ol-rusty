use ash::{vk, Entry, Instance};
use std::error::Error;
use std::ffi::{c_char, CString};
use std::marker::PhantomData;
use winit::window::Window;
use winit::raw_window_handle::{HasRawDisplayHandle as _, HasRawWindowHandle as _};

pub struct Renderer {
    _entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let entry = unsafe { Entry::load()? };
        let instance = Self::create_instance(&entry, window)?;

        // Create surface using ash_window only
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.raw_display_handle().unwrap(),
                window.raw_window_handle().unwrap(),
                None,
            )?
        };
        Ok(Renderer {
            _entry: entry,
            instance,
            surface,
        })
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
            ash_window::enumerate_required_extensions(window.raw_display_handle().unwrap()).unwrap();

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
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            // Surface is destroyed with the instance in ash 0.38, no need for a loader
            self.instance.destroy_instance(None);
        }
    }
}
