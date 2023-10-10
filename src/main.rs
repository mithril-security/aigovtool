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

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::result::Result::Ok;
mod identity;
mod model;
mod model_store;
use crate::client_communication::Exchanger;
use anyhow::Error;
// use anyhow::Ok;
use anyhow::Result;
use model_store::ModelStore;
mod client_communication;
use lazy_static::lazy_static;
use log::debug;
mod telemetry;
mod ureq_dns_resolver;
use rouille::Response;
use telemetry::Telemetry;
use ureq::OrAnyStatus;
use crate::ureq_dns_resolver::InternalAgent;
use crate::ureq_dns_resolver::fixed_resolver;

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

    pub static ref NAME_UUID_HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
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

// ----------------------------------------------------------------

// Enclave ready request
const DRM_ADDRESS_READY: &str = "https://localhost:7000";

const DRM_IP_READY: &str = "127.0.0.1";

const DRM_PORT_READY: &str = "7000";

// Consumption tracking 
const DRM_ADDRESS: &str = "https://localhost:6000";

const DRM_IP: &str = "127.0.0.1";

const DRM_PORT: &str = "6000";

const DRM_ROOT_CA : &'static str = "-----BEGIN CERTIFICATE-----
MIIDIzCCAgugAwIBAgIULR+skivFs9d++XkdEIARmQu05KwwDQYJKoZIhvcNAQEL
BQAwITELMAkGA1UEBhMCRlIxEjAQBgNVBAMMCWxvY2FsaG9zdDAeFw0yMzA3Mjgx
MTEzNDlaFw0yNjA1MTcxMTEzNDlaMCExCzAJBgNVBAYTAkZSMRIwEAYDVQQDDAls
b2NhbGhvc3QwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQCxiN6jeWlA
akrN2t2RTqyScpAeBE4TiFO2TbFWZOHSwx7QOHDnxbMqKzWm9KpKYccFb7CRfu/7
2MNVGqvyuCoxi1QPCvELy6Rf7teNh8LkAFm0DRPyXyMIHvdNV+YFB7bUf8YMlCrg
0fnqHEd9iVKuXuGg6O7Va9QoSbJb7+gi8M3jfocxdjPlzZX587k/w854tBqaZIJ9
7aUdL4jbDVgCU6uArf/NCk3af3L3UEwl2ZyhbRcn4lXLKaOYTJxeZZ3BzzIdWDKo
qolN97jvqW81xDOMTXW5MM4oUBpFpASWOGhmC+1rn0oS2Qrzm0xda3E//QsErkay
rdPPV6q/Sk+7AgMBAAGjUzBRMB0GA1UdDgQWBBTSwuA9bCTPiWWbdVpTM18qEydu
vjAfBgNVHSMEGDAWgBTSwuA9bCTPiWWbdVpTM18qEyduvjAPBgNVHRMBAf8EBTAD
AQH/MA0GCSqGSIb3DQEBCwUAA4IBAQChH9xwkqkbydAjUKjkQSrf8xU5aGx4UOKh
Kq2mR5P7K3JKqF3Q48s4w/RF5kGoz+63XiMxYytv9Dgn2YJL2K1Kw9LrKw2QUVay
nXSXMYsTZP8+n+olYnhD5qyWS/PWmywbrHYrxmwxieYv727xaKIV9E/95FlIVz8s
8lx3DCJM6fGPSl5PlPj0oHhJRbDJ/Qf1sE8k+8jnD8LPthO1poomcKH3oqcJDCGl
wCLUFz3yqNrV2oKDuMxtDbFfAAEwQL0ZPp6+YVL+xyT6LeD2EXRslooY2dcNmzUL
EMn1WQ7BCNwlZTfnkkN6soi4ZKCZMaavcPH6vRoqHCGqUyQHsrI4
-----END CERTIFICATE-----";

