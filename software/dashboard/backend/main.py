from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse
import asyncio
import socket
import struct
import json
import time
from datetime import datetime
from typing import List, Dict, Any
import uvicorn
from contextlib import asynccontextmanager

# Global variables for data storage and WebSocket connections
telemetry_data: List[Dict[str, Any]] = []
connected_clients: List[WebSocket] = []

@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup: Start UDP receiver
    asyncio.create_task(start_udp_receiver())
    yield
    # Shutdown: cleanup if needed
    pass

app = FastAPI(lifespan=lifespan)

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
        # sync: u64, timestamp: u64, temperature: f32, pressure: f32,
        # humidity: f32, altitude: f32, latitude: f32, longitude: f32, status: u8
        print(f"Received {len(data)} bytes")
        unpacked = struct.unpack('<QQffffffB', data)  # Big-endian format
        self.sync = unpacked[0]
        self.timestamp = unpacked[1]
        self.temperature = unpacked[2]
        self.pressure = unpacked[3]
        self.humidity = unpacked[4]
        self.altitude = unpacked[5]
        self.latitude = unpacked[6]
        self.longitude = unpacked[7]
        self.status = unpacked[8]
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            'sync': self.sync,
            'timestamp': self.timestamp,
            'temperature': self.temperature,
            'pressure': self.pressure,
            'humidity': self.humidity,
            'altitude': self.altitude,
            'latitude': self.latitude,
            'longitude': self.longitude,
            'status': self.status,
            'received_at': datetime.now().isoformat()
        }

async def start_udp_receiver():
    """Start UDP receiver to listen for telemetry packets"""
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('127.0.0.1', 3000))
    sock.setblocking(False)
    
    print("UDP receiver started on 127.0.0.1:3000")
    
    loop = asyncio.get_event_loop()
    
    while True:
        try:
            # Wait for data with timeout
            data, addr = await loop.run_in_executor(None, sock.recvfrom, 1024)
            
            # Parse telemetry packet
            try:
                packet = TelemetryPacket(data)
                packet_dict = packet.to_dict()
                print(f"Parsed packet: {packet_dict}")
                
                # Store the data
                telemetry_data.append(packet_dict)
                
                # Keep only last 1000 records to prevent memory issues
                if len(telemetry_data) > 1000:
                    telemetry_data.pop(0)
                
                # Broadcast to all connected WebSocket clients
                await broadcast_telemetry(packet_dict)
                
                print(f"Received telemetry from {addr}: {packet_dict}")
                
            except struct.error as e:
                print(f"Error parsing telemetry packet: {e}")
                
        except BlockingIOError:
            # No data available, sleep briefly
            await asyncio.sleep(0.1)
        except Exception as e:
            print(f"UDP receiver error: {e}")
            await asyncio.sleep(1)

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
    return {"data": telemetry_data}

@app.get("/api/telemetry/latest")
async def get_latest_telemetry():
    """Get the latest telemetry packet"""
    if telemetry_data:
        return {"data": telemetry_data[-1]}
    return {"data": None}

@app.get("/api/telemetry/stats")
async def get_telemetry_stats():
    """Get telemetry statistics"""
    if not telemetry_data:
        return {"stats": None}
    
    temperatures = [d['temperature'] for d in telemetry_data]
    pressures = [d['pressure'] for d in telemetry_data]
    humidities = [d['humidity'] for d in telemetry_data]
    altitudes = [d['altitude'] for d in telemetry_data]
    
    stats = {
        "total_packets": len(telemetry_data),
        "temperature": {
            "min": min(temperatures),
            "max": max(temperatures),
            "avg": sum(temperatures) / len(temperatures)
        },
        "pressure": {
            "min": min(pressures),
            "max": max(pressures),
            "avg": sum(pressures) / len(pressures)
        },
        "humidity": {
            "min": min(humidities),
            "max": max(humidities),
            "avg": sum(humidities) / len(humidities)
        },
        "altitude": {
            "min": min(altitudes),
            "max": max(altitudes),
            "avg": sum(altitudes) / len(altitudes)
        }
    }
    
    return {"stats": stats}

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    """WebSocket endpoint for real-time telemetry updates"""
    await websocket.accept()
    connected_clients.append(websocket)
    print(f"Client connected. Total clients: {len(connected_clients)}")
    
    try:
        # Send initial data
        if telemetry_data:
            initial_message = json.dumps({
                "type": "initial_data",
                "data": telemetry_data[-50:]  # Send last 50 records
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
