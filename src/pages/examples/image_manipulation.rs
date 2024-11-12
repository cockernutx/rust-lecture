use std::io::{BufWriter, Cursor, Write};
use dioxus::prelude::*;
use image::{ImageBuffer, Rgba, RgbaImage, DynamicImage};

use base64::{write::EncoderWriter, Engine};
use base64::engine::general_purpose;
use image::ImageFormat::Png;

#[component]
pub fn ImageManipulation() -> Element {
    let mut image_data = use_signal(|| None);
    let mut original_image_src = use_signal(|| String::new());

    let get_file = move |evt: Event<FormData>| async move {
        if let Some(file_engine) = evt.files() {
            let files = file_engine.files();
            for file_name in &files {
                if let Some(file) = file_engine.read_file(file_name).await {
                    let src = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&file));
                    original_image_src.set(src);
                    let image = image::load_from_memory(file.as_slice()).unwrap();
                    tracing::debug!("here");
                    *image_data.write() = Some(image);
                }
            }
        }
    };
    let mut contrast = use_signal(|| 0.0f32);
    let change_contrast = move || async move {
        
        if let Some(image) =  &mut *image_data.write_unchecked() {
            let new_image = image.adjust_contrast(*contrast.read_unchecked());
            
           *image = new_image;
        }
    };

    let mut blur = use_signal(|| 0.0f32);
    let blur_image = move || async move {
        if let Some(image) =  &mut *image_data.write_unchecked() {
            let new_image = image.blur(*blur.read_unchecked());

            *image = new_image;
        }
    };

  
    
    rsx! {
        div { class: "p-6",
            input {
                class: "file-input file-input-ghost w-full max-w-xs ",
                r#type: "file",
                accept: ".png",
                onchange: get_file,
            }
        }
        div {
            if let Some(Some(base_64)) = base_64_src() {
                div { class: "mt-6 p-6",
                    div { class: "grid-cols-12 grid gap-4",
                        input {
                            min: "-100",
                            value: "0",
                            r#type: "range",
                            max: "100",
                            class: "range range-xs col-span-7",
                            oninput: move |e| contrast.set(e.value().parse().unwrap()),
                        }
                        div { class: "text-bold font-figtree text-lg leading-3 col-span-1",
                            "{contrast}"
                        }
                        button {
                            class: "btn btn-neutral col-span-4",
                            onclick: move |_| change_contrast(),
                            "Change contrast"
                        }
                    }

                    div { class: "grid-cols-12 grid gap-4 mt-4",
                        input {
                            min: "0",
                            value: "0",
                            r#type: "range",
                            max: "10",
                            class: "range range-xs col-span-7",
                            oninput: move |e| blur.set(e.value().parse().unwrap()),
                        }
                        div { class: "text-bold font-figtree text-lg leading-3 col-span-1",
                            "{blur}"
                        }
                        button {
                            class: "btn btn-neutral col-span-4",
                            onclick: move |_| blur_image(),
                            "Blur image"
                        }
                    }
                }
                div { class: "mt-6 p-2 rounded-box",
                    img { src: base_64 }
                }
            }
        }
    }
}