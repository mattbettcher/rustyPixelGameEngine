use pge::{PGE, Pixel, GameLoop, Sprite};

struct GameState {
    logo: Sprite,
}

impl GameLoop for GameState {
    type GameType = GameState;

    fn init(_pge: &mut PGE) -> Self {
        let data = include_bytes!("../logo_long.png");
        let image = image::load_from_memory_with_format(data, image::ImageFormat::Png).unwrap();
        let raw_image = image.as_bytes();
        
        GameState {
            logo: Sprite::new_with_data(image.width(), image.height(), raw_image)
        }
    }

    fn update(&mut self, pge: &mut PGE, _dt: f64) {
        pge.clear(&Pixel::rgb(0,0,100));

        let x = pge.get_mouse_x();
        let y = pge.get_mouse_y();

        pge.draw_sprite(x, y, &self.logo, 1);
    }
}

fn main() {
    PGE::construct::<GameState>("Sprite", 640, 480, 2, 2);
}