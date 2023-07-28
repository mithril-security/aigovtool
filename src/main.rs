// Copyright 2022 Mithril Security. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![forbid(unsafe_code)]

use std::sync::Arc;
use std::thread;
mod identity;
mod model;
mod model_store;
use crate::client_communication::Exchanger;
use anyhow::Result;
use model_store::ModelStore;
mod client_communication;
use lazy_static::lazy_static;
use log::debug;
mod telemetry;
mod ureq_dns_resolver;
use telemetry::Telemetry;

// ra
use env_logger::Env;
use ring::digest;
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use sgx_isa::{Report, Targetinfo};

use rustls;
use std::fs::File;
use std::io::Read;
use pem::parse;
use x509_parser::pem::pem_to_der;
use x509_parser::parse_x509_certificate;

#[derive(Serialize)]
struct GetQuoteRequest {
    enclave_report: Report,
}

#[derive(Serialize)]
struct GetCollateralRequest {
    quote: Vec<u8>,
}

lazy_static! {
    static ref EXCHANGER: Arc<Exchanger> = Arc::new(Exchanger::new(
        Arc::new(ModelStore::new()),
        1_000_000_000,
        1_000_000,
    ));
    pub static ref TELEMETRY_CHANNEL: Arc<Telemetry> = Arc::new(Telemetry::new().unwrap());
}

// "Native" Rust type for sgx_ql_qve_collateral_t
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SgxCollateral {
    pub version: u32,                  // version = 1.  PCK Cert chain is in the Quote.
    pub pck_crl_issuer_chain: String,  // PCK CRL Issuer Chain in PEM format
    pub root_ca_crl: String,           // Root CA CRL in PEM format
    pub pck_crl: String,               // PCK Cert CRL in PEM format
    pub tcb_info_issuer_chain: String, // PEM
    pub tcb_info: String,              // TCB Info structure
    pub qe_identity_issuer_chain: String, // PEM
    pub qe_identity: String,           // QE Identity Structure
    pub pck_certificate: String,       // PCK certificate in PEM format
    pub pck_signing_chain: String,     // PCK signing chain in PEM format
}

// Consumption tracking 
const DRM_ADDRESS: &str = "https://127.0.0.1:6000";
const DRM_CERT: &'static str = "-----BEGIN CERTIFICATE-----
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
mWcerMrV8WCIoyG17iVTeTQUXJL9+h1CC/Vt
-----END CERTIFICATE-----";

fn request_consumption(inference_number: u32, arc_tls_config: &Arc<rustls::ClientConfig>) -> Result<String> {
    let inference_req = &inference_number.to_string();
    let agent = ureq::builder()
        .tls_config(arc_tls_config.clone())
        .build();
    let response = agent.post(&format!("{DRM_ADDRESS}/request_consumption"))
        .send_form(&[("number_inferences", inference_req)])?
        .into_json()?;
    Ok(response)
}

fn request_model_consumed(arc_tls_config: Arc<rustls::ClientConfig>) -> Result<String> {

    let mut agent = ureq::builder()
        .tls_config(arc_tls_config)
        .build();
    let response = agent.post(&format!("{DRM_ADDRESS}/consume_model"))
        .call()?
        .into_json()?;
    Ok(response)

}

const RUNNER_ADDRESS: &str = "http://127.0.0.1:11000";

fn get_target_info() -> Result<Targetinfo> {
    Ok(ureq::post(&format!("{RUNNER_ADDRESS}/get_target_info"))
        .call()?
        .into_json()?)
}

fn get_quote(report: Report) -> Result<Vec<u8>> {
    Ok(ureq::post(&format!("{RUNNER_ADDRESS}/get_quote"))
        .send_json(GetQuoteRequest {
            enclave_report: report,
        })?
        .into_json()?)
}

fn get_collateral(quote: &[u8]) -> Result<SgxCollateral> {
    Ok(ureq::post(&format!("{RUNNER_ADDRESS}/get_collateral"))
        .send_json(GetCollateralRequest {
            quote: quote.to_vec(),
        })?
        .into_json()?)
}

