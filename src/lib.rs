use core::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use std::sync::Arc;

#[cfg(feature = "proto")]
use proto_rs::proto_message;
use solana_address::Address;
use solana_hash::Hash;
use solana_keypair::Keypair;
use solana_message::{AddressLookupTableAccount, Instruction};
use wincode::{SchemaRead, SchemaWrite};

#[cfg(feature = "client")]
pub const API_KEY_HEADER: &str = "key";

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "client")]
pub trait TxnAuthInterceptor: Send + Sync + 'static + Sized {
    type Payload;
    fn intercept<T>(
        payload: Self::Payload,
        req: &mut tonic::Request<T>,
    ) -> Result<(), tonic::Status>;
}

#[cfg(feature = "client")]
impl TxnAuthInterceptor for String {
    type Payload = String;

    fn intercept<T>(
        payload: Self::Payload,
        req: &mut tonic::Request<T>,
    ) -> Result<(), tonic::Status> {
        use tonic::metadata::{Ascii, MetadataValue};

        let payload: MetadataValue<Ascii> = MetadataValue::try_from(payload)
            .map_err(|_| tonic::Status::failed_precondition("bad AURA api key"))?;
        req.metadata_mut().append(API_KEY_HEADER, payload);
        Ok(())
    }
}

#[cfg_attr(
    feature = "proto",
    proto_message(proto_path = "live_protos/aura_sender_types.proto")
)]
#[derive(SchemaWrite, SchemaRead)]
pub struct TxnData {
    pub signers: ArhivedSigners,
    pub meta: TxnMeta,
    pub kind: ArchivedTxnKind,
    pub slot: u64,
}

impl ArhivedSigners {
    pub fn new(signers: Vec<Keypair>, payer_idx: u8, tip_payer_idx: u8) -> Self {
        Self {
            signers: signers.into_iter().map(|x| *x.secret_bytes()).collect(),
            payer_idx,
            tip_payer_idx,
        }
    }
}

impl TryFrom<ArhivedSigners> for Signers {
    fn try_from(value: ArhivedSigners) -> Result<Self, Self::Error> {
        let len = value.signers.len();
        let payer = value.payer_idx as usize;
        let tip_payer = value.tip_payer_idx as usize;
        if payer >= len {
            return Err("Bad payer idx");
        }
        if tip_payer >= len {
            return Err("Bad tip_payer idx");
        }
        Ok(Self {
            signers: value
                .signers
                .into_iter()
                .map(Keypair::new_from_array)
                .collect(),
            payer: value.payer_idx as usize,
            tip_payer: value.tip_payer_idx as usize,
        })
    }

    type Error = &'static str;
}

pub struct Signers {
    pub signers: Vec<Keypair>,
    pub payer: usize,
    pub tip_payer: usize,
}

#[cfg_attr(
    feature = "proto",
    proto_message(proto_path = "live_protos/aura_sender_types.proto")
)]
#[derive(SchemaWrite, SchemaRead)]
pub struct ArhivedSigners {
    signers: Vec<[u8; 32]>,
    pub payer_idx: u8,
    pub tip_payer_idx: u8,
}

#[cfg_attr(
    feature = "proto",
    proto_message(proto_path = "live_protos/aura_sender_types.proto")
)]
#[derive(SchemaWrite, SchemaRead)]
pub struct TxnMeta {
    pub include_dont_front: bool,
    pub max_cu: Option<NonZeroU32>,
    pub max_loaded_data: Option<NonZeroU32>,
    pub max_tip: Option<NonZeroU64>,
    pub max_fee: Option<NonZeroU64>,
    pub max_total: Option<NonZeroU64>,
    pub insts: Vec<Instruction>,
    pub txn_priorities: Vec<TxnPriorityStrat>,
    pub procs: SendProcessors,
    pub recent_blockhash: Hash,
}

#[cfg_attr(
    feature = "proto",
    proto_message(proto_path = "live_protos/aura_sender_types.proto")
)]
#[derive(SchemaWrite, SchemaRead, Clone)]
pub struct AuraMeta {
    pub main_endpoint: bool,
    pub revert_endpoint: bool,
}

#[cfg_attr(
    feature = "proto",
    proto_message(proto_path = "live_protos/aura_sender_types.proto")
)]
#[derive(SchemaWrite, SchemaRead)]
pub struct SendProcessors {
    pub jito_validators: bool,
    pub jito_bundled: bool,
    pub aura: AuraMeta,
    pub bloxroute: bool,
    pub nozomi: bool,
    pub next_block: bool,
    pub slot0: bool,
    pub astra: bool,
    pub block_razor: bool,
    pub node1: bool,
    pub helius: bool,
    pub stellium: bool,
    pub soyas: bool,
    pub falcon: bool,
    pub raiden: bool,
    pub circular: bool,
    pub flash_block: bool,
    pub moon: bool,
    pub blocksprint: bool,
}
impl SendProcessors {
    pub fn is_some(&self) -> bool {
        self.jito_validators
            || self.jito_bundled
            || self.aura.main_endpoint
            || self.aura.revert_endpoint
            || self.bloxroute
            || self.nozomi
            || self.next_block
            || self.slot0
            || self.astra
            || self.block_razor
            || self.node1
            || self.helius
            || self.stellium
            || self.soyas
            || self.falcon
            || self.circular
            || self.raiden
            || self.flash_block
            || self.moon
            || self.blocksprint
    }
    pub fn number(&self) -> usize {
        let mut num_procs = 0;

        if self.jito_validators {
            num_procs += 1
        }
        if self.aura.main_endpoint || self.aura.revert_endpoint {
            num_procs += 1
        }
        if self.bloxroute {
            num_procs += 1
        }
        if self.nozomi {
            num_procs += 1
        }
        if self.next_block {
            num_procs += 1
        }
        if self.slot0 {
            num_procs += 1
        }
        if self.astra {
            num_procs += 1
        }
        if self.block_razor {
            num_procs += 1
        }
        if self.node1 {
            num_procs += 1
        }
        if self.helius {
            num_procs += 1
        }

        if self.helius {
            num_procs += 1
        }

        if self.soyas {
            num_procs += 1;
        }
        if self.falcon {
            num_procs += 1;
        }
        if self.circular {
            num_procs += 1;
        }
        if self.raiden {
            num_procs += 1;
        }
        if self.flash_block {
            num_procs += 1;
        }

        if self.moon {
            num_procs += 1;
        }
        if self.blocksprint {
            num_procs += 1;
        }
        num_procs
    }
}

