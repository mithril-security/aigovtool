use crate::connection::Connection;
use crate::util::refined_tcp_stream::Stream as RefinedStream;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;

/// A wrapper around an owned Rustls connection and corresponding stream.
///
/// Uses an internal Mutex to permit disparate reader & writer threads to access the stream independently.
pub(crate) struct RustlsStream(
    Arc<Mutex<rustls::StreamOwned<rustls::ServerConnection, Connection>>>,
);

impl RustlsStream {
    pub(crate) fn peer_addr(&mut self) -> std::io::Result<Option<SocketAddr>> {
        self.0
            .lock()
            .expect("Failed to lock SSL stream mutex")
            .sock
            .peer_addr()
    }

    pub(crate) fn shutdown(&mut self, how: Shutdown) -> std::io::Result<()> {
        self.0
            .lock()
            .expect("Failed to lock SSL stream mutex")
            .sock
            .shutdown(how)
    }
}

impl Clone for RustlsStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Read for RustlsStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .expect("Failed to lock SSL stream mutex")
            .read(buf)
    }
}

impl Write for RustlsStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .expect("Failed to lock SSL stream mutex")
            .write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0
            .lock()
            .expect("Failed to lock SSL stream mutex")
            .flush()
    }
}

pub(crate) struct RustlsContext(Arc<rustls::ServerConfig>);

impl RustlsContext {
    pub(crate) fn from_pem(
        certificates: Vec<u8>,
        private_key: Zeroizing<Vec<u8>>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let certificate_chain: Vec<rustls::Certificate> =
            rustls_pemfile::certs(&mut certificates.as_slice())?
                .into_iter()
                .map(|bytes| rustls::Certificate(bytes))
                .collect();

        if certificate_chain.is_empty() {
            return Err("Couldn't extract certificate chain from config.".into());
        }

        let private_key = rustls::PrivateKey({
            let pkcs8_keys = rustls_pemfile::pkcs8_private_keys(
                &mut private_key.clone().as_slice(),
            )
            .expect("file contains invalid pkcs8 private key (encrypted keys are not supported)");

            if let Some(pkcs8_key) = pkcs8_keys.first() {
                pkcs8_key.clone()
            } else {
                let rsa_keys = rustls_pemfile::rsa_private_keys(&mut private_key.as_slice())
                    .expect("file contains invalid rsa private key");
                rsa_keys[0].clone()
            }
        });

        let tls_conf = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certificate_chain, private_key)?;

        Ok(Self(Arc::new(tls_conf)))
    }

    /// Create a RustlsContext from DER encoded certificate chain and private key
    ///
    /// `certificates` is certificate chain encoded as a vector of DER-encoded certificates.
    /// `private_key` is the corresponding DER-encoded RSA, ECDSA, or Ed25519 private key.
    pub(crate) fn from_der(
        certificates: Vec<Vec<u8>>,
        private_key: Zeroizing<Vec<u8>>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let certificate_chain: Vec<rustls::Certificate> = certificates
            .into_iter()
            .map(|bytes| rustls::Certificate(bytes))
            .collect();

        if certificate_chain.is_empty() {
            return Err("Certificate chain cannot be empty.".into());
        }
        // Beware we have to extract the private from zeroize wrapper
        // to pass it to rustls, rustls does (yet) not do zeroisation of the secret key
        // Relevant GH issue : <https://github.com/rustls/rustls/issues/265>
        // Not really relevant in our case : zeroization would add almost no security for us.
        // Rationale : Zeroization only add protection to data that is no longer being used.
        // The private key needs to be used for the entire lifetime of the attested server
        // which is the same lifetime as the enclave.
        let private_key = rustls::PrivateKey(private_key.to_vec());

        let tls_conf = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certificate_chain, private_key)?;

        Ok(Self(Arc::new(tls_conf)))
    }

    pub(crate) fn accept(
        &self,
        stream: Connection,
    ) -> Result<RustlsStream, Box<dyn Error + Send + Sync + 'static>> {
        let connection = rustls::ServerConnection::new(self.0.clone())?;
        Ok(RustlsStream(Arc::new(Mutex::new(
            rustls::StreamOwned::new(connection, stream),
        ))))
    }
}

impl From<RustlsStream> for RefinedStream {
    fn from(stream: RustlsStream) -> Self {
        Self::Https(stream)
    }
}
