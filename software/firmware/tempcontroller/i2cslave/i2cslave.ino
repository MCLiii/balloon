#include <Wire.h>
#include <DallasTemperature.h>
#include <stdio.h>
#include <string.h>

volatile uint8_t address;
volatile uint8_t memory_map [256]; // array is initialize all low values

#define SLAVE_ID 0x0A

#define ONE_WIRE_BUS 0

// Memory map layout
#define TEMP_REGISTER 0x00  // Temperature register (4 bytes for float)
#define STATUS_REGISTER 0x04 // Status register (1 byte)
#define COMMAND_REGISTER 0x05 // Command register (1 byte)

// Commands
#define CMD_READ_TEMP 0x01
#define CMD_GET_STATUS 0x02

OneWire oneWire(ONE_WIRE_BUS);
DallasTemperature sensors(&oneWire);


float _18B20_get_temperature(void)
{
  sensors.requestTemperatures();
  float temp = sensors.getTempCByIndex(0);
  printf("Temperature: %f *C\n", temp);
  return temp;
}

void store_temperature_in_memory(float temperature)
{
  // Convert float to bytes and store in memory map
  uint8_t* temp_bytes = (uint8_t*)&temperature;
  for (int i = 0; i < 4; i++) {
    memory_map[TEMP_REGISTER + i] = temp_bytes[i];
  }
  
  // Update status register to indicate fresh data
  memory_map[STATUS_REGISTER] = 0x01; // Data valid
}

void setup()
{
  Wire.begin(SLAVE_ID);         // join i2c bus with slave_id SLAVE_ID
  Wire.onReceive(receiveEvent); // register write to slave
  Wire.onRequest(requestEvent); // register read from slave
  Serial.begin(115200);
  sensors.begin();
  
  // Initialize memory map
  memset(memory_map, 0, sizeof(memory_map));
  memory_map[STATUS_REGISTER] = 0x00; // No data initially
}

void loop()
{
  float temperature = _18B20_get_temperature();
  store_temperature_in_memory(temperature);
  
  Serial.print("Temperature: ");
  Serial.print(temperature);
  Serial.println(" °C");
  delay(1000); // Update every second
}

// function that executes when the master writes data to this slave
void receiveEvent(int bytes)
{
  if (Wire.available()) {
    address = Wire.read(); // read first byte to determine address
    Serial.print("I2C write request to address: 0x");
    Serial.println(address, HEX);
    
    // Handle command register writes
    if (address == COMMAND_REGISTER && Wire.available()) {
      uint8_t command = Wire.read();
      Serial.print("Command received: 0x");
      Serial.println(command, HEX);
      
      switch (command) {
        case CMD_READ_TEMP:
          // Trigger immediate temperature reading
          float temp = _18B20_get_temperature();
          store_temperature_in_memory(temp);
          Serial.println("Temperature reading triggered by command");
          break;
        case CMD_GET_STATUS:
          // Status is already in memory_map[STATUS_REGISTER]
          break;
        default:
          Serial.print("Unknown command: 0x");
          Serial.println(command, HEX);
          break;
      }
    }
    // Only allow writing to valid memory addresses
    else if (address < 256) {
      while (Wire.available() && address < 256)
      {
        memory_map[address++] = Wire.read();
      }
    } else {
      // Clear any remaining data if address is invalid
      while (Wire.available()) {
        Wire.read();
      }
    }
  }
}

// function that executes when the master reads from this slave
void requestEvent()
{
  Serial.print("I2C read request from address: 0x");
  Serial.println(address, HEX);
  
  // Send data based on the requested address
  switch (address) {
    case TEMP_REGISTER:
      // Send temperature data (4 bytes)
      for (int i = 0; i < 4; i++) {
        Wire.write(memory_map[TEMP_REGISTER + i]);
      }
      break;
    case STATUS_REGISTER:
      // Send status byte
      Wire.write(memory_map[STATUS_REGISTER]);
      break;
    case COMMAND_REGISTER:
      // Send command register (read-only, returns 0)
      Wire.write(0x00);
      break;
    default:
      // Send data from memory map at requested address
      if (address < 256) {
        Wire.write(memory_map[address]);
      } else {
        Wire.write(0x00); // Invalid address
      }
      break;
  }
}