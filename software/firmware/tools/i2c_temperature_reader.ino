// I2C Temperature Reader - Master device to read temperature from temp controller
// This demonstrates how to read temperature data from the I2C temperature controller

#include <Wire.h>

#define TEMP_CONTROLLER_ADDRESS 0x0A
#define TEMP_REGISTER 0x00
#define STATUS_REGISTER 0x04
#define COMMAND_REGISTER 0x05

// Commands
#define CMD_READ_TEMP 0x01
#define CMD_GET_STATUS 0x02

void setup() {
  Wire.begin(); // Initialize I2C as master
  Serial.begin(115200);
  
  Serial.println("I2C Temperature Reader");
  Serial.println("Reading from temperature controller at address 0x0A");
  Serial.println();
}

void loop() {
  // Read temperature data
  readTemperature();
  
  // Read status
  readStatus();
  
  // Trigger fresh temperature reading
  triggerTemperatureReading();
  
  delay(2000); // Wait 2 seconds between readings
}

void readTemperature() {
  Wire.beginTransmission(TEMP_CONTROLLER_ADDRESS);
  Wire.write(TEMP_REGISTER); // Request temperature register
  Wire.endTransmission();
  
  Wire.requestFrom(TEMP_CONTROLLER_ADDRESS, 4); // Request 4 bytes (float)
  
  if (Wire.available() == 4) {
    uint8_t bytes[4];
    for (int i = 0; i < 4; i++) {
      bytes[i] = Wire.read();
    }
    
    // Convert bytes back to float
    float temperature;
    memcpy(&temperature, bytes, sizeof(float));
    
    Serial.print("Temperature: ");
    Serial.print(temperature);
    Serial.println(" °C");
  } else {
    Serial.println("Error: Could not read temperature data");
  }
}

void readStatus() {
  Wire.beginTransmission(TEMP_CONTROLLER_ADDRESS);
  Wire.write(STATUS_REGISTER); // Request status register
  Wire.endTransmission();
  
  Wire.requestFrom(TEMP_CONTROLLER_ADDRESS, 1); // Request 1 byte
  
  if (Wire.available() == 1) {
    uint8_t status = Wire.read();
    Serial.print("Status: 0x");
    Serial.print(status, HEX);
    Serial.print(" (");
    if (status == 0x01) {
      Serial.print("Data valid");
    } else {
      Serial.print("No data");
    }
    Serial.println(")");
  } else {
    Serial.println("Error: Could not read status");
  }
}

void triggerTemperatureReading() {
  Wire.beginTransmission(TEMP_CONTROLLER_ADDRESS);
  Wire.write(COMMAND_REGISTER); // Write to command register
  Wire.write(CMD_READ_TEMP); // Send read temperature command
  Wire.endTransmission();
  
  Serial.println("Triggered fresh temperature reading");
}

