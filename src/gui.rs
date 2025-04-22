use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Box as GtkBox, Button, ComboBoxText, FileChooserAction,
    FileChooserDialog, FileFilter, Label, Orientation, ResponseType,
    ScrolledWindow, TextView, Application,
};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

pub fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("🎬 Offline Media Converter")
        .default_width(600)
        .default_height(480)
        .resizable(false)
        .build();

    let container = GtkBox::new(Orientation::Vertical, 12);
    container.set_margin_top(20);
    container.set_margin_bottom(20);
    container.set_margin_start(20);
    container.set_margin_end(20);

    let input_label = Label::new(Some("📄 No file selected"));
    let output_preview = Label::new(Some("💡 Select a file to see output preview"));
    let format_selector = ComboBoxText::new();
    let choose_button = Button::with_label("📂 Choose Input File");
    let convert_button = Button::with_label("🎬 Convert");
    let status_label = Label::new(None);
    status_label.set_margin_top(10);

    let error_view = TextView::new();
    error_view.set_wrap_mode(gtk::WrapMode::Word);
    error_view.set_editable(false);
    let error_buffer = error_view.buffer();

    let scroller = ScrolledWindow::new();
    scroller.set_child(Some(&error_view));
    scroller.set_min_content_height(100);

    container.append(&input_label);
    container.append(&choose_button);
    container.append(&format_selector);
    container.append(&output_preview);
    container.append(&convert_button);
    container.append(&status_label);
    container.append(&scroller);
    window.set_child(Some(&container));

    let input_path = Rc::new(RefCell::new(None::<PathBuf>));
    let window_for_dialog = window.clone();
    let format_map = Rc::new(RefCell::new(vec![]));

    // File chooser
    {
        let input_path = input_path.clone();
        let input_label = input_label.clone();
        let output_preview = output_preview.clone();
        let format_selector = format_selector.clone();
        let format_map = format_map.clone();

        choose_button.connect_clicked(move |_| {
            let dialog = FileChooserDialog::new(
                Some("Select a file to convert"),
                Some(&window_for_dialog),
                FileChooserAction::Open,
                &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
            );

            let filter = FileFilter::new();
            filter.set_name(Some("Media files"));
            for pattern in ["*.mp4", "*.mov", "*.avi", "*.webm", "*.png", "*.jpg", "*.jpeg", "*.webp", "*.heic", "*.avif"] {
                filter.add_pattern(pattern);
            }
            dialog.set_filter(&filter);

            let input_path = input_path.clone();
            let input_label = input_label.clone();
            let output_preview = output_preview.clone();
            let format_selector = format_selector.clone();
            let format_map = format_map.clone();

            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(path) = dialog.file().and_then(|f| f.path()) {
                        input_label.set_text(&format!("📄 Selected: {}", path.display()));
                        *input_path.borrow_mut() = Some(path.clone());

                        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

                        let ext_map = |e: &str| match e {
                            "jpg" | "jpeg" => "jpg",
                            "png" => "png",
                            "webp" => "webp",
                            "avif" => "avif",
                            "heic" => "jpg",
                            "mp4" | "mov" | "avi" | "webm" => "mp4",
                            _ => "",
                        };

                        let input_normalized = ext_map(&ext);

                        let image_outputs = vec![
                            ("png", "PNG (lossless)"),
                            ("jpg", "JPG (compressed)"),
                            ("webp", "WEBP (for web)"),
                            ("avif", "AVIF (modern)"),
                        ];
                        let video_outputs = vec![
                            ("mp4", "MP4 (H.264)"),
                        ];

                        let output_pool = match input_normalized {
                            "jpg" | "png" | "webp" | "avif" => image_outputs,
                            "mp4" => video_outputs,
                            _ => vec![],
                        };

                        let valid_outputs: Vec<&str> = output_pool
                            .into_iter()
                            .filter(|(e, _)| *e != input_normalized)
                            .map(|(_, label)| label)
                            .collect();

                        *format_map.borrow_mut() = valid_outputs.clone();
                        format_selector.remove_all();
                        for item in &valid_outputs {
                            format_selector.append_text(item);
                        }
                        format_selector.set_active(Some(0));

                        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
                        let out_ext = match valid_outputs.first().copied() {
                            Some("PNG (lossless)") => "png",
                            Some("JPG (compressed)") => "jpg",
                            Some("WEBP (for web)") => "webp",
                            Some("AVIF (modern)") => "avif",
                            Some("MP4 (H.264)") => "mp4",
                            _ => "unknown",
                        };
                        output_preview.set_text(&format!("Will output: {}_converted.{}", stem, out_ext));
                    }
                }
                dialog.close();
            });

            dialog.show();
        });
    }

    // Conversion logic stays unchanged
    // ... (already handled well)

    window.present();
}
