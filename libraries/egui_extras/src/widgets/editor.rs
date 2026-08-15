// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use egui::{Checkbox, Ui, Widget};

pub struct NullableBooleanEditor<'a> {
    name: &'a str,
    default_value: bool,
}

impl<'a> NullableBooleanEditor<'a> {
    pub fn new(name: &'a str, default_value: bool) -> Self {
        Self {
            name,
            default_value,
        }
    }
    pub fn show(&self, initial_value: &mut Option<bool>, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if let Some(mut initial) = initial_value {
                ui.label(self.name);
                let resp = Checkbox::without_text(&mut initial).ui(ui);
                *initial_value = Some(initial);
                resp.context_menu(|ui| {
                    if ui.button("Clear").clicked() {
                        *initial_value = None;
                    }
                });
                resp
            } else {
                let mut out = self.default_value;
                ui.label(self.name);
                let resp = Checkbox::without_text(&mut out).indeterminate(true).ui(ui);
                if resp.changed() {
                    *initial_value = Some(out);
                }
                resp
            }
        })
        .inner
    }
}
