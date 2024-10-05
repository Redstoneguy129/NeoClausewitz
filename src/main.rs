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
}

impl NeoClausewitz {
    pub unsafe fn new(event_loop: &winit::event_loop::EventLoop<()>) -> NeoClausewitz {
        let entry = ash::Entry::linked();
        let instance = NeoClausewitz::create_instance(&entry, event_loop);
        let surface = ash::khr::surface::Instance::new(&entry, &instance);

        NeoClausewitz {
            _entry: entry,
            instance,
            surface,
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
        let surface_extensions = ash_window::enumerate_required_extensions(event_loop.display_handle().unwrap().as_raw()).unwrap();
        let app_desc = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::default()
            .application_info(&app_desc)
            .enabled_extension_names(surface_extensions);
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