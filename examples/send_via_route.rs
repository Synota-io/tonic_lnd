// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use tonic_lnd::lnrpc::Route;

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
        .router()
        // All calls require at least empty parameter
        .send_to_route_v2(tonic_lnd::routerrpc::SendToRouteRequest {
            payment_hash: vec![],
            route: Some(Route { total_time_lock: 1000, total_fees: 100000, total_amt: 1000000, hops: todo!(), total_fees_msat: 100, total_amt_msat: 1000 }),
            skip_temp_err: true,
        })
        .await
        .expect("failed to get wallet balance");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
