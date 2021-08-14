use pollster::block_on;

fn main() {
    env_logger::init();

    let instance = wgpu::Instance::new(wgpu::Backends::GL);
    let adapter =
        block_on(instance.request_adapter(&Default::default())).expect("adapter creation failed");
    let (device, _queue) = block_on(adapter.request_device(&Default::default(), None))
        .expect("device creation failed");

    let shader_module = device.create_shader_module(&wgpu::include_wgsl!("shader.wgsl"));

    let bind_group_layout_1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            // Panic happens regardless of whether the unused uniform block is visible to
            // the fragment stage:
            //visibility: wgpu::ShaderStages::VERTEX,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout_1],
        push_constant_ranges: &[],
    });

    let _render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 8,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x2],
            }],
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "main",
            targets: &[],
        }),
    });
}
