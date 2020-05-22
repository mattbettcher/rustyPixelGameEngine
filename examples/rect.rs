use pge::{PGE, Pixel, PixelMode};

struct GameState;

fn on_user_update(gs: &mut GameState, pge: &mut PGE<GameState>, _dt: f32) -> bool {

    pge.clear(&Pixel::rgba(255,255,255, 255));

    pge.set_pixel_mode(PixelMode::Mask);

    let x = pge.get_mouse_x();
    let y = pge.get_mouse_y();

    pge.fill_rect(x, y, 250, 175, &Pixel::rgba(255,0,0, 255));
    pge.fill_rect(x, y, 150, 75, &Pixel::rgba(0,255,0, 128));

    true
}

fn main() {
    let mut gs = GameState{};
    let pge: PGE<GameState> = PGE::construct("Rect", 640, 480, 1, 1, on_user_update);
    PGE::start(pge);
}