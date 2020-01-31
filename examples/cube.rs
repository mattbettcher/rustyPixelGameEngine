use pge::Sprite;
use pge::{PGE, State, Pixel};
use pge::gfx3d::vec3d::Vec3d;
use pge::gfx3d::vec4d::Vec4d;
use pge::gfx3d::mat4x4::Mat4x4;
use pge::gfx3d::{Pipeline, Triangle};
use image::GenericImageView;

struct GameState {
    pipeline: Pipeline,
    tex: Sprite,
    time: f32
}

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, dt: f32) -> bool {

        self.time += dt;
        pge.clear(&Pixel::rgb(0, 0, 0));

        self.pipeline.set_projection(80.0, pge.screen_height as f32 / pge.screen_width as f32, 0.1, 1000.0, 0.0, 0.0, pge.screen_width as f32, pge.screen_height as f32);
        self.pipeline.set_camera(Vec3d{x: 0.0, y: 0.0, z: -3.0}, 
            Vec3d{x: 0.0, y: 0.0, z: 1.0},
            Vec3d{x: 0.0, y: 1.0, z: 0.0});
        

        //self.pipeline.set_texture(&self.tex);

        let cube = vec!{

            // SOUTH
		   Triangle { p: [Vec4d { x:0.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:0.0, w:1.0}], 
                        t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                        col: Pixel::rgb(255, 0, 0) 
                    },
            Triangle { p: [Vec4d { x:0.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:0.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:1.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                    },
            // EAST
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0}], 
                        t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                        col: Pixel::rgb(255, 0, 0) 
                    },
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:1.0, y:0.0, z:1.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:1.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                    },       
            // NORTH
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:1.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                },
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:1.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:1.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                    },
            // WEST
            Triangle { p: [Vec4d { x:0.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                },
            Triangle { p: [Vec4d { x:0.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:1.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                    },
            // TOP
            Triangle { p: [Vec4d { x:0.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 255) 
                },
            Triangle { p: [Vec4d { x:0.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:1.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 255) 
                    },
            // BOTTOM
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 255, 0) 
                    },
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:0.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:1.0, z:1.0,}],
                    col: Pixel::rgb(255, 255, 0) 
                    }
        };

        for y in 0..5 {
            for x in 0..5 {
                self.pipeline.set_transform(Mat4x4::make_scale(0.45, 0.45, 0.45) * 
                    Mat4x4::make_translation(-0.5, -0.5, -0.5) * 
                    Mat4x4::make_rotation_z(self.time * 0.5) * 
                    Mat4x4::make_rotation_y(self.time * 0.5) * 
                    Mat4x4::make_rotation_x(self.time * 0.5) *
                    Mat4x4::make_translation(x as f32 - 2.5, y as f32 - 2.5, 0.0));
                self.pipeline.render(pge, &cube, &self.tex);
            }
        }
        

        self.pipeline.clear_depth();
        true
    }
}

fn main() {
    let image = image::open("unwrap_helper.png").unwrap();
    let raw_image = image.raw_pixels();

    let mut gs = GameState{ 
        pipeline: Pipeline::new(640, 480), 
        time: 0.0,
        tex: Sprite::new_with_data(image.width() as usize, image.height() as usize, raw_image)
    };

    gs.tex.from_rgba_to_bgra(); // hack
    let mut pge = PGE::construct("Cube", 640, 480, 1, 1);
    pge.start(&mut gs);
}