fn main() -> Result<()> {
    println!("Starting BlindAI server...");

    // Setup TELEMETRY
    let telemetry_disabled = TELEMETRY_CHANNEL.is_disabled();
    let telemetry_disabled_string = format!(
        "BlindAI telemetry is {}",
        if telemetry_disabled {
            "disabled"
        } else {
            "enabled"
        }
    );

    println!("{telemetry_disabled_string}");

    const SERVER_NAME: &str = if cfg!(target_env = "sgx") {
        "blindai"
    } else {
        "blindai mock (testing)"
    };

    // Make debugging easier by enabling rust backtrace inside enclave
    std::env::set_var("RUST_BACKTRACE", "full");
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    #[cfg(not(debug_assertions))]
    env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();

    let certificate_with_secret = identity::create_tls_certificate()?;
    let enclave_cert_der = Arc::new(certificate_with_secret.serialize_der()?);
    let enclave_private_key_der = certificate_with_secret.serialize_private_key_der();

    fn respond(x: &(impl Serialize + ?Sized)) -> rouille::Response {
        match serde_cbor::to_vec(&x) {
            Ok(ser_data) => rouille::Response::from_data("application/cbor", ser_data),
            Err(e) => rouille::Response::from_data(
                "application/cbor",
                serde_cbor::to_vec(&format!("{:?}", &e)).unwrap(),
            )
            .with_status_code(500),
        }
        .with_additional_header("Server", SERVER_NAME)
    }

    // Remote attestation
    // Connecting to the runner

    // Enclave held data hash
    let report_binding = digest::digest(&digest::SHA256, &enclave_cert_der);
    let mut report_data = [0u8; 64];
    report_data[0..32].copy_from_slice(report_binding.as_ref());

    cfg_if::cfg_if! {
        if #[cfg(target_env = "sgx")] {
            let target_info = get_target_info()?;
            debug!("target info = {:?} ", &target_info);
            let report = Report::for_target(&target_info, &report_data);

            let quote = get_quote(report)?;
            debug!("Attestation : Quote is {:?} ", &quote);

            let collateral = get_collateral(&quote)?;
            debug!("Attestation : Collateral is {:?} ", collateral);

            let router = {
                let enclave_cert_der = Arc::clone(&enclave_cert_der);
                move |request: &rouille::Request| {
                    rouille::router!(request,
                        (GET)(/) => {
                            debug!("Requested enclave TLS certificate");
                            respond(Bytes::new(&enclave_cert_der))
                        },
                        (GET)(/quote) => {
                            debug!("Attestation : Sending quote to client.");
                            respond(Bytes::new(&quote))
                        },
                        (GET)(/collateral) => {
                            debug!("Attestation : Sending collateral to client.");
                            respond(&collateral)
                        },
                        _ => {
                            rouille::Response::empty_404()
                        },
                    )
                }
            };
        } else {
            let router = {
                let enclave_cert_der = Arc::clone(&enclave_cert_der);
                move |request: &rouille::Request| {
                    rouille::router!(request,
                        (GET)(/) => {
                            debug!("Requested enclave TLS certificate");
                            respond(Bytes::new(&enclave_cert_der))
                        },
                        _ => {
                            rouille::Response::empty_404()
                        },
                    )
                }
            };
        }
    };

    let unattested_server =
        rouille::Server::new("0.0.0.0:9923", router).expect("Failed to start unattested server");

    let (_unattested_handle, _unattested_sender) = unattested_server.stoppable();
    // set up connection to the DRM 
    /**
     * IN DEVELOPMENT : 
        DANGEROUS: All what is written here MUST be changed for production modes. 
        It is only to demonstrate that we can establish connection with a known DRM
        server for consumption tracking
    **/
    let mut root_store = rustls::RootCertStore::empty();
    let mut drm_certificate_pem = parse(DRM_CERT).unwrap(); 
    // let mut drm_certificate_der = pem_to_der(drm_certificate_pem).expect("X.509: decoding DER failed");
    let mut drm_certificate_der = parse_x509_certificate(&drm_certificate_pem.contents()).unwrap();
    println!("X.509 DRM certificate : {:?}", drm_certificate_der);
    let mut drm_certificate = drm_certificate_pem.contents().to_vec();
    root_store.add_parsable_certificates(&[drm_certificate]);

    let tls_config = rustls::ClientConfig::builder()
    .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    #[derive(Serialize)]
    struct DrmStatus {
        status: String,
    }
    let tls_config = Arc::new(tls_config);
    let router_management = {
        let arc_tls_config_clone = Arc::clone(&tls_config);
        move |request: &rouille::Request| {

        rouille::router!(request,
            (POST) (/upload) => {
                let reply = EXCHANGER.send_model(request);
                // add request handle to send the number of inferences needed

                EXCHANGER.respond(request, reply)
            },

            (POST) (/delete) => {
                let reply = EXCHANGER.delete_model(request);
                EXCHANGER.respond(request, reply)
            },

            (POST) (/drm-status) => {
                println!("DRM Server running and connected.");
                println!("Requesting 1000 Inferences.");

                let request_consumption = request_consumption(1000, &arc_tls_config_clone);
                println!("Consumption requested : {:?}", request_consumption);
                rouille::Response::json(&DrmStatus {status : "status up received by the Inference server.".to_string()})
            },
            _ => rouille::Response::empty_404()
        )
        }
    };

    thread::spawn({
        let enclave_cert_der_s = enclave_cert_der.to_vec();
        let priv_der = enclave_private_key_der.clone();
        move || {
            let management_server = rouille::Server::new_ssl(
                "0.0.0.0:9925",
                router_management,
                tiny_http::SslConfig::Der(tiny_http::SslConfigDer {
                    certificates: vec![enclave_cert_der_s],
                    private_key: priv_der.clone(),
                }),
            )
            .expect("Failed to start management server");

            let (_management_handle, _management_sender) = management_server.stoppable();
            _management_handle.join().unwrap();
        }
    });

    println!("Models can be managed on 0.0.0.0:9925");

    let router = move |request: &rouille::Request| {
        rouille::router!(request,
            (POST) (/run) => {
                // TODO: add condition to verify the drm number of requests
                let reply = EXCHANGER.run_model(request);
                EXCHANGER.respond(request, reply)
            },
            _ => rouille::Response::empty_404()
        )
    };

    thread::spawn({
        let enclave_cert_der = Arc::clone(&enclave_cert_der);
        move || {
            let attested_server = rouille::Server::new_ssl(
                "0.0.0.0:9924",
                router,
                tiny_http::SslConfig::Der(tiny_http::SslConfigDer {
                    certificates: vec![enclave_cert_der.to_vec()],
                    private_key: enclave_private_key_der,
                }),
            )
            .expect("Failed to start trusted server")
            .pool_size(8);
            let (_trusted_handle, _trusted_sender) = attested_server.stoppable();
            _trusted_handle.join().unwrap();
        }
    });

    println!("BlindAI server is running on the ports 9923 and 9924 for run and 9925 for management");

    // Emit the telemetry `Started` event
    telemetry::add_event(telemetry::TelemetryEventProps::Started {}, None, None);
    _unattested_handle.join().unwrap();



    Ok(())
}
