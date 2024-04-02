// use eframe::{egui, epi};
// use std::process::Command;
// // Use the `image` crate for image processing
// use image::{load_from_memory, ColorType};
//
//
// struct AndroidAppInstallerApplication {
//     apk_path: String,
//     devices: Vec<String>,
//     // To store the device IDs
//     message: String,
//     // To display messages or errors
//     apk_files: Vec<String>, // Store the names of APK files
//     //apk_icon: Option<egui::TextureId>, // Icon for APK files
// }
//
// impl Default for AndroidAppInstallerApplication {
//     fn default() -> Self {
//         Self {
//             apk_path: String::new(),
//             devices: Vec::new(),
//             message: String::from("No devices detected."),
//             apk_files: Vec::new(), // Initialize the vector
//             //apk_icon: None, // Initialize the icon to None
//         }
//     }
// }
//
// impl epi::App for AndroidAppInstallerApplication {
//     fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//
//             ui.heading("Android APK Installer");
//
//             // Section for refreshing lists
//             ui.horizontal(|ui| {
//                 if ui.button("Refresh Devices").clicked() {
//                     self.refresh_devices();
//                 }
//                 if ui.button("Refresh APK Files").clicked() {
//                     self.refresh_apk_files();
//                 }
//             });
//
//
//             // Separate the UI sections with a visual separator
//             ui.separator();
//             // ui.add_space(40.0); // Add some space for visual separation
//
//             // Display apk icon
//             //ui.image(self.apk_icon.expect("APK icon not found // asdf"), egui::vec2(20.0, 20.0));
//
//             ui.horizontal(|ui| {
//                 ui.heading("Connected Devices");
//             });
//             ui.add_space(10.0); // Add some space for visual separation
//
//             // Device selection section
//             if !self.devices.is_empty() {
//                 egui::ComboBox::from_label("Select Device")
//                     .selected_text(self.devices.first().unwrap_or(&String::from("No Devices")))
//                     .show_ui(ui, |ui| {
//                         for device in &self.devices {
//                             ui.selectable_value(&mut self.apk_path, device.clone(), device);
//                         }
//                     });
//             } else {
//                 ui.label("No connected devices found.");
//             }
//
//             // Another separator for visual separation between sections
//             ui.separator();
//             ui.heading("APK Files");
//             ui.add_space(10.0); // Add some space for visual separation
//
//             // APK file selection section
//             if !self.apk_files.is_empty() {
//                 egui::ComboBox::from_label("Select APK File")
//                     .selected_text(self.apk_files.first().unwrap_or(&String::from("No APK Files")))
//                     .show_ui(ui, |ui| {
//                         for apk_file in &self.apk_files {
//                             //ui.selectable_value(&mut self.apk_path, apk_file.clone(), apk_file);
//                             if ui.selectable_value(&mut self.apk_path, apk_file.clone(), apk_file).clicked() {
//                                 self.apk_path = apk_file.clone(); // APK 파일 선택 시 apk_path 업데이트
//                             }
//                         }
//                     });
//             } else {
//                 ui.label("No APK files found.");
//             }
//
//
//             // // Display connected devices section with icon
//             // if let Some(icon_id) = self.device_icon {
//             //     ui.horizontal(|ui| {
//             //         ui.image(icon_id, egui::vec2(20.0, 20.0));
//             //         ui.label("Connected Devices");
//             //     });
//             // }
//             //
//             // // Display APK files section with icon
//             // if let Some(icon_id) = self.apk_icon {
//             //     ui.horizontal(|ui| {
//             //         ui.image(icon_id, egui::vec2(20.0, 20.0));
//             //         ui.label("APK Files");
//             //     });
//             // }
//
//             ui.separator();
//             if ui.button("Install APK").clicked() {
//                 self.install_apk();
//             }
//         });
//     }
//
//     fn name(&self) -> &str {
//         "APK Installer"
//     }
// }
//
// impl AndroidAppInstallerApplication {
//     fn refresh_apk_files(&mut self) {
//         let mut apk_files = Vec::new();
//
//         if let Ok(entries) = std::fs::read_dir(".") { // Read the current directory
//             for entry in entries.filter_map(|e| e.ok()) {
//                 let path = entry.path();
//                 if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("apk") {
//                     if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
//                         apk_files.push(file_name.to_owned());
//                     }
//                 }
//             }
//         }
//
//         self.apk_files = apk_files;
//         if self.apk_files.is_empty() {
//             self.message = String::from("No APK files found.");
//         } else {
//             self.message = format!("{} APK file(s) found.", self.apk_files.len());
//         }
//     }
//
//     fn refresh_devices(&mut self) {
//         let output = Command::new("adb")
//             .arg("devices")
//             .output()
//             .expect("Failed to execute adb command");
//
//         let output_str = String::from_utf8_lossy(&output.stdout);
//
//         self.devices.clear(); // Clear the current list
//         for line in output_str.lines().skip(1) {
//             if !line.is_empty() && line.contains("device") {
//                 if let Some(device_id) = line.split_whitespace().next() {
//                     self.devices.push(device_id.to_string());
//                 }
//             }
//         }
//
//         if self.devices.is_empty() {
//             self.message = String::from("No devices detected.");
//         } else {
//             self.message = format!("{} device(s) detected.", self.devices.len());
//         }
//     }
//
//     fn install_apk(&self) {
//         if self.apk_path.is_empty() {
//             eprintln!("APK path is empty");
//             return;
//         }
//
//         let devices_output = Command::new("adb")
//             .arg("devices")
//             .output()
//             .expect("Failed to execute adb command");
//
//         let devices_str = String::from_utf8_lossy(&devices_output.stdout);
//
//         for line in devices_str.lines().skip(1) {
//             if !line.is_empty() {
//                 let device_id = line.split_whitespace().next().unwrap();
//                 println!("Installing APK on device: {}", device_id);
//
//                 let _install_output = Command::new("adb")
//                     .args(["-s", device_id, "install", &self.apk_path])
//                     .output()
//                     .expect("Failed to execute install command");
//
//                 println!("APK installed on device: {}", device_id);
//             }
//         }
//     }
//
//
// }
//
// fn main() {
//     let options = eframe::NativeOptions::default();
//     eframe::run_native(Box::new(AndroidAppInstallerApplication::default()),
//                        options );
// }


