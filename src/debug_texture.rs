use bevy::asset::RenderAssetUsages;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::encase::UniformBuffer;
use bevy::render::render_resource::*;

/// Creates a colorful test pattern
pub fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

pub fn spawn_debug_texture(image: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(image),
        ..Default::default()
    }
}

pub type DebugMaterial = ExtendedMaterial<StandardMaterial, HoverExtension>;

pub fn spawn_hoverable_debug_texture(image: Handle<Image>) -> DebugMaterial {
    DebugMaterial {
        base: StandardMaterial {
            base_color_texture: Some(image),
            ..Default::default()
        },
        extension: HoverExtension {
            is_active: false,
            trans_start: 0.,
        },
    }
}

const HOVER_EXTENSION_SHADER: &str = "shaders/hover_extension.wgsl";

#[derive(Asset, Reflect, Debug, Clone)]
pub struct HoverExtension {
    pub is_active: bool,
    pub trans_start: f32,
}

#[derive(ShaderType)]
struct HoverExtensionBinding<'a> {
    is_active: &'a u32,
    trans_start: &'a f32,
}

impl AsBindGroup for HoverExtension {
    type Data = ();
    type Param = ();

    fn label() -> &'static str {
        "HoverExtension"
    }

    fn bind_group_data(&self) -> Self::Data {
        ()
    }

    fn unprepared_bind_group(
        &self,
        _: &BindGroupLayout,
        render_device: &bevy::render::renderer::RenderDevice,
        _: &mut bevy::ecs::system::SystemParamItem<'_, '_, Self::Param>,
        _: bool,
    ) -> std::result::Result<UnpreparedBindGroup, AsBindGroupError> {
        let mut buffer = UniformBuffer::new(Vec::new());

        buffer
            .write(&HoverExtensionBinding {
                is_active: &(self.is_active as u32),
                trans_start: &self.trans_start,
            })
            .expect("is_active buffer write");

        Ok(UnpreparedBindGroup {
            bindings: BindingResources(vec![(
                100,
                OwnedBindingResource::Buffer(render_device.create_buffer_with_data(
                    &BufferInitDescriptor {
                        label: None,
                        contents: buffer.as_ref(),
                        usage: BufferUsages::UNIFORM,
                    },
                )),
            )]),
        })
    }

    fn bind_group_layout_entries(
        _: &bevy::render::renderer::RenderDevice,
        _: bool,
    ) -> Vec<BindGroupLayoutEntry>
    where
        Self: Sized,
    {
        vec![BindGroupLayoutEntry {
            binding: 100,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(HoverExtensionBinding::min_size()),
            },
            count: None,
        }]
    }
}

impl MaterialExtension for HoverExtension {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        HOVER_EXTENSION_SHADER.into()
    }

    fn deferred_fragment_shader() -> bevy::shader::ShaderRef {
        HOVER_EXTENSION_SHADER.into()
    }
}
