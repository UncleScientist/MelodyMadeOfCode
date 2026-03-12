use macroquad::prelude::*;
use quest2::{DrawState, part1::Part1Solver, part2::Part2Solver};

#[macroquad::main("Quest 2")]
async fn main() -> Result<(), macroquad::Error> {
    let part1bone = quest2::load_file("input/everybody_codes_e3_q02_p1.txt");
    let mut solver1 = Part1Solver::new(part1bone[0]);

    let part2bone = quest2::load_file("input/everybody_codes_e3_q02_p2.txt");
    let mut solver2 = Part2Solver::new(part2bone[0]);

    let mut screen = Screen::default();
    let mut current_state = State::Solver1;
    loop {
        match current_state {
            State::Solver1 => {
                screen.draw(solver1.state(), part1bone[0]);
                if solver1.next().is_none() || is_mouse_button_pressed(MouseButton::Left) {
                    current_state = State::Waiting1;
                }
            }
            State::Waiting1 => {
                screen.draw(solver1.state(), part1bone[0]);
                if is_mouse_button_pressed(MouseButton::Left) {
                    current_state = State::Solver2;
                }
            }
            State::Solver2 => {
                screen.draw(solver2.state(), part2bone[0]);
                if solver2.next().is_none() || is_mouse_button_pressed(MouseButton::Left) {
                    current_state = State::Waiting2;
                }
            }
            State::Waiting2 => {
                screen.draw(solver2.state(), part2bone[0]);
                if is_mouse_button_pressed(MouseButton::Left) {
                    current_state = State::_Solver3;
                }
            }
            State::_Solver3 => todo!(),
            State::_Waiting3 => todo!(),
        }
        next_frame().await
    }
}

enum State {
    Solver1,
    Waiting1,
    Solver2,
    Waiting2,
    _Solver3,
    _Waiting3,
}

#[derive(Default)]
struct Screen {
    transform: (f32, f32),
    width: f32,
    height: f32,
}

impl Screen {
    fn draw(&mut self, state: DrawState<'_>, bone: (i32, i32)) {
        self.width = screen_width() / 100.0;
        self.height = screen_height() / 100.0;

        self.transform = (self.width * 50., self.height * 50.);

        for &(y, x) in state.visited {
            self.rect(x as f32, y as f32, GREEN);
        }
        self.rect(bone.1 as f32, bone.0 as f32, RED);
        self.rect(state.cur_loc.1 as f32, state.cur_loc.0 as f32, BLUE);

        draw_text(&format!("Steps: {} ", state.steps), 10., 20., 24., WHITE);
    }

    fn rect(&self, x: f32, y: f32, color: Color) {
        draw_rectangle(
            self.transform.0 + x * self.width,
            self.transform.1 + y * self.height,
            self.width - 1.0,
            self.height - 1.0,
            color,
        );
    }
}
