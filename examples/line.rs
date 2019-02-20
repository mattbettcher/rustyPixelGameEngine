use pge::{PGE, State, Pixel};

struct GameState;

impl State for GameState {
    fn on_user_create(&mut self) -> bool {
        true
    }

    fn on_user_update(&mut self, pge: &mut PGE, _dt: f32) -> bool {

        pge.clear(&Pixel::rgb(0,0,100));

        let x = pge.get_mouse_x();
        let y = pge.get_mouse_y();

        pge.draw_line(320, 240, x, y, &Pixel::rgb(100,100,100));

        true
    }
}

fn main() {
    let mut gs = GameState{};
    let mut pge = PGE::construct("Line", 640, 480, 1, 1);
    pge.start(&mut gs);
}