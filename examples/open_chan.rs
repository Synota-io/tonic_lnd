// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use tonic_lnd::lnrpc::LightningAddress;

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
        .batch_open_channel(tonic_lnd::lnrpc::BatchOpenChannelRequest {
            channels: vec![
                tonic_lnd::lnrpc::BatchOpenChannel {
                    node_pubkey: decode("0296b2db342fcf87ea94d981757fdf4d3e545bd5cef4919f58b5d38dfdd73bf5c9".to_string()).unwrap(),
                    local_funding_amount:21000,
                    push_sat:1000,
                    private:true,
                    min_htlc_msat:10,
                    remote_csv_delay:0,
                    close_address:"bc1qrkr9mp2vr3jxas5csrlg32nerpapq6w07ntcna".to_string(),
                    pending_chan_id: create_nonce_32().to_vec(),
                    commitment_type: 2,
                },
                tonic_lnd::lnrpc::BatchOpenChannel {
                    node_pubkey: decode("0296b2db342fcf87ea94d981757fdf4d3e545bd5cef4919f58b5d38dfdd73bf5c9".to_string()).unwrap(),
                    local_funding_amount:21000,
                    push_sat:1000,
                    private:true,
                    min_htlc_msat:10,
                    remote_csv_delay:0,
                    close_address:"bc1qrkr9mp2vr3jxas5csrlg32nerpapq6w07ntcna".to_string(),
                    pending_chan_id: create_nonce_32().to_vec(),
                    commitment_type: 2,
                },
            ],
            target_conf: 0,
            sat_per_vbyte: 2,
            min_confs: 4,
            spend_unconfirmed: false,
            label: "hello opening many channels".to_string(),
        })
        .await
        .expect("expected to be able to batch open channels");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
