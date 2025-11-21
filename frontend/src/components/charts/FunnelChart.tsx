import React from 'react';
import { ChartComponentProps } from './ChartRegistry';

const FunnelChart: React.FC<ChartComponentProps> = () => {
  return (
    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', padding: '20px' }}>
      <div style={{ textAlign: 'center', color: '#888' }}>
        <div style={{ fontSize: '18px', fontWeight: 'bold' }}>Funnel Chart</div>
        <div style={{ fontSize: '14px', marginTop: '8px' }}>Chart implementation pending</div>
      </div>
    </div>
  );
};

export default FunnelChart;
