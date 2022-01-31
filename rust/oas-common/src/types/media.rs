use crate::mapping::Mappable;
use crate::record::TypedValue;
use crate::{ElasticMapping, Reference};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::ser;

use super::{Feed, Post};

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub content_url: String,
    // File size in bytes.
    pub content_size: Option<u32>,
    pub encoding_format: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "ser::deserialize_duration")]
    pub duration: Option<f32>,
    pub transcript: Option<Transcript>,
    pub nlp: Option<Value>,

    #[serde(flatten)]
    pub other: serde_json::Map<String, Value>,

    #[serde(default)]
    pub feeds: Vec<Reference<Feed>>,
    #[serde(default)]
    pub posts: Vec<Reference<Post>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct Transcript {
    pub text: String,
    pub parts: Vec<TranscriptPart>,
    pub segments: Vec<Segment>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct Segment {
    pub start: f32,
    pub end: f32,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct TranscriptPart {
    pub conf: f32,
    pub start: f32,
    pub end: f32,
    pub word: String,
    pub suffix: Option<String>,
}

impl TranscriptPart {
    pub fn duration(&self) -> f32 {
        self.end - self.start
    }
}

impl TypedValue for Media {
    const NAME: &'static str = "oas.Media";
}

impl Mappable for Media {}

impl ElasticMapping for Media {
    fn elastic_mapping() -> serde_json::Value {
        json!({
            "transcript": {
                "type": "object",
                "enabled": false
            },
            "contentUrl":{
                "type": "keyword"
            },
            "duration": {
                "type": "float"
            },
            "contentSize": {
                "type": "float"
            },
            "encodingFormat": {
                "type": "keyword"
            },
            "nlp": {
                "type": "object"
            },
            "posts": {
                "type": "keyword"
            },
            "feeds": {
                "type": "keyword"
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_duration() {
        let source = r#"
        {
            "contentUrl": "foo",
            "duration": "02:03"
        }
        "#;

        let media: Media = serde_json::from_str(source).expect("failed to deserialize");
        assert_eq!(media.duration, Some(123.));

        let source = r#"
       {
           "contentUrl": "foo",
           "duration": "02:03:01"
       }
       "#;

        let media: Media = serde_json::from_str(source).expect("failed to deserialize");
        assert_eq!(media.duration, Some(7381.));

        let source = r#"
       {
           "contentUrl": "foo",
           "duration": "64"
       }
       "#;

        let media: Media = serde_json::from_str(source).expect("failed to deserialize");
        assert_eq!(media.duration, Some(64.));

        let source = r#"
        {
            "contentUrl": "foo",
            "duration": 123
        }
        "#;

        let media: Media = serde_json::from_str(source).expect("failed to deserialize");
        assert_eq!(media.duration, Some(123.));

        let source = r#"
        {
            "contentUrl": "foo",
            "duration": 562.5011
        }
        "#;

        let media: Media = serde_json::from_str(source).expect("failed to deserialize");
        assert_eq!(media.duration, Some(562.5011));

        let source = r#"
       {
           "contentUrl": "foo"
       }
       "#;

        let media: Media = serde_json::from_str(source).expect("failed to deserialize");
        assert_eq!(media.duration, None);
    }
}
