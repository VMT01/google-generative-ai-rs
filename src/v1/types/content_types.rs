use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Defines a tool that model can call to access external knowledge.
#[derive(Debug)]
pub struct Tool {
    /// One or more function declarations to be passed to the model along with the current user query. Model may decide to call a subset of these functions by populating [FunctionCall]  in the response. User should provide a [FunctionResponse] for each function call in the next turn. Based on the function responses, Model will generate the final response back to the user. Maximum 64 function declarations can be provided.
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

/// Structured representation of a function declaration as defined by the [OpenAPI 3.0 specification](https://spec.openapis.org/oas/v3.0.3). Included in this declaration are the function name and parameters. This [FunctionDeclaration] is a representation of a block of code that can be used as a Tool by the model and executed by the client.
#[derive(Debug)]
pub struct FunctionDeclaration {
    /// The name of the function to call. Must start with a letter or an underscore. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a max length of 64.
    pub name: String,

    /// Description and purpose of the function. Model uses it to decide how and whether to call the function.
    pub description: Option<String>,

    /// Describes the parameters to this function in JSON Schema Object format. Reflects the Open API 3.03 Parameter Object.
    ///
    /// Key {string}: the name of the parameter. Parameter names are case sensitive.
    /// Value {Schema}: the Schema defining the type used for the parameter. For function with no parameters, this can be left unset.
    ///
    /// @example with 1 required and 1 optional parameter
    /// type: OBJECT
    /// properties:
    /// ```yaml
    /// param1:
    ///     type: STRING
    /// param2:
    ///     type: INTEGER
    /// required:
    ///     - param1
    /// ```
    pub parameters: Option<FunctionParameters>,
}

/// Schema for parameters passed to [FunctionDeclaration::parameters].
#[derive(Debug)]
pub struct FunctionParameters {
    /// The type of the parameter.
    pub r#type: FunctionParameterType,

    /// The format of the parameter.
    pub properties: HashMap<String, FunctionParametersProperty>,

    /// Description of the parameter.
    pub description: Option<String>,

    /// Array of required parameters.
    pub required: Option<Vec<String>>,
}

/// Contains the list of OpenAPI data types as defined by [Swagger](https://swagger.io/docs/specification/data-models/data-types/)
#[derive(Debug, Serialize)]
pub enum FunctionParameterType {
    #[serde(rename = "STRING")]
    String,

    #[serde(rename = "NUMBER")]
    Number,

    #[serde(rename = "INTEGER")]
    Integer,

    #[serde(rename = "BOOLEAN")]
    Boolean,

    #[serde(rename = "ARRAY")]
    Array,

    #[serde(rename = "OBJECT")]
    Object,
}

/// Schema is used to define the format of input/output data. Represents a select subset of an OpenAPI 3.0 schema object. More fields may be added in the future as needed.
#[derive(Debug)]
pub struct FunctionParametersProperty {
    /// The type of the property. [FunctionParameters::properties]
    pub r#type: Option<String>,

    /// The format of the property.
    pub format: Option<String>,

    /// The description of the property.
    pub description: Option<String>,

    /// Whether the property is nullable.
    pub nullable: Option<bool>,

    /// The items of the property. [FunctionParameters]
    pub items: Option<FunctionParameters>,

    /// The enum of the property.
    pub r#enum: Option<Vec<String>>,

    /// Map of [FunctionParameters]
    pub properties: HashMap<String, FunctionParameters>,

