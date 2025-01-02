use alloc::rc::Rc;
use core::cell::RefCell;
use noli::window::Window;
use saba_core::browser::Browser;

#[derive(Debug)]
#[allow(dead_code)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}
