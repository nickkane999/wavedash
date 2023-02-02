#![allow(warnings)]
mod components2;
use components2::{LabelData, InputData, FunnyData};

use druid::{Data, widget::{Button, Flex, Label, Padding, Align, TextBox, LensWrap}, TextAlignment, theme, FontWeight,
 AppLauncher, Env, WindowDesc, Widget, WidgetExt, Lens, FontDescriptor, FontFamily, kurbo::Insets, KeyOrValue};
use std::fs::File;
use std::io::prelude::*;
use std::any::Any;


// Create label format function with text and padding data type
fn label_format(text: &str, padding: KeyOrValue<druid::Insets>) -> impl Widget<()> {
    let label_container = Label::new("Sample data: ");
    let aligned_label = Align::left(label_container);    
    let sample_label = Padding::new((0.0, 0.0, 0.0, 60.0), aligned_label);
    return sample_label;
}

fn new_label2(label: LabelData) -> Padding<FunnyData, Align<FunnyData>> {
    println!("Creating new label");
    let font_label = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(24.0);
    let email_address_label = Padding::new((0.0, 20.0, 0.0, 10.0), 
    Align::left(Label::new(|data: &FunnyData, _: &Env| format!("Email Address: "))
            .with_font(font_label.clone())));
    /*
    let font_label = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(label.font_weight)
        .with_size(label.font_size);
    let applied_font_label = Label::new(format!("{}", label.text)).with_font(font_label.clone());
    let aligned_label = match label.alignment {
        Some(x) => Align::left(applied_font_label),
        None => Align::centered(applied_font_label),
    };
    let label = Padding::new(label.padding, aligned_label);
    return label;
    */
    return email_address_label;
}

pub fn new_label_replace(label: LabelData) -> Padding<FunnyData, Align<FunnyData>> {
    let font_label = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(label.font_weight)
        .with_size(label.font_size);
    let applied_font_label = Label::new(format!("{}", label.text)).with_font(font_label.clone());
    let aligned_label = match label.alignment {
        Some(x) =>  
        if x == "left" { Align::left(applied_font_label) }
        else { Align::centered(applied_font_label) },
        None => Align::centered(applied_font_label),
    };
    let label = Padding::new(label.padding, aligned_label);
    return label;
}

