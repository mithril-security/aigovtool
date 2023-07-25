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
use x509_parser::parse_x509_der;

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
MIIFazCCA1OgAwIBAgIUcXVJ7yAinVwU9YvGNQmGYmNKssAwDQYJKoZIhvcNAQEL
BQAwRTELMAkGA1UEBhMCQVUxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoM
GEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDAeFw0yMzA3MjUxMTU3NDlaFw0yNDA3
MjQxMTU3NDlaMEUxCzAJBgNVBAYTAkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEw
HwYDVQQKDBhJbnRlcm5ldCBXaWRnaXRzIFB0eSBMdGQwggIiMA0GCSqGSIb3DQEB
AQUAA4ICDwAwggIKAoICAQC6bw+3WkBwYnGK6Vkgwu3U0bCks07N4rHK8fKharXG
3NrLP14p6VJH4WnrVrfOdK8U3N5aRhbWLx9jplKiSFYHMJLMYk6wEu9aBWadAU+3
X4iKg3+55Xjdblb9BIp5dgJfbfQWF9a05No28HwFXp6Z+huJVE3kbwHc96E5IRGb
omGN2yyFtSv4oApJcaliOzyuh+UWFoNfGuHBhtFO0eEKN6h9OcwMgiMijS+3BmaX
mCa7ffhnvLlO0e/HM0IccilJRGMipLDSnpkKiDGqIEAE/qMfY0aw70hV8x/TJ5F3
5CNW+e/DnjJnlbK7Iv+wEHcb9UVcRNJ4hzC0lUWMx93VCl78swACxIQ1++fnbTMw
EHZSJzuyRTDnghoHISxGeNUcJF6ueykHyPmMIep7Qyi3uTfkjjXyzXmTtCx6Ax5w
VU1aCu678dazpEuTqARJiHS8LMvD6eD5VW8zPhdInPWRgFU+9d9SYyxBTuHYM8FN
n7cNMnEHfBveEA/9leEvU8oBwG8k/x20eqIBY4RTCHX/goULNm0NgODJeoklsxvh
A4Nn8oD0LX8vt9GKNJPBJIgs0iJSg2JBHCISa925GMiusCwMLTYfuEekcw7CDV75
NhHkvx5zZZlmDgxnzBgbGO847LPa9xDGkilvsOR4m7yDUMTP5Rhokl9Ix2XLGGlp
pwIDAQABo1MwUTAdBgNVHQ4EFgQUA4tp/FIvU2zOUteDT+UwHaMdiPcwHwYDVR0j
BBgwFoAUA4tp/FIvU2zOUteDT+UwHaMdiPcwDwYDVR0TAQH/BAUwAwEB/zANBgkq
hkiG9w0BAQsFAAOCAgEAhBNECpiDsVbXt2x5MaGkYkxCe4C0rfOiG0Zth+siJhfm
1MwDY4jcMHygoHtz4da/YfoxkqXGfyDAEHbwsA+f7XDl0Rv8SMVB1htvUbzl/FAf
DREZVui/jFO30Coh+PA6zL2W/RGl4XGQmfcHcrMj/fXz6TlCUjjyQhdU13tVqAkT
jCe8LEuHLjrAre1M5hEFqvPPRdCApBkNIv4+OKoP55MhCV6vBZCaO90DNq/elwwj
EemNIdOMcy9NwC1xV7EWdH0RNxcnbw50/XdP8ruVuREZkKaGeKddbW3xb7hsKjOo
8tFq9IqG1lDHBwQaCRIJne2fDRyq3fZjtQ8aVSnMJN8CKf3TWc194H5crCHt2ClI
p6V4hN0wOnh/WvxsF9T3zrHm4yrc65pZ7UBMWy/Nw9hMu/pxFSCFEF0qe0m2KiUy
X+TRRuxdIRrW4AKDXE5SDW0hjmt3Qhp3bJXd0ApyHNG7dq0hPyK5Nn7nr4FDbzN9
ROX3KIykteJlgD+0o+NKGRTBys18Tlz8f+9CDggYnSgUhBkltmczn9j/L/AcfK4y
KgopbLUKQTcpVofGLx4ShrlVJ5mue3Zx4OcjR7vm54OhOLSLnhvTaFJVbAFA97Tx
uKV7UO9aRTUWjDnttqPJMCmsE77DbFvuNl729PqTZ0s9mCSzcV/WMMzeX0NOdEQ=
-----END CERTIFICATE-----";

// fn request_consumption(inference_number: u32, root_store)

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
        DANGEROUS: All what is written here MUST be changed for production modes. 
        It is only to demonstrate that we can establish connection with a known DRM
        server for consumption tracking
    **/
    let mut root_store = rustls::RootCertStore::empty();
    let mut drm_certificate_pem = parse(DRM_CERT).unwrap(); 
    // let mut drm_certificate_der = pem_to_der(drm_certificate_pem).expect("X.509: decoding DER failed");
    let mut drm_certificate_der = parse_x509_der(&drm_certificate_pem.contents());
    println!("X.509 DRM certificate : {:?}", drm_certificate_der);
    let router_management = |request: &rouille::Request| {
        rouille::router!(request,
            (POST) (/upload) => {
                let reply = EXCHANGER.send_model(request);
                EXCHANGER.respond(request, reply)
            },

            (POST) (/delete) => {
                let reply = EXCHANGER.delete_model(request);
                EXCHANGER.respond(request, reply)
            },
            _ => rouille::Response::empty_404()
        )
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
