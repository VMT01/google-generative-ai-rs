use super::{
    models::{generative_models::GenerativeModel, get_model_info, get_model_list},
    types::{
        model::{Model, ModelParams},
        requests::RequestOptions,
        responses::ListModelResponse,
    },
};

/// Top-level class for this SDK
#[derive(Debug)]
pub struct GoogleGenerativeAI {
    api_key: String,
}

impl GoogleGenerativeAI {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn get_model_list(
        &self,
        request_options: Option<RequestOptions>,
    ) -> Result<ListModelResponse, reqwest::Error> {
        get_model_list(self.api_key.clone(), request_options).await
    }

    pub async fn get_model_info(
        &self,
        model: String,
        request_options: Option<RequestOptions>,
    ) -> Result<Model, reqwest::Error> {
        get_model_info(self.api_key.clone(), model, request_options).await
    }

    /// Gets a GenerativeModel instance for the provided model name.
    pub fn get_generative_model(
        &self,
        model_params: ModelParams,
        request_options: Option<RequestOptions>,
    ) -> GenerativeModel {
        GenerativeModel::new(self.api_key.clone(), model_params, request_options)
    }
}

// #[tokio::test]
// async fn test_get_model_list() {
//     dotenvy::dotenv().ok();
//     let api_key = std::env::var("GOOGLE_API_KEY").expect(".env not found");

//     let model_list = GoogleGenerativeAI::new(api_key).get_model_list(None).await;
//     if let Err(err) = model_list {
//         panic!("{err}");
//     }
// }

// #[tokio::test]
// async fn test_get_model_info() {
//     dotenvy::dotenv().ok();
//     let api_key = std::env::var("GOOGLE_API_KEY").expect(".env not found");

//     let model_list = GoogleGenerativeAI::new(api_key)
//         .get_model_info(String::from("gemini-pro"), None)
//         .await;
//     if let Err(err) = model_list {
//         panic!("{err}");
//     }
// }

// #[test]
// fn test_construct_google_genai() {
//     dotenvy::dotenv().ok();
//     let api_key = std::env::var("GOOGLE_API_KEY").expect(".env not found");

//     let _ = GoogleGenerativeAI::new(api_key)
//         .get_generative_model(ModelParams::new("gemini-1.5-flash"), None);
// }

// #[tokio::test]
// async fn test_generate_content() {
//     dotenvy::dotenv().ok();
//     let api_key = std::env::var("GOOGLE_API_KEY").expect(".env not found");

//     let genai = GoogleGenerativeAI::new(api_key)
//         .get_generative_model(ModelParams::new("gemini-1.5-flash"), None);
//     let content = genai
//         .generate_content(vec![crate::v1::types::content_types::Part {
//             text: Some("Hello Gemini".to_string()),
//             ..Default::default()
//         }])
//         .await;
//     if let Err(err) = content {
//         panic!("{err}");
//     }
// }

#[tokio::test]
async fn test_generate_content_stream() {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect(".env not found");

    let genai = GoogleGenerativeAI::new(api_key)
        .get_generative_model(ModelParams::new("gemini-1.5-flash"), None);
    let content = genai
        .generate_content_stream(vec![crate::v1::types::content_types::Part {
            text: Some("Hello Gemini. Can you tell me something about you?".to_string()),
            ..Default::default()
        }])
        .await;
    if let Err(err) = content {
        panic!("{err}");
    }
}
