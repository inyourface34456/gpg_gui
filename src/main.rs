mod screens;
mod state;

use std::num::NonZeroU32;
use std::path::PathBuf;
use std::time::Instant;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;

use raw_window_handle::HasRawWindowHandle;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;

use imgui::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use screens::hello_world;
use state::State;

fn main() {
    // --- Create window + GL context ---
    let event_loop = EventLoop::new();

    let window_builder = Some(
        WindowBuilder::new()
            .with_title("Gpg GUI")
            .with_inner_size(LogicalSize::new(800.0, 600.0)),
    );

    let template = ConfigTemplateBuilder::new();

    let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let window = window.unwrap();

    let raw_window_handle = window.raw_window_handle();
    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
        .build(Some(raw_window_handle));

    let not_current_gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .expect("Failed to create GL context")
    };

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(800).unwrap(),
        NonZeroU32::new(600).unwrap(),
    );

    let surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let gl_context = not_current_gl_context
        .make_current(&surface)
        .unwrap();

    gl::load_with(|s| {
        gl_display.get_proc_address(&std::ffi::CString::new(s).unwrap())
    });

    // --- Setup ImGui ---
    let mut imgui = Context::create();
    imgui.set_ini_filename(Some(PathBuf::new().with_file_name("gpg_gui").with_added_extension("ini")));

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
            gl_display.get_proc_address(&std::ffi::CString::new(s).unwrap())
        });

    let mut last_frame = Instant::now();
    
    let mut state = State::new();

    // --- Event Loop ---
    event_loop
        .run(move |event, _, control_flow| {
            platform.handle_event(imgui.io_mut(), &window, &event);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        surface.resize(
                            &gl_context,
                            NonZeroU32::new(size.width.max(1)).unwrap(),
                            NonZeroU32::new(size.height.max(1)).unwrap(),
                        );
                    }
                    _ => {}
                },

                Event::MainEventsCleared => {
                    window.request_redraw();
                }

                Event::RedrawRequested(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;

                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("prepare_frame failed");

                    let ui = imgui.frame();

                    hello_world(ui, &window, &mut state);

                    platform.prepare_render(&ui, &window);

                    unsafe {
                        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    
                    renderer.render(&mut imgui);

                    surface.swap_buffers(&gl_context).unwrap();
                }

                _ => {}
            }
        });
}