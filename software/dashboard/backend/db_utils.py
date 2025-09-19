#!/usr/bin/env python3
"""
Database utility script for telemetry data management
"""

import asyncio
import aiosqlite
import sys
from datetime import datetime

DB_PATH = "telemetry.db"

async def list_sessions():
    """List all telemetry sessions"""
    async with aiosqlite.connect(DB_PATH) as db:
        async with db.execute('''
            SELECT id, start_time, end_time, packet_count 
            FROM sessions 
            ORDER BY start_time DESC
        ''') as cursor:
            rows = await cursor.fetchall()
            
            if not rows:
                print("No sessions found.")
                return
            
            print("Telemetry Sessions:")
            print("-" * 80)
            for row in rows:
                session_id, start_time, end_time, packet_count = row
                print(f"Session ID: {session_id}")
                print(f"Start Time: {start_time}")
                print(f"End Time: {end_time or 'Active'}")
                print(f"Packet Count: {packet_count}")
                print("-" * 80)

async def get_session_data(session_id: str, limit: int = 10):
    """Get telemetry data for a specific session"""
    table_name = f"session_{session_id.replace('-', '_')}"
    
    async with aiosqlite.connect(DB_PATH) as db:
        # Check if table exists
        async with db.execute("SELECT name FROM sqlite_master WHERE type='table' AND name=?", (table_name,)) as cursor:
            table_exists = await cursor.fetchone()
            
        if not table_exists:
            print(f"Session table {table_name} does not exist.")
            return
        
        # Get data
        async with db.execute(f'''
            SELECT id, sync, timestamp, temperature, accel_x, accel_y, accel_z,
                   gyro_x, gyro_y, gyro_z, status, received_at
            FROM {table_name}
            ORDER BY id DESC
            LIMIT ?
        ''', (limit,)) as cursor:
            rows = await cursor.fetchall()
            
            if not rows:
                print(f"No data found for session {session_id}")
                return
            
            print(f"Latest {limit} telemetry records for session {session_id}:")
            print("-" * 140)
            print(f"{'ID':<5} {'Sync':<20} {'Timestamp':<12} {'Temp':<8} {'Accel X':<8} {'Accel Y':<8} {'Accel Z':<8} {'Gyro X':<8} {'Gyro Y':<8} {'Gyro Z':<8} {'Status':<6} {'Received':<20}")
            print("-" * 140)
            
            for row in rows:
                id_val, sync, timestamp, temp, accel_x, accel_y, accel_z, gyro_x, gyro_y, gyro_z, status, received_at = row
                print(f"{id_val:<5} {sync:<20} {timestamp:<12} {temp:<8.2f} {accel_x:<8.2f} {accel_y:<8.2f} {accel_z:<8.2f} {gyro_x:<8.1f} {gyro_y:<8.1f} {gyro_z:<8.1f} {status:<6} {received_at:<20}")

async def cleanup_old_sessions(days: int = 30):
    """Clean up sessions older than specified days"""
    cutoff_date = datetime.now().replace(hour=0, minute=0, second=0, microsecond=0)
    cutoff_date = cutoff_date.replace(day=cutoff_date.day - days)
    cutoff_str = cutoff_date.isoformat()
    
    async with aiosqlite.connect(DB_PATH) as db:
        # Get old sessions
        async with db.execute('''
            SELECT id FROM sessions 
            WHERE start_time < ? AND end_time IS NOT NULL
        ''', (cutoff_str,)) as cursor:
            old_sessions = await cursor.fetchall()
        
        if not old_sessions:
            print(f"No sessions older than {days} days found.")
            return
        
        print(f"Found {len(old_sessions)} sessions older than {days} days:")
        
        for (session_id,) in old_sessions:
            table_name = f"session_{session_id.replace('-', '_')}"
            print(f"  - {session_id}")
            
            # Drop the session table
            await db.execute(f"DROP TABLE IF EXISTS {table_name}")
        
        # Delete session records
        await db.execute('''
            DELETE FROM sessions 
            WHERE start_time < ? AND end_time IS NOT NULL
        ''', (cutoff_str,))
        
        await db.commit()
        print(f"Cleaned up {len(old_sessions)} old sessions and their data.")

async def main():
    """Main function to handle command line arguments"""
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python db_utils.py list                    - List all sessions")
        print("  python db_utils.py data <session_id> [n]   - Show data for session (n records, default 10)")
        print("  python db_utils.py cleanup [days]          - Clean up sessions older than days (default 30)")
        return
    
    command = sys.argv[1].lower()
    
    if command == "list":
        await list_sessions()
    elif command == "data":
        if len(sys.argv) < 3:
            print("Please provide session ID")
            return
        session_id = sys.argv[2]
        limit = int(sys.argv[3]) if len(sys.argv) > 3 else 10
        await get_session_data(session_id, limit)
    elif command == "cleanup":
        days = int(sys.argv[2]) if len(sys.argv) > 2 else 30
        await cleanup_old_sessions(days)
    else:
        print(f"Unknown command: {command}")

if __name__ == "__main__":
    asyncio.run(main())
