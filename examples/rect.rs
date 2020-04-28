use pge::{PGE, State, Pixel, PixelMode};

struct GameState;

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, _dt: f32) -> bool {

        pge.clear(&Pixel::rgba(255,255,255, 255));

        pge.set_pixel_mode(PixelMode::Mask);

        let x = pge.get_mouse_x();
        let y = pge.get_mouse_y();

        pge.fill_rect(x, y, 250, 175, &Pixel::rgba(255,0,0, 255));
        pge.fill_rect(x, y, 150, 75, &Pixel::rgba(0,255,0, 128));

        true
    }
}

fn main() {
    let mut gs = GameState{};
    let pge:  PGE  = PGE::construct("Rect", 640, 480, 1, 1);
    PGE::start(pge, &mut gs);
}