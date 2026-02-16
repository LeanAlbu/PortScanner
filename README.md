# 🦀 Rust Parallel Port Scanner

A high-performance, multithreaded TCP port scanner built with Rust. This project explores safe concurrency, message-passing architectures, and systems programming principles.

## 🚀 Overview
Developed by a **Computer Science student at UFT**, this tool was built to demonstrate how Rust's ownership model and strict type system can be leveraged to build fast and memory-safe networking utilities without the risks of data races.

## 🛠️ Technical Highlights
- **Safe Concurrency**: Utilizes `std::thread` for parallel scanning, significantly reducing execution time.
- **Arc (Atomic Reference Counting)**: Thread-safe sharing of the target IP address across multiple worker threads.
- **MPSC Channels**: Implements a "Multi-Producer, Single-Consumer" architecture to gather results asynchronously without needing complex mutex locks.
- **Timeout Management**: Fine-tuned `TcpStream` connections with specific `Duration` windows to handle firewalled or filtered ports efficiently.
- **Zero-Cost Abstractions**: Leveraging Rust's performance to match C++ while maintaining memory safety.



## ⚡ Performance
By moving from a sequential scan to a multithreaded approach, the scanner can verify hundreds of ports in a fraction of a second. 

| Method | Range | Time (Avg) |
| :--- | :--- | :--- |
| Sequential | 1 - 1000 | ~200.0s (worst case) |
| **Multithreaded** | **1 - 1000** | **~200ms** |

## 📦 Usage
Ensure you have the Rust toolchain installed.

```bash
# Clone the repository
git clone [https://github.com/LeanAlbu/PortScanner.git](https://github.com/LeanAlbu/PortScanner.git)

# Build and run
cargo run -- <TARGET_IP> <START_PORT> <END_PORT>

# Example
cargo run -- 127.0.0.1 1 1000