const DRM_CERT: &'static str = "-----BEGIN CERTIFICATE-----
MIIDnDCCAoSgAwIBAgIUQ391hUDaIeqJPp9/l0ALKGmUqPowDQYJKoZIhvcNAQEL
BQAwITELMAkGA1UEBhMCRlIxEjAQBgNVBAMMCWxvY2FsaG9zdDAeFw0yMzA3Mjgx
MTE0MjVaFw0yNjA1MTcxMTE0MjVaMGsxCzAJBgNVBAYTAkZSMRQwEgYDVQQIDAtJ
bGVkZWZyYW5jZTEOMAwGA1UEBwwFUGFyaXMxHDAaBgNVBAoME21pdGhyaWwtY2Vy
dGlmaWNhdGUxGDAWBgNVBAMMD2xvY2FsaG9zdC5sb2NhbDCCASIwDQYJKoZIhvcN
AQEBBQADggEPADCCAQoCggEBAO87YzZzynoDYXQ+JSL5nFYoXc11NAkyRsnEw5Ye
CxiXBBk2gZn1Ta949nhWnqdXMWz68K/SUWn8Df3BVy8eYR2O1EQY5RNlKfBXG75S
lLq1U8h7q9d7X26uNEbztQfU8ZKeTkw8cTkhR/maxkkBuBJr0od8ZKCr/U0KoMFH
tEvzgRot2XcMKmt5ftmOI9N9VNgT3ESsoTcu61qAWYuQ9WcLDXCyh0Q3wa72FHnL
0rBURA6zuPqoY3RY9x175yruBhgHQ9laJ66kyMKHnV03dqrL+ZtrAqCMfWqlcRBV
AbaaurJMVMa1BE62FXuNAcnsu3x2Mx1on/5tNTeeeMT8ydkCAwEAAaOBgTB/MB8G
A1UdIwQYMBaAFNLC4D1sJM+JZZt1WlMzXyoTJ26+MAkGA1UdEwQCMAAwCwYDVR0P
BAQDAgTwMCUGA1UdEQQeMByCCWxvY2FsaG9zdIIPbG9jYWxob3N0LmxvY2FsMB0G
A1UdDgQWBBTQmgQ8D/iqyqWEycqGTGfFJFnjZTANBgkqhkiG9w0BAQsFAAOCAQEA
M1pgbHIV5C0K7C1z/2zaGXILVsTxdm5eX9OshqXWn6YOVk5Daloy2RzQMiWedQZT
oqZ0+L9ImAd+w1YK3p1ACqWb9nw/EjhDwxgSdfebEwU97v3bJPt9wBRs7luAD3zk
HDSDJe7oxHueTeCGKgeGJ9uoO21+bsDMkPePqpyIlbbfzRcX9MSQpas0GcyAnyYj
Ez/3tBykFXsKiDKMYFt5DHQKL843hwN5fSNtDXYyh19EeChfxcnhbZL35qsTSjZT
QzmNtCVuE8fuySqT7WOdf70LUOoonu6diXUrufg4kxmcE1+Z/yVfHf4upXyUAm5M
78rBGYvykqo3p92JpCdpww==
-----END CERTIFICATE-----";

#[derive(Serialize, Deserialize, Debug)]
pub struct InferencesTracking {
    inferences: String
}


fn request_consumption(inference_number: u32, ip: &str, port: &str, arc_tls_config: &Arc<rustls::ClientConfig>) -> Result<InferencesTracking> {
    let inference_req = &inference_number.to_string();
    let agent = ureq::builder()
        .tls_config(arc_tls_config.clone())
        .resolver(fixed_resolver::FixedResolver(format!("{ip}:{port}").parse().unwrap()))
        .build();
    // let response = agent.post(&format!("{DRM_ADDRESS}/request_consumption"))
    //     .send_form(&[("number_inferences", inference_req)])?;
    thread::sleep(Duration::from_secs(2));
    match agent.post(&format!("{DRM_ADDRESS}/request_consumption"))
    .send_form(&[("number_inferences", inference_req)]) {
        Ok(response) => {
            let content_response = response.into_json()?; 
            Ok(content_response)
        }, 
        Err(ureq::Error::Status(500, response)) | Err(ureq::Error::Status(502, response) 
        | ureq::Error::Status(404, response)) => {
            let retry: Option<u64> = response.header("retry-after").and_then(|h| h.parse().ok());
            let retry = retry.unwrap_or(5);
            eprintln!("{} for {}, retry in {}", response.status(), response.get_url(), retry);
            thread::sleep(Duration::from_secs(retry));
            Ok(InferencesTracking { inferences : "Connection Retry.".to_string()})
        },
        Err(_) => {
            println!("The DRM server isn't responsive any longer/Disconnected.");
            Ok(InferencesTracking { inferences : "Connection Lost.".to_string()})
        }
        // Err(_) => {
        //     println!("The DRM server isn't responsive any longer/Disconnected.");
        //     Ok(InferencesTracking { inferences : "I/O transport error.".to_string()})
        // }
    }

}

fn request_model_consumed(ip: &str, port: &str, arc_tls_config: &Arc<rustls::ClientConfig>) -> Result<InferencesTracking> {
    let agent = ureq::builder()
        .tls_config(arc_tls_config.clone())
        .resolver(fixed_resolver::FixedResolver(format!("{ip}:{port}").parse().unwrap()))
        .build();
    let response = agent.post(&format!("{DRM_ADDRESS}/consume_model"))
        .send_form(&[("run_model", "requested")]);
    if let Err(e) = response {
        log::debug!("Cannot contact DRM server: {}", e);
        Ok(InferencesTracking { inferences : "Connection Lost.".to_string()})
    }
    else {
        Ok(response?.into_json()?)
    }
}
    

fn request_inferences_left(ip: &str, port: &str, arc_tls_config: &Arc<rustls::ClientConfig>) -> Result<InferencesTracking> {
    let agent = ureq::builder()
    .tls_config(arc_tls_config.clone())
    .resolver(fixed_resolver::FixedResolver(format!("{ip}:{port}").parse().unwrap()))
    .build();
    
    let response = agent.get(&format!("{DRM_ADDRESS}/request_consumption"))
    .call();
    if let Err(e) = response {
        log::debug!("Cannot contact DRM server. Connection lost: {}", e);
        Ok(InferencesTracking { inferences : "Connection Lost.".to_string()})
    }
    else {
        Ok(response?.into_json()?)
    }
}

