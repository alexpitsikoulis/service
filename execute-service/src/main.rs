// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use execute_service::*;

use snarkvm::prelude::Process;

use structopt::StructOpt;
use warp::Filter;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    network: String,
    #[structopt(short, long, default_value = "8081")]
    port: u16,
}

async fn run(port: u16) {
    pretty_env_logger::init();

    let routes = execute_route().with(warp::trace(
        |info| tracing::debug_span!("Debugging headers", headers = ?info.request_headers()),
    ));

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    match opt.network.as_str() {
        "mainnet" => {
            PROCESS.with(|process| {
                *process.borrow_mut() = Some(ProcessVariant::MainnetV0(
                    Process::load().expect("Failed to load mainnet process"),
                ));
            });
        }
        "testnet" => {
            PROCESS.with(|process| {
                *process.borrow_mut() = Some(ProcessVariant::TestnetV0(
                    Process::load().expect("Failed to load testnet process"),
                ));
            });
        }
        _ => panic!("Invalid network"),
    }
    run(opt.port).await;
}
