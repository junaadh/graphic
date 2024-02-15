use graphik::{
    lerpf, Error, GraphikBuilder, GraphikCircle, GraphikLine, GraphikRect, HEIGHT, WIDTH,
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

    let rows = 6 * 2;
    let cols = 8 * 2;
    let cell_width = WIDTH / cols;
    let cell_height = HEIGHT / rows;
    let mut checker = GraphikRect::new(cell_width, cell_height)
        .origin(0, 0)
        .color(0xffffffff);
    builder.fill(0x00000000);
    for y in 0..rows {
        for x in 0..cols {
            if (x + y) % 2 == 0 {
                builder.rect_fill(
                    &mut checker.origin((x * cell_width) as i32, (y * cell_height) as i32),
                );
            }
        }
    }
    builder.save_as_ppm("target/checkers.ppm")?;

    let mut radius = cell_width;
    if cell_height < radius {
        radius = cell_height;
    }
    let mut circ = GraphikCircle::new(radius).color(0xff0000ff);
    builder.fill(background);
    for y in 0..rows {
        for x in 0..cols {
            let u = x as f32 / cols as f32;
            let v = y as f32 / rows as f32;
            let t = (u + v) / 2f32;

            builder.circle_fill(
                &mut circ
                    .radius(lerpf(radius as f32 / 8.0, radius as f32 / 2.0, t) as usize)
                    .origin(
                        (x * cell_width + radius / 2) as i32,
                        (y * cell_height + radius / 2) as i32,
                    ),
            );
        }
    }
    builder.save_as_ppm("target/circle_checkers.ppm")?;

    Ok(())
}
