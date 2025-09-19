use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use std::mem;

// Conditional imports for ARM Linux (Raspberry Pi)
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
use rppal::i2c::I2c;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mod i2c;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
use i2c::MPU6050::{MPU6050, MotionReading};


#[repr(C, packed)]  // C layout, no padding
#[derive(Debug, Clone, Copy)]
struct TelemetryPacket {
    sync: u64,
    timestamp: u64,
    temperature: f32,
    humidity: f32,
    altitude: f32,
    latitude: f32,
    longitude: f32,
    accel_x: f32,
    accel_y: f32,
    accel_z: f32,
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
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
            humidity: rng.gen_range(0.0..=100.0),     // Humidity percentage
            altitude: rng.gen_range(0.0..=50000.0),   // Altitude in meters
            latitude: rng.gen_range(-90.0..=90.0),    // Latitude in degrees
            longitude: rng.gen_range(-180.0..=180.0), // Longitude in degrees
            accel_x: rng.gen_range(-20.0..=20.0),     // Accelerometer X in m/s²
            accel_y: rng.gen_range(-20.0..=20.0),     // Accelerometer Y in m/s²
            accel_z: rng.gen_range(-20.0..=20.0),     // Accelerometer Z in m/s²
            gyro_x: rng.gen_range(-2000.0..=2000.0),  // Gyroscope X in °/s
            gyro_y: rng.gen_range(-2000.0..=2000.0),  // Gyroscope Y in °/s
            gyro_z: rng.gen_range(-2000.0..=2000.0),  // Gyroscope Z in °/s
            status: rng.gen_range(0..=255),           // Status byte
        }
    }
    
    
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    fn new_with_motion_data(temperature_celsius: f32, motion: MotionReading) -> Self {
        let mut rng = rand::thread_rng();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            sync: 0xFF_FF_FF_FF_FF_FF_FF_FF,
            timestamp: now,
            temperature: temperature_celsius,
            humidity: rng.gen_range(0.0..=100.0),     // Humidity percentage (still simulated)
            altitude: rng.gen_range(0.0..=50000.0),   // Altitude in meters (simulated)
            latitude: rng.gen_range(-90.0..=90.0),    // Latitude in degrees (still simulated)
            longitude: rng.gen_range(-180.0..=180.0), // Longitude in degrees (still simulated)
            accel_x: motion.accelerometer.x,
            accel_y: motion.accelerometer.y,
            accel_z: motion.accelerometer.z,
            gyro_x: motion.gyroscope.x,
            gyro_y: motion.gyroscope.y,
            gyro_z: motion.gyroscope.z,
            status: 0x02, // Status byte indicating real temperature and motion data
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


#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn init_motion_sensor(i2c: I2c) -> Option<MPU6050> {
    let motion_sensor: Option<MPU6050> = match MPU6050::new(i2c, false) {
        Ok(sensor) => {
            println!("MPU6050 motion sensor initialized successfully");
            Some(sensor)
        }
        Err(e) => {
            eprintln!("Failed to initialize MPU6050 motion sensor: {}", e);
            eprintln!("Continuing with simulated motion data...");
            None
        }
    };

    return motion_sensor;
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn read_motion_sensor(motion_sensor: &mut Option<MPU6050>) -> Option<MotionReading> {
    if let Some(ref mut motion) = motion_sensor {
        match motion.read_all() {
            Ok(reading) => {
                println!("Motion reading: Accel({:.2}, {:.2}, {:.2}) m/s², Gyro({:.2}, {:.2}, {:.2}) °/s, Temp: {:.2}°C", 
                         reading.accelerometer.x, reading.accelerometer.y, reading.accelerometer.z,
                         reading.gyroscope.x, reading.gyroscope.y, reading.gyroscope.z,
                         reading.temperature);
                Some(reading)
            },
            Err(e) => {
                eprintln!("Failed to read motion sensor: {}", e);
                None
            }
        }
    } else {
        None
    }
}

// Fallback functions for non-ARM Linux systems

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
fn init_motion_sensor(_i2c: ()) -> () {
    println!("Running on non-ARM Linux system - using simulated data");
    ()
}

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
fn read_motion_sensor(_motion_sensor: &mut ()) -> () {
    ()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target_addr = "127.0.0.1:3000";
    
    println!("Starting telemetry packet generator...");
    println!("Sending packets to: {}", target_addr);
    
    // Check if running on ARM Linux (Raspberry Pi)
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        println!("Detected ARM Linux system - attempting to initialize Raspberry Pi sensors...");
        
        // Initialize MPU6050 motion sensor
        let i2c_motion = I2c::new()?;
        let mut motion_sensor = init_motion_sensor(i2c_motion);
        
        loop {
            let motion_reading = read_motion_sensor(&mut motion_sensor);
            
            let packet = match motion_reading {
                Some(motion) => {
                    TelemetryPacket::new_with_motion_data(
                        motion.temperature, 
                        motion
                    )
                },
                None => TelemetryPacket::new() // Fallback to simulated data
            };
            
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

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
    
    #[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
    {
        println!("Not running on ARM Linux - using simulated data only");
        
        // Initialize dummy sensor for non-ARM systems
        let mut motion_sensor = init_motion_sensor(());
        
        loop {
            let _motion_reading = read_motion_sensor(&mut motion_sensor);
            
            // Always use simulated data for non-ARM systems
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

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}