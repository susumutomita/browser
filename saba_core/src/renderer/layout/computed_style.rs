use crate::error::Error;
use crate::renderer::dom::node::ElementKind;
use crate::renderer::dom::node::Node;
use crate::renderer::dom::node::NodeKind;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use core::cell::RefCell;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DisplayType {
    Block,
    Inline,
    DisplayNone,
}

impl DisplayType {
    fn default(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Document => DisplayType::Block,
            NodeKind::Element(e) => {
                if e.is_block_element() {
                    DisplayType::Block
                } else {
                    DisplayType::Inline
                }
            }
            NodeKind::Text(_) => DisplayType::Inline,
        }
    }
    pub fn from_str_display(s: &str) -> Result<Self, Error> {
        match s {
            "block" => Ok(Self::Block),
            "inline" => Ok(Self::Inline),
            "none" => Ok(Self::DisplayNone),
            _ => Err(Error::UnexpectedInput(format!(
                "display type {:?} is not supported yet",
                s
            ))),
        }
    }
}

// 2. TextDecorationの定義を追加
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
}
impl TextDecoration {
    fn default(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Element(element) => match element.kind() {
                ElementKind::A => TextDecoration::Underline,
                _ => TextDecoration::None,
            },
            _ => TextDecoration::None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FontSize {
    Medium,
    XLarge,
    XXLarge,
}
impl FontSize {
    fn default(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Element(element) => match element.kind() {
                ElementKind::H1 => FontSize::XXLarge,
                ElementKind::H2 => FontSize::XLarge,
                _ => FontSize::Medium,
            },
            _ => FontSize::Medium,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    background_color: Option<Color>,
    color: Option<Color>,
    display: Option<DisplayType>,
    font_size: Option<FontSize>,
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

    pub fn defaulting(&mut self, node: &Rc<RefCell<Node>>, parent_style: Option<ComputedStyle>) {
        if let Some(parent_style) = parent_style {
            if self.background_color.is_none() && parent_style.background_color() != Color::white()
            {
                self.background_color = Some(parent_style.background_color());
            }
            if self.color.is_none() && parent_style.color() != Color::black() {
                self.color = Some(parent_style.color());
            }
            if self.font_size.is_none() && parent_style.font_size() != FontSize::Medium {
                self.font_size = Some(parent_style.font_size());
            }
            if self.text_decoration.is_none()
                && parent_style.text_decoration() != TextDecoration::None
            {
                self.text_decoration = Some(parent_style.text_decoration());
            }
        }
        if self.background_color.is_none() {
            self.set_background_color(Color::white());
        }
        if self.color.is_none() {
            self.set_color(Color::black());
        }
        if self.display.is_none() {
            self.set_display_type_default(node);
        }
        if self.font_size.is_none() {
            self.set_font_size_default(node);
        }
        if self.text_decoration.is_none() {
            self.set_text_decoration_default(node);
        }
        if self.height.is_none() {
            self.set_height(0);
        }
        if self.width.is_none() {
            self.set_width(0);
        }
    }

    pub fn display(&self) -> DisplayType {
        self.display
            .expect("failed to access Css property: display")
    }

    pub fn set_display_type_default(&mut self, node: &Rc<RefCell<Node>>) {
        let display_type = DisplayType::default(node);
        self.display = Some(display_type);
    }

    pub fn font_size(&self) -> FontSize {
        self.font_size
            .expect("failed to access CSS property: font_size")
    }
    pub fn set_font_size_default(&mut self, node: &Rc<RefCell<Node>>) {
        let fs = FontSize::default(node);
        self.font_size = Some(fs);
    }
    pub fn text_decoration(&self) -> TextDecoration {
        self.text_decoration
            .expect("failed to access Css property: text_decoration")
    }

    pub fn set_text_decoration_default(&mut self, node: &Rc<RefCell<Node>>) -> TextDecoration {
        let text_decoration = TextDecoration::default(node);
        self.text_decoration = Some(text_decoration);
        text_decoration
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
