use crate::input::TomlInput;
use dxf::{
    entities::{Circle, Entity, Line, Polyline, Vertex},
    Drawing, Point,
};
use std::error::Error;

pub fn write_rectangle(drawing: &mut Drawing, w: f64, h: f64) -> Result<(), Box<dyn Error>> {
    let mut polyline = Polyline {
        ..Default::default()
    };

    polyline.set_is_closed(true);

    let coords = vec![(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];
    for coord in coords {
        let vertex = Vertex {
            location: Point {
                x: coord.0,
                y: coord.1,
                z: 0.0,
            },
            ..Default::default()
        };
        polyline.add_vertex(drawing, vertex);
    }

    let polyline = Entity::new(dxf::entities::EntityType::Polyline(polyline));

    drawing.add_entity(polyline);

    Ok(())
}

fn get_rebar_coord(input: &TomlInput) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let w = input.beam_width;
    let h = input.beam_height;
    let d = input.cover_depth;
    let r = input.rebar_diameter / 2.0;

    let mut y = d + r;

    let dy = input.gap_between_rebar;

    let mut result = Vec::new();

    if input.num_rebar.bottom_1 < 2 {
        panic!("The number of bottom rebar is less than 2.");
    }
    let dx = (w - 2.0 * d - 2.0 * r) / (input.num_rebar.bottom_1 - 1) as f64;

    for n in [
        input.num_rebar.bottom_1,
        input.num_rebar.bottom_2,
        input.num_rebar.bottom_3,
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

    if input.num_rebar.top_1 < 2 {
        panic!("The number of top rebar is less than 2.");
    }
    let dx = (w - 2.0 * d - 2.0 * r) / (input.num_rebar.top_1 - 1) as f64;

    for n in [
        input.num_rebar.top_1,
        input.num_rebar.top_2,
        input.num_rebar.top_3,
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

pub fn write_circle(drawing: &mut Drawing, x: f64, y: f64, r: f64) -> Result<(), Box<dyn Error>> {
    let circle = Circle {
        center: Point { x, y, z: 0.0 },
        radius: r,
        ..Default::default()
    };

    let circle = Entity::new(dxf::entities::EntityType::Circle(circle));

    drawing.add_entity(circle);

    Ok(())
}

fn write_cross(drawing: &mut Drawing, x: f64, y: f64, r: f64) -> Result<(), Box<dyn Error>> {
    write_line(
        drawing,
        x - r / 2_f64.sqrt(),
        y + r / 2_f64.sqrt(),
        x + r / 2_f64.sqrt(),
        y - r / 2_f64.sqrt(),
    )?;

    write_line(
        drawing,
        x - r / 2_f64.sqrt(),
        y - r / 2_f64.sqrt(),
        x + r / 2_f64.sqrt(),
        y + r / 2_f64.sqrt(),
    )?;

    Ok(())
}

fn write_rebars(drawing: &mut Drawing, input: &TomlInput) -> Result<(), Box<dyn Error>> {
    let coords = get_rebar_coord(input)?;

    for coord in coords {
        let x = coord.0;
        let y = coord.1;
        let r = input.rebar_diameter / 2.0;
        write_circle(drawing, x, y, r)?;
        write_cross(drawing, x, y, r + 1.0)?;
    }

    Ok(())
}

fn write_line(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
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
    let line = Entity::new(dxf::entities::EntityType::Line(line));
    drawing.add_entity(line);
    Ok(())
}

fn write_stirrup(drawing: &mut Drawing, input: &TomlInput) -> Result<(), Box<dyn Error>> {
    let w = input.beam_width;
    let h = input.beam_height;
    let d = input.cover_depth;
    let r = input.rebar_diameter / 2.0;
    let g = input.gap_between_rebar;

    write_line(drawing, d + r, d, w - d - r, d)?;
    write_line(drawing, d + r, h - d, w - d - r, h - d)?;
    write_line(drawing, d, d + r, d, h - d - r)?;
    write_line(drawing, w - d, d + r, w - d, h - d - r)?;

    if input.num_rebar.bottom_2 > 0 {
        write_line(drawing, d + r, d + g, w - d - r, d + g)?;
    }

    if input.num_rebar.bottom_3 > 0 {
        write_line(drawing, d + r, d + 2.0 * g, w - d - r, d + 2.0 * g)?;
    }

    if input.num_rebar.top_2 > 0 {
        write_line(
            drawing,
            d + r,
            h - d - 2.0 * r - g,
            w - d - r,
            h - d - 2.0 * r - g,
        )?;
    }

    if input.num_rebar.top_3 > 0 {
        write_line(
            drawing,
            d + r,
            h - d - 2.0 * r - 2.0 * g,
            w - d - r,
            h - d - 2.0 * r - 2.0 * g,
        )?;
    }

    Ok(())
}

pub fn write(input: TomlInput, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    let w = input.beam_width;
    let h = input.beam_height;
    write_rectangle(&mut drawing, w, h)?;

    write_rebars(&mut drawing, &input)?;

    write_stirrup(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
