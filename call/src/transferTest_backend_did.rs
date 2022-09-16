use ic_cdk::export::candid::{self, CandidType, Deserialize};
use ic_cdk::api::call::CallResult;

pub type AccountIdentifier = Vec<u8>;
pub type BlockIndex__1 = u64;

#[derive(CandidType, Deserialize, Debug)]
pub struct Token { e8s: u64 }

pub type BlockIndex = u64;
#[derive(CandidType, Deserialize, Debug)]
pub enum TransferError {
    TxTooOld{ allowed_window_nanos: u64 },
    BadFee{ expected_fee: Token },
    TxDuplicate{ duplicate_of: BlockIndex },
    TxCreatedInFuture,
    InsufficientFunds{ balance: Token },
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Result { ok(BlockIndex__1), err(TransferError) }

type test = candid::Service;
struct SERVICE(candid::Principal);
impl SERVICE{
    pub async fn getSubAccount(&self) -> CallResult<(AccountIdentifier,)> {
        ic_cdk::call(self.0, "getSubAccount", ()).await
    }
    pub async fn transferOutICP(
        &self,
        arg0: AccountIdentifier,
        arg1: u64,
    ) -> CallResult<(Result,)> {
        ic_cdk::call(self.0, "transferOutICP", (arg0,arg1,)).await
    }
}
