import Array "mo:base/Array";
import Account "Account";
import Blob  "mo:base/Blob";
import Principal "mo:base/Principal";
import Prim "mo:⛔";
import Types "Types";
import Result "mo:base/Result";

shared(installer) actor class test()  = this {

    type BlockIndex = Types.BlockIndex;
    type TransferError = Types.TransferError;
    type AccountIdentifier= Types.AccountIdentifier;

    let ledger : Types.Ledger = actor("ryjl3-tyaaa-aaaaa-aaaba-cai");
    let LEDGER_TRANSFER_FEE = 10_000 : Nat64;

    public query({caller}) func getSubAccount(): async AccountIdentifier {
        Account.accountIdentifier(
            Principal.fromActor(this),
            Blob.fromArray(Account.principalToSubAccount(caller))
        )
    };

    public shared({caller}) func transferOutICP(to: AccountIdentifier, amount: Nat64): async Result.Result<BlockIndex, TransferError> {
        let subaccount = Blob.fromArray(Account.principalToSubAccount(caller));
        switch(await ledger.transfer({
            to = to;
            fee = { e8s = LEDGER_TRANSFER_FEE };
            memo = 0;
            from_subaccount = ?subaccount;
            amount = { e8s = amount };
            created_at_time = null;
        })) {
            case(#Ok(block_height)){ #ok(block_height) };
            case(#Err(e)){ #err(e) };
        }
    };

};