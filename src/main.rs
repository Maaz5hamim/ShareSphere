#![allow(warnings)]
use eframe::{run_native,egui};
use UI::Gui;
pub mod UI;
use request::{RequestType,Response};
pub mod request;




fn main() 
{
    let options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        "ShareSphere",
        options,
        Box::new(|_cc| Box::<Gui>::default()),
    );
}