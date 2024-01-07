use pge::{PGE, Pixel, GameLoop};

struct GameState;

impl GameLoop for GameState {
    type GameType = GameState;

    fn init(_pge: &mut PGE) -> Self {
        GameState
    }

    fn update(&mut self, pge: &mut PGE, _dt: f64) {
        pge.clear(&Pixel::rgb(0,0,100));

        let x = pge.get_mouse_x();
        let y = pge.get_mouse_y();

        pge.draw_line(320, 240, x, y, &Pixel::rgb(100,100,100));
    }
}

fn main() {
    // what we had
    //let mut pge = PGE::construct("Line", 640, 480, 1, 1);
    //pge.start(&mut gs);
    //let mut conf = conf::Conf::default();

    // Soon...
    PGE::construct::<GameState>("Line", 640, 480, 2, 2);
}