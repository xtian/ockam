use crate::profile::change_event::ChangeEventType::{CreateKey, RevokeKey, RotateKey};
use crate::profile::change_event::{
    ChangeEvent, ChangeEventType, CreateKeyEvent, ProfileKeyPurpose, ProfileKeyType,
    RevokeKeyEvent, RotateKeyEvent,
};
use crate::profile::error::Error;
use crate::profile::profile::Profile;
use crate::profile::signed_change_event::{Signature, SignatureType, SignedChangeEvent};
use crate::profile::{ProfileEventAttributes, ProfileVault};
use ockam_common::error::OckamResult;
use ockam_queue_topic::queue::ToMessage;
use ockam_vault::types::{SecretAttributes, SecretPersistence, SecretType};
use ockam_vault::Secret;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ProfileManager {}

impl ProfileManager {
    // pub fn new_key_event(
    //     prev_key_event: Option<(Box<dyn Secret>, SignedChangeEvent)>,
    //     prev_event_id: String,
    //     is_revoke: bool,
    //     attributes: ProfileEventAttributes,
    //     vault: Arc<Mutex<dyn ProfileVault>>,
    // ) -> OckamResult<SignedChangeEvent> {
    //     let mut vault = vault.lock().unwrap();
    //
    //     let etype = if is_revoke {
    //         // Revoke
    //         let prev_event;
    //         if let Some(event) = prev_key_event {
    //             prev_event = event;
    //         } else {
    //             return Err(Error::InvalidInternalState.into());
    //         }
    //
    //         let prev_public_key = match prev_event.1.change_event().etype() {
    //             CrateKey(e) => e.public_key(),
    //             RotateKey(e) => e.public_key(),
    //             default => return Err(Error::InvalidInternalState.into()),
    //         };
    //
    //         let prev_public_key_id = vault.sha256(prev_public_key.as_ref())?; // TODO: Move kid computation
    //
    //         RevokeKeyEvent::new(prev_public_key_id.to_vec())
    //     } else {
    //         let attributes = SecretAttributes {
    //             stype: SecretType::Curve25519,
    //             persistence: SecretPersistence::Persistent,
    //             length: 0,
    //         };
    //
    //         let private_key = vault.secret_generate(attributes)?;
    //         let public_key = vault.secret_public_key_get(&private_key)?;
    //
    //         if let Some(event) = prev_key_event {
    //             // Rotate
    //             let (prev_public_key, key_purpose, key_type) =
    //                 match prev_event.1.change_event().etype() {
    //                     CrateKey(e) => (e.public_key(), e.key_purpose(), e.key_type()),
    //                     RotateKey(e) => (e.public_key(), e.key_purpose(), e.key_type()),
    //                     default => return Err(Error::InvalidInternalState.into()),
    //                 };
    //
    //             let prev_public_key_id = vault.sha256(prev_public_key.as_ref())?; // TODO: Move kid computation
    //
    //             RotateKeyEvent::new(
    //                 key_type.clone(),
    //                 key_purpose.clone(),
    //                 public_key.as_ref().to_vec(),
    //                 prev_public_key_id.to_vec(),
    //             )
    //         } else { // Create
    //             CreateKeyEvent::new()
    //         }
    //     };
    //
    //     let model = ChangeEvent::new(1, prev_event_id, attributes.clone());
    //     let model_binary: Vec<u8> =
    //         serde_bare::to_vec(&model).map_err(|_| Error::BareError.into())?;
    //     let identifier = vault.sha256(&model_binary)?;
    //     let self_signature = match &keys.0 {
    //         Some(s) => Some(vault.sign(s, &identifier)?),
    //         None => None,
    //     };
    //
    //     let previous_self_signature = match previous_event {
    //         Some(event) => {
    //             let private_key;
    //             if let Some(key) = event.private_key() {
    //                 private_key = key;
    //             } else {
    //                 return Err(Error::InvalidInternalState.into());
    //             }
    //             Some(vault.sign(private_key, &identifier)?)
    //         }
    //         None => None,
    //     };
    //
    //     let identifier = format!("E_ID.{}", hex::encode(&identifier));
    //
    //     Ok(SignedChangeEvent::new {
    //         version: 1,
    //         identifier,
    //         model_binary,
    //         attributes,
    //         public_key: keys.1,
    //         prev_event_id,
    //         next_event_id: None,
    //         private_key: keys.0,
    //         self_signature,
    //         previous_self_signature,
    //     })
    // }
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_profile(
        &self,
        key_type: ProfileKeyType,
        key_purpose: ProfileKeyPurpose,
        attributes: Option<ProfileEventAttributes>,
        vault: Arc<Mutex<dyn ProfileVault>>,
    ) -> OckamResult<Profile> {
        let attributes = attributes.unwrap_or(ProfileEventAttributes::new());

        let mut v = vault.lock().unwrap();

        // TODO: Should be customisable
        let secret_attributes = SecretAttributes {
            stype: SecretType::Curve25519,
            persistence: SecretPersistence::Persistent,
            length: 0,
        };

        let private_key = v.secret_generate(secret_attributes)?;
        let public_key = v.secret_public_key_get(&private_key)?;

        let event = CreateKeyEvent::new(key_type, key_purpose, public_key.as_ref().to_vec());
        let prev_id = v.sha256(&[])?;
        let prev_id = format!("E_ID.{}", hex::encode(&prev_id));
        let change_event =
            ChangeEvent::new(1, prev_id, attributes, ChangeEventType::CreateKey(event));
        let change_event_binary = serde_bare::to_vec(&change_event)?;

        let event_id = v.sha256(&change_event_binary)?;

        let self_signature = v.sign(&private_key, &event_id)?;
        let self_signature = Signature::new(SignatureType::SelfSign, self_signature);

        let event_id = format!("E_ID.{}", hex::encode(&event_id));

        let signed_change_event = SignedChangeEvent::new(
            1,
            event_id.clone(),
            change_event_binary,
            change_event,
            vec![self_signature],
        )?;

        let public_kid = v.sha256(public_key.as_ref())?;
        let public_kid = format!("P_ID.{}", hex::encode(&public_kid));

        let mut keys = HashMap::new();
        keys.insert(event_id, Arc::new(Mutex::new(private_key)));

        let profile = Profile::new(public_kid, vec![signed_change_event], keys, vault);

        Ok(profile)
    }

    pub fn get_profile_public_key(&self, profile: &Profile) -> OckamResult<Option<Vec<u8>>> {
        profile
            .public_key()
            .map(|opt| opt.map(|slice| slice.to_vec()))
    }

    pub fn rotate_profile(
        &self,
        profile: &mut Profile,
        attributes: Option<ProfileEventAttributes>,
    ) -> OckamResult<()> {
        let attributes = attributes.unwrap_or(ProfileEventAttributes::new());
        profile.rotate(attributes)
    }

    pub fn revoke_profile(
        &self,
        mut profile: Profile,
        attributes: Option<ProfileEventAttributes>,
    ) -> OckamResult<()> {
        let attributes = attributes.unwrap_or(ProfileEventAttributes::new());
        profile.revoke(attributes)?;
        self.delete_profile(profile)
    }

    pub fn attest_profile(&self, profile: &Profile, nonce: &[u8]) -> OckamResult<[u8; 64]> {
        profile.attest(nonce)
    }

    pub fn delete_profile(&self, mut profile: Profile) -> OckamResult<()> {
        profile.delete()
    }
}
