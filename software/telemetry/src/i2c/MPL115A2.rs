// MPL115A2 I2C driver barometric pressure sensor

use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

const MPL115A2_ADDRESS: u8 = 0x60;

// MPL115A2 register addresses
const REGISTER_PRESSURE_MSB: u8 = 0x00;
const REGISTER_PRESSURE_LSB: u8 = 0x01;
const REGISTER_TEMP_MSB: u8 = 0x02;
const REGISTER_TEMP_LSB: u8 = 0x03;
const REGISTER_A0_COEFF_MSB: u8 = 0x04;
const REGISTER_A0_COEFF_LSB: u8 = 0x05;
const REGISTER_B1_COEFF_MSB: u8 = 0x06;
const REGISTER_B1_COEFF_LSB: u8 = 0x07;
const REGISTER_B2_COEFF_MSB: u8 = 0x08;
const REGISTER_B2_COEFF_LSB: u8 = 0x09;
const REGISTER_C12_COEFF_MSB: u8 = 0x0A;
const REGISTER_C12_COEFF_LSB: u8 = 0x0B;
const REGISTER_START_CONVERSION: u8 = 0x12;

#[derive(Debug, Clone)]
pub struct PressureReading {
    pub pressure_hpa: f32,
    pub temperature_celsius: f32,
}

pub struct MPL115A2 {
    i2c: I2c,
    a0: f32,
    b1: f32,
    b2: f32,
    c12: f32,
}

impl MPL115A2 {
    pub fn new(mut i2c: I2c) -> Result<Self, Box<dyn std::error::Error>> {
        // Set I2C address
        i2c.set_slave_address(MPL115A2_ADDRESS)?;
        
        let mut sensor = Self {
            i2c,
            a0: 0.0,
            b1: 0.0,
            b2: 0.0,
            c12: 0.0,
        };
        
        // Read calibration coefficients
        sensor.read_calibration_coefficients()?;
        
        Ok(sensor)
    }
    
    fn read_calibration_coefficients(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Read A0 coefficient (16-bit signed)
        let a0_msb = self.read_register(REGISTER_A0_COEFF_MSB)?;
        let a0_lsb = self.read_register(REGISTER_A0_COEFF_LSB)?;
        let a0_raw = ((a0_msb as i16) << 8) | (a0_lsb as i16);
        self.a0 = a0_raw as f32 / 8.0; // Convert to floating point
        
        // Read B1 coefficient (16-bit signed)
        let b1_msb = self.read_register(REGISTER_B1_COEFF_MSB)?;
        let b1_lsb = self.read_register(REGISTER_B1_COEFF_LSB)?;
        let b1_raw = ((b1_msb as i16) << 8) | (b1_lsb as i16);
        self.b1 = b1_raw as f32 / 8192.0; // Convert to floating point
        
        // Read B2 coefficient (16-bit signed)
        let b2_msb = self.read_register(REGISTER_B2_COEFF_MSB)?;
        let b2_lsb = self.read_register(REGISTER_B2_COEFF_LSB)?;
        let b2_raw = ((b2_msb as i16) << 8) | (b2_lsb as i16);
        self.b2 = b2_raw as f32 / 16384.0; // Convert to floating point
        
        // Read C12 coefficient (16-bit signed)
        let c12_msb = self.read_register(REGISTER_C12_COEFF_MSB)?;
        let c12_lsb = self.read_register(REGISTER_C12_COEFF_LSB)?;
        let c12_raw = ((c12_msb as i16) << 8) | (c12_lsb as i16);
        self.c12 = c12_raw as f32 / 4194304.0; // Convert to floating point
        
        println!("Calibration coefficients: A0={}, B1={}, B2={}, C12={}", 
                 self.a0, self.b1, self.b2, self.c12);
        
        Ok(())
    }
    
    fn read_register(&mut self, register: u8) -> Result<u8, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(&[register], &mut buffer)?;
        Ok(buffer[0])
    }
    
    fn read_register_16(&mut self, register: u8) -> Result<u16, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(&[register], &mut buffer)?;
        Ok(((buffer[0] as u16) << 8) | (buffer[1] as u16))
    }
    
    pub fn read_pressure(&mut self) -> Result<PressureReading, Box<dyn std::error::Error>> {
        // Start conversion
        self.i2c.write(&[REGISTER_START_CONVERSION, 0x00])?;
        
        // Wait for conversion to complete (3ms typical)
        thread::sleep(Duration::from_millis(5));
        
        // Read pressure (10-bit ADC value)
        let pressure_msb = self.read_register(REGISTER_PRESSURE_MSB)?;
        let pressure_lsb = self.read_register(REGISTER_PRESSURE_LSB)?;
        let pressure_raw = ((pressure_msb as u16) << 2) | ((pressure_lsb as u16) >> 6);
        
        // Read temperature (10-bit ADC value)
        let temp_msb = self.read_register(REGISTER_TEMP_MSB)?;
        let temp_lsb = self.read_register(REGISTER_TEMP_LSB)?;
        let temp_raw = ((temp_msb as u16) << 2) | ((temp_lsb as u16) >> 6);
        
        // Convert ADC values to pressure and temperature
        let pressure_adc = pressure_raw as f32;
        let temp_adc = temp_raw as f32;
        
        // Calculate pressure using the compensation formula
        let pressure_comp = self.a0 + (self.b1 + self.c12 * temp_adc) * pressure_adc + self.b2 * temp_adc;
        
        // Convert to hPa (hectopascals)
        let pressure_hpa = (pressure_comp * 65.0 / 1023.0) + 50.0;
        
        // Convert temperature to Celsius
        let temperature_celsius = (temp_adc - 498.0) / -5.35 + 25.0;
        
        Ok(PressureReading {
            pressure_hpa,
            temperature_celsius,
        })
    }
}