# Telemetry Dashboard Backend

This backend service receives telemetry data via UDP and stores it in a SQLite database, with each session creating a new table for data isolation.

## Features

- **Session-based Storage**: Each telemetry session creates a new table in the SQLite database
- **Real-time WebSocket Updates**: Live telemetry data streaming to connected clients
- **REST API**: Endpoints for retrieving telemetry data and session management
- **Persistent Storage**: All telemetry data is stored in SQLite database

## Installation

1. Install dependencies:
```bash
pip install -r requirements.txt
```

2. Run the server:
```bash
python main.py
```

## Database Structure

### Sessions Table
- `id`: Unique session identifier (UUID)
- `start_time`: Session start timestamp
- `end_time`: Session end timestamp (NULL for active sessions)
- `packet_count`: Number of telemetry packets received

### Session Tables
Each session creates a table named `session_{session_id}` with the following structure:
- `id`: Auto-incrementing primary key
- `sync`: Sync field from telemetry packet
- `timestamp`: Timestamp from telemetry packet
- `temperature`: Temperature reading (Celsius)
- `accel_x`, `accel_y`, `accel_z`: Accelerometer readings (m/s²)
- `gyro_x`, `gyro_y`, `gyro_z`: Gyroscope readings (°/s)
- `status`: Status byte from telemetry packet
- `received_at`: Server timestamp when packet was received

## API Endpoints

### Telemetry Data
- `GET /api/telemetry` - Get all telemetry data from current session
- `GET /api/telemetry/latest` - Get latest telemetry packet

### Session Management
- `GET /api/sessions` - List all telemetry sessions
- `GET /api/sessions/current` - Get current session information
- `POST /api/sessions/new` - Start a new telemetry session

### WebSocket
- `WS /ws` - Real-time telemetry updates

## Database Utilities

Use the `db_utils.py` script to manage the database:

```bash
# List all sessions
python db_utils.py list

# View data for a specific session
python db_utils.py data <session_id> [number_of_records]

# Clean up old sessions (default: 30 days)
python db_utils.py cleanup [days]
```

## Configuration

- **UDP Port**: 3000 (configurable in `start_udp_receiver()`)
- **HTTP Port**: 8000 (configurable in `main()`)
- **Database**: `telemetry.db` (SQLite file)

## Telemetry Packet Format

The backend expects UDP packets with the following binary structure:
- `sync`: u64 (sync field)
- `timestamp`: u64 (Unix timestamp)
- `temperature`: f32 (Celsius)
- `accel_x`, `accel_y`, `accel_z`: f32 (m/s²)
- `gyro_x`, `gyro_y`, `gyro_z`: f32 (°/s)
- `status`: u8 (status byte)

Total packet size: 49 bytes (little-endian format)
