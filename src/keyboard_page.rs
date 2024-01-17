// Use libraries
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use adw::prelude::*;
use adw::*;
use glib::*;
use gdk::Display;
use gtk::subclass::layout_child;

use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

pub fn keyboard_page(content_stack: &gtk::Stack) {

    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();
    
    // Next and back button
    let bottom_back_button = gtk::Button::builder()
        .label("Back")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();
    let bottom_next_button = gtk::Button::builder()
        .label("Next")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::End)
        .hexpand(true)
        .sensitive(false)
        .build();
    
    // Start Applying css classes
    bottom_next_button.add_css_class("suggested-action");
    
    // / bottom_box appends
    //// Add the next and back buttons
    bottom_box.append(&bottom_back_button);
    bottom_box.append(&bottom_next_button);

   // the header box for the keyboard page
   let keyboard_main_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // the header box for the keyboard page
   let keyboard_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the keyboard page
   let keyboard_header_text = gtk::Label::builder()
       .label("Select a keyboard")
       .halign(gtk::Align::End)
       .hexpand(true)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(5)
       .build();
    keyboard_header_text.add_css_class("header_sized_text");

   // the header icon for the keyboard icon
   let keyboard_header_icon = gtk::Image::builder()
       .icon_name("keyboard")
       .halign(gtk::Align::Start)
       .hexpand(true)
       .pixel_size(78)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(0)
       .margin_end(15)
       .build();

   // make keyboard selection box for choosing installation or live media 
   let keyboard_selection_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

    // / keyboard_header_box appends
    //// Add the keyboard page header text and icon
    keyboard_header_box.append(&keyboard_header_text);
    keyboard_header_box.append(&keyboard_header_icon);
    
    // / keyboard_main_box appends
    //// Add the keyboard header to keyboard main box
    keyboard_main_box.append(&keyboard_header_box);
    //// Add the keyboard selection/page content box to keyboard main box
    keyboard_main_box.append(&keyboard_selection_box);

    // text above keyboard selection box
    let keyboard_selection_text = gtk::Label::builder()
        .label("Please select a Keyboard layout for the system to use")
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    keyboard_selection_text.add_css_class("medium_sized_text");

    let keyboard_selection_expander_row = adw::ExpanderRow::builder()
        .title("No Keyboard Layout selected")
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .label("No Keyboard Layout selected")
        .build();

    let keyboard_selection_expander_row_viewport = gtk::ScrolledWindow::builder()
        .height_request(200)
        .build();

    let keyboard_selection_expander_row_viewport_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();

    let keyboard_selection_expander_row_viewport_listbox = gtk::ListBox::builder()
            .selection_mode(SelectionMode::None)
            .margin_top(15)
            .margin_bottom(15)
            .margin_start(15)
            .margin_end(15)
            .build();
    keyboard_selection_expander_row_viewport_listbox.add_css_class("boxed-list");
    keyboard_selection_expander_row_viewport_listbox.append(&keyboard_selection_expander_row);

    keyboard_selection_expander_row_viewport.set_child(Some(&keyboard_selection_expander_row_viewport_box));

    keyboard_selection_expander_row.add_row(&keyboard_selection_expander_row_viewport);

    keyboard_selection_expander_row_viewport_box.append(&null_checkbutton);

    let null_checkbutton_clone = null_checkbutton.clone();
    let keyboard_selection_expander_row_clone2 = keyboard_selection_expander_row.clone();
    let bottom_next_button_clone = bottom_next_button.clone();


    null_checkbutton.connect_toggled(move |_| {
        if null_checkbutton_clone.is_active() == true {
            keyboard_selection_expander_row_clone2.set_title("No Keyboard Layout selected");
            bottom_next_button_clone.set_sensitive(false);
        }
    });

    let mut keyboard_layout_cli = Command::new("localectl")
        .arg("list-x11-keymap-layouts")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed {}", e));

    let keyboard_layout_stdout = keyboard_layout_cli.stdout.as_mut().expect("could not get stdout");
    let keyboard_layout_reader = BufReader::new(keyboard_layout_stdout);

    for keyboard_layout in keyboard_layout_reader.lines() {
        let keyboard_layout = keyboard_layout.unwrap();
        let keyboard_layout_checkbutton = gtk::CheckButton::builder()
            .label(keyboard_layout.clone())
            .build();
        keyboard_layout_checkbutton.set_group(Some(&null_checkbutton));
        keyboard_selection_expander_row_viewport_box.append(&keyboard_layout_checkbutton); 
        let keyboard_selection_expander_row_clone = keyboard_selection_expander_row.clone();
        let keyboard_layout_checkbutton_clone = keyboard_layout_checkbutton.clone();
        let bottom_next_button_clone2 = bottom_next_button.clone();
        keyboard_layout_checkbutton.connect_toggled(move |_| {
            if keyboard_layout_checkbutton_clone.is_active() == true {
                keyboard_selection_expander_row_clone.set_title(&keyboard_layout);
                bottom_next_button_clone2.set_sensitive(true);
            }
        });
    }

    // / keyboard_selection_box appends
    //// add text and and entry to keyboard page selections
    keyboard_selection_box.append(&keyboard_selection_text);
    keyboard_selection_box.append(&keyboard_selection_expander_row_viewport_listbox);
    
    // / keyboard_header_box appends
    //// Add the keyboard page header text and icon
    keyboard_header_box.append(&keyboard_header_text);
    keyboard_header_box.append(&keyboard_header_icon);
    
    // / keyboard_main_box appends
    //// Add the keyboard header to keyboard main box
    keyboard_main_box.append(&keyboard_header_box);
    //// Add the keyboard selection/page content box to keyboard main box
    keyboard_main_box.append(&keyboard_selection_box);

    keyboard_main_box.append(&bottom_box);
    
    // / Content stack appends
    //// Add the keyboard_main_box as page: keyboard_page, Give it nice title
    content_stack.add_titled(&keyboard_main_box, Some("keyboard_page"), "Keyboard");

    let content_stack_clone = content_stack.clone();
    let content_stack_clone2 = content_stack.clone();
    bottom_next_button.connect_clicked(move |_| {
        content_stack_clone.set_visible_child_name("partitioning_page")
    });
    bottom_back_button.connect_clicked(move |_| {
        content_stack_clone2.set_visible_child_name("language_page")
    });

}
