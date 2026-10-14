use leptos::leptos_dom::helpers::set_interval_with_handle;
use leptos::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use web_sys::HtmlCanvasElement;
stylance::import_style!(
    #[allow(unused)]
    style,
    "../styles/home.module.css"
);

// css px per shader pixel
const PIXEL_SIZE: i32 = 3;

// from scripts/gen_earth_mask.py
const LAND_MASK: &[u8] = include_bytes!("../shaders/earth_mask.bin");
const LAND_W: u32 = 256;
const LAND_H: u32 = 128;

fn clock_string() -> String {
    let now = js_sys::Date::new_0();
    format!(
        "{:02}:{:02}:{:02}",
        now.get_hours(),
        now.get_minutes(),
        now.get_seconds()
    )
}

#[component]
pub fn OrbitClock() -> impl IntoView {
    let (time_str, set_time_str) = signal(clock_string());
    let (gpu_ok, set_gpu_ok) = signal(true);
    let (backend, set_backend) = signal(String::new());
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let running = Arc::new(AtomicBool::new(true));

    let handle = set_interval_with_handle(
        move || set_time_str.set(clock_string()),
        Duration::from_millis(1000),
    )
    .ok();

    Effect::new({
        let running = running.clone();
        move |_| {
            if let Some(canvas) = canvas_ref.get() {
                let running = running.clone();
                leptos::task::spawn_local(async move {
                    match start(canvas, running).await {
                        Ok(name) => set_backend.set(name.to_string()),
                        Err(e) => {
                            leptos::logging::warn!("no webgpu/webgl, falling back to clock: {e}");
                            set_gpu_ok.set(false);
                        }
                    }
                });
            }
        }
    });

    on_cleanup(move || {
        running.store(false, Ordering::Relaxed);
        if let Some(h) = handle {
            h.clear();
        }
    });

    view! {
        <div class=style::clock_panel>
            <Show
                when=move || gpu_ok.get()
                fallback=move || view! {
                    <div class=style::clock_label>"LOCAL TIME"</div>
                    <div class=style::clock_display>{time_str}</div>
                }
            >
                <canvas node_ref=canvas_ref class=style::orbit_canvas></canvas>
                <div class=style::orbit_header>
                    <span>"FIG_2.0: ORBITAL_TRACK"</span>
                    <span>"LOCAL TIME"</span>
                </div>
                <div class=style::orbit_readout>
                    <span class=style::orbit_time>"T+ " {time_str}</span>
                    <span>{backend}</span>
                </div>
            </Show>
        </div>
    }
}

struct Gpu {
    canvas: HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: std::cell::RefCell<wgpu::SurfaceConfiguration>,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniforms: wgpu::Buffer,
    srgb: bool,
    reduced_motion: bool,
    home_lon: f32,
}

fn canvas_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    (
        (canvas.client_width() / PIXEL_SIZE).max(1) as u32,
        (canvas.client_height() / PIXEL_SIZE).max(1) as u32,
    )
}

async fn start(
    canvas: HtmlCanvasElement,
    running: Arc<AtomicBool>,
) -> Result<&'static str, String> {
    let (width, height) = canvas_size(&canvas);
    canvas.set_width(width);
    canvas.set_height(height);

    // ?renderer=webgl to test the fallback
    let force_gl = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .is_some_and(|q| q.contains("renderer=webgl"));
    let backends = if force_gl {
        wgpu::Backends::GL
    } else {
        wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL
    };
    let instance = wgpu::util::new_instance_with_webgpu_detection(wgpu::InstanceDescriptor {
        backends,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    })
    .await;
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|e| e.to_string())?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|e| e.to_string())?;
    let backend_name = match adapter.get_info().backend {
        wgpu::Backend::BrowserWebGpu => "WEBGPU",
        _ => "WEBGL2",
    };
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
            ..Default::default()
        })
        .await
        .map_err(|e| e.to_string())?;

    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("surface not supported by adapter")?;
    // shader already outputs srgb colors
    if let Some(format) = surface
        .get_capabilities(&adapter)
        .formats
        .into_iter()
        .find(|f| !f.is_srgb())
    {
        config.format = format;
    }
    surface.configure(&device, &config);

    let land_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("land mask"),
        size: wgpu::Extent3d {
            width: LAND_W,
            height: LAND_H,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        land_texture.as_image_copy(),
        LAND_MASK,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(LAND_W),
            rows_per_image: Some(LAND_H),
        },
        land_texture.size(),
    );
    let land_view = land_texture.create_view(&Default::default());
    let land_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("orbit"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/orbit.wgsl").into()),
    });

    let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("orbit uniforms"),
        size: 32,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&land_view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&land_sampler),
            },
        ],
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(&bind_group_layout)],
        immediate_size: 0,
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("orbit"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(config.format.into())],
        }),
        multiview_mask: None,
        cache: None,
    });

    let reduced_motion = web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches());

    // rough longitude from the timezone
    let tz_offset_hours = -js_sys::Date::new_0().get_timezone_offset() as f32 / 60.0;
    let home_lon = (tz_offset_hours * 15.0).to_radians();

    let srgb = config.format.is_srgb();
    let gpu = Rc::new(Gpu {
        canvas,
        surface,
        device,
        queue,
        config: std::cell::RefCell::new(config),
        pipeline,
        bind_group,
        uniforms,
        srgb,
        reduced_motion,
        home_lon,
    });
    frame(gpu, running);
    Ok(backend_name)
}

fn frame(gpu: Rc<Gpu>, running: Arc<AtomicBool>) {
    if !running.load(Ordering::Relaxed) {
        return;
    }

    let (width, height) = canvas_size(&gpu.canvas);
    {
        let mut config = gpu.config.borrow_mut();
        if config.width != width || config.height != height {
            gpu.canvas.set_width(width);
            gpu.canvas.set_height(height);
            config.width = width;
            config.height = height;
            gpu.surface.configure(&gpu.device, &config);
        }
    }

    let now = js_sys::Date::new_0();
    let seconds = now.get_seconds() as f32 + now.get_milliseconds() as f32 / 1000.0;
    let time = if gpu.reduced_motion {
        0.0
    } else {
        js_sys::Date::now() as f32 % 1_000_000.0 / 1000.0
    };
    let dark = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
        .and_then(|el| el.get_attribute("data-theme"))
        .is_some_and(|t| t == "dark");

    let data = [
        width as f32,
        height as f32,
        time,
        seconds / 60.0,
        gpu.home_lon,
        if dark { 1.0 } else { 0.0 },
        if gpu.srgb { 1.0 } else { 0.0 },
        0.0,
    ];
    let bytes: Vec<u8> = data.iter().flat_map(|f| f.to_le_bytes()).collect();
    gpu.queue.write_buffer(&gpu.uniforms, 0, &bytes);

    let texture = match gpu.surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => {
            Some(t)
        }
        _ => None,
    };

    if let Some(texture) = texture {
        let view = texture.texture.create_view(&Default::default());
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                ..Default::default()
            });
            pass.set_pipeline(&gpu.pipeline);
            pass.set_bind_group(0, &gpu.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        gpu.queue.submit([encoder.finish()]);
        gpu.queue.present(texture);
    }

    request_animation_frame(move || frame(gpu, running));
}
