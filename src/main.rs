use std::str::FromStr;
use spinoff::{Spinner, Spinners};
use subxt::{Error, OnlineClient, PolkadotConfig, SubstrateConfig};
use sp_core_hashing::blake2_256;
use parity_scale_codec as codec;
use sp_core::H256;
use sp_keyring::AccountKeyring;
use subxt::events::StaticEvent;
use subxt::ext::sp_runtime::AccountId32;
use subxt::tx::{PairSigner};
use subxt::ext::sp_core::{Decode, Encode, Pair as PairTrait};
use subxt::ext::sp_core::sr25519::Pair;
//use subxt::ext::sp_runtime::app_crypto::sr25519::AppPair;

// return sp_keyring::sr25519::Pair -> another similar type, Implement Pair trait to it.

const CONTRACT_ID: &str ="0xfac951deb3a238c9e7346c8fe81bfcd083dbed15e0ed2d6181abd3899d9a227c";


#[tokio::main]
async fn main() {
    // Instatiating OnlineClient object
    let client = OnlineClient::<PolkadotConfig>::from_url("wss://poc5.phala.network:443/ws").await.unwrap();
    // Calling contract
    let contract_id = AccountId32::from_str(CONTRACT_ID).unwrap();
    // call data
    // It contains message name, parameters for create_applicant_profile
    let mut call_data = Vec::<u8>::new();
    call_data.append(&mut (&blake2_256("0xC0DE0002".as_bytes())).to_vec());
    call_data.append(&mut codec::Encode::encode(&(
        String::from("MrishoLukamba"),
        None::<AccountId32>,
        String::from("Meow"),
        None::<Vec<AccountId32>>
    )));

    // getter for applicant
    let mut get_data = Vec::<u8>::new();
    get_data.append(&mut (&blake2_256("0xC0DE1001".as_bytes())).to_vec());


    let hashed_id = H256::from_str(CONTRACT_ID).unwrap();
    println!("Hashed  Id:{:?}",hashed_id);

    let test_create_applicant_call = test_phala::tx().phala_fat_contracts()
        .push_contract_message(hashed_id,call_data,100);

    let test_get_applicant_profile = test_phala::tx().phala_fat_contracts()
        .push_contract_message(hashed_id,get_data,100);

    let acc_pair: Pair = Pair::from_string("//Mrisho",None)
        .expect("formating is a bitch");

    println!("Public-Key: {}",acc_pair.public());
    let signer = PairSigner::<PolkadotConfig,Pair>::new(acc_pair);

    let pre_result = client.tx()
        .sign_and_submit_then_watch_default(&test_create_applicant_call,&signer)
        .await.expect("send failed");

    let progress =  pre_result.wait_for_finalized_success().await.unwrap();
    //let events = progress.find().collect::<Vec<Result<test_event, Error>>>();
    let events = progress.iter().collect::<Vec<_>>();
    let value = events[2].as_ref().unwrap();
    println!("{:?}",value.event_metadata());


}

#[derive(Encode, Decode, Debug)]
pub struct test_event;

impl StaticEvent for test_event{
    const PALLET: &'static str = "PhalaFatContracts";
    const EVENT: &'static str = "ApplicantAccountCreated";
}

#[subxt::subxt(runtime_metadata_path = "testmetadata.scale")]
pub mod test_phala{}