fn send_ready_request(ip: &str, port: &str, arc_tls_config: &Arc<rustls::ClientConfig>) -> Result<String> {
    let agent = ureq::builder()
    .tls_config(arc_tls_config.clone())
    .resolver(fixed_resolver::FixedResolver(format!("{ip}:{port}").parse().unwrap()))
    .build();

    let response = agent.get(&format!("{DRM_ADDRESS_READY}/enclave_ready"))
    .call()?
    .into_json()?;

    Ok(response)
}

// ----------------------------------------------------------------


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
    /*
     * IN DEVELOPMENT : 
        DANGEROUS: All what is written here MUST be changed for production modes. 
        It is only to demonstrate that we can establish connection with a known DRM
        server for consumption tracking
    */
    let mut root_store = rustls::RootCertStore::empty();
    let  drm_certificate_pem = parse(DRM_CERT).unwrap(); 
    let root_ca_pem = parse(DRM_ROOT_CA).unwrap();
    // let mut drm_certificate_der = pem_to_der(drm_certificate_pem).expect("X.509: decoding DER failed");
    let drm_certificate_der = parse_x509_certificate(&drm_certificate_pem.contents()).unwrap();
    // let mut root_ca_der = parse_x509_certificate(&root_ca_pem.contents()).unwrap();
    println!("X.509 DRM certificate : {:?}", drm_certificate_der);
    let drm_certificate = drm_certificate_pem.contents().to_vec();
    let root_certificate = root_ca_pem.contents().to_vec();
    root_store.add_parsable_certificates(&[root_certificate]);
    root_store.add_parsable_certificates(&[drm_certificate]);

    let tls_config = rustls::ClientConfig::builder()
    .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    #[derive(Serialize)]
    struct DrmStatus {
        outputs: String,
    }
    let tls_config = Arc::new(tls_config);
    
    let router_management = {
        let arc_tls_config_clone = Arc::clone(&tls_config);
        move |request: &rouille::Request| {

        rouille::router!(request,
            (POST) (/upload) => {
                let reply = EXCHANGER.send_model(request);
                // add request handle to send the number of inferences needed
                println!("reply is : {:?}", reply);
                EXCHANGER.respond(request, reply)
            },

            (POST) (/delete) => {
                let reply = EXCHANGER.delete_model(request);
                EXCHANGER.respond(request, reply)
            },

            (POST) (/drm-status) => {
                println!("DRM Server running and connected.");
                println!("Requesting 5 Inferences.");

                let request_consumption = request_consumption(5, DRM_IP, DRM_PORT, &arc_tls_config_clone);
                println!("Consumption requested : {:?}", request_consumption);
                rouille::Response::json(&DrmStatus {outputs : "status up received by the Inference server.".to_string()})
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

    let router = {
        let arc_tls_config_clone = Arc::clone(&tls_config);

        move |request: &rouille::Request| {
        rouille::router!(request,
            (POST) (/run) => {
                // TODO: add condition to verify the drm number of requests
                println!("Running model. Verifying the number of inferences left.");
                let inference_left = request_inferences_left(DRM_IP, DRM_PORT, &arc_tls_config_clone).unwrap();
                println!("/run ; {:?}", inference_left.inferences.parse::<u32>().unwrap());

                if inference_left.inferences.parse::<u32>().unwrap() > 0 {
                    let _ = request_model_consumed(DRM_IP, DRM_PORT, &arc_tls_config_clone).unwrap();
                    let reply = EXCHANGER.run_model(request);
                    EXCHANGER.respond(request, reply)
                } 
                else {
                    // let drm_status = DrmStatus {outputs : "No inferences left available.".to_string()};
                    println!("No inferences available left. Requesting new number of inferences.");

                    println!("Requesting Inferences.");

                    let request_consumption = request_consumption(6, DRM_IP, DRM_PORT, &arc_tls_config_clone);
                    println!("Consumption requested : {:?}", request_consumption);
                    let reply = EXCHANGER.run_model(request);
                    EXCHANGER.respond(request, reply)
                    // rouille::Response::json(&DrmStatus {status : "No inferences left available.".to_string()})
                }
            },
            (GET) (/request-models) => {
                println!("Requesting available models.");
                let reply = EXCHANGER.get_models();
                EXCHANGER.respond(request, reply)

            }, 
            (GET) (/inferences-left) => {
                println!("Requesting the number of inferences left.");
                let inference_left = request_inferences_left(DRM_IP, DRM_PORT, &arc_tls_config_clone).unwrap();
                println!("Inferences left : {:?}", inference_left);
                rouille::Response::json(&inference_left)
            },
            _ => rouille::Response::empty_404()
        )
        }
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

    // Sending event to guardian
    let arc_tls_config_clone = Arc::clone(&tls_config);
    let drm_management_response = send_ready_request(DRM_IP_READY, DRM_PORT_READY, &arc_tls_config_clone);


    // Emit the telemetry `Started` event
    telemetry::add_event(telemetry::TelemetryEventProps::Started {}, None, None);
    _unattested_handle.join().unwrap();



    Ok(())
}
