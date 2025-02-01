use clap::Parser;
use std::net::{Ipv4Addr, SocketAddrV4};

use bureau::{Bureau, BureauConfig};
use wls::WlsOptions;

mod bureau;
mod wls;

#[derive(Parser)]
struct Args {
	/// If set, program will function in WLS mode.
	#[arg(short, long)]
	wls: bool,

	/// IP or Domain of the server.
	#[arg(long, default_value_t = ("127.0.0.1").into())]
	host_name: String,

	/// Maximum number of bureaus per wrl to create in WLS mode.
	#[arg(long, default_value_t = 3)]
	max_bureaus: usize,

	/// File path to a newline seperated list of wrls to allow in WLS mode.
	#[arg(long)]
	wrl_list: Option<String>,

	/// Bureau/WLS port.
	#[arg(short, long, default_value_t = 5126, value_parser = clap::value_parser!(u16).range(1..))]
	port: u16,

	/// Maximum number of users that each Bureau can have.
	#[arg(short, long, default_value_t = 256)]
	max_users: i32,

	/// Radius to add two users to each others aura.
	#[arg(short, long, default_value_t = 200.0)]
	aura_radius: f32,
}

fn main() {
	let args = Args::parse();

	let ip = Ipv4Addr::new(0, 0, 0, 0);
	let addr = SocketAddrV4::new(ip, args.port);

	let bureau_config = BureauConfig {
		connect_timeout: 10,
		max_users: 128,
		aura_radius: 200.0,
	};

	if args.wls {
		let err = wls::run(
			addr,
			WlsOptions {
				host_name: args.host_name,
				max_bureaus: args.max_bureaus,
				wrl_list: args.wrl_list,
				bureau_config,
			},
		)
		.unwrap_err();

		eprintln!("Error running WLS: {}", err);

		return;
	}

	Bureau::new(addr, bureau_config)
		.expect("failed to create bureau")
		.run();
}
