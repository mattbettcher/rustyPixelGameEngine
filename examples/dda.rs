use minifb::Key;
use pge::{PGE, State, Pixel, PixelMode};
use pge::gfx2d::vec2d::Vec2d;


struct GameState {
    pub player: Vec2d,
    pub map_size: Vec2d,
    pub cell_size: Vec2d,
    pub map: Vec<i32>,
}

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        self.map.resize((self.map_size.x * self.map_size.y) as usize, 0);
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, dt: f32) -> bool {

        pge.clear(&Pixel::rgb(0,0,0));

        let mouse = Vec2d::new(pge.get_mouse_x() as f32, pge.get_mouse_y() as f32);
        let mouse_cell = Vec2d::new(mouse.x / self.cell_size.x, mouse.y / self.cell_size.y);
        let cell = Vec2d::new(mouse_cell.x.floor(), mouse_cell.y.floor());

        // Paint with right mouse button "solid" tiles
        if pge.get_mouse(2).held { 
            self.map[(cell.y * self.map_size.x + cell.x) as usize] = 1; 
        }
        
        // Move "player" position
        if pge.get_key(Key::W).held { self.player.y -= 250.0 * dt; }
        if pge.get_key(Key::S).held { self.player.y += 250.0 * dt; }
        if pge.get_key(Key::A).held { self.player.x -= 250.0 * dt; }
        if pge.get_key(Key::D).held { self.player.x += 250.0 * dt; }

        // DDA Algorithm ==============================================
		// https://lodev.org/cgtutor/raycasting.html

        // Form ray cast from player into scene
        let ray_start = self.player;
        let ray_dir = (mouse_cell - self.player).norm();

        let ray_unit_step_size = Vec2d::new(
            (1.0 + (ray_dir.y / ray_dir.x)  * (ray_dir.y / ray_dir.x)).sqrt(),
            (1.0 + (ray_dir.x / ray_dir.y)  * (ray_dir.x / ray_dir.y)).sqrt());
        let mut map_check = Vec2d::new(ray_start.x.floor(), ray_start.y.floor());
        let mut ray_length1d = Vec2d::zero();
        let mut step = Vec2d::zero();

        // Estabilish starting conditions
        if ray_dir.x < 0.0 {
            step.x = -1.0;
            ray_length1d.x = (ray_start.x - map_check.x) * ray_unit_step_size.x;
        } else {
            step.x = 1.0;
            ray_length1d.x = ((map_check.x + 1.0) - ray_start.x) * ray_unit_step_size.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1.0;
            ray_length1d.y = (ray_start.y - map_check.y) * ray_unit_step_size.y;
        } else {
            step.y = 1.0;
            ray_length1d.y = ((map_check.y + 1.0) - ray_start.y) * ray_unit_step_size.y;
        }

        // Perform "Walk" until collision or range check
        let mut tile_found = false;
        let max_distance = 100.0;
        let mut distance = 0.0;
        while !tile_found && distance < max_distance {
            // Walk along shorest path
            if ray_length1d.x < ray_length1d.y {
                map_check.x += step.x;
                distance = ray_length1d.x;
                ray_length1d.x += ray_unit_step_size.x;
            } else {
                map_check.y += step.y;
                distance = ray_length1d.y;
                ray_length1d.y += ray_unit_step_size.y;
            }

            // Test tile as new test point
            if map_check.x >= 0.0 && map_check.x < self.map_size.x
                && map_check.y >= 0.0 && map_check.y < self.map_size.y {
                    /*pge.fill_rect(
                        (map_check.x * self.cell_size.x) as i32, 
                        (map_check.y * self.cell_size.y) as i32,  
                        self.cell_size.x as i32, 
                        self.cell_size.y as i32, 
                        &Pixel::rgb(230, 230, 0)); */
                    if self.map[(map_check.y * self.map_size.x + map_check.x) as usize] == 1 {
                        tile_found = true;
                    }
            }
        }
        
        // Calculate intersection location
        let mut intersection = Vec2d::zero();
        if tile_found {
            intersection = ray_start + ray_dir * distance;
        }
        
        // Draw Map
        for y in 0..self.map_size.y as i32 {
            for x in 0..self.map_size.x as i32 {
                let cell = self.map[(y * self.map_size.x as i32 + x) as usize];
                if cell == 1 { 
                    pge.fill_rect(x * self.cell_size.x as i32, 
                        y * self.cell_size.y as i32,  
                        self.cell_size.x as i32, 
                        self.cell_size.y as i32, 
                        &Pixel::rgb(0, 0, 200)); 
                }
                // Draw Cell Border
                pge.draw_rect(x * self.cell_size.x as i32, 
                    y * self.cell_size.y as i32,  
                    self.cell_size.x as i32, 
                    self.cell_size.y as i32, 
                    &Pixel::rgb(0, 200, 0)); 
            }
        }

        // Draw ray between player and mouse if left mouse button held
        if pge.get_mouse(0).held {
            pge.draw_line(
                (self.player.x * self.cell_size.x) as i32, 
                (self.player.y * self.cell_size.y) as i32, 
                mouse.x as i32, 
                mouse.y as i32, 
                &Pixel::rgb(255, 255, 255));

            if tile_found {
                pge.draw_circle(
                    (intersection.x * self.cell_size.x) as i32,
                    (intersection.y * self.cell_size.y) as i32,
                    4, 
                    &Pixel::rgb(150, 150, 150));
            }
        }

        // Draw Player
        pge.fill_circle(
            (self.player.x * self.cell_size.x) as i32, 
            (self.player.y * self.cell_size.y) as i32,  
            4, 
            &Pixel::rgb(230,0,0));

        // Draw Mouse
        pge.fill_circle(
            mouse.x as i32,
            mouse.y as i32,
            4, 
            &Pixel::rgb(0,230,0));

        true
    }
}

fn main() {
    let mut gs = GameState { 
        player: Vec2d::new(0.0, 0.0), 
        map_size: Vec2d::new(32.0, 30.0), 
        cell_size: Vec2d::new(16.0, 16.0), 
        map: vec![]
    };
    let mut pge = PGE::construct("RayCast with DDA Algorithm", 512, 480, 2, 2);
    pge.start(&mut gs);
}