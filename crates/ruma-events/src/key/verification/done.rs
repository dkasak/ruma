//! Types for the *m.key.verification.done* event.

use ruma_events_macros::EventContent;
use serde::{Deserialize, Serialize};

use super::Relation;
use crate::MessageEvent;

/// Event signaling that the interactive key verification has successfully
/// concluded.
pub type DoneEvent = MessageEvent<DoneEventContent>;

/// The payload for a to-device `m.key.verification.done` event.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.key.verification.done", kind = ToDevice)]
pub struct DoneToDeviceEventContent {
    /// An opaque identifier for the verification process.
    ///
    /// Must be the same as the one used for the *m.key.verification.start* message.
    pub transaction_id: String,
}

impl DoneToDeviceEventContent {
    /// Creates a new `DoneToDeviceEventContent` with the given transaction ID.
    pub fn new(transaction_id: String) -> Self {
        Self { transaction_id }
    }
}

/// The payload for a in-room `m.key.verification.done` event.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.key.verification.done", kind = Message)]
pub struct DoneEventContent {
    /// Relation signaling which verification request this event is responding to.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Relation,
}

impl DoneEventContent {
    /// Creates a new `DoneEventContent` with the given relation.
    pub fn new(relates_to: Relation) -> Self {
        Self { relates_to }
    }
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;
    use ruma_identifiers::event_id;
    use ruma_serde::Raw;
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::DoneEventContent;
    use crate::key::verification::Relation;

    #[test]
    fn serialization() {
        let event_id = event_id!("$1598361704261elfgc:localhost");

        let json_data = json!({
            "m.relates_to": {
                "rel_type": "m.reference",
                "event_id": event_id,
            }
        });

        let content = DoneEventContent { relates_to: Relation { event_id } };

        assert_eq!(to_json_value(&content).unwrap(), json_data);
    }

    #[test]
    fn deserialization() {
        let id = event_id!("$1598361704261elfgc:localhost");

        let json_data = json!({
            "m.relates_to": {
                "rel_type": "m.reference",
                "event_id": id,
            }
        });

        assert_matches!(
            from_json_value::<Raw<DoneEventContent>>(json_data)
                .unwrap()
                .deserialize()
                .unwrap(),
            DoneEventContent {
                relates_to: Relation {
                    event_id
                },
            } if event_id == id
        );
    }
}