use std::path::Path;
use egui::IconData;
use iced::{
    Application, button, Button, Column, Command, Container, Element, executor, Length, Settings, Text, Image
};

fn main() -> iced::Result {
    AndroidAppInstallerApplication::run(Settings::default())
}

struct AndroidAppInstallerApplication {
    refresh_devices_button_state: button::State,
    install_apk_button_state: button::State,
    apk_path: String,
    devices: Vec<String>,
    message: String,
    apk_files: Vec<String>,
    tablet_icon: iced::image::Handle, // Add this line
}

#[derive(Debug, Clone)]
enum Message {
    RefreshDevices,
    InstallApk,
    DevicesRefreshed(Vec<String>),
    ApkInstalled(String),
}

impl Application for AndroidAppInstallerApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (AndroidAppInstallerApplication, Command<Self::Message>) {
        //let tablet_icon = iced::image::Handle::from_path(Path::new("../apk-icon.png"));
        (
            AndroidAppInstallerApplication {
                refresh_devices_button_state: button::State::new(),
                install_apk_button_state: button::State::new(),
                apk_path: String::new(),
                devices: Vec::new(),
                message: String::from("Ready"),
                apk_files: Vec::new(),
                tablet_icon:iced::image::Handle::from_path(Path::new("../res/icon/apk-icon.png")), // Add this line
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("APK Installer - Iced Example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::RefreshDevices => {
                // Example: You would replace this with your actual logic to refresh devices
                let devices = vec![String::from("Device 1"), String::from("Device 2")];
                Command::perform(async { devices }, Message::DevicesRefreshed)
            }
            Message::InstallApk => {
                // Example: You would replace this with your actual logic to install APKs
                let result = String::from("APK Installed Successfully!");
                Command::perform(async { result }, Message::ApkInstalled)
            }
            Message::DevicesRefreshed(devices) => {
                self.devices = devices;
                self.message = String::from("Devices Refreshed Successfully!");
                Command::none()
            }
            Message::ApkInstalled(result) => {
                self.message = result;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Image::new(self.tablet_icon.clone()))
            // show me navigate icon
            .push(Text::new("APK Installer").size(30))
            .push(Text::new("Select a device to install APK"))
            .push(Text::new("Select an APK file to install"))
            .push(Text::new("Message:").size(20))
            .push(Text::new(&self.message))
            .push(
                Button::new(&mut self.refresh_devices_button_state, Text::new("Refresh Devices"))
                    .on_press(Message::RefreshDevices),
            )

            .push(
                Button::new(&mut self.install_apk_button_state, Text::new("Install APK"))
                    .on_press(Message::InstallApk),
            )
            .push(Text::new(&self.message));

            // Display the icon with a width of 100 units
            // .push(
            //     Button::new(&mut self.refresh_devices_button_state, Text::new("Refresh Devices"))
            //         .on_press(Message::RefreshDevices),
            // )
            // .push(
            //     Button::new(&mut self.install_apk_button_state, Text::new("Install APK"))
            //         .on_press(Message::InstallApk),
            // )
            // .push(Text::new(&self.message));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}