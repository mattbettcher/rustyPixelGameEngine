use std::{rc::Weak, cell::RefCell};

use miniquad::*;
use glam::*;
use crate::*;

#[derive(Debug)]
pub enum DecalMode {
    Normal,
    Additive,
    Multiplicative,
    Stencil,
    Illuminate,
    Wireframe,
    Model3D,
}

#[derive(Debug)]
pub enum DecalStructure {
    Line,
    Fan,
    Strip,
    List,
}

#[derive(Debug)]
pub struct Decal {
    pub sprite: Weak<RefCell<Sprite>>,
    pub texture_id: TextureId,
    pub uv_scale: Vec2,
    pub width: u32,
    pub height: u32,
}

impl Decal {
    // consumes sprite!!!!
    // returns a decal and a spriteref
    pub fn new_from_sprite(pge: &mut PGE, sprite: Sprite) -> (Decal, SpriteRef) {
        let id = pge.create_texture(sprite.width, sprite.height);
        pge.update_texture(id, &sprite);
        let (width, height) = (sprite.width, sprite.height);
        let sprite_ref = SpriteRef::new_from_sprite(sprite);
        (Decal {
            sprite: Rc::downgrade(&sprite_ref.0),   // create a weak pointer to the sprite_ref
            texture_id: id,
            uv_scale: Vec2::ONE,
            width: width,
            height: height
        }, sprite_ref)
    }

    // does not consume anything!
    pub fn new_from_sprite_ref(pge: &mut PGE, sprite_ref: &SpriteRef) -> Decal {
        let id = pge.create_texture(sprite_ref.width(), sprite_ref.height());
        let sprite = sprite_ref.0.borrow();
        pge.update_texture(id, &sprite);
        let (width, height) = (sprite_ref.width(), sprite_ref.height());
        Decal {
            sprite: Rc::downgrade(&sprite_ref.0),
            texture_id: id,
            uv_scale: Vec2::ONE,
            width: width,
            height: height
        }
    }
}

#[derive(Debug)]
pub struct DecalInstance {
    pub vertices: Vec<Vertex>,
    pub tint: Color,
    pub mode: DecalMode,
    pub structure: DecalStructure,
}