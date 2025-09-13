import React, { useEffect, useState } from 'react';
import { MapContainer, TileLayer, Marker, Popup, Polyline } from 'react-leaflet';
import L from 'leaflet';
import 'leaflet/dist/leaflet.css';
import { TelemetryData } from '../types/telemetry';

// Fix for default markers in react-leaflet
delete (L.Icon.Default.prototype as any)._getIconUrl;
L.Icon.Default.mergeOptions({
  iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
  iconUrl: require('leaflet/dist/images/marker-icon.png'),
  shadowUrl: require('leaflet/dist/images/marker-shadow.png'),
});

interface TelemetryMapProps {
  data: TelemetryData[];
  latestData: TelemetryData | null;
}

const TelemetryMap: React.FC<TelemetryMapProps> = ({ data, latestData }) => {
  const [mapCenter, setMapCenter] = useState<[number, number]>([0, 0]);
  const [hasValidData, setHasValidData] = useState(false);

  useEffect(() => {
    if (latestData && latestData.latitude !== 0 && latestData.longitude !== 0) {
      setMapCenter([latestData.latitude, latestData.longitude]);
      setHasValidData(true);
    } else if (data.length > 0) {
      const validData = data.find(d => d.latitude !== 0 && d.longitude !== 0);
      if (validData) {
        setMapCenter([validData.latitude, validData.longitude]);
        setHasValidData(true);
      }
    }
  }, [data, latestData]);

  // Filter out invalid coordinates (0,0)
  const validCoordinates = data
    .filter(d => d.latitude !== 0 && d.longitude !== 0)
    .map(d => [d.latitude, d.longitude] as [number, number]);

  if (!hasValidData) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Balloon Location</h3>
        <div className="flex items-center justify-center h-64 bg-gray-100 rounded-lg">
          <p className="text-gray-500">Waiting for GPS coordinates...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <h3 className="text-lg font-semibold text-gray-900 mb-4">Balloon Location & Trajectory</h3>
      <div className="h-64 rounded-lg overflow-hidden">
        <MapContainer
          center={mapCenter}
          zoom={13}
          style={{ height: '100%', width: '100%' }}
        >
          <TileLayer
            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          />
          
          {/* Trajectory line */}
          {validCoordinates.length > 1 && (
            <Polyline
              positions={validCoordinates}
              color="#3b82f6"
              weight={3}
              opacity={0.7}
            />
          )}
          
          {/* Current position marker */}
          {latestData && latestData.latitude !== 0 && latestData.longitude !== 0 && (
            <Marker position={[latestData.latitude, latestData.longitude]}>
              <Popup>
                <div>
                  <h4 className="font-semibold">Current Position</h4>
                  <p>Lat: {latestData.latitude.toFixed(6)}</p>
                  <p>Lng: {latestData.longitude.toFixed(6)}</p>
                  <p>Alt: {latestData.altitude.toFixed(1)}m</p>
                  <p>Time: {new Date(latestData.timestamp * 1000).toLocaleString()}</p>
                </div>
              </Popup>
            </Marker>
          )}
        </MapContainer>
      </div>
    </div>
  );
};

export default TelemetryMap;
