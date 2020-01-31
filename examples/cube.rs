use pge::{PGE, State, Pixel};
use pge::gfx3d::vec3d::Vec3d;
use pge::gfx3d::vec4d::Vec4d;
use pge::gfx3d::mat4x4::Mat4x4;
use pge::gfx3d::{Pipeline, Triangle};


struct GameState {
    pipeline: Pipeline,
    time: f32
}

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, dt: f32) -> bool {

        self.time += dt;
        pge.clear(&Pixel::rgb(0, 0, 0));

        self.pipeline.set_projection(80.0, 1.0, 0.1, 1000.0, 0.0, 0.0, 400.0, 400.0);
        self.pipeline.set_camera(Vec3d{x: 0.0, y: 0.0, z: -3.0}, 
            Vec3d{x: 0.0, y: 0.0, z: 1.0},
            Vec3d{x: 0.0, y: 1.0, z: 0.0});
        self.pipeline.set_transform(Mat4x4::make_translation(-0.5, -0.5, -0.5) * Mat4x4::make_rotation_z(self.time) * Mat4x4::make_rotation_y(self.time));

        let cube = vec!{

            // SOUTH
		   Triangle { p: [Vec4d { x:0.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:0.0, w:1.0}], 
                        t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                        col: Pixel::rgb(255, 0, 0) 
                    },
            Triangle { p: [Vec4d { x:0.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:0.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                    },
            // EAST
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0}], 
                        t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                        col: Pixel::rgb(255, 0, 0) 
                    },
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:1.0, y:0.0, z:1.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                    },       
            // NORTH
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:1.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                },
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:1.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                    },
            // WEST
            Triangle { p: [Vec4d { x:0.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                },
            Triangle { p: [Vec4d { x:0.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 0) 
                    },
            // TOP
            Triangle { p: [Vec4d { x:0.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:0.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 255) 
                },
            Triangle { p: [Vec4d { x:0.0, y:1.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:1.0, w:1.0 }, Vec4d { x:1.0, y:1.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 0, 255) 
                    },
            // BOTTOM
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 255, 0) 
                    },
            Triangle { p: [Vec4d { x:1.0, y:0.0, z:1.0, w:1.0 }, Vec4d { x:0.0, y:0.0, z:0.0, w:1.0 }, Vec4d { x:1.0, y:0.0, z:0.0, w:1.0}], 
                    t: [Vec3d{ x:0.0, y:1.0, z:1.0}, Vec3d{ x:0.0, y:0.0, z:1.0}, Vec3d{ x:1.0, y:0.0, z:1.0,}],
                    col: Pixel::rgb(255, 255, 0) 
                    }
        };

        self.pipeline.render(pge, cube);
        true
    }
}

fn main() {
    let mut gs = GameState{ pipeline: Pipeline::new(), time: 0.0 };
    let mut pge = PGE::construct("Cube", 640, 480, 1, 1);
    pge.start(&mut gs);
}