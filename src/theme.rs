pub mod application;
pub mod button;
pub mod container;
pub mod slider;
pub mod svg;
pub mod text;
pub mod scrollable;

use iced::{Color, color};
// use iced_native::{
//     application,
//     widget::{container, text},
// };
pub struct Theme {
    text: Color,
    svg: Color,

    background: Color,
    currant_line: Color,
    foreground: Color,
    comment: Color,
    cyan: Color,
    green: Color,
    orange: Color,
    pink: Color,
    purple: Color,
    red: Color,
    yellow: Color,

    light_blue: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            text: Color::WHITE,
            svg: Color::WHITE,
    
            background: color!(40, 42, 54),
            currant_line: color!(68, 71, 90),
            foreground: color!(248, 248, 242),
            comment: color!(98, 114, 164),
            cyan: color!(139, 233, 253),
            green: color!(80, 250, 123),
            orange: color!(255, 184, 108),
            pink: color!(255, 121, 198),
            purple: color!(189, 147, 249),
            red: color!(255, 85, 85),
            yellow: color!(241, 250, 140),
    
            light_blue: color!(46, 144, 255),
        }
    }
}
