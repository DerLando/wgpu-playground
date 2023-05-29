use clap::{command, Parser, Subcommand};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::borrow::Cow;
use wgpu::{
    Device, PipelineLayout, PipelineLayoutDescriptor, RenderPipeline, ShaderModule,
    SurfaceCapabilities, TextureFormat,
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::Window,
};

fn load_shader(path: &str, device: &Device) -> Result<ShaderModule, std::io::Error> {
    let shader = read_or_create_shader(path);
    Ok(device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(shader),
    }))
}

fn create_pipeline(
    device: &Device,
    pipeline_layout: &PipelineLayout,
    capabilities: &SurfaceCapabilities,
    shader: &ShaderModule,
) -> RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(capabilities.formats[0].into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}

struct RenderState {
    pipeline: RenderPipeline,
}

async fn run(event_loop: EventLoop<CustomEvent>, window: Window, shader_path: &str) {
    let size = window.inner_size();

    let instance = wgpu::Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Load the shaders from disk
    let shader = load_shader(shader_path, &device).unwrap();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline =
        create_pipeline(&device, &pipeline_layout, &swapchain_capabilities, &shader);

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let mut state = RenderState {
        pipeline: render_pipeline,
    };

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
                // On macos the window needs to be redrawn manually after resizing
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&state.pipeline);
                    rpass.draw(0..6, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::UserEvent(event) => match event {
                CustomEvent::ShaderFileChangedEvent(path) => {
                    let shader = load_shader(&path, &device).unwrap();
                    let render_pipeline = create_pipeline(
                        &device,
                        &pipeline_layout,
                        &swapchain_capabilities,
                        &shader,
                    );
                    state.pipeline = render_pipeline;
                    window.request_redraw();
                }
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path of shader to load
    shader_path: String,
}

#[derive(Debug)]
enum CustomEvent {
    ShaderFileChangedEvent(String),
}

fn read_or_create_shader(path: &str) -> Cow<str> {
    if std::path::Path::exists(std::path::Path::new(path)) {
        Cow::Owned(std::fs::read_to_string(path).unwrap())
    } else {
        let shader = include_str!("shader.wgsl");
        std::fs::write(path, shader);
        Cow::Owned(shader.to_string())
    }
}

fn main() {
    let cli = Cli::parse();
    let shader_path = cli.shader_path.clone();

    // make sure the shader file exists
    let _ = read_or_create_shader(&cli.shader_path);

    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event().build();

    // Proxy lets us fire custom events form any thread
    let event_loop_proxy = event_loop.create_proxy();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_inner_size(LogicalSize::new(600.0, 600.0));
    window.set_resizable(false);

    // file system changes notification
    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

        watcher
            .watch(
                std::path::Path::new(&shader_path),
                RecursiveMode::NonRecursive,
            )
            .unwrap();

        for res in rx {
            match res {
                Ok(event) => {
                    // println!("{event:?}");
                    event_loop_proxy
                        .send_event(CustomEvent::ShaderFileChangedEvent(String::from(
                            &shader_path,
                        )))
                        .ok();
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run(event_loop, window, &cli.shader_path));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window, &cli.shader_path));
    }
}
