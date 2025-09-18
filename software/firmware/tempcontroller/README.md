# I2C Temperature Controller

This Arduino sketch implements an I2C slave device that provides temperature readings from a DS18B20 sensor via the I2C bus.

## Hardware Requirements

- Arduino-compatible microcontroller (Arduino Uno, ESP32, etc.)
- DS18B20 temperature sensor
- Pull-up resistor (4.7kΩ) for OneWire bus
- I2C pull-up resistors (4.7kΩ) on SDA and SCL lines

## Wiring

### DS18B20 Connection
- VCC: 3.3V or 5V (depending on your Arduino)
- GND: Ground
- Data: Digital pin 0 (configurable via `ONE_WIRE_BUS` define)
- Pull-up resistor: 4.7kΩ between Data and VCC

### I2C Connection
- SDA: Connect to SDA pin on your Arduino
- SCL: Connect to SCL pin on your Arduino
- Pull-up resistors: 4.7kΩ on both SDA and SCL lines

## I2C Interface

### Device Address
- **I2C Address**: `0x0A` (configurable via `SLAVE_ID` define)

### Register Map

| Address | Register | Size | Description |
|---------|----------|------|-------------|
| 0x00-0x03 | Temperature | 4 bytes | Current temperature reading (float, °C) |
| 0x04 | Status | 1 byte | Data validity status (0x01 = valid, 0x00 = no data) |
| 0x05 | Command | 1 byte | Command register (write-only) |

### Commands

| Command | Value | Description |
|---------|-------|-------------|
| CMD_READ_TEMP | 0x01 | Trigger immediate temperature reading |
| CMD_GET_STATUS | 0x02 | Get current status (redundant with status register) |

## Usage Examples

### Reading Temperature (Master Device)

```cpp
#include <Wire.h>

#define TEMP_CONTROLLER_ADDRESS 0x0A
#define TEMP_REGISTER 0x00

void readTemperature() {
  Wire.beginTransmission(TEMP_CONTROLLER_ADDRESS);
  Wire.write(TEMP_REGISTER);
  Wire.endTransmission();
  
  Wire.requestFrom(TEMP_CONTROLLER_ADDRESS, 4);
  
  if (Wire.available() == 4) {
    uint8_t bytes[4];
    for (int i = 0; i < 4; i++) {
      bytes[i] = Wire.read();
    }
    
    float temperature;
    memcpy(&temperature, bytes, sizeof(float));
    
    Serial.print("Temperature: ");
    Serial.print(temperature);
    Serial.println(" °C");
  }
}
```

### Triggering Fresh Reading

```cpp
void triggerTemperatureReading() {
  Wire.beginTransmission(TEMP_CONTROLLER_ADDRESS);
  Wire.write(0x05); // Command register
  Wire.write(0x01); // CMD_READ_TEMP
  Wire.endTransmission();
}
```

### Checking Status

```cpp
void checkStatus() {
  Wire.beginTransmission(TEMP_CONTROLLER_ADDRESS);
  Wire.write(0x04); // Status register
  Wire.endTransmission();
  
  Wire.requestFrom(TEMP_CONTROLLER_ADDRESS, 1);
  
  if (Wire.available() == 1) {
    uint8_t status = Wire.read();
    if (status == 0x01) {
      Serial.println("Temperature data is valid");
    } else {
      Serial.println("No temperature data available");
    }
  }
}
```

## Features

- **Automatic Updates**: Temperature is read and stored every second
- **On-Demand Reading**: Commands can trigger immediate temperature readings
- **Status Indication**: Status register indicates data validity
- **Error Handling**: Robust I2C communication with bounds checking
- **Memory Map**: Organized register layout for easy access

## Configuration

### Pin Configuration
```cpp
#define ONE_WIRE_BUS 0  // Digital pin for DS18B20 data line
#define SLAVE_ID 0x0A   // I2C slave address
```

### Update Frequency
```cpp
delay(1000); // Update every second (in main loop)
```

## Testing

Use the provided `i2c_temperature_reader.ino` sketch to test the temperature controller:

1. Upload `i2cslave.ino` to your temperature controller Arduino
2. Upload `i2c_temperature_reader.ino` to a separate Arduino (master)
3. Connect both devices via I2C
4. Open serial monitor on the master device to see temperature readings

## Troubleshooting

### No I2C Communication
- Check I2C wiring (SDA, SCL, pull-up resistors)
- Verify I2C address (0x0A)
- Use I2C scanner to detect devices

### No Temperature Reading
- Check DS18B20 wiring and pull-up resistor
- Verify OneWire pin configuration
- Check serial output for error messages

### Invalid Temperature Data
- Check status register before reading temperature
- Ensure proper byte order when converting float
- Verify I2C communication is working correctly

