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

#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

#[cfg(debug_assertions)]
extern crate better_panic;

mod cli;
mod commands;

use clap::CommandFactory;
use clap::Parser;

fn main() {
    #[cfg(not(debug_assertions))]
    {
        setup_panic!();
    }
    #[cfg(debug_assertions)]
    {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .install();
    }

    let cli = cli::Cli::parse();

    if cli.list {
        if let Err(e) = commands::list() {
            eprintln!("Failed to list commands: {}", e);
        }
    } else if cli.paths {
        if let Err(e) = commands::paths() {
            eprintln!("Failed to list search paths: {}", e);
        }
    } else if !cli.external_command.is_empty() {
        let command_name = &cli.external_command[0];
        let command_args = if cli.external_command.len() > 1 {
            Some(&cli.external_command[1..])
        } else {
            None
        };
        if let Err(e) = commands::execute_external_command(command_name, command_args, cli.dev) {
            eprintln!("Failed to execute command: {}", e);
        }
    } else {
        cli::Cli::command()
            .print_help()
            .expect("Unrecognized CLI input. Exiting.");
    }
}
