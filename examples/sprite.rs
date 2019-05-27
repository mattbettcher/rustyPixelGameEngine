use pge::{PGE, State, Pixel, PixelMode, Sprite};
use image::GenericImageView;

struct GameState {
    spr: Sprite,
}

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, _dt: f32) -> bool {

        pge.clear(&Pixel::rgb(0,0,100));

        pge.set_pixel_mode(PixelMode::Alpha);

        let x = pge.get_mouse_x();
        let y = pge.get_mouse_y();

        pge.draw_sprite(x, y, &self.spr, 3);
        pge.draw_parital_sprite(x - 55, y - 55, &self.spr, 25, 25, 50, 50, 3);

        true
    }
}

fn main() {
    let image = image::open("logo_long.png").unwrap();
    let raw_image = image.raw_pixels();

    let mut gs = GameState{
        spr: Sprite::new_with_data(image.width() as usize, image.height() as usize, raw_image),
    };
    gs.spr.from_rgba_to_bgra(); // hack
    let mut pge = PGE::construct("Sprite", 640, 480, 1, 1);
    pge.start(&mut gs);
}