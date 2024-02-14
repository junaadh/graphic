const WIDTH: usize = 800;
const HEIGHT: usize = 600;
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

use graphik::{
    graphik_draw_circle, graphik_fill, graphik_save_to_ppm, Error, GraphikBuffer, GraphikRect,
};

fn main() -> Result<(), Error> {
    let mut rect = GraphikRect::new(300, 200).center(true).color(0xff2020ff);
    GraphikBuffer::new(800, 600)
        .fill(0xff20ff20)
        .rect_fill(&mut rect)
        .save_as_ppm("target/rectangle2.ppm")?;

    unsafe {
        // TODO: implement as GraphikBuffer functions
        graphik_fill(&mut BUFFER, 0x0f0f00ff);
        graphik_draw_circle(
            &mut BUFFER,
            WIDTH,
            HEIGHT,
            (WIDTH / 2) as i32,
            (HEIGHT / 2) as i32,
            150,
            0xff0000ff,
        );
        graphik_save_to_ppm(&BUFFER, WIDTH, HEIGHT, "target/circle.ppm".to_string())?;

        // graphik_fill(&mut BUFFER, 0x0f0f00ff);
        // graphik_draw_line(
        //     &mut BUFFER,
        //     WIDTH,
        //     HEIGHT,
        //     0,
        //     0,
        //     WIDTH as i32,
        //     HEIGHT as i32,
        //     0xffffffff,
        // );
        // graphik_draw_line(
        //     &mut BUFFER,
        //     WIDTH,
        //     HEIGHT,
        //     WIDTH as i32,
        //     0,
        //     0,
        //     HEIGHT as i32,
        //     0xffffffff,
        // );
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
        // graphik_save_to_ppm(&BUFFER, WIDTH, HEIGHT, "target/line.ppm".to_string())?;
    }
    Ok(())
}
