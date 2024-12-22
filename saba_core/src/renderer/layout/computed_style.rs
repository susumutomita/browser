use crate::error::Error;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    background_color: Option<Color>,
    color: Option<Color>,
    display: Option<DisplayType>,
    font_size: Option<i64>,
    text_decoration: Option<TextDecoration>,
    height: Option<i64>,
    width: Option<i64>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            background_color: None,
            color: None,
            display: None,
            font_size: None,
            text_decoration: None,
            height: None,
            width: None,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = Some(color);
    }

    pub fn background_color(&self) -> Color {
        self.background_color
            .clone()
            .expect("failed to access Css property: background_color")
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn color(&self) -> Color {
        self.color
            .clone()
            .expect("failed to access Css property: color")
    }

    pub fn set_display(&mut self, display: DisplayType) {
        self.display = Some(display);
    }

    pub fn display(&self) -> DisplayType {
        self.display
            .clone()
            .expect("failed to access Css property: display")
    }

    pub fn font_size(&self) -> FontSize {
        self.font_size
            .clone()
            .expect("failed to access Css property: font_size")
    }

    pub fn text_decoration(&self) -> TextDecoration {
        self.text_decoration
            .clone()
            .expect("failed to access Css property: text_decoration")
    }

    pub fn set_height(&mut self, height: i64) {
        self.height = Some(height);
    }

    pub fn height(&self) -> i64 {
        self.height.expect("failed to access Css property: height")
    }

    pub fn set_width(&mut self, width: i64) {
        self.width = Some(width);
    }

    pub fn width(&self) -> i64 {
        self.width.expect("failed to access Css property: width")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    name: Option<String>,
    code: String,
}

impl Color {
    pub fn from_name(name: &str) -> Result<Self, Error> {
        let code = match name {
            "black" => "#000000",
            "white" => "#FFFFFF",
            "red" => "#FF0000",
            "green" => "#00FF00",
            "blue" => "#0000FF",
            _ => {
                return Err(Error::UnexpectedInput(formt!(
                    "color name {:?} is not supported yet",
                    name
                )));
            }
        };
        Ok(Self {
            name: Some(name.to_string()),
            code,
        })
    }

    pub fn from_code(code: &str) -> Result<Self, Error> {
        if code.chars().nth(0) != Some('#') || code.len() != 7 {
            return Err(Error::UnexpectedInput(formt!(
                "invalid color code {}",
                code
            )));
        }
        let name = match code {
            "#000000" => "black".to_string(),
            "#FFFFFF" => "white".to_string(),
            "#FF0000" => "red".to_string(),
            "#00FF00" => "green".to_string(),
            "#0000FF" => "blue".to_string(),
            _ => {
                return Err(Error::UnexpectedInput(formt!(
                    "color code {:?} is not supported yet",
                    code
                )));
            }
        };
        Ok(Self {
            name: Some(name),
            code: code.to_string(),
        })
    }

    pub fn white() -> Self {
        Self {
            name: Some("white".to_string()),
            code: "#FFFFFF".to_string(),
        }
    }

    pub fn black() -> Self {
        Self {
            name: Some("black".to_string()),
            code: "#000000".to_string(),
        }
    }

    pub fn code_u32(&self) -> u32 {
        u32::from_str_radix(&self.code.trimg_start_matches('#'), 16).unwrap()
    }
}
