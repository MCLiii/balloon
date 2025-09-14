/*
 * Balloon Telemetry Firmware
 * 
 * This firmware includes MPU6050 calibration functionality:
 * - Takes initial readings when device is stationary
 * - Calculates offsets to zero X and Y axes
 * - Sets Z axis to 9.8 m/s² (gravity)
 * - Applies calibration offsets to all subsequent readings
 * - Verifies calibration accuracy periodically
 */

#include <Wire.h>
#include <Adafruit_MPL115A2.h>
#include <Adafruit_MPU6050.h>
#include <Adafruit_Sensor.h>


Adafruit_MPL115A2 mpl115a2;
Adafruit_MPU6050 mpu;

// MPU6050 Calibration offsets
float accel_offset_x = 0.0;
float accel_offset_y = 0.0;
float accel_offset_z = 0.0;
bool calibration_done = false;

void MPL115A2_setup(void) {
  // barometric pressure sensor
  printf("Getting barometric pressure ...\n");
  if (! mpl115a2.begin()) {
    printf("Sensor not found! Check wiring\n");
    while (1);
  }
}

void MPL115A2_read(void) {
  float pressureKPA = 0, temperatureC = 0;    

  pressureKPA = mpl115a2.getPressure();  
  printf("Pressure (kPa): ");
  printf("%f", pressureKPA); printf(" kPa\n");

  temperatureC = mpl115a2.getTemperature();  
  printf("Temperature: "); printf("%f", temperatureC); printf(" *C\n");
}

void MPU6050_setup(void) {
  printf("Adafruit MPU6050 test!\n");

  // Try to initialize!
  if (!mpu.begin()) {
    printf("Failed to find MPU6050 chip\n");
    while (1) {
      delay(10);
    }
  }
  printf("MPU6050 Found!\n");

  mpu.setAccelerometerRange(MPU6050_RANGE_2_G);
  printf("Accelerometer range set to: ");
  switch (mpu.getAccelerometerRange()) {
  case MPU6050_RANGE_2_G:
    printf("+-2G\n");
    break;
  case MPU6050_RANGE_4_G:
    printf("+-4G\n");
    break;
  case MPU6050_RANGE_8_G:
    printf("+-8G\n");
    break;
  case MPU6050_RANGE_16_G:
    printf("+-16G\n");
    break;
  }
  mpu.setGyroRange(MPU6050_RANGE_250_DEG);
  printf("Gyro range set to: ");
  switch (mpu.getGyroRange()) {
  case MPU6050_RANGE_250_DEG:
    printf("+- 250 deg/s\n");
    break;
  case MPU6050_RANGE_500_DEG:
    printf("+- 500 deg/s\n");
    break;
  case MPU6050_RANGE_1000_DEG:
    printf("+- 1000 deg/s\n");
    break;
  case MPU6050_RANGE_2000_DEG:
    printf("+- 2000 deg/s\n");
    break;
  }

  mpu.setFilterBandwidth(MPU6050_BAND_21_HZ);
  printf("Filter bandwidth set to: ");
  switch (mpu.getFilterBandwidth()) {
  case MPU6050_BAND_260_HZ:
    printf("260 Hz\n");
    break;
  case MPU6050_BAND_184_HZ:
    printf("184 Hz\n");
    break;
  case MPU6050_BAND_94_HZ:
    printf("94 Hz\n");
    break;
  case MPU6050_BAND_44_HZ:
    printf("44 Hz\n");
    break;
  case MPU6050_BAND_21_HZ:
    printf("21 Hz\n");
    break;
  case MPU6050_BAND_10_HZ:
    printf("10 Hz\n");
    break;
  case MPU6050_BAND_5_HZ:
    printf("5 Hz\n");
    break;
  }

  printf("\n");
}

