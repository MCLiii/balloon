use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use std::mem;

#[repr(C, packed)]  // C layout, no padding
#[derive(Debug, Clone, Copy)]
struct TelemetryPacket {
    sync: u64,
    timestamp: u64,
    temperature: f32,
    pressure: f32,
    humidity: f32,
    altitude: f32,
    latitude: f32,
    longitude: f32,
    status: u8,
}

impl TelemetryPacket {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            sync: 0xFF_FF_FF_FF_FF_FF_FF_FF,
            timestamp: now,
            temperature: rng.gen_range(-40.0..=60.0), // Temperature in Celsius
            pressure: rng.gen_range(800.0..=1200.0),  // Pressure in hPa
            humidity: rng.gen_range(0.0..=100.0),     // Humidity percentage
            altitude: rng.gen_range(0.0..=50000.0),   // Altitude in meters
            latitude: rng.gen_range(-90.0..=90.0),    // Latitude in degrees
            longitude: rng.gen_range(-180.0..=180.0), // Longitude in degrees
            status: rng.gen_range(0..=255),           // Status byte
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                mem::size_of::<Self>()
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target_addr = "127.0.0.1:3000";
    
    println!("Starting telemetry packet generator...");
    println!("Sending packets to: {}", target_addr);
    
    loop {
        let packet = TelemetryPacket::new();
        let bytes = packet.as_bytes();
        
        match socket.send_to(bytes, target_addr) {
            Ok(bytes_sent) => {
                println!("Sent telemetry packet ({} bytes): {:?}", bytes_sent, packet);
                println!("Packet size: {} bytes", mem::size_of::<TelemetryPacket>());
            }
            Err(e) => {
                eprintln!("Failed to send packet: {}", e);
            }
        }
        
        // Send a packet every 2 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}