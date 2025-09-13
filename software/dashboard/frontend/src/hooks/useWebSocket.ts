import { useEffect, useRef, useState } from 'react';
import { TelemetryData, WebSocketMessage } from '../types/telemetry';

export const useWebSocket = (url: string) => {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [telemetryData, setTelemetryData] = useState<TelemetryData[]>([]);
  const [latestData, setLatestData] = useState<TelemetryData | null>(null);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    const connect = () => {
      try {
        const ws = new WebSocket(url);
        
        ws.onopen = () => {
          console.log('WebSocket connected');
          setIsConnected(true);
          setSocket(ws);
        };

        ws.onmessage = (event) => {
          try {
            const message: WebSocketMessage = JSON.parse(event.data);
            
            if (message.type === 'initial_data' && Array.isArray(message.data)) {
              setTelemetryData(message.data);
              if (message.data.length > 0) {
                setLatestData(message.data[message.data.length - 1]);
              }
            } else if (message.type === 'telemetry' && !Array.isArray(message.data)) {
              setTelemetryData(prev => {
                const newData = [...prev, message.data as TelemetryData];
                // Keep only last 100 records for performance
                return newData.slice(-100);
              });
              setLatestData(message.data as TelemetryData);
            }
          } catch (error) {
            console.error('Error parsing WebSocket message:', error);
          }
        };

        ws.onclose = () => {
          console.log('WebSocket disconnected');
          setIsConnected(false);
          setSocket(null);
          
          // Reconnect after 3 seconds
          reconnectTimeoutRef.current = setTimeout(() => {
            console.log('Attempting to reconnect...');
            connect();
          }, 3000);
        };

        ws.onerror = (error) => {
          console.error('WebSocket error:', error);
        };

      } catch (error) {
        console.error('Failed to create WebSocket connection:', error);
        // Retry after 3 seconds
        reconnectTimeoutRef.current = setTimeout(() => {
          connect();
        }, 3000);
      }
    };

    connect();

    return () => {
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
      if (socket) {
        socket.close();
      }
    };
  }, [url]);

  return {
    socket,
    isConnected,
    telemetryData,
    latestData
  };
};
