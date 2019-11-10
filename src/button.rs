pub use crate::prelude::*;
use fltk_sys::button::*;
use std::{ffi, mem, os::raw};

#[derive(WidgetTrait, Debug, Clone)]
pub struct Button {
    _inner: *mut Fl_Button,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum ButtonType {
    NormalButton = 0,
    ToggleButton = 1,
    RadioButton = 102,
    HiddenButton = 3,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct RadioButton {
    _inner: *mut Fl_Radio_Button,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct RoundButton {
    _inner: *mut Fl_Round_Button,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct CheckButton {
    _inner: *mut Fl_Check_Button,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct ToggleButton {
    _inner: *mut Fl_Toggle_Button,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct LightButton {
    _inner: *mut Fl_Light_Button,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct RepeatButton {
    _inner: *mut Fl_Repeat_Button,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct ReturnButton {
    _inner: *mut Fl_Return_Button,
}
