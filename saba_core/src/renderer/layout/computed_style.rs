use crate::error::Error;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayType {
    Block,
    Inline,
    None,
}

// 2. TextDecorationの定義を追加
#[derive(Debug, Clone, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
}

// 3. FontSizeの定義を追加
#[derive(Debug, Clone, PartialEq)]
pub struct FontSize {
    pub value: i64,
}

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

impl Default for ComputedStyle {
    fn default() -> Self {
        Self::new()
    }
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
        FontSize {
            value: self
                .font_size
                .expect("failed to access Css property: font_size"),
        }
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
            "black" => "#000000".to_string(),
            "silver" => "#C0C0C0".to_string(),
            "gray" => "#808080".to_string(),
            "white" => "#FFFFFF".to_string(),
            "maroon" => "#800000".to_string(),
            "red" => "#FF0000".to_string(),
            "purple" => "#800080".to_string(),
            "fuchsia" => "#FF00FF".to_string(),
            "green" => "#00FF00".to_string(),
            "lime" => "#00FF00".to_string(),
            "olive" => "#808000".to_string(),
            "yellow" => "#FFFF00".to_string(),
            "navy" => "#000080".to_string(),
            "blue" => "#0000FF".to_string(),
            "teal" => "#008080".to_string(),
            "aqua" => "#00FFFF".to_string(),
            "orange" => "#FFA500".to_string(),
            "lightgray" => "#D3D3D3".to_string(),
            _ => {
                return Err(Error::UnexpectedInput(format!(
                    "color name {:?} is not supported yet",
                    name
                )));
            }
        };
        Ok(Self {
            name: Some(name.to_string()),
            code: code.to_string(),
        })
    }

    pub fn from_code(code: &str) -> Result<Self, Error> {
        if code.chars().nth(0) != Some('#') || code.len() != 7 {
            return Err(Error::UnexpectedInput(format!(
                "invalid color code {}",
                code
            )));
        }
        let name = match code {
            "#000000" => "black".to_string(),
            "#C0C0C0" => "silver".to_string(),
            "#808080" => "gray".to_string(),
            "#FFFFFF" => "white".to_string(),
            "#800000" => "maroon".to_string(),
            "#FF0000" => "red".to_string(),
            "#800080" => "purple".to_string(),
            "#FF00FF" => "fuchsia".to_string(),
            "#008000" => "green".to_string(),
            "#00FF00" => "lime".to_string(),
            "#808000" => "olive".to_string(),
            "#FFFF00" => "yellow".to_string(),
            "#000080" => "navy".to_string(),
            "#0000FF" => "blue".to_string(),
            "#008080" => "teal".to_string(),
            "#00FFFF" => "aqua".to_string(),
            "#FFA500" => "orange".to_string(),
            "#D3D3D3" => "lightgray".to_string(),
            _ => {
                return Err(Error::UnexpectedInput(format!(
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
        u32::from_str_radix(self.code.trim_start_matches('#'), 16).unwrap()
    }
}