    /// Array of required property.
    pub required: Option<Vec<String>>,
}

/// Tool config. This config is shared for all tools provided in the request.
#[derive(Debug)]
pub struct ToolConfig {
    pub function_calling_config: FunctionCallingConfig,
}

#[derive(Debug)]
pub struct FunctionCallingConfig {
    pub mode: Option<FunctionCallingMode>,
    pub allowed_function_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub enum FunctionCallingMode {
    /// Unspecified function calling mode. This value should not be used.
    #[serde(rename = "MODE_UNSPECIFIED")]
    ModeUnspecified,

    /// Default model behavior, model decides to predict either a function call
    /// or a natural language repspose.
    #[serde(rename = "AUTO")]
    Auto,

    /// Model is constrained to always predicting a function call only. If [FunctionCallingConfig::allowed_function_names] are set, the predicted function call will be limited to any one of [FunctionCallingConfig::allowed_function_names], else the predicted function call will be any one of the provided [Tool::function_declarations].
    #[serde(rename = "ANY")]
    Any,

    /// Model will not predict any function call. Model behavior is same as when not passing any function declarations.
    #[serde(rename = "NONE")]
    None,
}

/// Content type for both prompts and response candidates.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentType {
    pub role: String,
    pub parts: Vec<Part>,
}

/// Content part - includes text or image part types.
#[derive(Debug, Serialize)]
pub enum Part {
    TextPart(TextPart),
    InlineDataPart(InlineDataPart),
    FunctionCallPart(FunctionCallPart),
    FunctionResponsePart(FunctionResponsePart),
    FileDataPart(FileDataPart),
}

impl<'de> Deserialize<'de> for Part {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Object(obj) => {
                for (key, value) in obj.iter() {
                    if key.eq("text") {
                        return Ok(Part::TextPart(TextPart {
                            text: value.as_str().unwrap().to_string(),
                        }));
                    }

                    if key.eq("inlineData") {
                        let inline_data = value.as_object().unwrap();
                        return Ok(Part::InlineDataPart(InlineDataPart {
                            inline_data: GenerativeContentBlob {
                                mime_type: inline_data["mimeType"].as_str().unwrap().to_string(),
                                data: inline_data["data"].as_str().unwrap().to_string(),
                            },
                        }));
                    }

                    if key.eq("functionCall") {
                        let function_call = value.as_object().unwrap();
                        let mut args: HashMap<String, serde_json::Value> = HashMap::new();

                        for (key, value) in function_call["args"].as_object().unwrap().to_owned() {
                            args.insert(key, value);
                        }

                        return Ok(Part::FunctionCallPart(FunctionCallPart {
                            function_call: FunctionCall {
                                name: function_call["name"].as_str().unwrap().to_string(),
                                args,
                            },
                        }));
                    }

                    if key.eq("functionResponse") {
                        let function_response = value.as_object().unwrap();
                        let mut response: HashMap<String, serde_json::Value> = HashMap::new();

                        for (key, value) in function_response["response"]
                            .as_object()
                            .unwrap()
                            .to_owned()
                        {
                            response.insert(key, value);
                        }

                        return Ok(Part::FunctionResponsePart(FunctionResponsePart {
                            function_response: FunctionResponse {
                                name: function_response["name"].as_str().unwrap().to_string(),
                                response,
                            },
                        }));
                    }

                    if key.eq("fileData") {
                        let file_data = value.as_object().unwrap();

                        return Ok(Part::FileDataPart(FileDataPart {
                            file_data: FileData {
                                mime_type: file_data["mimeType"].as_str().unwrap().to_string(),
                                file_uri: file_data["fileUri"].as_str().unwrap().to_string(),
                            },
                        }));
                    }
                }

                Err(serde::de::Error::custom("Invalid data structure."))
            }
            _ => Err(serde::de::Error::custom("Expected JSON object")),
        }
    }
}

/// Content part interface if the part represents a text string.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPart {
    pub text: String,
}

/// Content part interface if the part represents an image.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineDataPart {
    pub inline_data: GenerativeContentBlob,
}

/// Interface for sending an image.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerativeContentBlob {
    pub mime_type: String,

    /// Image as a base64 string.
    pub data: String,
}

/// Content part interface if the part represents FunctionResponse.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallPart {
    pub function_call: FunctionCall,
}

/// A predicted [FunctionCall] returned from the model that contains a string representing the [FunctionDeclaration::name] and a structured JSON object containing the parameters and their values.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub name: String,
    pub args: HashMap<String, serde_json::Value>,
}

/// Content part interface if the part represents FunctionResponse.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResponsePart {
    pub function_response: FunctionResponse,
}

/// The result output from a [FunctionCall] that contains a string representing the [FunctionDeclaration::name] and a structured JSON object containing any output from the function is used as context to the model. This should contain the result of a [FunctionCall] made based on model prediction.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResponse {
    pub name: String,
    pub response: HashMap<String, serde_json::Value>,
}

/// Content part interface if the part represents FunctionResponse.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDataPart {
    pub file_data: FileData,
}

/// Data pointing to a file uploaded with the Files API.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileData {
    pub mime_type: String,
    pub file_uri: String,
}
