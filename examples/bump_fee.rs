// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use std::{fs::File, io::Write};

use tonic_lnd::lnrpc::OutPoint;

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

    let info = client
        .wallet()
        // All calls require at least empty parameter
        .bump_fee(tonic_lnd::walletrpc::BumpFeeRequest {
            outpoint: Some(OutPoint {
                txid_bytes: b"".to_vec(),
                txid_str: "4f3e37bb0f351adc508f1b13f99bec200f2f0c8e0737ad86f696a76d2a348bf0".to_string(),
                output_index: 0,
            }),
            target_conf: 0,
            sat_per_byte: 0,
            force: false,
            sat_per_vbyte: 5,
        })
        .await
        .expect("failed to bump fee");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);

}