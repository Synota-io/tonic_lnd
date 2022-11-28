// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use tonic_lnd::{lnrpc::{LightningAddress, ListPeersResponse}, routerrpc::SendPaymentRequest};

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

        // let peer = LightningAddress {
        //     host: "34.79.58.84:9735".to_string(),
        //     pubkey: "0296b2db342fcf87ea94d981757fdf4d3e545bd5cef4919f58b5d38dfdd73bf5c9".to_string(),
        // };

        // let peer = LightningAddress {
        //     host: "fulcrum-sandbox.m.voltageapp.io:9735".to_string(),
        //     pubkey: "039819ecdcfd3bd78ef7574db028a4d6ea13acd5f45a903b9e85c3d9d2b76a6e96".to_string(),
        // };

        // Supplier sandbox routing
        let peer = LightningAddress {
            host: "34.214.14.213:9735".to_string(),
            pubkey: "03cdf2b73fe7f6d92eaa8f2a10992e8a9df6f1aaae8fd748f62c5a93608a1c2f24".to_string(),
        };

        let connected_peers: ListPeersResponse = client
            .lightning()
            .list_peers(tonic_lnd::lnrpc::ListPeersRequest {
                latest_error: true,
            })
            .await
            .expect("failed to list peers")
            .into_inner();

            println!("connected peers: {:#?}", connected_peers);

            let connected_peer_pubkeys: Vec<String> = connected_peers
                .peers
                .iter()
                .map(|already_connected_peer| already_connected_peer.pub_key.clone())
                .collect();

            if !connected_peer_pubkeys.contains(&peer.pubkey) {
                let info = client
                    .lightning()
                    // All calls require at least empty parameter
                    .connect_peer(tonic_lnd::lnrpc::ConnectPeerRequest {
                        addr: Some(peer),
                        perm: false,
                        timeout: 6000,
                    })
                    .await
                    .expect("failed to connect to peer");

                    println!("connected to peer! {:#?}", info);
            } else {
                println!("Already connected to peer with pubkey: {}", peer.pubkey);
            }

}
