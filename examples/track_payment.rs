// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use std::{time::Duration, thread::sleep};

use hex::decode;
use tonic_lnd::lnrpc;

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

    let mut payment_attempt = client
        .router()
        // All calls require at least empty parameter
        .track_payment_v2(tonic_lnd::routerrpc::TrackPaymentRequest {
            payment_hash: decode("9dcf4a169cb5b20e0837470be8b728829a10b5967e6e486110c8b996eb37b490").expect("should be bytes"),
            no_inflight_updates: false,
        })
        .await
        .expect("failed to get wallet balance").into_inner().message().await.expect("result");

    // let mut count = 0;
    // let mut outer_fail_reason: String = "No reason found".to_string();

    // while count < 10 {
    //     if let Some(response_from_payment_attempt) = payment_attempt.message().await.expect("Should respond") {
    //         println!("message from sending payment: {response_from_payment_attempt:#?}");
    //         let payment_status = response_from_payment_attempt.status();

    //         if payment_status == lnrpc::payment::PaymentStatus::Succeeded
    //             || payment_status == lnrpc::payment::PaymentStatus::InFlight
    //         {
    //             println!("Payment succeeded");
    //         } else if payment_status == lnrpc::payment::PaymentStatus::Failed {
    //             println!("Payment failed");
    //             let optional_attempt_failure_reason =
    //                 tonic_lnd::lnrpc::payment::PaymentStatus::from_i32(
    //                     response_from_payment_attempt.failure_reason,
    //                 );

    //             if let Some(attempt_failure_reason) = optional_attempt_failure_reason {
    //                 outer_fail_reason = format!("{attempt_failure_reason:#?}");
    //             }
    //         } else {
    //             println!("current status: {payment_status:#?}");
    //         }
    //     } else {
    //         println!("There was supposed to be a response from making a payment - got None");
    //     }

    //     sleep(Duration::from_secs(1));

    //     count += 1;
    //     println!("attempt to pay count: {count}");
    // }

    // println!("Timed out or got reason: {outer_fail_reason}");
    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", payment_attempt);
}
