// MPU6050 I2C driver for 6-axis motion tracking (3-axis gyroscope + 3-axis accelerometer)

use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

const MPU6050_ADDRESS: u8 = 0x68; // Default I2C address (AD0 = 0)
const MPU6050_ADDRESS_ALT: u8 = 0x69; // Alternative I2C address (AD0 = 1)

// MPU6050 register addresses
const REGISTER_SMPLRT_DIV: u8 = 0x19;
const REGISTER_CONFIG: u8 = 0x1A;
const REGISTER_GYRO_CONFIG: u8 = 0x1B;
const REGISTER_ACCEL_CONFIG: u8 = 0x1C;
const REGISTER_ACCEL_XOUT_H: u8 = 0x3B;
const REGISTER_ACCEL_YOUT_H: u8 = 0x3D;
const REGISTER_ACCEL_ZOUT_H: u8 = 0x3F;
const REGISTER_TEMP_OUT_H: u8 = 0x41;
const REGISTER_GYRO_XOUT_H: u8 = 0x43;
const REGISTER_GYRO_YOUT_H: u8 = 0x45;
const REGISTER_GYRO_ZOUT_H: u8 = 0x47;
const REGISTER_PWR_MGMT_1: u8 = 0x6B;
const REGISTER_WHO_AM_I: u8 = 0x75;

// Configuration values
const PWR_MGMT_1_RESET: u8 = 0x80;
const PWR_MGMT_1_CLKSEL_PLL_X: u8 = 0x01;

// Accelerometer sensitivity settings (LSB/g)
const ACCEL_SENSITIVITY_2G: f32 = 16384.0;
const ACCEL_SENSITIVITY_4G: f32 = 8192.0;
const ACCEL_SENSITIVITY_8G: f32 = 4096.0;
const ACCEL_SENSITIVITY_16G: f32 = 2048.0;

// Gyroscope sensitivity settings (LSB/°/s)
const GYRO_SENSITIVITY_250DPS: f32 = 131.0;
const GYRO_SENSITIVITY_500DPS: f32 = 65.5;
const GYRO_SENSITIVITY_1000DPS: f32 = 32.8;
const GYRO_SENSITIVITY_2000DPS: f32 = 16.4;

#[derive(Debug, Clone)]
pub struct AccelerometerReading {
    pub x: f32, // m/s²
    pub y: f32, // m/s²
    pub z: f32, // m/s²
}

#[derive(Debug, Clone)]
pub struct GyroscopeReading {
    pub x: f32, // °/s
    pub y: f32, // °/s
    pub z: f32, // °/s
}

#[derive(Debug, Clone)]
pub struct MotionReading {
    pub accelerometer: AccelerometerReading,
    pub gyroscope: GyroscopeReading,
    pub temperature: f32, // °C
}

#[derive(Debug, Clone, Copy)]
pub enum AccelSensitivity {
    AFS_SEL_2G = 0x00,
    AFS_SEL_4G = 0x08,
    AFS_SEL_8G = 0x10,
    AFS_SEL_16G = 0x18,
}

#[derive(Debug, Clone, Copy)]
pub enum GyroSensitivity {
    FS_SEL_250DPS = 0x00,
    FS_SEL_500DPS = 0x08,
    FS_SEL_1000DPS = 0x10,
    FS_SEL_2000DPS = 0x18,
}

pub struct MPU6050 {
    i2c: I2c,
    accel_sensitivity: AccelSensitivity,
    gyro_sensitivity: GyroSensitivity,
    accel_scale: f32,
    gyro_scale: f32,
}

