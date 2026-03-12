use macroquad::prelude::*;

#[macroquad::main("Quest 2")]
async fn main() -> Result<(), macroquad::Error> {
    loop {
        draw_rectangle(100., 100., 40., 20., GREEN);
        next_frame().await
    }
}
