use anyhow::Result;
use dxf::{
    entities::{Circle, Entity, Line},
    enums::{HorizontalTextJustification, VerticalTextJustification},
    Drawing, Point,
};

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

pub fn write_cross(drawing: &mut Drawing, x: f64, y: f64, r: f64, layer: &str) -> Result<()> {
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

pub fn write_line(
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

pub fn write_text(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    text_height: f64,
    value: &str,
    layer: &str,
) -> Result<()> {
    let location = Point { x, y, z: 0.0 };

    let text = dxf::entities::Text {
        location: location.clone(),
        second_alignment_point: location,
        text_height,
        value: value.to_string(),
        horizontal_text_justification: HorizontalTextJustification::Middle,
        vertical_text_justification: VerticalTextJustification::Baseline,
        ..Default::default()
    };
    let mut entity = Entity::new(dxf::entities::EntityType::Text(text));
    entity.common.layer = layer.to_string();
    drawing.add_entity(entity);
    Ok(())
}
