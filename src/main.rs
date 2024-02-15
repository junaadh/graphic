use graphik::{Error, GraphikBuffer, GraphikCircle, GraphikLine, GraphikRect, HEIGHT, WIDTH};

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

    let mut line = GraphikLine::new().color(0xff0000ff);
    GraphikBuffer::new(WIDTH, HEIGHT)
        .fill(0xff202020)
        .line_draw2(&mut line)
        .line_draw2(&mut line.start(WIDTH as i32, 0).end(0, HEIGHT as i32))
        .line_draw2(&mut line.horizontal_center(0, WIDTH as i32))
        .line_draw2(&mut line.vertical_center(0, HEIGHT as i32))
        .line_draw2(&mut line.vertical(WIDTH as i32 / 3, 0, HEIGHT as i32))
        .line_draw2(&mut line.horizontal(HEIGHT as i32 / 3, 0, WIDTH as i32))
        .line_draw2(&mut line.start(0, 0).end(WIDTH as i32 / 5, HEIGHT as i32))
        .line_draw2(&mut line.start(0, 0).end(WIDTH as i32 / 5 * 3, HEIGHT as i32))
        .save_as_ppm("target/line2.ppm")?;

    Ok(())
}
