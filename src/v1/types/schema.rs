use std::collections::HashMap;

/// Schema is the [Schema] object allows the definition of input and output data types.
/// These types can be objects, but also primitives and arrays.
/// Represents a select subset of an [OpenAPI 3.0 schema object](https://spec.openapis.org/oas/v3.0.3#schema).
#[derive(Debug, Clone, serde::Serialize)]
pub struct Schema {
    /// The type of the property.
    pub r#type: SchemaType,

    /// The format of the data.
    /// This is used only for primitive datatypes.
    /// Supported formats:
    /// - NUMBER: float, double
    /// - INTEGER: i32, i64
    pub format: Option<String>,

    /// A brief description of the parameter.
    /// This could contain examples of use.
    /// Parameter description may be formatted as Markdown.
    pub description: Option<String>,

    /// Indicates if the value may be null.
    pub nullable: Option<bool>,

    /// Possible values of the element of [SchemaType::String] with enum format.
    /// For example we can define an Enum Direction as:
    /// ```
    /// use google_generative_ai_rs::v1::types::schema::Schema;
    /// use google_generative_ai_rs::v1::types::schema::SchemaType;
    ///
    /// let directionSchema = Schema {
    ///     r#type: SchemaType::String,
    ///     format: Some("enum".to_string()),
    ///     description: None,
    ///     nullable: None,
    ///     r#enum: Some(vec![
    ///         "EAST".to_string(),
    ///         "NORTH".to_string(),
    ///         "SOUTH".to_string(),
    ///         "WEST".to_string(),
    ///     ]),
    ///     items: None,
    ///     properties: None,
    ///     required: None,
    ///     example: None,
    /// };
    /// ```
    pub r#enum: Option<Vec<String>>,

    /// Schema of the elements of [SchemaType::Array].
    pub items: Option<Box<Schema>>,

    /// Properties of [SchemaType::Object].
    pub properties: Option<HashMap<String, Schema>>,

    /// Required properties of [SchemaType::Object].
    pub required: Option<Vec<String>>,

    /// The example of the property.
    pub example: Option<serde_json::Value>,
}

/// Type contains the list of OpenAPI data types as defined by https://spec.openapis.org/oas/v3.0.3#data-types
#[derive(Debug, Clone, serde::Serialize)]
pub enum SchemaType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
}
