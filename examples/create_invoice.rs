// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use tonic_lnd::{lnrpc::LightningAddress, routerrpc::SendPaymentRequest};

use hex::decode;
use rand::{distributions::Alphanumeric, Rng, RngCore};

pub fn create_nonce_32() -> [u8; 32] {
    let mut data: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut data);

    data
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

    // let info = client
    //     .lightning()
    //     // All calls require at least empty parameter
    //     .connect_peer(tonic_lnd::lnrpc::ConnectPeerRequest {
    //         addr: Some(LightningAddress {
    //             host: "34.79.58.84:9735".to_string(),
    //             pubkey: "0296b2db342fcf87ea94d981757fdf4d3e545bd5cef4919f58b5d38dfdd73bf5c9".to_string(),
    //         }),
    //         perm: false,
    //         timeout: 6000,
    //     })
    //     .await
    //     .expect("failed to connect to peer");

    let info = client
        .lightning()
        .add_invoice(tonic_lnd::lnrpc::InvoiceRequest {
            memo: "Hello there 2".to_string(),
            value_msat: 10000,
            private: true,
            is_keysend: false,
            is_amp: false,
        })
        .await
        .expect("expected to be able to batch open channels");



    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
