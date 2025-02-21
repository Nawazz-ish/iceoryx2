// Copyright (c) 2024 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod cli;
mod commands;
mod filter;

use clap::CommandFactory;
use clap::Parser;
use cli::Action;
use cli::Cli;
use iceoryx2_bb_log::{set_log_level, LogLevel};

#[cfg(debug_assertions)]
extern crate better_panic;

fn main() {
    #[cfg(not(debug_assertions))]
    {
        std::panic::set_hook(Box::new(|info| {
            eprintln!("Panic occurred: {:?}", info);
        }));
    }
    #[cfg(debug_assertions)]
    {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .install();
    }

    set_log_level(LogLevel::Warn);

    match Cli::try_parse() {
        Ok(cli) => {
            if let Some(action) = cli.action {
                match action {
                    Action::List(options) => {
                        if let Err(e) = commands::list(options.filter, cli.format) {
                            eprintln!("Failed to list nodes: {}", e);
                        }
                    }
                    Action::Details(options) => {
                        if let Err(e) = commands::details(options.node, options.filter, cli.format)
                        {
                            eprintln!("Failed to retrieve node details: {}", e);
                        }
                    }
                }
            } else {
                Cli::command().print_help().expect("Failed to print help");
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
