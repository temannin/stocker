use anylist;
use crossbeam::channel::Receiver;
use log::{error, info};
use upc;

use crossbeam::channel;
use evdev::Device;
use std::{io, thread};

use crate::{
    scan::scan,
    user_config::{self, load_config},
};

pub fn run(rx: Receiver<String>) {
    let config = user_config::load_config();

    // Continuously listen for jobs from the channel
    while let Ok(job) = rx.recv() {
        // Attempt to get the item from the UPC service
        info!("Received job: {:?}", job);
        match upc::get_item(&job, &config.upc_api_key) {
            Ok(item) => {
                // Add the item and check the success
                if let Err(e) = anylist::add_item(item) {
                    // Log failure to add item
                    error!("Failed to add item for job '{}': {}", job, e);
                } else {
                    info!("Successfully added item for job '{}'.", job);
                }
            }
            Err(e) => {
                // Log error fetching item from UPC service
                error!("Error processing job '{}': {}", job, e);
            }
        }
    }

    info!("No more jobs to process.");
}

pub fn start() -> io::Result<()> {
    let configuration = load_config();

    // Create a channel for job communication
    let (tx, rx) = channel::unbounded();

    // Try to open the device and handle the case if it fails
    let mut device = Device::open(&configuration.device_path).expect(&format!(
        "Failed to open device at path: {}",
        configuration.device_path
    ));

    info!("Listening for input events on {:?}", device.physical_path());

    // Start the worker thread
    let _worker_handle = thread::spawn(move || {
        run(rx); // Run the worker with the receiver
    });

    loop {
        // Call the scan function to get the complete sequence
        let sequence = scan(&mut device)?;
        if !sequence.is_empty() {
            tx.send(sequence).expect("Failed to process scan");
        }
    }
}
