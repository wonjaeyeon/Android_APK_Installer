#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::{ run_native, NativeOptions, App};
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
    device_message: String,
    apk_message: String,
    install_message: Arc<Mutex<String>>,
    install_progress: Arc<Mutex<Vec<InstallProgress>>>,
    //install_message: String,
    //apk_files: Vec<String>,
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

            ui.label(&self.device_message);

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


            ui.label(&self.apk_message);
            ui.separator();

            if ui.button("Install APK").clicked() {
                self.install_apk();
            }

            // 현재 설치 메시지를 표시
            // let message = self.install_message.lock().unwrap();
            // ui.label(message.as_str());
            let progress = self.install_progress.lock().unwrap();
            for p in progress.iter() {
                ui.label(format!("{}: {}{}", p.device_id, p.status, p.loading_indicator));
            }


            //ui.label(&self.install_message);

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

    //     let install_message = Arc::clone(&self.install_message);
    //
    //     thread::spawn(move || {
    //     for device_id in &self.devices {
    //         //self.install_message += &format!("{} : Installing\n", device_id);
    //         {
    //             let mut msg = install_message.lock().unwrap();
    //             *msg = format!("{} : Installing\n", device_id);
    //         } // MutexGuard가 범위를 벗어나면서 Mutex가 자동으로 해제됩니다.
    //         println!("Installing APK on device: {}", device_id);
    //
    //         let _install_output = Command::new("adb")
    //             .args(["-s", device_id, "install", &self.apk_path])
    //             .output()
    //             .expect("Failed to execute install command");
    //
    //         // Update the message to indicate success
    //         // self.install_message = self.install_message.trim_end_matches('\n').to_string(); // Remove the trailing newline
    //         // self.install_message = self.install_message.trim_end_matches("Installing").to_string(); // Remove the "Installing" part
    //         // self.install_message += "Installed Success\n"; // Add "Installed Success" message
    //         {
    //             let mut msg = install_message.lock().unwrap();
    //             *msg = format!("{} : Installed Success\n", device_id);
    //         }
    //         println!("APK installed on device: {}", device_id);
    //     }
    //
    //     // if !self.install_message.is_empty() {
    //     //     // Remove the last newline character for the final message
    //     //     self.install_message.pop();
    //     // }
    //     });
    // }
        // `self.devices`와 `self.apk_path`를 스레드 내부에서 사용하기 위해 복사합니다.
        let devices = self.devices.clone();
        let apk_path = self.apk_path.clone();
        let install_message = Arc::clone(&self.install_message);
        let progress = Arc::clone(&self.install_progress);

        // Clear previous progress
        *progress.lock().unwrap() = vec![];

        thread::spawn(move || {
            for device_id in devices {
                let mut loading = ".".to_string();
                {
                    // let mut msg = install_message.lock().unwrap();
                    // *msg = format!("{} : Installing\n", device_id);
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
                    // let mut msg = install_message.lock().unwrap();
                    // *msg = format!("{} : Installed Success\n", device_id);
                    let mut p = progress.lock().unwrap();
                    if let Some(index) = p.iter().position(|x| x.device_id == device_id) {
                        p[index].status = "Installed Success".to_string();
                        p[index].loading_indicator = "".to_string();
                    }
                }
                println!("APK installed on device: {}", device_id);
            }
        });
}}

impl App for AndroidAppInstallerApplication {


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.update_ui(ctx);
    }
}
