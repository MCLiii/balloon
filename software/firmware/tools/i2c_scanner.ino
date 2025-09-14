// #include <Wire.h>
// #include <Adafruit_MPL115A2.h>

// Adafruit_MPL115A2 mpl115a2;

// void setup(void) 
// {
//   Wire.begin(6,7);
//   printf("hello\n");
  
//   printf("Getting barometric pressure ...\n");
//   if (! mpl115a2.begin()) {
//     printf("Sensor not found! Check wiring\n");
//     while (1);
//   }
// }

// void loop(void) 
// {
//   float pressureKPA = 0, temperatureC = 0;    

//   pressureKPA = mpl115a2.getPressure();  
//   printf("Pressure (kPa): \n");
//   printf("%f", pressureKPA); printf(" kPa\n");

//   temperatureC = mpl115a2.getTemperature();  
//   printf("Temp (*C): \n"); printf("%f", temperatureC); printf(" *C");
//   delay(1000);
// }

#include <Wire.h>
void setup() {
  Wire.begin(6,7);

  printf("\nI2C Scanner\n");
}


void loop() {
  byte error, address;
  int nDevices;

  printf("Scanning...\n");

  nDevices = 0;
  for(address = 1; address < 127; address++ )
  {
    // The i2c_scanner uses the return value of
    // the Write.endTransmisstion to see if
    // a device did acknowledge to the address.
    Wire.beginTransmission(address);
    error = Wire.endTransmission();

    if (error == 0)
    {
      printf("I2C device found at address 0x");
      if (address<16)
        Serial.print("0");
      printf("%x", address);
      printf("  !\n");

      nDevices++;
    }
    else if (error==4)
    {
      printf("Unknown error at address 0x");
      if (address<16)
        printf("0");
      printf("%x", address);
      printf("\n");
    }
  }
  if (nDevices == 0)
    printf("No I2C devices found\n");
  else
    printf("done\n");

  delay(5000);           // wait 5 seconds for next scan
}