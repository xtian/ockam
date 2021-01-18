use crate::profile::change_event::ChangeEventType::{CreateKey, RevokeKey, RotateKey};
use crate::profile::change_event::{ChangeEventType, ProfileKeyPurpose, ProfileKeyType};
use crate::profile::error::Error;
use crate::profile::signed_change_event::SignedChangeEvent;
use crate::profile::{ProfileEventAttributes, ProfileVault};
use ockam_common::error::OckamResult;
use ockam_queue_topic::queue::ToMessage;
use ockam_vault::Secret;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Profile {
    identifier: String, // First public key id
    change_events: Vec<SignedChangeEvent>,
    keys: HashMap<String, Arc<Mutex<dyn Secret>>>, // HashMap key is event_id
    vault: Arc<Mutex<dyn ProfileVault>>,
}

impl Profile {
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
    pub fn change_events(&self) -> &[SignedChangeEvent] {
        &self.change_events
    }
    pub fn keys(&self) -> &HashMap<String, Arc<Mutex<dyn Secret>>> {
        &self.keys
    }
    pub fn vault(&self) -> &Arc<Mutex<dyn ProfileVault>> {
        &self.vault
    }
}

impl Profile {
    pub(crate) fn new(
        identifier: String,
        change_events: Vec<SignedChangeEvent>,
        keys: HashMap<String, Arc<Mutex<dyn Secret>>>,
        vault: Arc<Mutex<dyn ProfileVault>>,
    ) -> Self {
        Profile {
            identifier,
            change_events,
            keys,
            vault,
        }
    }

    pub(crate) fn public_key(
        &self,
        key_type: ProfileKeyType,
        key_purpose: ProfileKeyPurpose,
    ) -> OckamResult<Option<&[u8]>> {
        let last = self
            .change_events
            .iter()
            .filter(|e| match e.change_event().etype() {
                CreateKey(event) => {
                    event.key_type() == key_type && event.key_purpose() == key_purpose
                }
                RotateKey(event) => {
                    event.key_type() == key_type && event.key_purpose() == key_purpose
                }
                RevokeKey(event) => {
                    event.key_type() == key_type && event.key_purpose() == key_purpose
                }
                _ => false,
            })
            .last();

        let last_event;
        if let Some(e) = last {
            last_event = e;
        } else {
            return Ok(None);
        }

        let public_key = match last_event.change_event().etype() {
            CreateKey(event) => Some(event.public_key()),
            RotateKey(event) => Some(event.public_key()),
            _ => None,
        };

        Ok(public_key)
    }

    pub(crate) fn add_event(
        &mut self,
        event: SignedChangeEvent,
        key: Option<Box<dyn Secret>>,
    ) -> OckamResult<()> {
        let event_id = event.identifier().to_string();
        self.change_events.push(event);
        if let Some(key) = key {
            unsafe {
                self.keys
                    .insert(event_id, Arc::new(Mutex::new(*Box::into_raw(key))));
            }
        }

        Ok(())
    }
}
