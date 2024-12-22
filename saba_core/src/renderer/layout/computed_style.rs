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
