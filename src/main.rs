#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{run_native, NativeOptions, App};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
        ),
    ).expect("TODO: panic message");
}

#[derive(Default)]
struct AndroidAppInstallerApplication {
    //egui_ctx: egui::Context,
    apk_path: String,
    devices: Vec<String>,
    device_message: String,
    apk_message: String,
    //install_message: Arc<Mutex<String>>,
    install_progress: Arc<Mutex<Vec<InstallProgress>>>,
    show_settings: bool, // Flag to control the visibility of the settings window
}

struct InstallProgress {
    device_id: String,
    status: String,
    loading_indicator: String,
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
            self.apk_message = format!("Selected APK: {}", self.apk_path);
        }
    }

    // Method to display the settings sub-screen
    fn show_settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .open(&mut self.show_settings) // This will allow the window to be closed
            .show(ctx, |ui| {
                let version = env!("CARGO_PKG_VERSION");
                let authors = env!("CARGO_PKG_AUTHORS"); // Note: `CARGO_PKG_AUTHORS` is a colon-separated list

                ui.label(format!("Version: {}", version));
                ui.label(format!("Author(s): {}", authors.replace(':', ", ")));
            });
    }

    fn update_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let img = egui::include_image!("../res/icon/setting-icon.png");
                if ui
                    .add_sized(
                        [20.0, 20.0],
                        egui::ImageButton::new(img.clone()).frame(false),
                    )
                    .clicked()
                {
                    self.show_settings = true;
                }
            });


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


            // Conditionally show the settings window
            if self.show_settings {
                self.show_settings_window(ctx);
            }

            ui.add(
                egui::Image::new(egui::include_image!("../res/icon/tablet-icon.png"))
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

            ui.label(&self.device_message);

            ui.separator();

            ui.add(
                egui::Image::new(egui::include_image!("../res/icon/apk-icon.png"))
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

            ui.label(&self.apk_message);
            ui.separator();

            if ui.button("Install APK").clicked() {
                self.install_apk();
            }

            // 현재 설치 메시지를 표시
            let progress = self.install_progress.lock().unwrap();
            for p in progress.iter() {
                ui.label(format!("{}: {}{}", p.device_id, p.status, p.loading_indicator));
            }
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
            self.device_message = String::from("No devices detected.");
        } else {
            self.device_message = format!("{} device(s) detected.", self.devices.len());
        }
    }

    fn install_apk(&mut self) {
        if self.apk_path.is_empty() {
            self.apk_message = String::from("APK path is empty");
            eprintln!("APK path is empty");
            return;
        }

        // `self.devices`와 `self.apk_path`를 스레드 내부에서 사용하기 위해 복사합니다.
        let devices = self.devices.clone();
        let apk_path = self.apk_path.clone();
        // let install_message = Arc::clone(&self.install_message);
        let progress = Arc::clone(&self.install_progress);

        // Clear previous progress
        *progress.lock().unwrap() = vec![];

        thread::spawn(move || {
            for device_id in devices {
                let mut loading = ".".to_string();
                {
                    for _ in 0..3 { // Simulate updating loading indicator
                        {
                            let mut p = progress.lock().unwrap();
                            // Remove old status if exists
                            if let Some(index) = p.iter().position(|x| x.device_id == device_id) {
                                p.remove(index);
                            }
                            // Add new progress status
                            p.push(InstallProgress {
                                device_id: device_id.clone(),
                                status: "Installing".to_string(),
                                loading_indicator: loading.clone(),
                            });
                        }
                        println!("Installing APK on device: {}", device_id);
                        thread::sleep(Duration::from_secs(1)); // Simulate installation time
                        loading.push('.'); // Update loading indicator
                    }
                }
                println!("Installing APK on device: {}", device_id);

                // `apk_path`는 이제 스레드 내에서 직접적으로 접근 가능한 클론된 데이터입니다.
                let _install_output = Command::new("adb")
                    .args(["-s", &device_id, "install", &apk_path])
                    .output()
                    .expect("Failed to execute install command");

                {
                    let mut p = progress.lock().unwrap();
                    if let Some(index) = p.iter().position(|x| x.device_id == device_id) {
                        p[index].status = "Installed Success".to_string();
                        p[index].loading_indicator = "".to_string();
                    }
                }
                println!("APK installed on device: {}", device_id);
            }
        });
    }
}

impl App for AndroidAppInstallerApplication {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_ui(ctx);
    }
}
