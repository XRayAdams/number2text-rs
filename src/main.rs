use gtk4::prelude::*;
use libadwaita as adw;
use gtk4::{Application, ApplicationWindow, Label, Entry, ComboBoxText, Box as GtkBox, TextView, 
           MenuButton, gio, HeaderBar};
use std::cell::RefCell;
use std::rc::Rc;

mod providers;
mod view_model;
mod settings;
use view_model::AppViewModel;

const SPACING_MEDIUM: i32 = 12;
const SPACING_LARGE: i32 = 18;

fn main() {
    adw::init().expect("Failed to initialize Libadwaita");
    let app = Application::builder()
        .application_id("com.example.number2text")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {

    let view_model = Rc::new(RefCell::new(AppViewModel::new()));

    let window = ApplicationWindow::builder()
        .title("Number 2 Text")
        .default_width(720)
        .default_height(600)
        .application(app)
        .build();

    let header_bar = HeaderBar::new();
    
    let menu = gio::Menu::new();
    menu.append(Some("About"), Some("app.about"));
    
    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu)
        .build();
    
    header_bar.pack_end(&menu_button);
    
    window.set_titlebar(Some(&header_bar));
    
    let about_action = gio::SimpleAction::new("about", None);
    let window_clone = window.clone();
    about_action.connect_activate(move |_, _| {
        let about = adw::AboutWindow::builder()
            .application_name("Number 2 Text")
            .version(AppViewModel::get_app_version())
            .developers(vec!["Konstantin Adamov".to_string()])
            .website("https://github.com/xrayadams/number2text-rs")
            .license_type(gtk4::License::MitX11)
            .transient_for(&window_clone)
            .modal(true)
            .build();
        about.present();
    });
    app.add_action(&about_action);

    // Create a vertical box container for main layout with padding
    let vbox = GtkBox::new(gtk4::Orientation::Vertical, SPACING_LARGE);
    vbox.set_margin_start(SPACING_MEDIUM);
    vbox.set_margin_end(SPACING_MEDIUM);
    vbox.set_margin_top(SPACING_MEDIUM);
    vbox.set_margin_bottom(SPACING_MEDIUM);
    
    
    // Create a horizontal box for the top row (entry, label, combobox)
    let hbox = GtkBox::new(gtk4::Orientation::Horizontal, SPACING_MEDIUM);
    
    // Create an entry
    let entry = Entry::new();
    entry.set_placeholder_text(Some("Enter number"));
    entry.set_width_request(200);
    
    let label = Label::with_mnemonic("_Select language");
    
    let combo_box = ComboBoxText::new();
    label.set_mnemonic_widget(Some(&combo_box));
    let language_names = view_model.borrow().get_language_names();
    for language_name in &language_names {
        combo_box.append_text(language_name.as_str());
    }

    // Load saved index and clamp to available range
    let saved_index = settings::load_selected_index().unwrap_or(0);
    let clamped_index = saved_index.min(language_names.len().saturating_sub(1));
    view_model.borrow_mut().set_language(clamped_index);
    combo_box.set_active(Some(clamped_index as u32));

    // Add widgets to the horizontal box in the specified order
    hbox.append(&entry);
    hbox.append(&label);
    hbox.append(&combo_box);

    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_vexpand(true);
    text_view.set_hexpand(true);
    text_view.set_tooltip_text(Some("Output"));
    text_view.set_wrap_mode(gtk4::WrapMode::Word);
    text_view.set_left_margin(SPACING_MEDIUM);
    text_view.set_right_margin(SPACING_MEDIUM);
    text_view.set_top_margin(SPACING_MEDIUM);
    text_view.set_bottom_margin(SPACING_MEDIUM);
    text_view.add_css_class("card");

    let vm_for_combo = view_model.clone();
    let entry_for_combo = entry.clone();
    let text_view_for_combo = text_view.clone();
    combo_box.connect_changed(move |combo| {
        if let Some(active) = combo.active() {
            vm_for_combo.borrow_mut().set_language(active as usize);
            let _ = settings::save_selected_index(active as usize);

            if let Some(number) = entry_for_combo.text().as_str().parse::<i64>().ok() {
                let result = vm_for_combo.borrow().convert_number(number);
                let buffer = text_view_for_combo.buffer();
                buffer.set_text(&result);
            }
        }
    });
    
    // Connect entry changed signal to run conversion
    let vm_for_entry = view_model.clone();
    let text_view_for_entry = text_view.clone();
    entry.connect_changed(move |entry| {
        let input_text = entry.text();
        let buffer = text_view_for_entry.buffer();
        
        if input_text.is_empty() {
            buffer.set_text("");
            return;
        }
        
        match input_text.as_str().parse::<i64>() {
            Ok(number) => {
                let result = vm_for_entry.borrow().convert_number(number);
                buffer.set_text(&result);
            }
            Err(_) => {
                buffer.set_text("Invalid number");
            }
        }
    });
    
    vbox.append(&hbox);
    vbox.append(&text_view);
    window.set_child(Some(&vbox));
    
    window.show();
}
