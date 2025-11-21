/**
 * Dashboard View Component
 * Renders a dashboard with all its widgets
 */

import React from 'react';
import { useParams } from 'react-router-dom';
import { Box, Typography } from '@mui/material';
import DashboardBuilder from './DashboardBuilder';

const DashboardView: React.FC = () => {
  const { id } = useParams<{ id: string }>();

  if (!id) {
    return (
      <Box sx={{ p: 3 }}>
        <Typography variant="h5">Dashboard not found</Typography>
      </Box>
    );
  }

  return <DashboardBuilder dashboardId={id} isEditMode={false} />;
};

export default DashboardView;
