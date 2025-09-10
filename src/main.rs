use dioxus::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        #[derive(serde::Deserialize)]
        struct DogApi {
            message: String,
        }

        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: {img_src().unwrap_or_default()} }
        }
        div { id: "buttons",
            button {
                id: "skip",
                onclick: move |_| {
                    img_src.restart();
                },
                "skip"
            }
                //button { onclick: save, id: "save", "save!" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_api_deserialization() {
        // Test that serde can deserialize the DogApi structure
        #[derive(serde::Deserialize)]
        struct DogApi {
            message: String,
        }
        
        let json_data = r#"{"message":"https://images.dog.ceo/breeds/hound-afghan/n02088094_1007.jpg","status":"success"}"#;
        let result: Result<DogApi, _> = serde_json::from_str(json_data);
        
        assert!(result.is_ok());
        let dog_api = result.unwrap();
        assert!(dog_api.message.starts_with("https://"));
        assert!(dog_api.message.contains("dog.ceo"));
    }
}
