use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use std::mem;
use rppal::i2c::I2c;

mod i2c;
use i2c::MPL115A2::{MPL115A2, PressureReading};
use i2c::MPU6050::{MPU6050, MotionReading};

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
            pressure: rng.gen_range(800.0..=1200.0),  // Pressure in hPa
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
    
    fn new_with_sensor_data(pressure_kpa: f32, temperature_celsius: f32) -> Self {
        let mut rng = rand::thread_rng();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let pressure_hpa = pressure_kpa * 10.0;
        // Calculate altitude from pressure using barometric formula
        // h = 44330 * (1 - (P/P0)^0.1903) where P0 = 1013.25 hPa (sea level pressure)
        let sea_level_pressure = 1013.25;
        let altitude = if pressure_hpa > 0.0 {
            44330.0 * (1.0 - (pressure_hpa / sea_level_pressure).powf(0.1903))
        } else {
            0.0
        };

        Self {
            sync: 0xFF_FF_FF_FF_FF_FF_FF_FF,
            timestamp: now,
            temperature: temperature_celsius,
            pressure: pressure_kpa,
            humidity: rng.gen_range(0.0..=100.0),     // Humidity percentage (still simulated)
            altitude,
            latitude: rng.gen_range(-90.0..=90.0),    // Latitude in degrees (still simulated)
            longitude: rng.gen_range(-180.0..=180.0), // Longitude in degrees (still simulated)
            accel_x: rng.gen_range(-20.0..=20.0),     // Accelerometer X in m/s² (simulated)
            accel_y: rng.gen_range(-20.0..=20.0),     // Accelerometer Y in m/s² (simulated)
            accel_z: rng.gen_range(-20.0..=20.0),     // Accelerometer Z in m/s² (simulated)
            gyro_x: rng.gen_range(-2000.0..=2000.0),  // Gyroscope X in °/s (simulated)
            gyro_y: rng.gen_range(-2000.0..=2000.0),  // Gyroscope Y in °/s (simulated)
            gyro_z: rng.gen_range(-2000.0..=2000.0),  // Gyroscope Z in °/s (simulated)
            status: 0x01, // Status byte indicating real sensor data
        }
    }
    
    fn new_with_full_sensor_data(pressure_kpa: f32, temperature_celsius: f32, motion: MotionReading) -> Self {
        let mut rng = rand::thread_rng();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let pressure_hpa = pressure_kpa * 10.0;
        // Calculate altitude from pressure using barometric formula
        // h = 44330 * (1 - (P/P0)^0.1903) where P0 = 1013.25 hPa (sea level pressure)
        let sea_level_pressure = 1013.25;
        let altitude = if pressure_hpa > 0.0 {
            44330.0 * (1.0 - (pressure_hpa / sea_level_pressure).powf(0.1903))
        } else {
            0.0
        };

        Self {
            sync: 0xFF_FF_FF_FF_FF_FF_FF_FF,
            timestamp: now,
            temperature: temperature_celsius,
            pressure: pressure_kpa,
            humidity: rng.gen_range(0.0..=100.0),     // Humidity percentage (still simulated)
            altitude,
            latitude: rng.gen_range(-90.0..=90.0),    // Latitude in degrees (still simulated)
            longitude: rng.gen_range(-180.0..=180.0), // Longitude in degrees (still simulated)
            accel_x: motion.accelerometer.x,
            accel_y: motion.accelerometer.y,
            accel_z: motion.accelerometer.z,
            gyro_x: motion.gyroscope.x,
            gyro_y: motion.gyroscope.y,
            gyro_z: motion.gyroscope.z,
            status: 0x03, // Status byte indicating real pressure, temperature, and motion data
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

fn init_barometer(i2c: I2c) -> Option<MPL115A2> {
    let barometer: Option<MPL115A2> = match MPL115A2::new(i2c) {
        Ok(sensor) => {
            println!("MPL115A2 barometer initialized successfully");
            Some(sensor)
        }
        Err(e) => {
            eprintln!("Failed to initialize MPL115A2 barometer: {}", e);
            eprintln!("Continuing with simulated data...");
            None
        }
    };

    return barometer;
}

fn read_barometer(barometer: &mut Option<MPL115A2>) -> Option<PressureReading> {
    if let Some(ref mut baro) = barometer {
        match baro.read_pressure() {
            Ok(reading) => {
                println!("Barometer reading: {:.2} kPa, {:.2}°C", 
                         reading.pressure_kpa, reading.temperature_celsius);
                Some(reading)
            },
            Err(e) => {
                eprintln!("Failed to read barometer: {}", e);
                None
            }
        }
    } else {
        None
    }
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target_addr = "127.0.0.1:3000";
    
    println!("Starting telemetry packet generator...");
    println!("Sending packets to: {}", target_addr);
    
    // Initialize I2C and sensors
    let i2c = I2c::new()?;
    let mut barometer = init_barometer(i2c);
    
    // Initialize MPU6050 motion sensor (using a new I2C instance)
    let i2c_motion = I2c::new()?;
    let mut motion_sensor = init_motion_sensor(i2c_motion);
    
    loop {
        let pressure_reading = read_barometer(&mut barometer);
        let motion_reading = read_motion_sensor(&mut motion_sensor);
        
        let packet = match (pressure_reading, motion_reading) {
            (Some(pressure), Some(motion)) => {
                TelemetryPacket::new_with_full_sensor_data(
                    pressure.pressure_kpa, 
                    pressure.temperature_celsius, 
                    motion
                )
            },
            (Some(pressure), None) => {
                TelemetryPacket::new_with_sensor_data(
                    pressure.pressure_kpa, 
                    pressure.temperature_celsius
                )
            },
            _ => TelemetryPacket::new() // Fallback to simulated data
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