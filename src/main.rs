#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::{ run_native, NativeOptions, App};
//use eframe::egui; //  already imported by eframe
use std::process::Command;

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    run_native(
        "APK Installer",
        options,
        Box::new(|cc|
                     {
                         // This gives us image support:
                         egui_extras::install_image_loaders(&cc.egui_ctx);
                         Box::<AndroidAppInstallerApplication>::default()
                     }

            //Box::new(AndroidAppInstallerApplication::new(cc.egui_ctx.clone()))
        ),
    ).expect("TODO: panic message");
}
#[derive(Default)]
struct AndroidAppInstallerApplication {
    //egui_ctx: egui::Context,
    apk_path: String,
    devices: Vec<String>,
    message: String,
    //apk_files: Vec<String>,
}

impl AndroidAppInstallerApplication {
    // fn new(egui_ctx: egui::Context) -> Self {
    //     Self {
    //         egui_ctx,
    //         apk_path: String::new(),
    //         devices: Vec::new(),
    //         message: String::from("No devices detected."),
    //         apk_files: Vec::new(),
    //     }
    // }

    fn find_apk_files(&mut self) {
        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter("APK files", &["apk"])
            .pick_file() {
            self.apk_path = file_path.as_path().to_string_lossy().into_owned();
            self.message = format!("Selected APK: {}", self.apk_path);
        }
    }

    fn update_ui(&mut self, ctx: &egui::Context) {

        egui::CentralPanel::default().show(ctx, |ui| {


            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../res/icon/android-logo.png"))
                            .max_width(50.0)
                            .max_height(50.0)
                            .rounding(10.0),
                    );
                    ui.heading("Android APK Installer");
                });
            });


            ui.add_space(10.0);

            ui.add(
                egui::Image::new(egui::include_image!("../res/icon/tablet-icon-7.png"))
                    .max_width(40.0)
                    .max_height(40.0)
                    .rounding(5.0),
            );

            if ui.button("Refresh Devices").clicked() {
                self.refresh_devices();
            }

            ui.horizontal(|ui| {
                ui.label("Devices:");
                for device in &self.devices {
                    ui.label(device);
                }
            });

            ui.separator();

            ui.add(
                egui::Image::new(egui::include_image!("../res/icon/apk-icon-3.png"))
                    .max_width(40.0)
                    .max_height(40.0)
                    .rounding(5.0),
            );

            if ui.button("Find APK Files").clicked() {
                self.find_apk_files();
            }

            ui.horizontal(|ui| {
                ui.label("APK Path:");
                ui.label(&self.apk_path);
            });

            ui.separator();


            ui.label(&self.message);

            if ui.button("Install APK").clicked() {
                self.install_apk();
            }


            //ui.image(egui::include_image!("../res/icon/android-icon.png"));

        });
    }

    fn refresh_devices(&mut self) {
        let output = Command::new("adb")
            .arg("devices")
            .output()
            .expect("Failed to execute adb command");

        let output_str = String::from_utf8_lossy(&output.stdout);

        self.devices.clear();
        for line in output_str.lines().skip(1) {
            if !line.is_empty() && line.contains("device") {
                if let Some(device_id) = line.split_whitespace().next() {
                    self.devices.push(device_id.to_string());
                }
            }
        }

        if self.devices.is_empty() {
            self.message = String::from("No devices detected.");
        } else {
            self.message = format!("{} device(s) detected.", self.devices.len());
        }
    }

    fn install_apk(&self) {
        if self.apk_path.is_empty() {
            eprintln!("APK path is empty");
            return;
        }

        for device_id in &self.devices {
            println!("Installing APK on device: {}", device_id);

            let _install_output = Command::new("adb")
                .args(["-s", device_id, "install", &self.apk_path])
                .output()
                .expect("Failed to execute install command");

            println!("APK installed on device: {}", device_id);
        }
    }
}

impl App for AndroidAppInstallerApplication {


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.update_ui(ctx);
    }
}
