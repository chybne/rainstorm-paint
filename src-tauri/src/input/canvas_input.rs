use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// All of the different actions the user can perform on the canvas
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CanvasInput {
    /// The ui should handle changing the zoom so instead of send the delta we send a zoom
    /// we want the canvas. Mouse_x and Mouse_y is to zoom relative to point, probably should
    /// make a seperate action specifically for zooming relateive to point
    #[serde(rename_all = "camelCase")]
    ZoomCanvas { zoom: f32 },
    #[serde(rename_all = "camelCase")]
    PanCanvas { offset_x: f32, offset_y: f32 },

    StartStroke {
        mouse_x: f32,
        mouse_y: f32,
        pressure: f32,
    },
}

impl Display for CanvasInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanvasInput::ZoomCanvas { zoom } => {
                write!(f, "ZoomCanvas(zoom: {zoom})")
            }
            CanvasInput::PanCanvas { offset_x, offset_y } => {
                write!(f, "PanCanvas(offset_x: {offset_x}, offset_y: {offset_y})")
            }
            CanvasInput::StartStroke {
                mouse_x,
                mouse_y,
                pressure,
            } => {
                write!(
                    f,
                    "StartStroke(mouse_x: {mouse_x}, mouse_y: {mouse_y}, pressure: {pressure})"
                )
            }
        }
    }
}
