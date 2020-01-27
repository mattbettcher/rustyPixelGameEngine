use pge::*;
use pge::gfx2d::{Transform2D, GFX2D};
use pge::gfx2d::vec2d::Vec2d;
use image::GenericImageView;
use minifb::Key;

struct GameState {
    spr: Sprite,
    total_time: f32,    // todo: move????
	list_events: Vec<String>,
}

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
		for _ in 0..16 {
			self.list_events.push("".to_owned());
		}
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, dt: f32) -> bool {

        pge.set_pixel_mode(PixelMode::Normal);
        pge.clear(&BLUE);
        
        pge.draw_circle(32, 32, 30, &WHITE);
        pge.draw_circle(96, 32, 30, &WHITE);

        let mx = pge.get_mouse_x() as f32;
        let my = pge.get_mouse_y() as f32;

		// test v2d type
		// p1 & p2 are inferred to be f32 because mx & my are f32
		let mut p1 = Vec2d::new(mx - 32.0, my - 32.0);
		let pr1 = 1.0 / p1.length();
		p1 = p1 * 22.0 * pr1 + Vec2d::new(32.0, 32.0);

		let mut p2 = Vec2d::new(mx - 96.0, my - 32.0);
		let pr2 = 1.0 / p2.length();
		p2 = p2 * 22.0 * pr2 + Vec2d::new(96.0, 32.0);

		pge.fill_circle(p1.x as i32, p1.y as i32, 8, &CYAN);
		pge.fill_circle(p2.x as i32, p2.y as i32, 8, &CYAN);

        pge.draw_line(10, 70, 54, 70, &WHITE);	// Lines
		pge.draw_line(54, 70, 70, 54, &WHITE);

		pge.draw_rect(10, 80, 54, 30, &WHITE);
		pge.fill_rect(10, 80, 54, 30, &WHITE);

		// Multiline Text
		let mouse_pos_string = format!("Your Mouse Position is:\nX={0:.6}\nY={1:.6}", mx as f32, my as f32);
		pge.draw_string(10, 130, &mouse_pos_string, &WHITE, 1);
		
		let mut add_event = |s: String| -> String {
			self.list_events.push(s);
			self.list_events.remove(0)
		};

		if pge.get_mouse(0).pressed { add_event("Mouse Button 0 Down".to_owned());} 
		if pge.get_mouse(0).released { add_event("Mouse Button 0 Up".to_owned());} 
		if pge.get_mouse(1).pressed  { add_event("Mouse Button 1 Down".to_owned());} 
		if pge.get_mouse(1).released { add_event("Mouse Button 1 Up".to_owned());} 
		if pge.get_mouse(2).pressed  { add_event("Mouse Button 2 Down".to_owned());} 
		if pge.get_mouse(2).released { add_event("Mouse Button 2 Up".to_owned());}

		if pge.get_key(Key::A).pressed { add_event("A key Down".to_owned());} 
		if pge.get_key(Key::A).released { add_event("A key Up".to_owned());} 

		// Draw Event Log
		let mut n_log: u8 = 0;
		for s in &self.list_events {
			pge.draw_string(200, n_log as i32 * 8 + 20, &s, &Pixel::rgb(n_log * 16, n_log * 16, n_log * 16), 1);
			n_log += 1;
		}

        // Test Text scaling and colours
		pge.draw_string(0, 360, "Text Scale = 1", &WHITE, 1);
		pge.draw_string(0, 368, "Text Scale = 2", &BLUE, 2);
		pge.draw_string(0, 384, "Text Scale = 3", &RED, 3);
		pge.draw_string(0, 408, "Text Scale = 4", &YELLOW, 4);
		pge.draw_string(0, 440, "Text Scale = 5", &GREEN, 5);

        self.total_time += dt;

		let angle = self.total_time;

		// Draw Sprite using extension, first create a transformation stack
        let mut t1 = Transform2D::new();
        t1.reset();

		// Translate sprite so center of image is at 0,0
		t1.translate(-250.0, -35.0);
		// Scale the sprite
		t1.scale(1.0 * angle.sin() + 1.0, 1.0 * angle.sin() + 1.0);
		// Rotate it
		t1.rotate(angle*2.0);
		// Translate to 0,100
		t1.translate(0.0, 100.0);
		// Rotate different speed
		t1.rotate(angle / 3.0);
		// Translate to centre of screen
		t1.translate(320.0, 240.0);

		pge.set_pixel_mode(PixelMode::Alpha);

		// Use extension to draw sprite with transform applied
		GFX2D::draw_sprite(pge, &self.spr, &mut t1);

		pge.draw_sprite(mx as i32, my as i32, &self.spr, 4);

        true
    }
}

fn main() {
    let image = image::open("logo_long.png").unwrap();
    let raw_image = image.raw_pixels();

    let mut gs = GameState{
        spr: Sprite::new_with_data(image.width() as usize, image.height() as usize, raw_image),
        total_time: 0.0,
		list_events: vec![],
    };
    gs.spr.from_rgba_to_bgra(); // hack
    let mut pge = PGE::construct("Testing Graphics2D", 640, 480, 2, 2);
    pge.start(&mut gs);
}