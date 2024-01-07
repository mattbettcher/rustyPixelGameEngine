use glam::Vec2;

use crate::{*, decal::DecalInstance};

pub struct Layer {
    pub offset: Vec2,
    pub scale: Vec2,
    pub show: bool,
    pub update: bool,
    //pub surface: Renderable,
    pub decal_instances: Vec<DecalInstance>,
    pub tint: Color,
    pub id: usize,
}

//impl<'a> Layer<'a> {
//    pub fn create(width: usize, height: usize) -> Self {
//        Layer { 
//            offset: Vec2::ZERO, 
//            scale: Vec2::ONE, 
//            show: false, 
//            update: false, 
//            surface: Renderable::create(width, height, filter, clamp), 
//            decal_instances: vec![], 
//            tint: WHITE, 
//            id: 0 }
//    }
//}