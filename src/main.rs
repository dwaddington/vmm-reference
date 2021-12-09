// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause
use std::convert::TryFrom;
use std::env;
use std::fs;
use api::Cli;
use vmm::Vmm;

use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream,UnixListener};
use std::thread;

fn main() {
    match Cli::launch(
        env::args()
            .collect::<Vec<String>>()
            .iter()
            .map(|s| s.as_str())
            .collect(),
    ) {
        Ok(vmm_config) => {
            let mut vmm =
                Vmm::try_from(vmm_config).expect("Failed to create VMM from configurations");
            // For now we are just unwrapping here, in the future we might use a nicer way of
            // handling errors such as pretty printing them.
            vmm.run().unwrap();

            loop {
                let rc = vmm.process_events();
                if rc == 1 {                    
                    break;
                }

                if std::path::Path::new("/tmp/sig").exists() == true {
                    let x = fs::remove_file("/tmp/sig");
                    println!("Collecting dirty bitmap info....");
                    vmm.async_collect_dirty();
                }

            }
            vmm.shutdown();

        }
        Err(e) => {
            eprintln!("Failed to parse command line options. {}", e);
        }
    }
}
