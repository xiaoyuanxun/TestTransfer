use ic_agent::{ic_types::Principal, identity::Secp256k1Identity, Agent};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use hex;
use garcon::Delay;
use candid::{CandidType, Decode, Encode, Deserialize};

mod transferTest_backend_did;
use transferTest_backend_did::Result;

fn build_agent(pem_identity_path: &str) -> Agent {
    let url = "https://ic0.app".to_string();
    let identity = Secp256k1Identity::from_pem_file(String::from(pem_identity_path)).unwrap();
    let transport = ReqwestHttpReplicaV2Transport::create(url).expect("transport error");
    let agent = Agent::builder()
        .with_transport(transport)
        .with_identity(identity)
        .build()
        .expect("build agent error");
    agent
}

fn get_waiter() -> Delay {
    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();
    waiter
}

#[tokio::main]
async fn main() {
    let canister_id = Principal::from_text("jaxse-zaaaa-aaaan-qau5a-cai").unwrap();
    let agent = build_agent("identity.pem");
    let waiter = get_waiter();
    let account = hex::decode("3eee9b4671b8fde5a501288d74d21ee93042dc202104fa35051563ae35d24f2f").unwrap();
    let response_blob = agent
        .update(&canister_id, "transferOutICP")
        .with_arg(Encode!(&account, &(10000000 as u64)).expect("encode error"))
        .call_and_wait(waiter)
        .await
        .expect("response error");

    let response = Decode!(&response_blob, Result).unwrap();
    println!("{:?}", response);
}
