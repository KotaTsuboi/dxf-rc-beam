use crate::input::RcBeamDrawing;
use anyhow::Result;
use dxf::{
    entities::{Circle, Entity, Line, Polyline},
    enums::{HorizontalTextJustification, VerticalTextJustification},
    tables::Layer,
    Color, Drawing, Point,
};

fn set_layer(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let concrete_layer = Layer {
        name: input.layer_name.concrete.clone(),
        color: Color::from_index(2),
        ..Default::default()
    };

    drawing.add_layer(concrete_layer);

    let rebar_layer = Layer {
        name: input.layer_name.rebar.clone(),
        color: Color::from_index(4),
        ..Default::default()
    };

    drawing.add_layer(rebar_layer);

    Ok(())
}

fn write_concrete(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let mut polyline = Polyline {
        ..Default::default()
    };

    polyline.set_is_closed(true);

    let w = input.dimension.beam_width;
    let h = input.dimension.beam_height;

    let coords = vec![(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];

    for i in 0..4 {
        let x1 = coords[i % 4].0;
        let x2 = coords[(i + 1) % 4].0;
        let y1 = coords[i % 4].1;
        let y2 = coords[(i + 1) % 4].1;
        let layer = input.layer_name.concrete.clone();

        write_line(drawing, x1, y1, x2, y2, &layer)?;
    }

    Ok(())
}

fn get_rebar_coord(input: &RcBeamDrawing) -> Result<Vec<(f64, f64)>> {
    let w = input.dimension.beam_width;
    let h = input.dimension.beam_height;
    let d = input.dimension.cover_depth;
    let r = input.dimension.rebar_diameter / 2.0;

    let mut y = d + r;

    let dy = input.dimension.gap_between_rebar;

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

pub fn write_circle(drawing: &mut Drawing, x: f64, y: f64, r: f64, layer: &str) -> Result<()> {
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

fn write_cross(drawing: &mut Drawing, x: f64, y: f64, r: f64, layer: &str) -> Result<()> {
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

fn write_rebars(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let coords = get_rebar_coord(input)?;
    let layer = &input.layer_name.rebar;

    for coord in coords {
        let x = coord.0;
        let y = coord.1;
        let r = input.dimension.rebar_diameter / 2.0;
        write_circle(drawing, x, y, r, layer)?;
        write_cross(drawing, x, y, r + 1.0, layer)?;
    }

    Ok(())
}

fn get_side_rebar_coord(input: &RcBeamDrawing) -> Result<Vec<(f64, f64)>> {
    let mut coords = Vec::new();

    let n = input.num_rebar.side_rebar_row;

    if n == 0 {
        return Ok(coords);
    }

    let w = input.dimension.beam_width;
    let h = input.dimension.beam_height;
    let d = input.dimension.cover_depth;
    let r = input.dimension.rebar_diameter;
    let dy = (h - 2.0 * d - 2.0 * r) / (n + 1) as f64;

    for i in 1..=n {
        let xi = d + 10.0;
        let xj = w - d - 10.0;
        let y = d + r + dy * i as f64;

        coords.push((xi, y));
        coords.push((xj, y));
    }

    Ok(coords)
}

fn write_side_rebar(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let coords = get_side_rebar_coord(input)?;
    let layer = &input.layer_name.rebar;

    for coord in &coords {
        let x = coord.0;
        let y = coord.1;
        let r = 10.0;
        write_cross(drawing, x, y, r, layer)?;
    }

    let mut i = 0;
    while i < coords.len() {
        let x1 = coords[i].0 - 10.0;
        let y1 = coords[i].1 + 10.0;
        let x2 = coords[i + 1].0 + 10.0;
        let y2 = coords[i + 1].1 + 10.0;

        write_line(drawing, x1, y1, x2, y2, layer)?;

        i += 2;
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
) -> Result<()> {
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

fn write_stirrup(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let w = input.dimension.beam_width;
    let h = input.dimension.beam_height;
    let d = input.dimension.cover_depth;
    let r = input.dimension.rebar_diameter / 2.0;
    let g = input.dimension.gap_between_rebar;
    let layer = &input.layer_name.rebar;

    write_line(drawing, d + r, d, w - d - r, d, layer)?;
    write_line(drawing, d + r, h - d, w - d - r, h - d, layer)?;
    write_line(drawing, d, d + r, d, h - d - r, layer)?;
    write_line(drawing, w - d, d + r, w - d, h - d - r, layer)?;

    if input.num_rebar.bottom_2 > 0 {
        write_line(drawing, d + r, d + g, w - d - r, d + g, layer)?;
    }

    if input.num_rebar.bottom_3 > 0 {
        write_line(drawing, d + r, d + 2.0 * g, w - d - r, d + 2.0 * g, layer)?;
    }

    if input.num_rebar.top_2 > 0 {
        write_line(
            drawing,
            d + r,
            h - d - 2.0 * r - g,
            w - d - r,
            h - d - 2.0 * r - g,
            layer,
        )?;
    }

    if input.num_rebar.top_3 > 0 {
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

fn write_text(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    text_height: f64,
    value: &str,
    layer: &str,
) -> Result<()> {
    let location = Point { x, y, z: 0.0 };

    let text = dxf::entities::Text {
        location,
        text_height,
        value: value.to_string(),
        horizontal_text_justification: HorizontalTextJustification::Middle,
        vertical_text_justification: VerticalTextJustification::Middle,
        ..Default::default()
    };
    let mut entity = Entity::new(dxf::entities::EntityType::Text(text));
    entity.common.layer = layer.to_string();
    drawing.add_entity(entity);
    Ok(())
}

fn write_texts(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let text_height = input.layout.text_height;
    let base = -1000.0;
    write_text(
        drawing,
        0.0,
        base,
        text_height,
        &input.beam_name,
        &input.layer_name.text,
    )?;
    write_text(
        drawing,
        0.0,
        base - 2.0 * text_height,
        text_height,
        &format!(
            "{}x{}",
            input.dimension.beam_width, input.dimension.beam_height
        ),
        &input.layer_name.text,
    )?;
    write_text(
        drawing,
        0.0,
        base - 4.0 * text_height,
        text_height,
        &format!(
            "{}-D{}",
            input.num_rebar.top_1 + input.num_rebar.top_2 + input.num_rebar.top_3,
            input.dimension.rebar_diameter
        ),
        &input.layer_name.text,
    )?;
    write_text(
        drawing,
        0.0,
        base - 6.0 * text_height,
        text_height,
        &format!(
            "{}-D{}",
            input.num_rebar.bottom_1 + input.num_rebar.bottom_2 + input.num_rebar.bottom_3,
            input.dimension.rebar_diameter
        ),
        &input.layer_name.text,
    )?;
    Ok(())
}

pub fn write(input: RcBeamDrawing, output_file: &str) -> Result<()> {
    let mut drawing = Drawing::new();

    set_layer(&mut drawing, &input)?;

    write_concrete(&mut drawing, &input)?;

    write_rebars(&mut drawing, &input)?;

    write_side_rebar(&mut drawing, &input)?;

    write_stirrup(&mut drawing, &input)?;

    write_texts(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
