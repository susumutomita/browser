use crate::renderer::layout::computed_style::ComputedStyle;
use crate::renderer::layout::layout_object::LayoutPoint;
use crate::renderer::layout::layout_object::LayoutSize;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayItem {
    Rect {
        style: ComputedStyle,
        layout_point: LayoutPoint,
        layout_size: LayoutSize,
    },
    Text {
        style: ComputedStyle,
        layout_point: LayoutPoint,
        text: String,
    },
    Img {
        src: String,
        style: ComputedStyle,
        layout_point: LayoutPoint,
    },
}

impl DisplayItem {
    pub fn is_rect(&self) -> bool {
        matches!(
            self,
            DisplayItem::Rect {
                style: _,
                layout_point: _,
                layout_size: _,
            }
        )
    }

    pub fn is_text(&self) -> bool {
        matches!(
            self,
            DisplayItem::Text {
                text: _,
                style: _,
                layout_point: _,
            }
        )
    }
}
