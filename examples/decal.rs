use pge::{PGE, Pixel, GameLoop, Sprite, Decal, SpriteRef, Layer, BLANK, WHITE, PixelMode, color};

struct GameState {
    logo_ref: SpriteRef,
    logo_decal: Decal,
}

impl GameLoop for GameState {
    type GameType = GameState;

    fn init(pge: &mut PGE) -> Self {
        let data = include_bytes!("../logo_long.png");
        let image = image::load_from_memory_with_format(data, image::ImageFormat::Png).unwrap();
        let raw_image = image.as_bytes();
        
        let logo = Sprite::new_with_data(image.width(), image.height(), raw_image);
        let (logo_decal, logo_ref) = Decal::new_from_sprite(pge, logo);

        // or 

        //let logo_ref = SpriteRef::new_with_data(image.width(), image.height(), raw_image);
        //let logo_decal = Decal::new_from_sprite_ref(pge, &logo_ref);

        let layer = Layer::new(pge, 640, 480);
        pge.layers.push(layer);

        GameState {
            logo_ref,
            logo_decal
        }
    }

    fn update(&mut self, pge: &mut PGE, _dt: f64) {
        pge.clear(&WHITE);

        let x = pge.get_mouse_x();
        let y = pge.get_mouse_y();

        // we can still draw a SpriteRef with the CPU drawing methods and even edit it,
        // we just have to use this ugly syntax.
        pge.pixel_mode = PixelMode::Alpha;
        pge.draw_sprite(x, y, &self.logo_ref.get_sprite(), 1);
        pge.current_layer = 1;
        pge.clear(&BLANK);
        pge.draw_sprite(x + 100, y - 40, &self.logo_ref.get_sprite(), 2);
        pge.current_layer = 0;
    }
}

fn main() {
    PGE::construct::<GameState>("Decal", 640, 480, 1, 1);
}