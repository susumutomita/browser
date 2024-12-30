use crate::alloc::string::ToString;
use crate::browser::Browser;
use crate::http::HttpResponse;
use crate::renderer::dom::node::Window;
use alloc::rc::Rc;
use alloc::rc::Weak;
use core::cell::RefCell;

use crate::display_item::DisplayItem;
use crate::renderer::css::cssom::StyleSheet;
use crate::renderer::html::parser::HtmlParser;
use crate::renderer::html::token::HtmlTokenizer;
use crate::renderer::layout::layout_view::LayoutView;
use crate::utils::convert_dom_to_string;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Page {
    browser: Weak<RefCell<Browser>>,
    frame: Option<Rc<RefCell<Window>>>,
    style: Option<StyleSheet>,
    layout_view: Option<LayoutView>,
    display_items: Vec<DisplayItem>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            browser: Weak::new(),
            frame: None,
            style: None,
            layout_view: None,
            display_items: Vec::new(),
        }
    }

    pub fn set_browser(&mut self, browser: Weak<RefCell<Browser>>) {
        self.browser = browser;
    }

    pub fn set_frame(&mut self, frame: Rc<RefCell<Window>>) {
        self.frame = Some(frame);
    }

    pub fn set_style(&mut self, style: StyleSheet) {
        self.style = Some(style);
    }

    pub fn set_layout_view(&mut self, layout_view: LayoutView) {
        self.layout_view = Some(layout_view);
    }

    pub fn set_display_items(&mut self, display_items: Vec<DisplayItem>) {
        self.display_items = display_items;
    }

    pub fn receive_response(&mut self, response: HttpResponse) -> String {
        self.create_frame(response.body());

        if let Some(frame) = &self.frame {
            let dom = frame.borrow().document().clone();
            let debug = convert_dom_to_string(&Some(dom));
            return debug;
        }
        "".to_string()
    }

    fn create_frame(&mut self, html: String) {
        let html_tokenizer = HtmlTokenizer::new(html);
        let frame = HtmlParser::new(html_tokenizer).construct_tree();
        self.frame = Some(frame);
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}
