from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse
import asyncio
import socket
import struct
import json
import time
import aiosqlite
import uuid
from datetime import datetime
from typing import List, Dict, Any
import uvicorn
from contextlib import asynccontextmanager

# Global variables for data storage and WebSocket connections
connected_clients: List[WebSocket] = []
current_session_id: str = ""
db_path = "telemetry.db"

@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup: Initialize database and start UDP receiver
    await init_database()
    await start_new_session()
    asyncio.create_task(start_udp_receiver())
    yield
    # Shutdown: cleanup if needed
    pass

app = FastAPI(lifespan=lifespan)

async def init_database():
    """Initialize the SQLite database"""
    async with aiosqlite.connect(db_path) as db:
        await db.execute('''
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                start_time TEXT NOT NULL,
                end_time TEXT,
                packet_count INTEGER DEFAULT 0
            )
        ''')
        await db.commit()

async def start_new_session():
    """Start a new telemetry session and create a new table"""
    global current_session_id
    current_session_id = str(uuid.uuid4())
    session_start = datetime.now().isoformat()
    
    # Create session record
    async with aiosqlite.connect(db_path) as db:
        await db.execute(
            'INSERT INTO sessions (id, start_time) VALUES (?, ?)',
            (current_session_id, session_start)
        )
        
        # Create table for this session
        table_name = f"session_{current_session_id.replace('-', '_')}"
        await db.execute(f'''
            CREATE TABLE IF NOT EXISTS {table_name} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sync TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                temperature REAL NOT NULL,
                accel_x REAL NOT NULL,
                accel_y REAL NOT NULL,
                accel_z REAL NOT NULL,
                gyro_x REAL NOT NULL,
                gyro_y REAL NOT NULL,
                gyro_z REAL NOT NULL,
                status INTEGER NOT NULL,
                received_at TEXT NOT NULL
            )
        ''')
        await db.commit()
    
    print(f"Started new session: {current_session_id}")
    print(f"Created table: session_{current_session_id.replace('-', '_')}")

async def insert_telemetry_data(data: Dict[str, Any]):
    """Insert telemetry data into the current session's table"""
    table_name = f"session_{current_session_id.replace('-', '_')}"
    
    async with aiosqlite.connect(db_path) as db:
        await db.execute(f'''
            INSERT INTO {table_name} 
            (sync, timestamp, temperature, accel_x, accel_y, accel_z, 
             gyro_x, gyro_y, gyro_z, status, received_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ''', (
            str(data['sync']), str(data['timestamp']), data['temperature'],
            data['accel_x'], data['accel_y'], data['accel_z'],
            data['gyro_x'], data['gyro_y'], data['gyro_z'],
            data['status'], data['received_at']
        ))
        
        # Update session packet count
        await db.execute(
            'UPDATE sessions SET packet_count = packet_count + 1 WHERE id = ?',
            (current_session_id,)
        )
        
        await db.commit()

async def get_telemetry_data(limit: int = 1000):
    """Get telemetry data from the current session"""
    table_name = f"session_{current_session_id.replace('-', '_')}"
    
    async with aiosqlite.connect(db_path) as db:
        async with db.execute(f'''
            SELECT sync, timestamp, temperature, accel_x, accel_y, accel_z,
                   gyro_x, gyro_y, gyro_z, status, received_at
            FROM {table_name}
            ORDER BY id DESC
            LIMIT ?
        ''', (limit,)) as cursor:
            rows = await cursor.fetchall()
            
            return [
                {
                    'sync': int(row[0]),
                    'timestamp': int(row[1]),
                    'temperature': row[2],
                    'accel_x': row[3],
                    'accel_y': row[4],
                    'accel_z': row[5],
                    'gyro_x': row[6],
                    'gyro_y': row[7],
                    'gyro_z': row[8],
                    'status': row[9],
                    'received_at': row[10]
                }
                for row in rows
            ]

async def get_latest_telemetry_data():
    """Get the latest telemetry data from the current session"""
    table_name = f"session_{current_session_id.replace('-', '_')}"
    
    async with aiosqlite.connect(db_path) as db:
        async with db.execute(f'''
            SELECT sync, timestamp, temperature, accel_x, accel_y, accel_z,
                   gyro_x, gyro_y, gyro_z, status, received_at
            FROM {table_name}
            ORDER BY id DESC
            LIMIT 1
        ''') as cursor:
            row = await cursor.fetchone()
            
            if row:
                return {
                    'sync': int(row[0]),
                    'timestamp': int(row[1]),
                    'temperature': row[2],
                    'accel_x': row[3],
                    'accel_y': row[4],
                    'accel_z': row[5],
                    'gyro_x': row[6],
                    'gyro_y': row[7],
                    'gyro_z': row[8],
                    'status': row[9],
                    'received_at': row[10]
                }
            return None

