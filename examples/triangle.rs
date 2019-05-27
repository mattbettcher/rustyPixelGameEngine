use pge::{PGE, State, Pixel, PixelMode};

struct GameState;

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, _dt: f32) -> bool {

        pge.clear(&Pixel::rgb(0,0,100));

        pge.set_pixel_mode(PixelMode::Normal);

        let x = pge.get_mouse_x();
        let y = pge.get_mouse_y();

        pge.fill_triangle(x, y, 200, 200, 175, 50, &Pixel::rgba(100,0,0, 255));

        true
    }
}

fn main() {
    let mut gs = GameState{};
    let mut pge = PGE::construct("Triangle", 640, 480, 1, 1);
    pge.start(&mut gs);
}