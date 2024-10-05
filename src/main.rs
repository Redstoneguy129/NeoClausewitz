mod util;

use crate::util::constants::{APPLICATION_NAME, WINDOW_HEIGHT, WINDOW_WIDTH};
use ash::vk;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::keyboard::{Key, NamedKey};

struct NeoClausewitz {
    _entry: ash::Entry,
    instance: ash::Instance,
    surface: ash::khr::surface::Instance,
    debug_utils_loader: ash::ext::debug_utils::Instance,
    debug_call_back: vk::DebugUtilsMessengerEXT,
}

impl NeoClausewitz {
    pub unsafe fn new(event_loop: &winit::event_loop::EventLoop<()>) -> NeoClausewitz {
        let entry = ash::Entry::linked();
        let instance = NeoClausewitz::create_instance(&entry, event_loop);
        let surface = ash::khr::surface::Instance::new(&entry, &instance);
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback));
        let debug_utils_loader = ash::ext::debug_utils::Instance::new(&entry, &instance);
        let debug_call_back = debug_utils_loader
            .create_debug_utils_messenger(&debug_info, None)
            .unwrap();

        NeoClausewitz {
            _entry: entry,
            instance,
            surface,
            debug_utils_loader,
            debug_call_back,
        }
    }

    fn init_window(event_loop: &winit::event_loop::EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new()
            .with_title(APPLICATION_NAME)
            .with_min_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    unsafe fn create_instance(entry: &ash::Entry, event_loop: &winit::event_loop::EventLoop<()>) -> ash::Instance {
        let layer_names = [std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )];
        let layers_names_raw: Vec<*const std::ffi::c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

        let mut extension_names =
                ash_window::enumerate_required_extensions(event_loop.display_handle().unwrap().as_raw())
                    .unwrap()
                    .to_vec();
            extension_names.push(ash::ext::debug_utils::NAME.as_ptr());

        let app_desc = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));

        let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                vk::InstanceCreateFlags::default()
            };

        let instance_desc = vk::InstanceCreateInfo::default()
            .application_info(&app_desc)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names)
            .flags(create_flags);


        entry.create_instance(&instance_desc, None).expect("Failed to create instance.")
    }

    pub fn main_loop(mut self, event_loop: winit::event_loop::EventLoop<()>, window: winit::window::Window) {
        let mut surface = None;
        event_loop.run(move |event, elwp| match event {
            winit::event::Event::WindowEvent {
                event:
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                    ..
                },
                window_id: _,
            } => {
                elwp.exit();
            }
            Event::LoopExiting => unsafe {
                if let Some(surface) = surface.take() {
                    self.surface.destroy_surface(surface, None);
                }
            }
            Event::Resumed => unsafe {
                // Create a surface from winit window.
                let s = ash_window::create_surface(
                    &self._entry,
                    &self.instance,
                    window.display_handle().unwrap().as_raw(),
                    window.window_handle().unwrap().as_raw(),
                    None,
                )
                    .unwrap();
                println!("surface: {s:?}");
                assert!(
                    surface.replace(s).is_none(),
                    "Surface must not yet exist when Resumed is called"
                );
            }
            Event::Suspended => unsafe {
                let surface = surface
                    .take()
                    .expect("Surface must have been created in Resumed");
                self.surface.destroy_surface(surface, None);
            }
            _ => {}
        }).expect("Failed to run event loop.");
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop.");
    let window = NeoClausewitz::init_window(&event_loop);
    unsafe {
        let neo_clausewitz = NeoClausewitz::new(&event_loop);
        neo_clausewitz.main_loop(event_loop, window);
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        std::borrow::Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        std::borrow::Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}