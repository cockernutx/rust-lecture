use std::io::{BufWriter, Cursor, Write};
use dioxus::prelude::*;
use image::{ImageBuffer, Rgba, RgbaImage, DynamicImage};

use base64::{write::EncoderWriter, Engine};
use base64::engine::general_purpose;
use image::ImageFormat::Png;

#[component]
pub fn ImageManipulation() -> Element {
    let mut image_data = use_signal(|| None);
    let get_file = move |evt: Event<FormData>| async move {
        if let Some(file_engine) = evt.files() {
            let files = file_engine.files();
            for file_name in &files {
                if let Some(file) = file_engine.read_file(file_name).await {
                    let image = image::load_from_memory(file.as_slice()).unwrap();
                    
                    *image_data.write() = Some(image);
                }
            }
        }
    };
    
    let base_64_src = use_resource(move || async move {
        if let Some(image) = &*image_data.read_unchecked() {
            let mut buf = Cursor::new(vec![]);


            image.write_to(&mut buf, Png).unwrap();

            return Some(format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(buf.get_ref())))
        }
        
        None
    });
    
    rsx! {
        input {
            class: "file-input file-input-ghost w-full max-w-xs",
            r#type: "file",
            accept: ".png",
            onchange: get_file,
        }
        div {
            if let Some(Some(base_64)) = base_64_src() {
                div { class: "mt-6",
                    
                }
                div { class: "mt-6 flex  justify-center rounded",
                    img { class: "rounded", src: base_64, width: "1000vw" }
                }
                
            }
        }
    }
}