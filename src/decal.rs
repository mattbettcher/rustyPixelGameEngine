use miniquad::*;
use glam::*;
use crate::*;

pub enum DecalMode {
    Normal,
    Additive,
    Multiplicative,
    Stencil,
    Illuminate,
    Wireframe,
    Model3D,
}

pub enum DecalStructure {
    Line,
    Fan,
    Strip,
    List,
}

pub struct Decal {
    pub id: TextureId,
    pub uv_scale: Vec2,
    pub width: u32,
    pub height: u32,
}

pub struct DecalInstance {
    //pub decal: Decal,
    pub pos: Vec<Vec2>,
    pub uv: Vec<Vec2>,
    pub w: Vec<f32>,
    pub tint: Color,
    pub mode: DecalMode,
    pub structure: DecalStructure,
}