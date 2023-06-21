// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

use tonic_lnd::{lnrpc::{LightningAddress, payment}, routerrpc::SendPaymentRequest, invoicesrpc::{AddHoldInvoiceRequest, SettleInvoiceMsg, CancelInvoiceMsg}};

use hex::decode;
use rand::{distributions::Alphanumeric, Rng, RngCore};
use sha2::{Sha256, Digest};
use std::{convert::TryInto, thread::sleep, time::Duration};

pub fn create_nonce_32() -> [u8; 32] {
    let mut data: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut data);

    data
}

pub fn hash_256(data: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(data);

    hasher
        .finalize()
        .to_vec()
        .as_slice()
        .try_into()
        .expect("Expected to be able to run hash_256 on data provided")
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

    let secret_random_bytes = rand::thread_rng().gen::<[u8; 32]>();

    let hash = hash_256(secret_random_bytes.to_vec());

    let hodl_invoice = client
        .invoices()
        .add_hold_invoice(AddHoldInvoiceRequest {
            memo: "Hello there Hold Invoice".to_string(),
            hash: hash.to_vec(),
            value: 10,
            private: true,
        })
        .await
        .expect("Expected creation of hodl invoice to succeed");


    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("hodl_invoice {:#?}", hodl_invoice);


    // self-pay
    let mut payment_attempt = client
        .router()
        .send_payment_v2(tonic_lnd::routerrpc::SendPaymentRequest {
            payment_request: hodl_invoice.into_inner().payment_request,
            timeout_seconds: 10,
            fee_limit_sat: 10,
            allow_self_payment: true,
            amp: false,
        })
        .await
        .expect("expected to be able to pay invoice")
        .into_inner();

    let mut count = 0;

    let mut outer_fail_reason: String = "No reason found".to_string();

    while count < 1 {
        if let Some(response_from_payment_attempt) = payment_attempt.message().await.unwrap() {
            println!("message from sending payment: {response_from_payment_attempt:#?}");
            let payment_status = response_from_payment_attempt.status();

            if payment_status == payment::PaymentStatus::Succeeded {
                // mark as paid
                break;
            } else if payment_status == payment::PaymentStatus::Failed {
                println!("Payment failed");
                let optional_attempt_failure_reason =
                    tonic_lnd::lnrpc::payment::PaymentStatus::from_i32(
                        response_from_payment_attempt.failure_reason,
                    );

                if let Some(attempt_failure_reason) = optional_attempt_failure_reason {
                    outer_fail_reason = format!("{attempt_failure_reason:#?}");
                }
            } else {
                println!("current status: {payment_status:#?}");
            }
        } else {
            println!("There was supposed to be a response from making a payment - got None");
        }

        sleep(Duration::from_secs(1));

        count += 1;
        println!("attempt to pay count: {count}");
    }

    println!("Timed out or got reason: {outer_fail_reason}");

    // NOTE:
    // You CAN cancel a hodl invoice prior to it getting into an ACCEPTED state
    //
    //
    // let cancel_hodl_invoice = client
    //         .invoices()
    //         .cancel_invoice(CancelInvoiceMsg { payment_hash: hash.to_vec() })
    //         .await
    //         .expect("Expected to cancel hodl invoice to succeed");

    //     println!("cancel_hodl_invoice {:#?}", cancel_hodl_invoice);


    // wait for invoice to be marked as ACCEPTED on the receiver end
    // then settle or cancel
    sleep(Duration::from_secs(5));

    let should_settle = true;

    if should_settle {
        let settle_hodl_invoice = client
            .invoices()
            .settle_invoice(SettleInvoiceMsg { preimage: secret_random_bytes.to_vec() })
            .await
            .expect("Expected to settle hodl invoice to succeed");

        println!("settle_hodl_invoice {:#?}", settle_hodl_invoice);
    } else {
        let cancel_hodl_invoice = client
            .invoices()
            .cancel_invoice(CancelInvoiceMsg { payment_hash: hash.to_vec() })
            .await
            .expect("Expected to cancel hodl invoice to succeed");

        println!("cancel_hodl_invoice {:#?}", cancel_hodl_invoice);
    }
}










