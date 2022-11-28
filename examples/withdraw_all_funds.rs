// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use tonic_lnd::lnrpc::{ChannelPoint, channel_point::FundingTxid};

#[tokio::main]
#[allow(deprecated)]
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

    let channels = client
        .lightning()
        // All calls require at least empty parameter
        .list_channels(tonic_lnd::lnrpc::ListChannelsRequest {
            active_only: false,
            inactive_only: false,
            public_only: false,
            private_only: false,
            peer: vec![],
        })
        .await
        .expect("failed to get all channels").into_inner();

    for channel in channels.channels {
        if let Some((funding_txid, output_index)) = channel.channel_point.split_once(':'){
            let close_chan_req = tonic_lnd::lnrpc::CloseChannelRequest {
                channel_point: Some(ChannelPoint {
                    funding_txid: Some(FundingTxid::FundingTxidStr(funding_txid.to_string())),
                    output_index: output_index.parse().expect("Expected to parse as u32"),
                }),
                force: false,
                target_conf: 0,
                sat_per_vbyte: 2,
                max_fee_per_vbyte: 10,
                delivery_address: "".to_string(),
            };
        
    
        // let close_chan_req = tonic_lnd::lnrpc::CloseChannelRequest;
    
            client
                .lightning()
                .close_channel(close_chan_req)
                .await
                .expect("expected to be able to close channels (active only)");
        }
    }

    let address = client
    .wallet()
    // All calls require at least empty parameter
    .next_addr(tonic_lnd::walletrpc::AddrRequest {
        account: "".to_string(),
        r#type: 1,
        change: false,
    })
    .await
    .expect("failed to get next address")
    .into_inner();

    let info = client
        .lightning()
        // All calls require at least empty parameter
        .send_coins(tonic_lnd::lnrpc::SendCoinsRequest {
            addr: address.addr,
            amount: 0,
            target_conf: 0,
            sat_per_byte: 0,
            sat_per_vbyte: 5,
            send_all: true,
            label: "Withdrawing all funds".to_string(),
            min_confs: 0,
            spend_unconfirmed: true,
        })
        .await
        .expect("failed to send all utxos");
        

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
