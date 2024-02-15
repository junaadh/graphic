const WIDTH: usize = 800;
const HEIGHT: usize = 600;
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

use graphik::{
    graphik_draw_line, graphik_fill, graphik_save_to_ppm, Error, GraphikBuffer, GraphikCircle,
    GraphikRect,
};

fn main() -> Result<(), Error> {
    let mut rect = GraphikRect::new(300, 200).center(true).color(0xff2020ff);
    GraphikBuffer::new(WIDTH, HEIGHT)
        .fill(0xff20ff20)
        .rect_fill(&mut rect)
        .save_as_ppm("target/rectangle.ppm")?;

    let mut circ = GraphikCircle::new(150).center(true).color(0xff0000ff);
    GraphikBuffer::new(WIDTH, HEIGHT)
        .fill(0xffffffff)
        .circle_fill(&mut circ)
        .save_as_ppm("target/circle.ppm")?;

    unsafe {
        // TODO: implement as GraphikBuffer functions
        graphik_fill(&mut BUFFER, 0x0f0f00ff);
        graphik_draw_line(
            &mut BUFFER,
            WIDTH,
            HEIGHT,
            0,
            0,
            WIDTH as i32,
            HEIGHT as i32,
            0xffffffff,
        );
        graphik_draw_line(
            &mut BUFFER,
            WIDTH,
            HEIGHT,
            WIDTH as i32,
            0,
            0,
            HEIGHT as i32,
            0xffffffff,
        );
        // graphik_draw_line(
        //     &mut BUFFER,
        //     WIDTH,
        //     HEIGHT,
        //     WIDTH as i32 / 2,
        //     0,
        //     WIDTH as i32 / 2,
        //     HEIGHT as i32,
        //     0xffffffff,
        // );
        // graphik_draw_line(
        //     &mut BUFFER,
        //     WIDTH,
        //     HEIGHT,
        //     0,
        //     HEIGHT as i32 / 2,
        //     WIDTH as i32,
        //     HEIGHT as i32 / 2,
        //     0xffffffff,
        // );
        graphik_save_to_ppm(&BUFFER, WIDTH, HEIGHT, "target/line.ppm".to_string())?;
    }
    Ok(())
}
