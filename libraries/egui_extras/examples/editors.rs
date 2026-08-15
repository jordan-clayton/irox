// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use eframe::emath::Vec2;
use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Ui, ViewportBuilder};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_egui_extras::widgets::NullableBooleanEditor;
use log::{error, Level};

pub fn main() {
    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-egui-editors",
        native_options,
        Box::new(|cc| {
            let comp = Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc))));
            Ok(comp)
        }),
    ) {
        error!("{e:?}");
    };
}

pub struct TestApp {
    field1: Option<bool>,
    field1_desc: NullableBooleanEditor<'static>,
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        TestApp {
            field1: None,
            field1_desc: NullableBooleanEditor::new("Field1", false),
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        self.field1_desc.show(&mut self.field1, ui);
    }
}

impl App for TestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            self.show(ui);
        });
    }
}
impl ToolApp for TestApp {}
