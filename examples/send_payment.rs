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
        .router()
        .send_payment_v2(tonic_lnd::routerrpc::SendPaymentRequest {
            payment_request: "lnbc100n1p3kj9fmpp5q7n2unptt0uekkf78pcw9ak3xzxyuvz5nhr5y2spskpa5j7p45jsdq4fpjkcmr0yp6xsetjv5srycqzpgxqyz5vqrzjq2tt9ke59l8c0655mxqh2l7lf5l9gk74em6fr86ckhfcmlwh806ujzaq85qqfasqqgqqqqqqqqqqqqgq9qsp5vmcmgyx5s5tkvxn5cuzpwvlafhs0c3vl4xw27qfyr4e6zy4nhpts9qyyssq44l6564kj70ue4dpngvtm453rzkutm7p4tpyujwaaer67pdd4ymys9yuvrlnp2jeet6mvl8encmjmhtpetse82h5hd69fhxk3e020scpgfurn7".to_string(),
            timeout_seconds: 10,
            fee_limit_sat: 10,
            allow_self_payment: true,
            amp: false,
        })
        .await
        .expect("expected to be able to batch open channels");



    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
