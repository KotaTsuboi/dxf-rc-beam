use crate::input::RcBeamDrawing;
use crate::output_util::*;
use anyhow::Result;
use dxf::{entities::Polyline, tables::Layer, Color, Drawing};

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

    let rebar_layer = Layer {
        name: input.layer_name.text.clone(),
        color: Color::from_index(3),
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

    let w = input.concrete.beam_width;
    let h = input.concrete.beam_height;

    let coords = [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];

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
    let w = input.concrete.beam_width;
    let h = input.concrete.beam_height;
    let d = input.concrete.cover_depth;
    let r = input.main_rebar.diameter / 2.0;

    let mut y = d + r;

    let dy = input.main_rebar.gap;

    let mut result = Vec::new();

    if input.main_rebar.bottom_1 < 2 {
        panic!("The number of bottom rebar is less than 2.");
    }
    let dx = (w - 2.0 * d - 2.0 * r) / (input.main_rebar.bottom_1 - 1) as f64;

    for n in [
        input.main_rebar.bottom_1,
        input.main_rebar.bottom_2,
        input.main_rebar.bottom_3,
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

    if input.main_rebar.top_1 < 2 {
        panic!("The number of top rebar is less than 2.");
    }
    let dx = (w - 2.0 * d - 2.0 * r) / (input.main_rebar.top_1 - 1) as f64;

    for n in [
        input.main_rebar.top_1,
        input.main_rebar.top_2,
        input.main_rebar.top_3,
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

fn write_main_rebar(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let coords = get_rebar_coord(input)?;
    let layer = &input.layer_name.rebar;

    for coord in coords {
        let x = coord.0;
        let y = coord.1;
        let r = input.main_rebar.diameter / 2.0;
        write_circle(drawing, x, y, r, layer)?;
        write_cross(drawing, x, y, r + 1.0, layer)?;
    }

    Ok(())
}

fn get_web_rebar_coord(input: &RcBeamDrawing) -> Result<Vec<(f64, f64)>> {
    let mut coords = Vec::new();

    let n = input.web_rebar.num_row;

    if n == 0 {
        return Ok(coords);
    }

    let w = input.concrete.beam_width;
    let h = input.concrete.beam_height;
    let d = input.concrete.cover_depth;
    let r = input.main_rebar.diameter;
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

fn write_web_rebar(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let coords = get_web_rebar_coord(input)?;
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

fn write_stirrup(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let w = input.concrete.beam_width;
    let h = input.concrete.beam_height;
    let d = input.concrete.cover_depth;
    let r = input.main_rebar.diameter / 2.0;
    let g = input.main_rebar.gap;
    let layer = &input.layer_name.rebar;

    write_line(drawing, d + r, d, w - d - r, d, layer)?;
    write_line(drawing, d + r, h - d, w - d - r, h - d, layer)?;
    write_line(drawing, d, d + r, d, h - d - r, layer)?;
    write_line(drawing, w - d, d + r, w - d, h - d - r, layer)?;

    if input.main_rebar.bottom_2 > 0 {
        write_line(drawing, d + r, d + g, w - d - r, d + g, layer)?;
    }

    if input.main_rebar.bottom_3 > 0 {
        write_line(drawing, d + r, d + 2.0 * g, w - d - r, d + 2.0 * g, layer)?;
    }

    if input.main_rebar.top_2 > 0 {
        write_line(
            drawing,
            d + r,
            h - d - 2.0 * r - g,
            w - d - r,
            h - d - 2.0 * r - g,
            layer,
        )?;
    }

    if input.main_rebar.top_3 > 0 {
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

fn write_texts(drawing: &mut Drawing, input: &RcBeamDrawing) -> Result<()> {
    let text_height = input.layout.text_height;
    let x = 0.0;
    let mut y = -1000.0;
    let dy = 2.0 * text_height;

    let values = [
        input.beam_name.clone(),
        format!(
            "{}x{}",
            input.concrete.beam_width, input.concrete.beam_height
        ),
        format!(
            "{}-D{}",
            input.main_rebar.top_1 + input.main_rebar.top_2 + input.main_rebar.top_3,
            input.main_rebar.diameter
        ),
        format!(
            "{}-D{}",
            input.main_rebar.bottom_1 + input.main_rebar.bottom_2 + input.main_rebar.bottom_3,
            input.main_rebar.diameter
        ),
        format!(
            "{}-D{}@{}",
            input.stirrup.num, input.stirrup.diameter, input.stirrup.pitch
        ),
        format!(
            "{}-D{}",
            2 * input.web_rebar.num_row,
            input.web_rebar.diameter,
        ),
    ];

    for value in values {
        write_text(drawing, x, y, text_height, &value, &input.layer_name.text)?;
        y -= dy;
    }

    Ok(())
}

pub fn write(input: RcBeamDrawing, output_file: &str) -> Result<()> {
    let mut drawing = Drawing::new();

    set_layer(&mut drawing, &input)?;

    write_concrete(&mut drawing, &input)?;

    write_main_rebar(&mut drawing, &input)?;

    write_web_rebar(&mut drawing, &input)?;

    write_stirrup(&mut drawing, &input)?;

    write_texts(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
