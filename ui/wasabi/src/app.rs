use crate::alloc::string::ToString;
use alloc::format;
use alloc::rc::Rc;
use core::cell::RefCell;
use noli::error::Result as OsResult;
// use noli::prelude::SystemApi;
// use noli::println;
// use noli::sys::api::MouseEvent;
// use noli::sys::wasabi::Api;
use noli::window::StringSize;
use noli::window::Window;
use saba_core::browser::Browser;
use saba_core::constants::ADDRESS_BAR_HEIGHT;
use saba_core::constants::BLACK;
use saba_core::constants::DARK_GRAY;
use saba_core::constants::GRAY;
use saba_core::constants::LIGHT_GRAY;
use saba_core::constants::TITLE_BAR_HEIGHT;
use saba_core::constants::TOOLBAR_HEIGHT;
use saba_core::constants::WHITE;
use saba_core::constants::WINDOW_HEIGHT;
use saba_core::constants::WINDOW_INIT_X_POS;
use saba_core::constants::WINDOW_INIT_Y_POS;
use saba_core::constants::WINDOW_WIDTH;
use saba_core::error::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}

#[allow(dead_code)]
impl WasabiUI {
    fn handle_mouse_input(&mut self) -> Result<(), Error> {
        // if let Some(MouseEvent {
        //     button: _button,
        //     position,
        // }) = Api::get_mouse_cursor_info()
        // {
        //     println!("mouse position {:?}", position);
        // }
        Ok(())
    }

    fn setup(&mut self) -> Result<(), Error> {
        if let Err(error) = self.setup_toolbar() {
            return Err(Error::InvalidUI(format!(
                "failed to initialize a toolbar with error: {:#?}",
                error
            )));
        }
        self.window.flush();
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.setup()?;
        self.run_app()?;
        Ok(())
    }

    fn run_app(&mut self) -> Result<(), Error> {
        loop {
            self.handle_mouse_input()?;
        }
    }

    fn setup_toolbar(&mut self) -> OsResult<()> {
        self.window
            .fill_rect(LIGHT_GRAY, 0, 0, WINDOW_WIDTH, TOOLBAR_HEIGHT)?;

        self.window.draw_line(
            DARK_GRAY,
            0,
            TITLE_BAR_HEIGHT + 1,
            WINDOW_WIDTH - 1,
            TOOLBAR_HEIGHT + 1,
        )?;

        self.window.draw_string(
            BLACK,
            5,
            5,
            "Address:",
            StringSize::Medium,
            /*underline=*/ false,
        )?;

        self.window
            .fill_rect(WHITE, 70, 2, WINDOW_WIDTH - 74, 2 + ADDRESS_BAR_HEIGHT)?;

        self.window.draw_line(GRAY, 70, 2, WINDOW_WIDTH - 4, 2)?;
        self.window
            .draw_line(GRAY, 70, 2, 70, 2 + ADDRESS_BAR_HEIGHT)?;
        self.window.draw_line(BLACK, 71, 3, WINDOW_WIDTH - 5, 3)?;

        self.window
            .draw_line(GRAY, 71, 3, 71, 1 + ADDRESS_BAR_HEIGHT)?;
        Ok(())
    }
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap(),
        }
    }
}