void MPU6050_calibrate(void) {
  printf("Starting MPU6050 calibration...\n");
  printf("Keep the device stationary during calibration!\n");
  delay(2000); // Give user time to read the message
  
  const int num_samples = 100;
  float sum_x = 0, sum_y = 0, sum_z = 0;
  
  printf("Taking %d samples for calibration...\n", num_samples);
  
  for (int i = 0; i < num_samples; i++) {
    sensors_event_t a, g, temp;
    mpu.getEvent(&a, &g, &temp);
    
    sum_x += a.acceleration.x;
    sum_y += a.acceleration.y;
    sum_z += a.acceleration.z;
    
    if (i % 20 == 0) {
      printf("Sample %d/%d\n", i, num_samples);
    }
    delay(10);
  }
  
  // Calculate average readings
  float avg_x = sum_x / num_samples;
  float avg_y = sum_y / num_samples;
  float avg_z = sum_z / num_samples;
  
  // Calculate offsets
  // X and Y should be zero when stationary
  accel_offset_x = -avg_x;
  accel_offset_y = -avg_y;
  // Z should be 9.8 m/s^2 when stationary (assuming Z points up)
  accel_offset_z = 9.8 - avg_z;
  
  printf("Calibration complete!\n");
  printf("Raw averages - X: %f, Y: %f, Z: %f\n", avg_x, avg_y, avg_z);
  printf("Calculated offsets - X: %f, Y: %f, Z: %f\n", accel_offset_x, accel_offset_y, accel_offset_z);
  
  calibration_done = true;
  printf("Calibration offsets applied. Device ready for use.\n\n");
}

void MPU6050_verify_calibration(void) {
  if (!calibration_done) {
    printf("Calibration not performed yet!\n");
    return;
  }
  
  printf("Verifying calibration...\n");
  sensors_event_t a, g, temp;
  mpu.getEvent(&a, &g, &temp);
  
  float calibrated_x = a.acceleration.x + accel_offset_x;
  float calibrated_y = a.acceleration.y + accel_offset_y;
  float calibrated_z = a.acceleration.z + accel_offset_z;
  
  printf("Calibrated readings (should be ~0, 0, 9.8):\n");
  printf("X: %f (target: 0.0)\n", calibrated_x);
  printf("Y: %f (target: 0.0)\n", calibrated_y);
  printf("Z: %f (target: 9.8)\n", calibrated_z);
  
  // Check if calibration is within acceptable range
  float tolerance = 0.5; // ±0.5 m/s² tolerance
  bool x_ok = abs(calibrated_x) < tolerance;
  bool y_ok = abs(calibrated_y) < tolerance;
  bool z_ok = abs(calibrated_z - 9.8) < tolerance;
  
  printf("Calibration verification: %s\n", 
         (x_ok && y_ok && z_ok) ? "PASS" : "FAIL");
  printf("X-axis: %s, Y-axis: %s, Z-axis: %s\n\n", 
         x_ok ? "OK" : "FAIL", y_ok ? "OK" : "FAIL", z_ok ? "OK" : "FAIL");
}

void MPU6050_read(void) {
    /* Get new sensor events with the readings */
    sensors_event_t a, g, temp;
    mpu.getEvent(&a, &g, &temp);
  
    /* Apply calibration offsets if calibration has been done */
    float calibrated_x = a.acceleration.x;
    float calibrated_y = a.acceleration.y;
    float calibrated_z = a.acceleration.z;
    
    if (calibration_done) {
      calibrated_x += accel_offset_x;
      calibrated_y += accel_offset_y;
      calibrated_z += accel_offset_z;
    }
  
    /* Print out the values */
    printf("Acceleration m/s^2: \n \
    X: %f %s\n \
    Y: %f %s\n \
    Z: %f %s\n\n", 
    calibrated_x, calibration_done ? "(calibrated)" : "(raw)",
    calibrated_y, calibration_done ? "(calibrated)" : "(raw)",
    calibrated_z, calibration_done ? "(calibrated)" : "(raw)");
  
    printf("Rotation rad/s: \n \
    X: %f \n \
    Y: %f \n \
    Z: %f \n\n", g.gyro.x, g.gyro.y, g.gyro.z);
  
}

void setup(void) 
{
  Wire.begin(6,7);
  MPL115A2_setup();
  MPU6050_setup();
  
  // Perform MPU6050 calibration
  MPU6050_calibrate();
}

void loop(void) 
{
  static int loop_count = 0;
  
  MPL115A2_read();
  MPU6050_read();

  // // Verify calibration every 10 loops (every 5 seconds)
  // if (loop_count % 10 == 0 && calibration_done) {
  //   MPU6050_verify_calibration();
  // }
  
  loop_count++;
  delay(200);
}
