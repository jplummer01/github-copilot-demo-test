use dioxus::prelude::*;
use reqwest;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

// API data structures
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DogApi {
    pub message: String,
}

// Utility functions for API handling
pub async fn fetch_random_dog_image() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json::<DogApi>()
        .await?;
    Ok(response.message)
}

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
        fetch_random_dog_image().await.unwrap_or_default()
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src().unwrap_or_default() }
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
    use serde_json;

    #[test]
    fn test_dog_api_deserialization() {
        let json_data = r#"{"message": "https://images.dog.ceo/breeds/retriever-golden/IMG_7257.jpg"}"#;
        
        let dog_api: DogApi = serde_json::from_str(json_data).expect("Failed to deserialize");
        
        assert_eq!(dog_api.message, "https://images.dog.ceo/breeds/retriever-golden/IMG_7257.jpg");
    }

    #[test]
    fn test_dog_api_struct_debug() {
        let dog_api = DogApi {
            message: "https://test-image.jpg".to_string(),
        };
        
        let debug_output = format!("{:?}", dog_api);
        assert!(debug_output.contains("DogApi"));
        assert!(debug_output.contains("https://test-image.jpg"));
    }

    #[test]
    fn test_dog_api_struct_partial_eq() {
        let dog_api1 = DogApi {
            message: "https://test1.jpg".to_string(),
        };
        let dog_api2 = DogApi {
            message: "https://test1.jpg".to_string(),
        };
        let dog_api3 = DogApi {
            message: "https://test2.jpg".to_string(),
        };
        
        assert_eq!(dog_api1, dog_api2);
        assert_ne!(dog_api1, dog_api3);
    }

    #[test]
    fn test_dog_api_deserialization_with_different_urls() {
        let test_cases = vec![
            (r#"{"message": "https://images.dog.ceo/breeds/hound-afghan/IMG_123.jpg"}"#, "https://images.dog.ceo/breeds/hound-afghan/IMG_123.jpg"),
            (r#"{"message": "https://images.dog.ceo/breeds/bulldog-french/IMG_456.jpg"}"#, "https://images.dog.ceo/breeds/bulldog-french/IMG_456.jpg"),
            (r#"{"message": "https://images.dog.ceo/breeds/terrier-yorkshire/IMG_789.jpg"}"#, "https://images.dog.ceo/breeds/terrier-yorkshire/IMG_789.jpg"),
        ];

        for (json, expected_url) in test_cases {
            let dog_api: DogApi = serde_json::from_str(json).expect("Failed to deserialize");
            assert_eq!(dog_api.message, expected_url);
        }
    }

    #[test]
    fn test_dog_api_deserialization_error_handling() {
        let invalid_json_cases = vec![
            r#"{"msg": "https://test.jpg"}"#,  // wrong field name
            r#"{"message": 123}"#,             // wrong type
            r#"{}"#,                           // missing field
            r#"invalid json"#,                 // invalid JSON
        ];

        for invalid_json in invalid_json_cases {
            let result = serde_json::from_str::<DogApi>(invalid_json);
            assert!(result.is_err(), "Expected error for: {}", invalid_json);
        }
    }

    // Test for the constants (even though they're unused)
    #[test]
    fn test_asset_constants_exist() {
        // We can't easily test the actual asset values, but we can ensure they compile
        // This test mainly serves to document that these constants exist
        let _favicon = FAVICON;
        let _main_css = MAIN_CSS; 
        let _header_svg = HEADER_SVG;
        // If we reach here without compilation errors, the constants are properly defined
    }

    // Helper function for testing URL validation
    fn is_valid_dog_image_url(url: &str) -> bool {
        url.starts_with("https://") && (url.contains("dog.ceo") || url.contains("images"))
    }

    #[test]
    fn test_dog_image_url_validation() {
        let valid_urls = vec![
            "https://images.dog.ceo/breeds/retriever-golden/IMG_123.jpg",
            "https://images.dog.ceo/breeds/bulldog-french/IMG_456.jpg",
            "https://dog.ceo/images/breeds/terrier/IMG_789.jpg",
        ];

        let invalid_urls = vec![
            "http://images.dog.ceo/breeds/retriever/IMG_123.jpg", // http instead of https
            "https://other-site.com/image.jpg",                   // wrong domain
            "not-a-url",                                          // not a URL
            "",                                                   // empty string
        ];

        for url in valid_urls {
            assert!(is_valid_dog_image_url(url), "Should be valid: {}", url);
        }

        for url in invalid_urls {
            assert!(!is_valid_dog_image_url(url), "Should be invalid: {}", url);
        }
    }
}
