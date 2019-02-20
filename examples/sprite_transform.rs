use pge::{PGE, State, Pixel, PixelMode, Sprite};
use pge::gfx2d::{Transform2D, GFX2D};
use image::GenericImageView;

struct GameState {
    car_sprite: Sprite,
    car_angle: f32,
}

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, dt: f32) -> bool {

        pge.clear(&Pixel::rgba(100, 149, 237, 0));
        pge.set_pixel_mode(PixelMode::Mask);

        let mut transform = Transform2D::new();
        transform.reset();
        transform.translate(self.car_sprite.width as f32 * -0.5, self.car_sprite.height as f32 * -0.5);
        self.car_angle += 1.0 * dt;
        transform.rotate(self.car_angle);
        transform.translate(320.0, 240.0);

        GFX2D::draw_sprite(pge, &self.car_sprite, &mut transform);

        true
    }
}

fn main() {
    let image = image::open("car_top1.png").unwrap();
    let raw_image = image.raw_pixels();

    let mut gs = GameState{
        car_sprite: Sprite::new_with_data(image.width() as usize, image.height() as usize, raw_image),
        car_angle: 0.0,
    };
    gs.car_sprite.from_rgba_to_bgra();
    let mut pge = PGE::construct("Sprite Transform", 640, 480, 1, 1);
    pge.start(&mut gs);
}