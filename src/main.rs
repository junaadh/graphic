use graphik::{
    constants::*, graphik_circle::GraphikCircle, graphik_line::GraphikLine,
    graphik_rect::GraphikRect, lerpf, Error, GraphikBuilder,
};

fn main() -> Result<(), Error> {
    let background = 0xff202020_u32;
    let mut builder = GraphikBuilder::new(WIDTH, HEIGHT);

    let mut rect = GraphikRect::new(300, 200).center(true).color(0xff2020ff);
    builder.fill(background);
    builder.rect_fill(&mut rect);
    builder.save_as_ppm("target/rectangle.ppm")?;

    let mut circ = GraphikCircle::new(150).center(true).color(0xff0000ff);
    builder.fill(background);
    builder.circle_fill(&mut circ);
    builder.save_as_ppm("target/circle.ppm")?;

    let mut line = GraphikLine::new().color(0xff0000ff);
    builder.fill(background);
    builder.line_draw(&mut line);
    builder.line_draw(&mut line.start(WIDTH as i32, 0).end(0, HEIGHT as i32));
    builder.line_draw(&mut line.horizontal_center(0, WIDTH as i32));
    builder.line_draw(&mut line.vertical_center(0, HEIGHT as i32));
    builder.line_draw(&mut line.vertical(WIDTH as i32 / 3, 0, HEIGHT as i32));
    builder.line_draw(&mut line.horizontal(HEIGHT as i32 / 3, 0, WIDTH as i32));
    builder.line_draw(&mut line.start(0, 0).end(WIDTH as i32 / 5, HEIGHT as i32));
    builder.line_draw(&mut line.start(0, 0).end(WIDTH as i32 / 5 * 3, HEIGHT as i32));
    builder.save_as_ppm("target/line.ppm")?;

    let mut checker = GraphikRect::new(CELL_WIDTH, CELL_HEIGHT)
        .origin(0, 0)
        .color(0xffffffff);
    builder.fill(0x00000000);
    for y in 0..ROWS {
        for x in 0..COLS {
            if (x + y) % 2 == 0 {
                builder.rect_fill(
                    &mut checker.origin((x * CELL_WIDTH) as i32, (y * CELL_HEIGHT) as i32),
                );
            }
        }
    }
    builder.save_as_ppm("target/checkers.ppm")?;

    let mut radius = CELL_WIDTH;
    if CELL_HEIGHT < radius {
        radius = CELL_HEIGHT;
    }
    let mut circ = GraphikCircle::new(radius).color(0xff0000ff);
    builder.fill(background);
    for y in 0..ROWS {
        for x in 0..COLS {
            let u = x as f32 / COLS as f32;
            let v = y as f32 / ROWS as f32;
            let t = (u + v) / 2f32;

            builder.circle_fill(
                &mut circ
                    .radius(lerpf(radius as f32 / 8.0, radius as f32 / 2.0, t) as usize)
                    .origin(
                        (x * CELL_WIDTH + radius / 2) as i32,
                        (y * CELL_HEIGHT + radius / 2) as i32,
                    ),
            );
        }
    }
    builder.save_as_ppm("target/circle_checkers.ppm")?;

    Ok(())
}
