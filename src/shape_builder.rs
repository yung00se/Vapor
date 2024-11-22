use std::{default, panic::Location};

use eframe::{egui::{ text::LayoutJob, Color32, FontDefinitions, FontId, Galley, Rect, Rounding, Shape, Stroke}, 
             epaint::{text::layout, FontFamily, Fonts, RectShape, TextShape}};
//use emath::{Pos2, Vec2};

#[derive(Default)]
pub struct RectAttributes{ // Attributes for creating shapes
    pub dimensions: (u8, u8), // Height/Width for shapes that have different height/width (i.e., Rectangles, Ovals)
    pub fill_color: Color32, // Fill color for the shape
    pub rounding: Rounding, // Rounding for the shape's corners
    pub outline: Stroke, // Stroke for the shape's outline
}

impl From<RectAttributes> for RectShape{ // Converts ShapeAttributes to RectShape

    fn from(attributes: RectAttributes) -> Self{
        Self{ rect: Rect::from(attributes.dimensions), // Convert the dimensions to a Rect
              rounding: Rounding::from(attributes.rounding), // Convert the rounding to a Rounding
              fill: attributes.fill_color, // Assign the fill color
              stroke: attributes.outline, // Assign the outline
              blur_width: 0.0, // Default blur width
              fill_texture_id: Default::default(), // Default fill texture
              uv: Rect::ZERO}} // Default UV
}
