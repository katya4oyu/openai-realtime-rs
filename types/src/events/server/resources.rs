use crate::content::items::Item;
use crate::session::Session;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct _Resource {
    /// The unique ID of the resource
    id: String,

    /// The object type
    object: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionResource {
    #[serde(flatten)]
    resource: _Resource,

    #[serde(flatten)]
    session: Session,
}

impl SessionResource {
    pub fn id(&self) -> &str {
        &self.resource.id
    }

    pub fn session(&self) -> Session {
        self.session.clone()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConversationResource {
    #[serde(flatten)]
    resource: _Resource,
}

impl ConversationResource {
    pub fn id(&self) -> &str {
        &self.resource.id
    }
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemResource {
    #[serde(flatten)]
    resource: _Resource,
    #[serde(flatten)]
    content: Item,
}

impl ItemResource {
    pub fn id(&self) -> &str {
        &self.resource.id
    }

    pub fn content(&self) -> Item {
        self.content.clone()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseResource {
    #[serde(flatten)]
    resource: _Resource,

    /// The status of the response: "in_progress", "completed", "failed", "canceled", "incomplete"
    status: String,
    /// Additional details about the status
    status_details: Option<serde_json::Value>,
    /// The list of output items generated by the response
    output: Vec<ItemResource>,
    /// Usage Statistics for the response
    usage: Option<Usage>,
}

impl ResponseResource {
    pub fn id(&self) -> &str {
        &self.resource.id
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn status_details(&self) -> Option<serde_json::Value> {
        self.status_details.clone()
    }

    pub fn output(&self) -> Vec<ItemResource> {
        self.output.clone()
    }

    pub fn usage(&self) -> Option<Usage> {
        self.usage.clone()
    }
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Usage {
    total_tokens: i32,
    input_tokens: i32,
    output_tokens: i32,
}

impl Usage {
    pub fn total_tokens(&self) -> i32 {
        self.total_tokens
    }
    pub fn input_tokens(&self) -> i32 {
        self.input_tokens
    }
    pub fn output_tokens(&self) -> i32 {
        self.output_tokens
    }
}


/// Rate limit information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateLimitInformation {
    /// The name of the rate limit ("requests", "tokens", "input_tokens", "output_tokens").
    name: String,
    /// The maximum allowed value for the rate limit.
    limit: i32,
    /// The remaining value before the limit is reached.
    remaining: i32,
    /// Seconds until the rate limit resets.
    reset_seconds: i32,
}

impl RateLimitInformation {
    /// The name of the rate limit ("requests", "tokens", "input_tokens", "output_tokens").
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The maximum allowed value for the rate limit.
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// The remaining value before the limit is reached.
    pub fn remaining(&self) -> i32 {
        self.remaining
    }
    /// Seconds until the rate limit resets.
    pub fn reset_seconds(&self) -> i32 {
        self.reset_seconds
    }
}