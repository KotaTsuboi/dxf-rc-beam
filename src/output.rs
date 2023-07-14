use crate::input::RcBeamDrawing;
use dxf::{
    entities::{Circle, Entity, Line, Polyline},
    tables::Layer,
    Color, Drawing, Point,
};
use std::error::Error;

fn set_layer(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<(), Box<dyn Error>> {
    let concrete_layer = Layer {
        name: input.layer_name().concrete(),
        color: Color::from_index(2),
        ..Default::default()
    };

    drawing.add_layer(concrete_layer);

    let rebar_layer = Layer {
        name: input.layer_name().rebar(),
        color: Color::from_index(4),
        ..Default::default()
    };

    drawing.add_layer(rebar_layer);

    Ok(())
}

fn write_concrete(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<(), Box<dyn Error>> {
    let mut polyline = Polyline {
        ..Default::default()
    };

    polyline.set_is_closed(true);

    let w = input.beam_width();
    let h = input.beam_height();

    let coords = vec![(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];

    for i in 0..4 {
        let x1 = coords[i % 4].0;
        let x2 = coords[(i + 1) % 4].0;
        let y1 = coords[i % 4].1;
        let y2 = coords[(i + 1) % 4].1;
        let layer = input.layer_name().concrete().clone();

        write_line(drawing, x1, y1, x2, y2, &layer)?;
    }

    Ok(())
}

fn get_rebar_coord(input: &RcBeamDrawing) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let w = input.beam_width();
    let h = input.beam_height();
    let d = input.cover_depth();
    let r = input.rebar_diameter() / 2.0;

    let mut y = d + r;

    let dy = input.gap_between_rebar();

    let mut result = Vec::new();

    if input.num_rebar().bottom_1() < 2 {
        panic!("The number of bottom rebar is less than 2.");
    }
    let dx = (w - 2.0 * d - 2.0 * r) / (input.num_rebar().bottom_1() - 1) as f64;

    for n in [
        input.num_rebar().bottom_1(),
        input.num_rebar().bottom_2(),
        input.num_rebar().bottom_3(),
    ] {
        for i in 0..n {
            let base = if i % 2 == 0 { d + r } else { w - d - r };
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            let x = base + sign * (i / 2) as f64 * dx;
            result.push((x, y));
        }

        y += dy;
    }

    let mut y = h - d - r;

    if input.num_rebar().top_1() < 2 {
        panic!("The number of top rebar is less than 2.");
    }
    let dx = (w - 2.0 * d - 2.0 * r) / (input.num_rebar().top_1() - 1) as f64;

    for n in [
        input.num_rebar().top_1(),
        input.num_rebar().top_2(),
        input.num_rebar().top_3(),
    ] {
        for i in 0..n {
            let base = if i % 2 == 0 { d + r } else { w - d - r };
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            let x = base + sign * (i / 2) as f64 * dx;
            result.push((x, y));
        }

        y -= dy;
    }

    Ok(result)
}

pub fn write_circle(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    r: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    let circle = Circle {
        center: Point { x, y, z: 0.0 },
        radius: r,
        ..Default::default()
    };

    let mut circle = Entity::new(dxf::entities::EntityType::Circle(circle));

    circle.common.layer = layer.to_string();

    drawing.add_entity(circle);

    Ok(())
}

fn write_cross(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    r: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    write_line(
        drawing,
        x - r / 2_f64.sqrt(),
        y + r / 2_f64.sqrt(),
        x + r / 2_f64.sqrt(),
        y - r / 2_f64.sqrt(),
        layer,
    )?;

    write_line(
        drawing,
        x - r / 2_f64.sqrt(),
        y - r / 2_f64.sqrt(),
        x + r / 2_f64.sqrt(),
        y + r / 2_f64.sqrt(),
        layer,
    )?;

    Ok(())
}

fn write_rebars(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<(), Box<dyn Error>> {
    let coords = get_rebar_coord(input)?;

    for coord in coords {
        let x = coord.0;
        let y = coord.1;
        let r = input.rebar_diameter() / 2.0;
        let layer = &input.layer_name().rebar();
        write_circle(drawing, x, y, r, layer)?;
        write_cross(drawing, x, y, r + 1.0, layer)?;
    }

    Ok(())
}

fn write_line(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    let line = Line {
        p1: Point {
            x: x1,
            y: y1,
            z: 0.0,
        },
        p2: Point {
            x: x2,
            y: y2,
            z: 0.0,
        },
        ..Default::default()
    };
    let mut line = Entity::new(dxf::entities::EntityType::Line(line));
    line.common.layer = layer.to_string();
    drawing.add_entity(line);
    Ok(())
}

fn write_stirrup(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<(), Box<dyn Error>> {
    let w = input.beam_width();
    let h = input.beam_height();
    let d = input.cover_depth();
    let r = input.rebar_diameter() / 2.0;
    let g = input.gap_between_rebar();
    let layer = &input.layer_name().rebar();

    write_line(drawing, d + r, d, w - d - r, d, layer)?;
    write_line(drawing, d + r, h - d, w - d - r, h - d, layer)?;
    write_line(drawing, d, d + r, d, h - d - r, layer)?;
    write_line(drawing, w - d, d + r, w - d, h - d - r, layer)?;

    if input.num_rebar().bottom_2() > 0 {
        write_line(drawing, d + r, d + g, w - d - r, d + g, layer)?;
    }

    if input.num_rebar().bottom_3() > 0 {
        write_line(drawing, d + r, d + 2.0 * g, w - d - r, d + 2.0 * g, layer)?;
    }

    if input.num_rebar().top_2() > 0 {
        write_line(
            drawing,
            d + r,
            h - d - 2.0 * r - g,
            w - d - r,
            h - d - 2.0 * r - g,
            layer,
        )?;
    }

    if input.num_rebar().top_3() > 0 {
        write_line(
            drawing,
            d + r,
            h - d - 2.0 * r - 2.0 * g,
            w - d - r,
            h - d - 2.0 * r - 2.0 * g,
            layer,
        )?;
    }

    Ok(())
}

pub fn write(input: RcBeamDrawing, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    set_layer(&mut drawing, &input)?;

    write_concrete(&mut drawing, &input)?;

    write_rebars(&mut drawing, &input)?;

    write_stirrup(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