//fn ui_builder() -> impl Widget<FunnyData> {
fn ui_builder() -> impl Widget<FunnyData> {
    let email_label_data = LabelData {
        text: String::from("Email Address:"),
        font_size: 24.0,
        font_weight: FontWeight::BOLD,
        padding: (0.0, 20.0, 0.0, 10.0).into(),
        alignment: Some(String::from("left")),
    };
    let email_address_label = new_label_replace(email_label_data);
    /*
    let email_input = new_field(InputData {
        text: String::from("Email Address"),
        padding: (0.0, 0.0, 0.0, 20.0).into(),
        alignment: Some(String::from("left")),sss
        field: String::from("email_address")
    });
    */
    let test = InputData::new(InputData {
        text: String::from("Email Address"),
        padding: (0.0, 0.0, 0.0, 20.0).into(),
        alignment: Some(String::from("left")),
        field_type: String::from("email_address"),
    });
    test.create_textbox();
    let new_email_address = test.format_textbox();
    
    //let test_textbox = test.create_label();


    //let new_email_address_input: LensWrap<FunnyData, String, components2::funny_data_derived_lenses::email_address, TextBox<String>> = TextBox::new().with_placeholder(email_input_data.text).lens(FunnyData::email_address);
    //let new_email_address = new_field(email_input_data);
    
    
    
    let font_label = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(24.0);





    let email_subject_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Subject: "))
            .with_font(font_label.clone()));
    let email_subject = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Subject")
            .lens(FunnyData::email_subject));
    let email_body_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Body: "))
            .with_font(font_label.clone()));
    let email_body = Padding::new((0.0, 0.0, 00.0, 20.0),
        TextBox::multiline()
            .with_line_wrapping(true)
            .with_text_alignment(TextAlignment::Start)
            .with_text_size(theme::TEXT_SIZE_NORMAL)
            .with_placeholder("Email Body")
            .lens(FunnyData::email_body));

            Flex::column().with_child(
                Flex::column()
                    .with_child(email_address_label)
                    .with_child(new_email_address)
            ).with_child(
                Flex::column()
                    .with_child(email_subject_label)
                    .with_child(email_subject)
            ).with_child(
                Flex::column()
                    .with_child(email_body_label)
                    .with_child(email_body)
            )
    /* 
    let email_address_label = Padding::new((0.0, 20.0, 0.0, 10.0), 
    Align::left(Label::new(|data: &FunnyData, _: &Env| format!("Email Address: "))
            .with_font(font_label.clone())));
    let email_address = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Address").lens(FunnyData::email_address));
    let email_subject_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Subject: "))
            .with_font(font_label.clone()));
    let email_subject = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Subject")
            .lens(FunnyData::email_subject));
    let email_body_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Body: "))
            .with_font(font_label.clone()));
    let email_body = Padding::new((0.0, 0.0, 00.0, 20.0),
        TextBox::multiline()
            .with_line_wrapping(true)
            .with_text_alignment(TextAlignment::Start)
            .with_text_size(theme::TEXT_SIZE_NORMAL)
            .with_placeholder("Email Body")
            .lens(FunnyData::email_body));


    let font_label = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(24.0);

    let count_label = Label::new(|data: &FunnyData, _: &Env| format!("Counter: {}", data.num));
    let increment = Button::new("+").on_click(|ctx, data: &mut FunnyData, _: &Env| data.num += 1);
    let decrement = Button::new("-").on_click(|ctx, data: &mut FunnyData, _: &Env| data.num -= 1);

    /*
    let label_container = Label::new("Sample data: ");
    let aligned_label = Align::left(label_container);    
    let sample_label = Padding::new((0.0, 0.0, 0.0, 60.0), aligned_label);
    
    */
    let label_container = Label::new("Sample data: ");
    let aligned_label = Align::left(label_container);    
    let sample_label = Padding::new((0.0, 0.0, 0.0, 60.0), aligned_label);
    //let sample_label = label_format("Sample Label", (0.0, 0.0, 0.0, 60.0).into());

    // Width padding -> (left, top, right, bottom)
    /* 

    let email_address_label = Padding::new((0.0, 20.0, 0.0, 10.0), 
    Align::left(Label::new(|data: &FunnyData, _: &Env| format!("Email Address: "))
            .with_font(font_label.clone())));
    let email_address = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Address").lens(FunnyData::email_address));
    let email_subject_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Subject: "))
            .with_font(font_label.clone()));
    let email_subject = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Subject")
            .lens(FunnyData::email_subject));
    let email_body_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Body: "))
            .with_font(font_label.clone()));
    let email_body = Padding::new((0.0, 0.0, 00.0, 20.0),
        TextBox::multiline()
            .with_line_wrapping(true)
            .with_text_alignment(TextAlignment::Start)
            .with_text_size(theme::TEXT_SIZE_NORMAL)
            .with_placeholder("Email Body")
            .lens(FunnyData::email_body));

    
    */
    let email_address_label = Padding::new((0.0, 20.0, 0.0, 10.0), 
    Align::left(Label::new(|data: &FunnyData, _: &Env| format!("Email Address: "))
            .with_font(font_label.clone())));
    let email_address = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Address").lens(FunnyData::email_address));
    let email_subject_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Subject: "))
            .with_font(font_label.clone()));
    let email_subject = Padding::new((0.0, 0.0, 0.0, 20.0), 
        TextBox::new().with_placeholder("Email Subject")
            .lens(FunnyData::email_subject));
    let email_body_label = Padding::new((0.0, 0.0, 0.0, 10.0),
        Label::new(|data: &FunnyData, _: &Env| format!("Email Body: "))
            .with_font(font_label.clone()));
    let email_body = Padding::new((0.0, 0.0, 00.0, 20.0),
        TextBox::multiline()
            .with_line_wrapping(true)
            .with_text_alignment(TextAlignment::Start)
            .with_text_size(theme::TEXT_SIZE_NORMAL)
            .with_placeholder("Email Body")
            .lens(FunnyData::email_body));


    let save = Button::new("Save").on_click(|ctx, data: &mut FunnyData, _: &Env| save_data(&data));

    /*
    Flex::column().with_child(
        Flex::column()
            .with_child(sample_label)
            .with_child(email_address)
    ).with_child(
        Flex::column()
            .with_child(email_subject_label)
            .with_child(email_subject)
    ).with_child(
        Flex::column()
            .with_child(email_body_label)
            .with_child(email_body)
    ).with_child(
        Flex::row()
            .with_child(save)
    )
    */
    */
}

fn save_data(data: &FunnyData) {
    let data_string = format!("{}\n{}\n{}\n", data.email_address, data.email_subject, data.email_body);

    // Write command to command line 
    let mut file = File::create("emails/data.txt").expect("Couldn't create file");
    file.write_all(data_string.as_bytes()).expect("Couldn't write to file");
}

fn main() {
    let main_window = WindowDesc::new(ui_builder()).title("Wavedash - Email Builder");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(FunnyData { num: 0, email_address: "".to_string(), email_subject: "".to_string(), email_body: "".to_string() })
        .expect("launch failed");
}
