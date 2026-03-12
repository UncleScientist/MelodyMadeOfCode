use macroquad::prelude::*;
use quest2::part1::Part1Solver;

#[macroquad::main("Quest 2")]
async fn main() -> Result<(), macroquad::Error> {
    let bone = quest2::load_file("input/everybody_codes_e3_q02_p1.txt");
    let mut solver = Part1Solver::new(bone[0]);
    let mut done = false;
    loop {
        let w = screen_width() / 100.0;
        let h = screen_height() / 100.0;

        let transform = (w * 50., h * 50.);

        let (v, (yloc, xloc)) = solver.state();

        for &(y, x) in v {
            draw_rectangle(
                transform.0 + x as f32 * w,
                transform.1 + y as f32 * h,
                w,
                h,
                GREEN,
            );
        }
        draw_rectangle(
            transform.0 + bone[0].1 as f32 * w,
            transform.1 + bone[0].0 as f32 * h,
            w,
            h,
            RED,
        );
        draw_rectangle(
            transform.0 + xloc as f32 * w,
            transform.1 + yloc as f32 * h,
            w,
            h,
            BLUE,
        );
        if !done && solver.next().is_none() {
            done = true;
        }
        next_frame().await
    }
}