#[cfg_attr(
    feature = "proto",
    proto_message(proto_path = "live_protos/aura_sender_types.proto")
)]
#[derive(SchemaWrite, SchemaRead)]
pub enum TxnPriorityStrat {
    Exact,
    BpsFeeAndExactTip(NonZeroU16),
    BpsFeeFromTotal(NonZeroU16),
    MinTipAndFeeRest,
    MinTipAndExactFee,
}

#[cfg_attr(
    feature = "proto",
    proto_message(proto_path = "live_protos/aura_sender_types.proto")
)]
#[derive(SchemaWrite, SchemaRead)]
pub enum ArchivedTxnKind {
    Legacy,
    Versioned(Vec<ArchivedAddressLookupTableAccount>),
}

pub enum TxnKindOwned {
    Legacy,
    Versioned(Vec<AddressLookupTableAccount>),
}

impl From<ArchivedTxnKind> for TxnKindOwned {
    fn from(value: ArchivedTxnKind) -> Self {
        match value {
            ArchivedTxnKind::Legacy => Self::Legacy,
            ArchivedTxnKind::Versioned(v) => {
                Self::Versioned(v.into_iter().map(Into::into).collect())
            }
        }
    }
}

#[cfg_attr(
    feature = "proto",
    proto_message(proto_path = "live_protos/aura_sender_types.proto")
)]
#[derive(SchemaWrite, SchemaRead, Debug, PartialEq, Eq, Clone)]
pub struct ArchivedAddressLookupTableAccount {
    pub key: Address,
    pub addresses: Vec<Address>,
}
impl From<ArchivedAddressLookupTableAccount> for AddressLookupTableAccount {
    fn from(value: ArchivedAddressLookupTableAccount) -> Self {
        Self {
            key: value.key,
            addresses: value.addresses,
        }
    }
}

pub enum TxnKind<'a> {
    Legacy,
    Versioned(&'a [AddressLookupTableAccount]),
}

pub struct TxnVersionedOneTable(AddressLookupTableAccount);

impl TxnVersionedOneTable {
    pub fn new(key: Address, addresses: Vec<Address>) -> Self {
        Self(AddressLookupTableAccount { key, addresses })
    }
}

impl<'a> TxnKindExt<'a> for TxnVersionedOneTable {
    fn compile(&'a self) -> TxnKind<'a> {
        TxnKind::Versioned(std::slice::from_ref(&self.0))
    }
}

pub trait TxnKindExt<'a> {
    fn compile(&'a self) -> TxnKind<'a>;
}

impl<'a> TxnKindExt<'a> for TxnKind<'a> {
    fn compile(&'a self) -> TxnKind<'a> {
        match self {
            TxnKind::Legacy => TxnKind::Legacy,
            TxnKind::Versioned(address_lookup_table_accounts) => {
                TxnKind::Versioned(address_lookup_table_accounts)
            }
        }
    }
}

impl<'a> TxnKindExt<'a> for Arc<AddressLookupTableAccount> {
    fn compile(&'a self) -> TxnKind<'a> {
        TxnKind::Versioned(std::slice::from_ref(self.as_ref()))
    }
}
impl<'a> TxnKindExt<'a> for Vec<AddressLookupTableAccount> {
    fn compile(&'a self) -> TxnKind<'a> {
        TxnKind::Versioned(self.as_slice())
    }
}

impl<'a> TxnKindExt<'a> for Arc<Vec<AddressLookupTableAccount>> {
    fn compile(&'a self) -> TxnKind<'a> {
        TxnKind::Versioned(&self[..])
    }
}

impl<'a> TxnKindExt<'a> for Arc<[AddressLookupTableAccount]> {
    fn compile(&'a self) -> TxnKind<'a> {
        TxnKind::Versioned(&self[..])
    }
}

impl<'a> TxnKindExt<'a> for AddressLookupTableAccount {
    fn compile(&'a self) -> TxnKind<'a> {
        TxnKind::Versioned(std::slice::from_ref(self))
    }
}

impl<'a> TxnKindExt<'a> for () {
    fn compile(&'a self) -> TxnKind<'a> {
        TxnKind::Legacy
    }
}

impl<'a> TxnKindExt<'a> for TxnKindOwned {
    fn compile(&'a self) -> TxnKind<'a> {
        match self {
            TxnKindOwned::Legacy => TxnKind::Legacy,
            TxnKindOwned::Versioned(v) => TxnKind::Versioned(v.as_slice()),
        }
    }
}
