use glam::Vec2;

use crate::{decal::DecalInstance, *};

/*
    Each layer should be almost fully self contained
*/

pub struct Layer {
    pub offset: Vec2,
    pub scale: Vec2,
    pub show: bool,
    pub update: bool,
    pub surface: Renderable,
    pub decal_instances: Vec<DecalInstance>,
    pub tint: Color,
    pub id: usize,
    pub pipeline: Pipeline,
    pub bindings: Bindings,
}

impl Layer {
    pub fn new(pge: &mut PGE, width: u32, height: u32) -> Self {
        let cpu_bb = SpriteRef::new(width as u32, height as u32);
        let cpu_bb_len = cpu_bb.get_data_len();

        let cpu_bb_tex = pge.ctx.new_texture_from_rgba8(width as u16, height as u16, unsafe {
            std::slice::from_raw_parts(cpu_bb.get_data_ptr(), cpu_bb_len * 4)
        });

        pge.ctx.texture_set_filter(cpu_bb_tex, FilterMode::Nearest, MipmapFilterMode::None);
        let cpu_bb_weak_ref = Rc::downgrade(&cpu_bb.0);

        // screen space vertex's for 2 triangles
        let vertices: [Vertex; 4] = [
            vert(vec2(-1.0, -1.0), vec2(0., 1.)),
            vert(vec2( 1.0, -1.0), vec2(1., 1.)),
            vert(vec2( 1.0,  1.0), vec2(1., 0.)),
            vert(vec2(-1.0,  1.0), vec2(0., 0.)),
        ];

        let vertex_buffer = pge.ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Dynamic,
            BufferSource::slice(&vertices),
        );

        // TODO: index buffer should be fully pre-initialized?
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = pge.ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![cpu_bb_tex],
        };

        // NOTE: Below is how we update our vertex buffer.
        //      This is why we set the BufferUsage to Dynamic
        //      Index buffer is pre filled out
        // pge.ctx.buffer_update(bindings.vertex_buffers[0], data)

        // TODO: need to change this to reference a shader inside this file or included with
        //      include! macro
        let shader = pge.ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::GL_VERTEX,
                    fragment: shader::GL_FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        // TODO: add offset, scale, tint, into the shader as variables
        let pipeline = pge.ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
                // TODO: add per triangle stuff here?
            ],
            shader,
        );

        Layer {
            offset: Vec2::ZERO,
            scale: Vec2::ONE,
            tint: WHITE,
            show: false,
            update: false,
            surface: Renderable { 
                sprite: cpu_bb,
                decal: Decal { 
                    sprite: cpu_bb_weak_ref,
                    texture_id: cpu_bb_tex,
                    uv_scale: Vec2::ONE,
                    width: width as u32,
                    height: height as u32,
                }},
            decal_instances: vec![],
            id: 0,  // TODO: not used
            pipeline,
            bindings,
        }
    }

    pub fn render(&mut self, pge: &mut PGE) {
        pge.ctx.texture_update(self.bindings.images[0], unsafe {
            let layer = if pge.current_layer < pge.layers.len() { pge.current_layer }
            else { 0 };
            let len = pge.layers[layer].surface.sprite.get_data_len();
            std::slice::from_raw_parts(pge.layers[layer].surface.sprite.get_data_ptr(), len * 4)
        });

        pge.ctx.begin_default_pass(Default::default());

        pge.ctx.apply_pipeline(&self.pipeline);
        pge.ctx.apply_bindings(&self.bindings);
        pge.ctx.draw(0, 6, 1);
        pge.ctx.end_render_pass();

        pge.ctx.commit_frame();
    }
}
