use glam::Vec2;

use crate::{*, decal::DecalInstance};

pub struct Layer {
    pub offset: Vec2,
    pub scale: Vec2,
    pub show: bool,
    pub update: bool,
    pub surface: Renderable,
    pub decal_instances: Vec<DecalInstance>,
    pub tint: Color,
    pub id: usize,
}