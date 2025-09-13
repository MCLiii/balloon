# Balloon Telemetry Dashboard Frontend

A modern React TypeScript dashboard for monitoring balloon telemetry data in real-time.

## Features

- **Real-time Data**: WebSocket connection for live telemetry updates
- **Interactive Charts**: Historical data visualization using Recharts
- **Live Map**: Balloon location and trajectory tracking with Leaflet
- **Responsive Design**: Works on desktop and mobile devices
- **Modern UI**: Clean, professional interface with custom CSS

## Data Displayed

- Temperature (°C)
- Pressure (hPa)
- Humidity (%)
- Altitude (m)
- GPS Coordinates (Latitude/Longitude)
- Mission status and packet count

## Technologies Used

- React 18 with TypeScript
- Recharts for data visualization
- Leaflet/React-Leaflet for maps
- Lucide React for icons
- Custom CSS for styling

## Development

```bash
# Install dependencies
npm install

# Start development server
npm start

# Build for production
npm run build
```

## Production

The built files are served by the FastAPI backend at `http://localhost:8000`.

The backend expects the frontend to be built in the `build/` directory and serves it statically.