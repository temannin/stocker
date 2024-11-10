# Stocker

**Stocker** is a Rust-based tool designed to run on a Raspberry Pi. It listens for UPC (Universal Product Code) scans and automatically adds scanned items to your [AnyList](https://www.anylist.com/) grocery list via [Home Assistant](https://www.home-assistant.io/). This setup is ideal for quickly adding groceries or household items by simply scanning their barcodes, streamlining the restocking process with minimal effort.

## Features
- **UPC Scanning**: Listens for barcode scans and reads UPC codes from a connected USB scanner.
- **Integration with Home Assistant**: Communicates with Home Assistant to relay UPC data.
- **Automatic Addition to AnyList**: Uses Home Assistant’s API to send UPCs to AnyList for quick, convenient grocery list management.

## Requirements
- **Hardware**: Raspberry Pi with a connected USB barcode scanner.
- **Software**:
  - Rust (for building and customization)
  - Home Assistant with AnyList integration configured
  - Network access to Home Assistant instance

## Installation

### Prerequisites
1. **Install Rust** on your Raspberry Pi. Follow the [official Rust installation guide](https://www.rust-lang.org/learn/get-started).
2. **Install Home Assistant** with the AnyList integration enabled. Ensure you have API access to Home Assistant.
