use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let agora = Instant::now();
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        println!("Uso: ./scanner <ip> <porta_inicial> <porta_final>");
        return;
    }

    let ip_string = &args[1];
    let start_port: u16 = args[2].parse().expect("Porta inicial inválida");
    let end_port: u16 = args[3].parse().expect("Porta final inválida");

    println!("Iniciando scan em {} ({} até {})...", ip_string, start_port, end_port);
    
    let resultados = scanner(ip_string, start_port, end_port);

    println!("\n--- Relatório Final ---");
    println!("{:#?}", resultados);
    println!("Tempo total: {:?}", agora.elapsed());
}

fn scanner(ip_string: &str, start_port: u16, end_port: u16) -> HashMap<u16, String> {
    let mut result = HashMap::new();
    let ip_address: IpAddr = ip_string.parse().expect("IP inválido");

    let (tx, rx) = mpsc::channel();
    let ip_arc = Arc::new(ip_address);

    for port in start_port..=end_port {
        let tx_thread = tx.clone();
        let ip_thread = Arc::clone(&ip_arc);

        thread::spawn(move || {
            let addr = SocketAddr::new(*ip_thread, port);
            let timeout = Duration::from_millis(200);

            if TcpStream::connect_timeout(&addr, timeout).is_ok() {
                let _ = tx_thread.send(port); 
            }
        });
    }

    drop(tx);

    for porta_aberta in rx {
        println!("  [+] Porta {} está ABERTA", porta_aberta); 
        result.insert(porta_aberta, "Aberta".to_string());
    }

    result
}
