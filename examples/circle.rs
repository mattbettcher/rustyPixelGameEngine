use pge::{PGE, State, Pixel, PixelMode};

struct GameState;

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, _dt: f32) -> bool {

        pge.clear(&Pixel::rgb(0,0,100));

        pge.set_pixel_mode(PixelMode::Alpha);

        let x = pge.get_mouse_x();
        let y = pge.get_mouse_y();

        pge.fill_circle(x, y, 200, &Pixel::rgba(100,0,0, 255));
        pge.fill_circle(x, y, 50, &Pixel::rgba(0,100,0, 128));

        true
    }
}

fn main() {
    let mut gs = GameState{};
    let mut pge = PGE::construct("Circle", 640, 480, 1, 1);
    pge.start(&mut gs);
}