# Enable CORS for React frontend
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class TelemetryPacket:
    """Represents the telemetry packet structure from Rust code"""
    def __init__(self, data: bytes):
        # Unpack the binary data according to the Rust struct
        # sync: u64, timestamp: u64, temperature: f32,
        # humidity: f32, altitude: f32, latitude: f32, longitude: f32,
        # accel_x: f32, accel_y: f32, accel_z: f32,
        # gyro_x: f32, gyro_y: f32, gyro_z: f32, status: u8
        unpacked = struct.unpack('<QQfffffffB', data)  # Little-endian format
        self.sync = unpacked[0]
        self.timestamp = unpacked[1]
        self.temperature = unpacked[2]
        # self.humidity = unpacked[3]
        # self.altitude = unpacked[4]
        # self.latitude = unpacked[5]
        # self.longitude = unpacked[6]
        self.accel_x = unpacked[3]
        self.accel_y = unpacked[4]
        self.accel_z = unpacked[5]
        self.gyro_x = unpacked[6]
        self.gyro_y = unpacked[7]
        self.gyro_z = unpacked[8]
        self.status = unpacked[9]
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            'sync': self.sync,
            'timestamp': self.timestamp,
            'temperature': self.temperature,
            # 'humidity': self.humidity,
            # 'altitude': self.altitude,
            # 'latitude': self.latitude,
            # 'longitude': self.longitude,
            'accel_x': self.accel_x,
            'accel_y': self.accel_y,
            'accel_z': self.accel_z,
            'gyro_x': self.gyro_x,
            'gyro_y': self.gyro_y,
            'gyro_z': self.gyro_z,
            'status': self.status,
            'received_at': datetime.now().isoformat()
        }

async def start_udp_receiver():
    """Start UDP receiver to listen for telemetry packets"""
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('0.0.0.0', 3000))
    sock.setblocking(False)
    
    loop = asyncio.get_event_loop()
    
    while True:
        try:
            # Wait for data with timeout
            data, addr = await loop.run_in_executor(None, sock.recvfrom, 1024)
            
            # Parse telemetry packet
            try:
                packet = TelemetryPacket(data)
                packet_dict = packet.to_dict()
                
                # Store the data in database
                await insert_telemetry_data(packet_dict)
                
                # Broadcast to all connected WebSocket clients
                await broadcast_telemetry(packet_dict)
                
            except struct.error as e:
                print(f"Error parsing telemetry packet: {e}")
                
        except BlockingIOError:
            # No data available, sleep briefly
            await asyncio.sleep(0.1)
        except Exception as e:
            print(f"UDP receiver error: {e}")
            await asyncio.sleep(0.1)

async def broadcast_telemetry(data: Dict[str, Any]):
    """Broadcast telemetry data to all connected WebSocket clients"""
    if connected_clients:
        message = json.dumps({
            "type": "telemetry",
            "data": data
        })
        
        # Send to all connected clients
        disconnected_clients = []
        for client in connected_clients:
            try:
                await client.send_text(message)
            except:
                disconnected_clients.append(client)
        
        # Remove disconnected clients
        for client in disconnected_clients:
            connected_clients.remove(client)

@app.get("/api/telemetry")
async def get_telemetry():
    """Get all telemetry data"""
    data = await get_telemetry_data()
    return {"data": data}

@app.get("/api/telemetry/latest")
async def get_latest_telemetry():
    """Get the latest telemetry packet"""
    data = await get_latest_telemetry_data()
    return {"data": data}

@app.post("/api/sessions/new")
async def start_new_session_endpoint():
    """Start a new telemetry session"""
    await start_new_session()
    return {"session_id": current_session_id, "message": "New session started"}

@app.get("/api/sessions")
async def get_sessions():
    """Get all telemetry sessions"""
    async with aiosqlite.connect(db_path) as db:
        async with db.execute('''
            SELECT id, start_time, end_time, packet_count 
            FROM sessions 
            ORDER BY start_time DESC
        ''') as cursor:
            rows = await cursor.fetchall()
            
            return {
                "sessions": [
                    {
                        "id": row[0],
                        "start_time": row[1],
                        "end_time": row[2],
                        "packet_count": row[3]
                    }
                    for row in rows
                ]
            }

@app.get("/api/sessions/current")
async def get_current_session():
    """Get current session information"""
    async with aiosqlite.connect(db_path) as db:
        async with db.execute(
            'SELECT id, start_time, end_time, packet_count FROM sessions WHERE id = ?',
            (current_session_id,)
        ) as cursor:
            row = await cursor.fetchone()
            
            if row:
                return {
                    "session": {
                        "id": row[0],
                        "start_time": row[1],
                        "end_time": row[2],
                        "packet_count": row[3]
                    }
                }
            return {"session": None}

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    """WebSocket endpoint for real-time telemetry updates"""
    await websocket.accept()
    connected_clients.append(websocket)
    print(f"Client connected. Total clients: {len(connected_clients)}")
    
    try:
        # Send initial data
        initial_data = await get_telemetry_data(limit=50)
        if initial_data:
            initial_message = json.dumps({
                "type": "initial_data",
                "data": initial_data
            })
            await websocket.send_text(initial_message)
        
        # Keep connection alive
        while True:
            try:
                # Wait for any message from client (ping/pong)
                await websocket.receive_text()
            except WebSocketDisconnect:
                break
                
    except WebSocketDisconnect:
        pass
    finally:
        connected_clients.remove(websocket)
        print(f"Client disconnected. Total clients: {len(connected_clients)}")

# Serve React frontend
app.mount("/static", StaticFiles(directory="../frontend/build/static"), name="static")

@app.get("/")
async def serve_frontend():
    """Serve the React frontend"""
    return FileResponse("../frontend/build/index.html")

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
