use crate::{transaction::RlpEcdsaTx, SignableTransaction, Signed, Transaction, TxType};
use alloc::vec::Vec;
use alloy_eips::{eip2930::AccessList, eip7702::SignedAuthorization, Typed2718};
use alloy_primitives::{
    keccak256, Bytes, ChainId, PrimitiveSignature as Signature, TxKind, B256, U256,
};
use alloy_rlp::{length_of_length, BufMut, Decodable, Encodable, Header, Result};
use core::mem;

/// Legacy transaction.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[doc(alias = "LegacyTransaction", alias = "TransactionLegacy", alias = "LegacyTx")]
pub struct TxExtended {}

impl Typed2718 for TxExtended {
    fn ty(&self) -> u8 {
        todo!()
    }
}

impl Transaction for TxExtended {
    fn chain_id(&self) -> Option<ChainId> {
        todo!()
    }

    fn nonce(&self) -> u64 {
        todo!()
    }

    fn gas_limit(&self) -> u64 {
        todo!()
    }

    fn gas_price(&self) -> Option<u128> {
        todo!()
    }

    fn max_fee_per_gas(&self) -> u128 {
        todo!()
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        todo!()
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        todo!()
    }

    fn priority_fee_or_price(&self) -> u128 {
        todo!()
    }

    fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        todo!()
    }

    fn is_dynamic_fee(&self) -> bool {
        todo!()
    }

    fn kind(&self) -> TxKind {
        todo!()
    }

    fn is_create(&self) -> bool {
        todo!()
    }

    fn value(&self) -> U256 {
        todo!()
    }

    fn input(&self) -> &Bytes {
        todo!()
    }

    fn access_list(&self) -> Option<&AccessList> {
        todo!()
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        todo!()
    }

    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        todo!()
    }
}

impl SignableTransaction<Signature> for TxExtended {
    fn set_chain_id(&mut self, chain_id: ChainId) {
        todo!()
    }

    fn encode_for_signing(&self, out: &mut dyn alloy_rlp::BufMut) {
        todo!()
    }

    fn payload_len_for_signature(&self) -> usize {
        todo!()
    }

    fn into_signed(self, signature: Signature) -> Signed<Self, Signature>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl RlpEcdsaTx for TxExtended {
    const DEFAULT_TX_TYPE: u8 = 0;

    fn rlp_encoded_fields_length(&self) -> usize {
        todo!()
    }

    fn rlp_encode_fields(&self, out: &mut dyn alloy_rlp::BufMut) {
        todo!()
    }

    fn rlp_decode_fields(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        todo!()
    }
}
