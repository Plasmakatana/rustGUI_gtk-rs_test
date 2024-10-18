use gtk4::prelude::*;
use gtk4::{glib, Application,Box,Button,Label,Entry,Image};
use gtk4::ApplicationWindow;
use crate::glib::clone;
use std::fs;
use std::path::PathBuf;
const APP_ID: &str = "org.gtk_rs.HelloWorld";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hellew")
        .build();
    let vbox = Box::new(gtk4::Orientation::Vertical, 5);
    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("Enter image URL"));

    // Create an Image widget
    let image = Image::new();
    image.set_pixel_size(300);
    let load_image_button = Button::with_label("Load Image");
    
    // Connect the load image button's clicked signal
    load_image_button.connect_clicked(clone!(@weak url_entry, @weak image => move |_| {
        let url = url_entry.text().to_string();
        let image_path = download_image(&url);
        image.set_from_pixbuf(None);
        if let Some(path) = Some(image_path.as_deref()) {
            image.set_from_file(path); // Set the image from the downloaded file
        }
    }));
    // Add the button and label to the vertical box
    vbox.append(&url_entry);
    vbox.append(&load_image_button);
    vbox.append(&image);

    // Add the box to the window
    window.set_child(Some(&vbox));
    // Present window
    window.present();
}
fn download_image(url: &str) -> Option<String> {
    let response = reqwest::blocking::get(url).ok()?;
    let bytes = response.bytes().ok()?;
    
    // Create a temporary file
    let path = PathBuf::from("downloaded_image.png");
    fs::write(&path, &bytes).ok()?;
    
    Some(path.to_string_lossy().to_string())
}