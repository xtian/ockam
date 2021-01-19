use super::*;

#[derive(Clone, Debug)]
pub struct ProfileRuleEd25519Verification {
    public_key: Vec<u8>,
    permissions: Vec<ProfilePermission>,
}

#[derive(Clone, Debug)]
pub struct ProfileRuleEcdsaP256Verification {
    public_key: Vec<u8>,
    permissions: Vec<ProfilePermission>,
}

#[derive(Clone, Debug)]
pub enum ProfileRule {
    Ed25519Verification(ProfileRuleEd25519Verification),
    EcdsaP256Verification(ProfileRuleEcdsaP256Verification),
}
