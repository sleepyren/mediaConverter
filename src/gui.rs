use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Box as GtkBox, Button, ComboBoxText, FileChooserAction,
    FileChooserDialog, FileFilter, Label, Orientation, ResponseType,
    ScrolledWindow, TextView, Application,
};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::process::Command;

pub fn build_ui(app: &gtk::Application) {
    let window = create_main_window(app);
    let (container, widgets, error_buffer) = create_ui_widgets();

    setup_file_chooser(
        &window,
        &widgets.input_label,
        &widgets.output_preview,
        &widgets.format_selector,
        &widgets.choose_button,
        widgets.input_path.clone(),
        widgets.format_map.clone(),
    );

    setup_format_selector(
        &widgets.output_preview,
        &widgets.input_path,
        &widgets.format_selector,
        &widgets.format_map,
    );

    setup_convert_button(
        &widgets.convert_button,
        &widgets.input_path,
        &widgets.format_selector,
        &widgets.format_map,
        &widgets.output_preview,
        &error_buffer,
    );

    window.set_child(Some(&container));
    window.present();
}

struct UIWidgets {
    input_label: Label,
    output_preview: Label,
    format_selector: ComboBoxText,
    choose_button: Button,
    convert_button: Button,
    input_path: Rc<RefCell<Option<PathBuf>>>,
    format_map: Rc<RefCell<Vec<(&'static str, &'static str)>>>,
}

fn create_main_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("🎬 Offline Media Converter")
        .default_width(600)
        .default_height(480)
        .resizable(false)
        .build()
}

fn create_ui_widgets() -> (GtkBox, UIWidgets, gtk::TextBuffer) {
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
    let error_buffer = error_view.buffer().clone();

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

    let input_path = Rc::new(RefCell::new(None::<PathBuf>));
    let format_map = Rc::new(RefCell::new(vec![]));

    let widgets = UIWidgets {
        input_label,
        output_preview,
        format_selector,
        choose_button,
        convert_button,
        input_path,
        format_map,
    };

    (container, widgets, error_buffer)
}

fn setup_file_chooser(
    window: &ApplicationWindow,
    input_label: &Label,
    output_preview: &Label,
    format_selector: &ComboBoxText,
    choose_button: &Button,
    input_path: Rc<RefCell<Option<PathBuf>>>,
    format_map: Rc<RefCell<Vec<(&'static str, &'static str)>>>,
) {
    let window_clone = window.clone();
    let input_label = input_label.clone();
    let output_preview = output_preview.clone();
    let format_selector = format_selector.clone();
    let choose_button = choose_button.clone();

    choose_button.connect_clicked(move |_| {
        let dialog = FileChooserDialog::new(
            Some("Select a file to convert"),
            Some(&window_clone),
            FileChooserAction::Open,
            &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
        );

        let filter = FileFilter::new();
        filter.set_name(Some("Media files"));
        for pattern in [
            "*.mp4", "*.mov", "*.avi", "*.webm", "*.png", "*.jpg", "*.jpeg",
            "*.webp", "*.heic", "*.avif"
        ] {
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

                    let output_pool: Vec<(&str, &str)> = match input_normalized {
                        "jpg" | "png" | "webp" | "avif" => image_outputs,
                        "mp4" => video_outputs,
                        _ => vec![],
                    }
                    .into_iter()
                    .filter(|(e, _)| *e != input_normalized)
                    .collect();

                    *format_map.borrow_mut() = output_pool.clone();
                    format_selector.remove_all();
                    for (_, label) in &output_pool {
                        format_selector.append_text(label);
                    }
                    format_selector.set_active(Some(0));
                }
            }
            dialog.close();
        });

        dialog.show();
    });
}

fn setup_format_selector(
    output_preview: &Label,
    input_path: &Rc<RefCell<Option<PathBuf>>>,
    format_selector: &ComboBoxText,
    format_map: &Rc<RefCell<Vec<(&'static str, &'static str)>>>,
) {
    let output_preview = output_preview.clone();
    let input_path = input_path.clone();
    let format_selector = format_selector.clone();
    let format_map = format_map.clone();

    format_selector.connect_changed(move |selector| {
        if let Some(path) = input_path.borrow().as_ref() {
            let active_index = selector.active().unwrap_or(0) as usize;
            let selected_format = format_map.borrow().get(active_index).map(|(ext, _)| *ext).unwrap_or("unknown");
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
            output_preview.set_text(&format!("Will output: {}_converted.{}", stem, selected_format));
        }
    });
}

fn setup_convert_button(
    convert_button: &Button,
    input_path: &Rc<RefCell<Option<PathBuf>>>,
    format_selector: &ComboBoxText,
    format_map: &Rc<RefCell<Vec<(&'static str, &'static str)>>>,
    output_preview: &Label,
    error_buffer: &gtk::TextBuffer,
) {
    let input_path = input_path.clone();
    let format_map = format_map.clone();
    let format_selector = format_selector.clone();
    let output_preview = output_preview.clone();
    let error_buffer = error_buffer.clone();

    convert_button.connect_clicked(move |_| {
        let Some(path) = input_path.borrow().clone() else { return };
        let active_index = format_selector.active().unwrap_or(0) as usize;
        let selected_format = format_map.borrow().get(active_index).map(|(ext, _)| *ext).unwrap_or("unknown");

        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
        let output_file = path.with_file_name(format!("{}_converted.{}", stem, selected_format));

        output_preview.set_text(&format!("Will output: {}", output_file.display()));

        let output = Command::new("ffmpeg")
            .args(["-y", "-i", path.to_str().unwrap(), output_file.to_str().unwrap()])
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    error_buffer.set_text(&format!("✅ Conversion succeeded: {}", output_file.display()));
                } else {
                    let err = String::from_utf8_lossy(&output.stderr);
                    error_buffer.set_text(&format!("❌ ffmpeg failed:\n{}", err));
                }
            }
            Err(err) => {
                error_buffer.set_text(&format!("❌ Failed to run ffmpeg: {}", err));
            }
        }
    });
}