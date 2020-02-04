use pge::Sprite;
use pge::{PGE, State, Pixel};
use pge::gfx3d::vec3d::Vec3d;
use pge::gfx3d::vec4d::Vec4d;
use pge::gfx3d::mat4x4::Mat4x4;
use pge::gfx3d::{Pipeline, Triangle};
use image::GenericImageView;
use minifb::Key;

struct GameState {
    pipeline: Pipeline,
    tex: Sprite,
    time: f32,
    camera: Vec3d,
    look_dir: Vec3d,
    yaw: f32,
    theta: f32,
}

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, dt: f32) -> bool {

        self.time += dt;

        pge.clear(&Pixel::rgb(0, 0, 0));
        self.pipeline.clear_depth();

        if pge.get_key(Key::Up).held { self.camera.y += 8.0 * dt; }     // Travel Upwards
        if pge.get_key(Key::Down).held { self.camera.y -= 8.0 * dt; }   // Travel Downwards

        if pge.get_key(Key::Left).held { self.camera.x -= 8.0 * dt; }     // Travel Along X-axis
        if pge.get_key(Key::Right).held { self.camera.x += 8.0 * dt; }   // Travel Along X-axis

        let forward = self.look_dir * (8.0 * dt);

		// Standard FPS Control scheme, but turn instead of strafe
        if pge.get_key(Key::W).held { self.camera = self.camera + forward; }
        if pge.get_key(Key::S).held { self.camera = self.camera - forward; }

        if pge.get_key(Key::A).held { self.yaw += 8.0 * dt; }
        if pge.get_key(Key::D).held { self.yaw -= 8.0 * dt; }

        self.theta += 0.5 * dt;

        let rot_z = Mat4x4::make_rotation_z(self.theta * 0.5);
        let rot_x = Mat4x4::make_rotation_x(self.theta);
        let translate = Mat4x4::make_translation(0.0, 0.0, 0.0);

        let world = rot_z * rot_x * translate;

        let up = Vec3d {x: 0.0, y: 1.0, z:0.0 };
        let mut target = Vec3d {x: 0.0, y: 0.0, z:1.0 };
        let cam_rot = Mat4x4::make_rotation_y(self.yaw);
        self.look_dir = cam_rot * target;
        target = self.camera + self.look_dir;
        self.pipeline.set_camera(self.camera, target, up);
        self.pipeline.set_projection(80.0, pge.screen_height as f32 / pge.screen_width as f32, 0.01, 1000.0, 0.0, 0.0, pge.screen_width as f32, pge.screen_height as f32);
        self.pipeline.set_transform(world);
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

        
        self.pipeline.render(pge, &cube, &self.tex);

        true
    }
}

fn main() {
    let image = image::open("unwrap_helper.png").unwrap();
    let raw_image = image.raw_pixels();

    let mut gs = GameState{ 
        pipeline: Pipeline::new(640, 480), 
        time: 0.0,
        tex: Sprite::new_with_data(image.width() as usize, image.height() as usize, raw_image),
        camera: Vec3d {x: 0.0, y: 0.0, z:-2.0},
        look_dir: Vec3d {x: 0.0, y: 0.0, z:1.0},
        theta: 0.0,
        yaw: 0.0
    };

    gs.tex.from_rgba_to_bgra(); // hack
    let mut pge = PGE::construct("Cube", 640, 480, 1, 1);
    pge.start(&mut gs);
}