mod util;

use crate::util::constants::{API_VERSION, APPLICATION_NAME, APPLICATION_VERSION, ENGINE_NAME, ENGINE_VERSION, WINDOW_HEIGHT, WINDOW_WIDTH};
use ash::vk;
use std::ffi::CString;
use std::ptr;
use std::ptr::null;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window};

struct NeoClausewitz {
    _entry: ash::Entry,
    instance: ash::Instance,
}

impl NeoClausewitz {
    pub fn new() -> NeoClausewitz {
        let entry = ash::Entry::linked();
        let instance = NeoClausewitz::create_instance(&entry);

        NeoClausewitz {
            _entry: entry,
            instance,
        }
    }

    fn init_window(event_loop: &EventLoop<()>) -> Window {
        winit::window::WindowBuilder::new()
            .with_title(APPLICATION_NAME)
            .with_min_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        let app_name = CString::new(APPLICATION_NAME).unwrap();
        let engine_name = CString::new(ENGINE_NAME).unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: APPLICATION_VERSION,
            p_engine_name: engine_name.as_ptr(),
            engine_version: ENGINE_VERSION,
            api_version: API_VERSION,
            _marker: Default::default(),
        };

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: null(),
            enabled_layer_count: 0,
            _marker: Default::default(),
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        };

        instance
    }

    fn draw_frame(&mut self) {
        // Drawing will be here
    }

    pub fn main_loop(mut self, event_loop: EventLoop<()>, window: Window) {
        event_loop.run(move |event, _, control_flow| {
            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        }
                        | WindowEvent::KeyboardInput { input, .. } => {}
                        | _ => {}
                    }
                }
                | Event::MainEventsCleared => {
                    window.request_redraw();
                }
                | Event::RedrawRequested(_window_id) => {
                    self.draw_frame();
                }
                _ => (),
            }
        })
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = NeoClausewitz::init_window(&event_loop);

    let vulkan_app = NeoClausewitz::new();
    vulkan_app.main_loop(event_loop, window);
}
