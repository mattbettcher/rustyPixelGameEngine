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
    pub uv_scale: Vec2,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct DecalInstance {
    pub decal: Decal,
    pub pos: Vec<Vec2>,
    pub uv: Vec<Vec2>,
    pub w: Vec<f32>,
    pub tint: Color,
    pub mode: DecalMode,
    pub structure: DecalStructure,
}