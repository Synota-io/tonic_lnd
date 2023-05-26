// This example creates and saves to a file, the designated macaroon.
// This macaroon has all permissions to run additional services like
// loop and faraday.
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!
// Run xxd -r -p superadmin.macaroon > super.macaroon to get Binary file
use std::{fs::File, io::Write};
use tonic_lnd::lnrpc::{MacaroonPermission};

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

    let permissions = vec![
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/autopilotrpc.Autopilot/ModifyStatus".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/autopilotrpc.Autopilot/QueryScores".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/autopilotrpc.Autopilot/SetScores".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/autopilotrpc.Autopilot/Status".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/chainrpc.ChainKit/GetBestBlock".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/chainrpc.ChainKit/GetBlock".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/chainrpc.ChainKit/GetBlockHash".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/chainrpc.ChainNotifier/RegisterBlockEpochNtfn".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/chainrpc.ChainNotifier/RegisterConfirmationsNtfn".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/chainrpc.ChainNotifier/RegisterSpendNtfn".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/frdrpc.FaradayServer/ChannelInsights".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/frdrpc.FaradayServer/CloseReport".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/frdrpc.FaradayServer/ExchangeRate".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/frdrpc.FaradayServer/NodeAudit".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/frdrpc.FaradayServer/OutlierRecommendations".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/frdrpc.FaradayServer/RevenueReport".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/frdrpc.FaradayServer/ThresholdRecommendations".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/invoicesrpc.Invoices/AddHoldInvoice".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/invoicesrpc.Invoices/CancelInvoice".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/invoicesrpc.Invoices/LookupInvoiceV2".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/invoicesrpc.Invoices/SettleInvoice".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/invoicesrpc.Invoices/SubscribeSingleInvoice".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Accounts/CreateAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Accounts/ListAccounts".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Accounts/RemoveAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Accounts/UpdateAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Autopilot/AddAutopilotSession".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Autopilot/ListAutopilotFeatures".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Autopilot/ListAutopilotSessions".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Autopilot/RevokeAutopilotSession".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Firewall/ListActions".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Firewall/PrivacyMapConversion".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Proxy/GetInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Proxy/StopDaemon".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Sessions/AddSession".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Sessions/ListSessions".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/litrpc.Sessions/RevokeSession".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/AbandonChannel".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/AddInvoice".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/BakeMacaroon".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/BatchOpenChannel".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ChannelAcceptor".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ChannelBalance".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/CheckMacaroonPermissions".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/CloseChannel".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ClosedChannels".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ConnectPeer".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/DebugLevel".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/DecodePayReq".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/DeleteAllPayments".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/DeleteMacaroonID".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/DeletePayment".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/DescribeGraph".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/DisconnectPeer".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/EstimateFee".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ExportAllChannelBackups".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ExportChannelBackup".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/FeeReport".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ForwardingHistory".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/FundingStateStep".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/GetChanInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/GetInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/GetNetworkInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/GetNodeInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/GetNodeMetrics".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/GetRecoveryInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/GetTransactions".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ListAliases".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ListChannels".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ListInvoices".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ListMacaroonIDs".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ListPayments".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ListPeers".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ListPermissions".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/ListUnspent".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/LookupHtlcResolution".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/LookupInvoice".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/NewAddress".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/OpenChannel".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/OpenChannelSync".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/PendingChannels".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/QueryRoutes".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/RegisterRPCMiddleware".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/RestoreChannelBackups".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SendCoins".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SendCustomMessage".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SendMany".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SendPayment".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SendPaymentSync".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SendToRoute".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SendToRouteSync".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SignMessage".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/StopDaemon".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SubscribeChannelBackups".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SubscribeChannelEvents".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SubscribeChannelGraph".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SubscribeCustomMessages".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SubscribeInvoices".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SubscribePeerEvents".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/SubscribeTransactions".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/UpdateChannelPolicy".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/VerifyChanBackup".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/VerifyMessage".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/lnrpc.Lightning/WalletBalance".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/GetLiquidityParams".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/GetLoopInQuote".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/GetLoopInTerms".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/GetLsatTokens".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/ListSwaps".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/LoopIn".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/LoopOut".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/LoopOutQuote".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/LoopOutTerms".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/Monitor".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/Probe".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/SetLiquidityParams".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/SuggestSwaps".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/looprpc.SwapClient/SwapInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/neutrinorpc.NeutrinoKit/AddPeer".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/neutrinorpc.NeutrinoKit/DisconnectPeer".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/neutrinorpc.NeutrinoKit/GetBlock".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/neutrinorpc.NeutrinoKit/GetBlockHash".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/neutrinorpc.NeutrinoKit/GetBlockHeader".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/neutrinorpc.NeutrinoKit/GetCFilter".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/neutrinorpc.NeutrinoKit/IsBanned".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/neutrinorpc.NeutrinoKit/Status".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/peersrpc.Peers/UpdateNodeAnnouncement".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/AccountModificationFees".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/AuctionFee".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/BatchSnapshot".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/BatchSnapshots".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/BumpAccountFee".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/CancelOrder".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/CancelSidecar".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/CloseAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/DecodeSidecarTicket".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/DepositAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/ExpectSidecarChannel".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/GetInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/GetLsatTokens".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/InitAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/LeaseDurations".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/Leases".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/ListAccounts".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/ListOrders".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/ListSidecars".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/NextBatchInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/NodeRatings".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/OfferSidecar".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/QuoteAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/QuoteOrder".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/RecoverAccounts".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/RegisterSidecar".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/RenewAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/StopDaemon".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/SubmitOrder".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/poolrpc.Trader/WithdrawAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/BuildRoute".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/EstimateRouteFee".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/GetMissionControlConfig".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/HtlcInterceptor".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/QueryMissionControl".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/QueryProbability".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/ResetMissionControl".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/SendPayment".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/SendPaymentV2".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/SendToRoute".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/SendToRouteV2".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/SetMissionControlConfig".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/SubscribeHtlcEvents".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/TrackPayment".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/TrackPaymentV2".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/TrackPayments".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/UpdateChanStatus".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/routerrpc.Router/XImportMissionControl".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/ComputeInputScript".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/DeriveSharedKey".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/MuSig2Cleanup".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/MuSig2CombineKeys".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/MuSig2CombineSig".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/MuSig2CreateSession".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/MuSig2RegisterNonces".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/MuSig2Sign".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/SignMessage".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/SignOutputRaw".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/signrpc.Signer/VerifyMessage".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/verrpc.Versioner/GetVersion".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/BumpFee".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/DeriveKey".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/DeriveNextKey".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/EstimateFee".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/FinalizePsbt".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/FundPsbt".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ImportAccount".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ImportPublicKey".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ImportTapscript".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/LabelTransaction".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/LeaseOutput".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ListAccounts".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ListAddresses".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ListLeases".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ListSweeps".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ListUnspent".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/NextAddr".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/PendingSweeps".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/PublishTransaction".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/ReleaseOutput".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/RequiredReserve".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/SendOutputs".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/SignMessageWithAddr".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/SignPsbt".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/walletrpc.WalletKit/VerifyMessageWithAddr".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/watchtowerrpc.Watchtower/GetInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/wtclientrpc.WatchtowerClient/AddTower".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/wtclientrpc.WatchtowerClient/GetTowerInfo".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/wtclientrpc.WatchtowerClient/ListTowers".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/wtclientrpc.WatchtowerClient/Policy".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/wtclientrpc.WatchtowerClient/RemoveTower".to_string(),
        },
        MacaroonPermission{
            entity: "uri".to_string(),
            action: "/wtclientrpc.WatchtowerClient/Stats".to_string(),
        },
    ];

    let macaroon = client
        .lightning()
        // All calls require at least empty parameter
        .bake_macaroon(tonic_lnd::lnrpc::BakeMacaroonRequest {
            permissions,
            root_key_id: 18441921392371827006,
            allow_external_permissions: false,
            
        })
        .await
        .expect("failed to get info").into_inner();

    // Run xxd -r -p superadmin.macaroon > super.macaroon to get Binary file
    let mut file = File::create("superadmin.macaroon").expect("Should be a file");
    _ = file.write_all(macaroon.macaroon.as_bytes());
}
