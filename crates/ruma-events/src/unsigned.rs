use js_int::Int;
use serde::{Deserialize, Serialize};

#[cfg(feature = "unstable-pre-spec")]
use crate::relation::Relations;
use crate::room::redaction::SyncRedactionEvent;

/// Extra information about an event that is not incorporated into the event's hash.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct Unsigned {
    /// The time in milliseconds that has elapsed since the event was sent.
    ///
    /// This field is generated by the local homeserver, and may be incorrect if the local time on
    /// at least one of the two servers is out of sync, which can cause the age to either be
    /// negative or greater than it actually is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<Int>,

    /// The client-supplied transaction ID, if the client being given the event is the same one
    /// which sent it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,

    /// Server-compiled information from other events relating to this event.
    #[cfg(feature = "unstable-pre-spec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-pre-spec")))]
    #[serde(rename = "m.relations", skip_serializing_if = "Option::is_none")]
    pub relations: Option<Relations>,
}

impl Unsigned {
    /// Create a new `Unsigned` with fields set to `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether this unsigned data is empty (all fields are `None`).
    ///
    /// This method is used to determine whether to skip serializing the `unsigned` field in room
    /// events. Do not use it to determine whether an incoming `unsigned` field was present - it
    /// could still have been present but contained none of the known fields.
    pub fn is_empty(&self) -> bool {
        self.age.is_none() && self.transaction_id.is_none()
    }
}

/// Extra information about a redacted event that is not incorporated into the event's hash.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct RedactedUnsigned {
    /// The event that redacted this event, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_because: Option<Box<SyncRedactionEvent>>,
}

impl RedactedUnsigned {
    /// Create a new `RedactedUnsigned` with field set to `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `RedactedUnsigned` with the given redacted because.
    pub fn new_because(redacted_because: Box<SyncRedactionEvent>) -> Self {
        Self { redacted_because: Some(redacted_because) }
    }

    /// Whether this unsigned data is empty (`redacted_because` is `None`).
    ///
    /// This method is used to determine whether to skip serializing the `unsigned` field in
    /// redacted room events. Do not use it to determine whether an incoming `unsigned` field
    /// was present - it could still have been present but contained none of the known fields.
    pub fn is_empty(&self) -> bool {
        self.redacted_because.is_none()
    }
}

#[doc(hidden)]
#[cfg(feature = "compat")]
#[derive(Deserialize)]
pub struct UnsignedWithPrevContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<Int>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transaction_id: Option<String>,

    #[cfg(feature = "unstable-pre-spec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-pre-spec")))]
    #[serde(rename = "m.relations", skip_serializing_if = "Option::is_none")]
    relations: Option<Relations>,

    pub prev_content: Option<Box<serde_json::value::RawValue>>,
}

#[cfg(feature = "compat")]
impl From<UnsignedWithPrevContent> for Unsigned {
    fn from(u: UnsignedWithPrevContent) -> Self {
        Self {
            age: u.age,
            transaction_id: u.transaction_id,
            #[cfg(feature = "unstable-pre-spec")]
            relations: u.relations,
        }
    }
}

#[doc(hidden)]
#[cfg(feature = "compat")]
#[derive(Deserialize)]
pub struct RedactedUnsignedWithPrevContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    redacted_because: Option<Box<SyncRedactionEvent>>,

    pub prev_content: Option<Box<serde_json::value::RawValue>>,
}

#[cfg(feature = "compat")]
impl From<RedactedUnsignedWithPrevContent> for RedactedUnsigned {
    fn from(u: RedactedUnsignedWithPrevContent) -> Self {
        Self { redacted_because: u.redacted_because }
    }
}
