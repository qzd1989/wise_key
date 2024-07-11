use async_std::task::block_on;
use std::{any::Any, borrow::Cow, iter, num::NonZeroU32};
use wgpu::{
    core::device::{self, queue},
    util::DeviceExt,
    BindGroup, BindGroupDescriptor, Queue,
};
use wgpu_types::CommandEncoderDescriptor;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 0.0],
    },
];

const INDICES: &[u16] = &[0u16, 1, 2, 2, 3, 0];

async fn load_texture(device: &wgpu::Device, queue: &wgpu::Queue, path: &str) -> wgpu::Texture {
    let img = image::open(path).unwrap().to_rgba8();
    let size = img.dimensions();
    let texture_extent = wgpu::Extent3d {
        width: size.0,
        height: size.1,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        label: Some("Texture"),
        view_formats: &[],
    });
    // device.create_texture(&wgpu::TextureDescriptor {
    //     label: Some("Texture"),
    //     size: texture_size,
    //     mip_level_count: 1,
    //     sample_count: 1,
    //     dimension: wgpu::TextureDimension::D2,
    //     format: wgpu::TextureFormat::Bgra8UnormSrgb,
    //     usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    // })
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &img,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(NonZeroU32::new(4 * size.0).unwrap().into()),
            rows_per_image: Some(NonZeroU32::new(size.1).unwrap().into()),
        },
        texture_extent,
    );

    texture
}

fn new_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Texture Rendering Example")
        .with_inner_size(LogicalSize::new(500, 500))
        .build(&event_loop)
        .unwrap();
    (event_loop, window)
}

async fn init_wgpu(
    window: &Window,
) -> (
    wgpu::Device,
    wgpu::Texture,
    wgpu::Queue,
    wgpu::Surface,
    wgpu::SurfaceConfiguration,
) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    let surface = instance.create_surface(window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::util::power_preference_from_env()
                .unwrap_or(wgpu::PowerPreference::HighPerformance),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();
    println!("adapter is :{:#?}", adapter.get_info());

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
            },
            None,
        )
        .await
        .unwrap();
    let size = window.inner_size();
    let width = size.width.max(1);
    let height = size.height.max(1);
    let texture = block_on(load_texture(&device, &queue, "examples/happy-tree.png"));
    let caps = surface.get_capabilities(&adapter);
    println!("supported formats :{:?}", caps.formats);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        format: caps.formats[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);
    (device, texture, queue, surface, config)
}

fn calculate_viewport(
    window_width: f32,
    window_height: f32,
    texture_width: f32,
    texture_height: f32,
) -> (f32, f32, f32, f32) {
    let window_aspect_ratio = window_width / window_height;
    let texture_aspect_ratio = texture_width / texture_height;

    if window_aspect_ratio > texture_aspect_ratio {
        // 窗口更宽，调整宽度
        let new_width = window_height * texture_aspect_ratio;
        let x_offset = (window_width - new_width) / 2.0;
        (x_offset, 0.0, new_width, window_height)
    } else {
        // 窗口更高，调整高度
        let new_height = window_width / texture_aspect_ratio;
        let y_offset = (window_height - new_height) / 2.0;
        (0.0, y_offset, window_width, new_height)
    }
}

fn main() {
    let (event_loop, window) = new_window();
    let (device, texture, queue, surface, config) = block_on(init_wgpu(&window));
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    });
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let vs_src = r#"
    struct VertexInput {
        @location(0) position: vec2<f32>,
        @location(1) tex_coords: vec2<f32>,
    };
    struct VertexOutput {
        @builtin(position) clip_position: vec4<f32>,
        @location(0) tex_coords: vec2<f32>,
    };
    @vertex
        fn vs_main(
            model: VertexInput,
        ) -> VertexOutput {
            var out: VertexOutput;
            out.tex_coords = model.tex_coords;
            out.clip_position = vec4f(model.position, 0.0, 1.0);
            return out;
        }
    @group(0) 
        @binding(0)
            var texture: texture_2d<f32>;
    @group(0)
        @binding(1)
            var texture_sampler: sampler;
    @fragment
        fn fs_main(in: VertexOutput) -> @location(0) vec4f {
            var color: vec4<f32> = textureSample(texture, texture_sampler, in.tex_coords);
            return color;
        }
"#;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(vs_src)),
    });
    let fragment_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(vs_src)),
    });
    let pipline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader_module,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format.add_srgb_suffix(),
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(
                    &texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&device.create_sampler(
                    &wgpu::SamplerDescriptor {
                        address_mode_u: wgpu::AddressMode::ClampToEdge,
                        address_mode_v: wgpu::AddressMode::ClampToEdge,
                        address_mode_w: wgpu::AddressMode::ClampToEdge,
                        mag_filter: wgpu::FilterMode::Linear,
                        min_filter: wgpu::FilterMode::Nearest,
                        mipmap_filter: wgpu::FilterMode::Nearest,
                        ..Default::default()
                    },
                )),
            },
        ],
        label: Some("diffuse_bind_group"),
    });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = INDICES.len() as u32;

    let (w, h, w_w, w_h) = (
        texture.width() as f32,
        texture.height() as f32,
        window.inner_size().width as f32,
        window.inner_size().height as f32,
    );

    //begin

    if let Err(err) = EventLoop::run(event_loop, move |event, elwt| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::RedrawRequested => {
                //_queue不出来的话,就会在init_wgpu被drop掉,导致surface_texture.present()失败
                let surface_texture = surface.get_current_texture();
                match surface_texture {
                    Ok(surface_texture) => {
                        //texture do sth
                        //get textview
                        let view =
                            surface_texture
                                .texture
                                .create_view(&wgpu::TextureViewDescriptor {
                                    label: Some("frame texture view"),
                                    format: Some(config.format.add_srgb_suffix()),
                                    ..Default::default()
                                });
                        let mut encoder =
                            device.create_command_encoder(&CommandEncoderDescriptor {
                                label: Some("Render Encoder"),
                            });
                        // do sth for render
                        {
                            let mut render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("Render Pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                                r: 0.1,
                                                g: 0.2,
                                                b: 0.3,
                                                a: 1.0,
                                            }),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    ..Default::default()
                                });
                            render_pass.set_pipeline(&pipline);
                            render_pass.set_bind_group(0, &bind_group, &[]);
                            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            render_pass.set_index_buffer(
                                index_buffer.slice(..),
                                wgpu::IndexFormat::Uint16,
                            );
                            render_pass.set_viewport(0.0, 0.0, (w) as f32, (h) as f32, 0.0, 1.0);
                            render_pass.draw_indexed(0..num_indices, 0, 0..1);
                        }
                        queue.submit(iter::once(encoder.finish()));
                        surface_texture.present();
                    }
                    Err(err) => {
                        println!("surface_texture.present: {:?}", err);
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }) {
        println!("EventLoop::run: {:?}", err);
    }
}