impl MPU6050 {
    pub fn new(mut i2c: I2c, use_alt_address: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let address = if use_alt_address { MPU6050_ADDRESS_ALT } else { MPU6050_ADDRESS };
        i2c.set_slave_address(address as u16)?;
        
        let mut sensor = Self {
            i2c,
            accel_sensitivity: AccelSensitivity::AFS_SEL_2G,
            gyro_sensitivity: GyroSensitivity::FS_SEL_250DPS,
            accel_scale: ACCEL_SENSITIVITY_2G,
            gyro_scale: GYRO_SENSITIVITY_250DPS,
        };
        
        // Initialize the sensor
        sensor.initialize()?;
        
        Ok(sensor)
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Reset the device
        self.write_register(REGISTER_PWR_MGMT_1, PWR_MGMT_1_RESET)?;
        thread::sleep(Duration::from_millis(100));
        
        // Wake up the device and set clock source to PLL with X-axis gyroscope reference
        self.write_register(REGISTER_PWR_MGMT_1, PWR_MGMT_1_CLKSEL_PLL_X)?;
        
        // Configure gyroscope sensitivity
        self.set_gyro_sensitivity(self.gyro_sensitivity)?;
        
        // Configure accelerometer sensitivity
        self.set_accel_sensitivity(self.accel_sensitivity)?;
        
        // Set sample rate divider (1kHz / (1 + SMPLRT_DIV))
        self.write_register(REGISTER_SMPLRT_DIV, 0x07)?; // ~125Hz
        
        // Configure digital low-pass filter
        self.write_register(REGISTER_CONFIG, 0x06)?; // ~5Hz cutoff
        
        // Verify device identity
        let who_am_i = self.read_register(REGISTER_WHO_AM_I)?;
        if who_am_i != 0x68 {
            return Err(format!("Invalid WHO_AM_I value: 0x{:02X}, expected 0x68", who_am_i).into());
        }
        
        println!("MPU6050 initialized successfully (WHO_AM_I: 0x{:02X})", who_am_i);
        
        Ok(())
    }
    
    pub fn set_accel_sensitivity(&mut self, sensitivity: AccelSensitivity) -> Result<(), Box<dyn std::error::Error>> {
        self.accel_sensitivity = sensitivity;
        
        // Update scale factor
        self.accel_scale = match sensitivity {
            AccelSensitivity::AFS_SEL_2G => ACCEL_SENSITIVITY_2G,
            AccelSensitivity::AFS_SEL_4G => ACCEL_SENSITIVITY_4G,
            AccelSensitivity::AFS_SEL_8G => ACCEL_SENSITIVITY_8G,
            AccelSensitivity::AFS_SEL_16G => ACCEL_SENSITIVITY_16G,
        };
        
        // Write configuration
        self.write_register(REGISTER_ACCEL_CONFIG, sensitivity as u8)?;
        
        println!("Accelerometer sensitivity set to {:?}", sensitivity);
        Ok(())
    }
    
