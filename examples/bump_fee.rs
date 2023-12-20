// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use tonic_lnd::lnrpc::{OutPoint};

use rand::{RngCore};

pub fn create_nonce_32() -> [u8; 32] {
    let mut data: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut data);

    data
}

//https://stackoverflow.com/questions/66428638/how-do-i-reverse-a-hex-string
pub fn read_le_u8(input: Vec<u8>) -> Vec<u8> {
    let mut bytes_reversed = Vec::new();
    for i in input.iter().rev() {
        bytes_reversed.push(*i);
    }
    bytes_reversed
}

pub fn calculated_bump_fee_rate(previous_fee_rate: f64, targeted_fee_rate: f64) -> u64 {
    ((targeted_fee_rate - (192.25 / 301.0 * previous_fee_rate)) / (109.0/301.0)).ceil() as u64
}

#[tokio::main]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let address = args.next().expect("missing arguments: address, cert file, macaroon file");
    let cert_file = args.next().expect("missing arguments: cert file, macaroon file");
    let macaroon_file = args.next().expect("missing argument: macaroon file");
    let address = address.into_string().expect("address is not UTF-8");

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_lnd::connect(address, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    let txid_str = "TXID".to_string();
    let transaction_bytes = hex::decode(&txid_str).expect("should get bytes");
    let le_transaction_bytes = read_le_u8(transaction_bytes);
    let fee_rate_to_bump_with = calculated_bump_fee_rate(12.6, 150.0);

    let bump_fee_req = tonic_lnd::walletrpc::BumpFeeRequest {
        outpoint: Some(OutPoint {
            txid_bytes: le_transaction_bytes,
            txid_str: "".to_string(),
            output_index: 1,
        }),
        force: false,
        target_conf: 0,
        sat_per_vbyte: fee_rate_to_bump_with,
        sat_per_byte: 0,
    };

    // let close_chan_req = tonic_lnd::lnrpc::CloseChannelRequest;

    let info = client
        .wallet()
        .bump_fee(bump_fee_req)
        .await
        .expect("expected to be able to bump fee on tx");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
