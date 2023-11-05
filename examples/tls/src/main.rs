use anyhow::Result;
use snowboard::{
	native_tls::{Identity, TlsAcceptor},
	response, Server,
};

use std::fs;

fn main() -> Result<()> {
	let der = fs::read("identity.pfx")?;
	let password = "1234";
	let tls_acceptor = TlsAcceptor::new(Identity::from_pkcs12(&der, password)?)?;

	Server::new("localhost:3000", tls_acceptor)?
		.run(|request| response!(ok, format!("{:?}", request)))
}
