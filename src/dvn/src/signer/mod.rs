use ethers_core::{k256::{ecdsa::{RecoveryId, Signature as RecoverableSignature, VerifyingKey}, elliptic_curve::FieldBytes, Secp256k1}, types::{transaction::eip2718::TypedTransaction, Address, Signature, U256}, utils::public_key_to_address};
use ethers_signers::to_eip155_v;
use ic_cdk::api::management_canister::ecdsa::{ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument, SignWithEcdsaArgument};

#[derive(Clone, Default)]
pub struct Signer {
    public_key: Option<Vec<u8>>,
    derivation_path: Vec<Vec<u8>>,
    key_id: EcdsaKeyId
}

impl Signer {
    pub fn new(name: String) -> Self {
        Self {
            public_key: None,
            derivation_path: vec![],
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name
            }
        }
    }

    pub async fn init(&mut self) {
        let (result,) = ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: self.derivation_path.clone(),
            key_id: self.key_id.clone()
        }).await.unwrap();

        self.public_key = Some(result.public_key);
    }

    pub async fn sign_transaction(&self, transaction: &TypedTransaction) -> Signature {
        let hash = transaction.sighash().as_bytes().to_vec();
        let (result,) = sign_with_ecdsa(SignWithEcdsaArgument {
            message_hash: hash.clone(),
            derivation_path: self.derivation_path.clone(),
            key_id: self.key_id.clone()
        }).await.unwrap();
        let raw_signature = result.signature;
        let recoverable_signature = RecoverableSignature::from_slice(&raw_signature).unwrap();
        let verifying_key = VerifyingKey::from_sec1_bytes(self.public_key.as_ref().unwrap()).unwrap();
        let recovery_id = RecoveryId::trial_recovery_from_prehash(&verifying_key, &hash, &recoverable_signature).unwrap();

        let r_bytes: FieldBytes<Secp256k1> = recoverable_signature.r().into();
        let s_bytes: FieldBytes<Secp256k1> = recoverable_signature.s().into();
        let r = U256::from_big_endian(r_bytes.as_slice());
        let s = U256::from_big_endian(s_bytes.as_slice());
        let v = to_eip155_v(recovery_id, transaction.chain_id().unwrap().as_u64());

        Signature { r, s, v }
    }

    pub fn address(&self) -> Address {
        let verifying_key = &VerifyingKey::from_sec1_bytes(self.public_key.as_ref().unwrap()).unwrap();
        public_key_to_address(verifying_key)
    }
}