    pub fn set_gyro_sensitivity(&mut self, sensitivity: GyroSensitivity) -> Result<(), Box<dyn std::error::Error>> {
        self.gyro_sensitivity = sensitivity;
        
        // Update scale factor
        self.gyro_scale = match sensitivity {
            GyroSensitivity::FS_SEL_250DPS => GYRO_SENSITIVITY_250DPS,
            GyroSensitivity::FS_SEL_500DPS => GYRO_SENSITIVITY_500DPS,
            GyroSensitivity::FS_SEL_1000DPS => GYRO_SENSITIVITY_1000DPS,
            GyroSensitivity::FS_SEL_2000DPS => GYRO_SENSITIVITY_2000DPS,
        };
        
        // Write configuration
        self.write_register(REGISTER_GYRO_CONFIG, sensitivity as u8)?;
        
        println!("Gyroscope sensitivity set to {:?}", sensitivity);
        Ok(())
    }
    
    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.i2c.write(&[register, value])?;
        Ok(())
    }
    
    fn read_register(&mut self, register: u8) -> Result<u8, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(&[register], &mut buffer)?;
        Ok(buffer[0])
    }
    
    fn read_register_16(&mut self, register: u8) -> Result<i16, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(&[register], &mut buffer)?;
        Ok(((buffer[0] as i16) << 8) | (buffer[1] as i16))
    }
    
    pub fn read_accelerometer(&mut self) -> Result<AccelerometerReading, Box<dyn std::error::Error>> {
        let x_raw = self.read_register_16(REGISTER_ACCEL_XOUT_H)?;
        let y_raw = self.read_register_16(REGISTER_ACCEL_YOUT_H)?;
        let z_raw = self.read_register_16(REGISTER_ACCEL_ZOUT_H)?;
        
        let x = (x_raw as f32 / self.accel_scale) * 9.80665; // Convert to m/s²
        let y = (y_raw as f32 / self.accel_scale) * 9.80665;
        let z = (z_raw as f32 / self.accel_scale) * 9.80665;
        
        Ok(AccelerometerReading { x, y, z })
    }
    
    pub fn read_gyroscope(&mut self) -> Result<GyroscopeReading, Box<dyn std::error::Error>> {
        let x_raw = self.read_register_16(REGISTER_GYRO_XOUT_H)?;
        let y_raw = self.read_register_16(REGISTER_GYRO_YOUT_H)?;
        let z_raw = self.read_register_16(REGISTER_GYRO_ZOUT_H)?;
        
        let x = x_raw as f32 / self.gyro_scale;
        let y = y_raw as f32 / self.gyro_scale;
        let z = z_raw as f32 / self.gyro_scale;
        
        Ok(GyroscopeReading { x, y, z })
    }
    
    pub fn read_temperature(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_raw = self.read_register_16(REGISTER_TEMP_OUT_H)?;
        let temperature = (temp_raw as f32 / 340.0) + 36.53; // Convert to °C
        Ok(temperature)
    }
    
    pub fn read_all(&mut self) -> Result<MotionReading, Box<dyn std::error::Error>> {
        let accelerometer = self.read_accelerometer()?;
        let gyroscope = self.read_gyroscope()?;
        let temperature = self.read_temperature()?;
        
        Ok(MotionReading {
            accelerometer,
            gyroscope,
            temperature,
        })
    }
    
    pub fn calibrate(&mut self, samples: usize) -> Result<(AccelerometerReading, GyroscopeReading), Box<dyn std::error::Error>> {
        println!("Calibrating MPU6050 with {} samples...", samples);
        
        let mut accel_offset = AccelerometerReading { x: 0.0, y: 0.0, z: 0.0 };
        let mut gyro_offset = GyroscopeReading { x: 0.0, y: 0.0, z: 0.0 };
        
        for i in 0..samples {
            let reading = self.read_all()?;
            
            accel_offset.x += reading.accelerometer.x;
            accel_offset.y += reading.accelerometer.y;
            accel_offset.z += reading.accelerometer.z;
            
            gyro_offset.x += reading.gyroscope.x;
            gyro_offset.y += reading.gyroscope.y;
            gyro_offset.z += reading.gyroscope.z;
            
            if i % 50 == 0 {
                print!(".");
            }
            
            thread::sleep(Duration::from_millis(10));
        }
        
        accel_offset.x /= samples as f32;
        accel_offset.y /= samples as f32;
        accel_offset.z /= samples as f32;
        
        gyro_offset.x /= samples as f32;
        gyro_offset.y /= samples as f32;
        gyro_offset.z /= samples as f32;
        
        // For accelerometer, subtract gravity from Z-axis if device is stationary
        accel_offset.z -= 9.80665; // Assume device is flat during calibration
        
        println!("\nCalibration complete!");
        println!("Accelerometer offsets: X={:.3}, Y={:.3}, Z={:.3}", 
                 accel_offset.x, accel_offset.y, accel_offset.z);
        println!("Gyroscope offsets: X={:.3}, Y={:.3}, Z={:.3}", 
                 gyro_offset.x, gyro_offset.y, gyro_offset.z);
        
        Ok((accel_offset, gyro_offset))
    }
}
