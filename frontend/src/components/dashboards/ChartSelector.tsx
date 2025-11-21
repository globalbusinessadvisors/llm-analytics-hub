/**
 * Chart Selector Component
 * Modal for selecting chart type when adding a new widget
 */

import React from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  Grid,
  Card,
  CardContent,
  Typography,
  Box,
} from '@mui/material';
import { ChartType } from '@/types/dashboard';
import { getAvailableChartTypes, getChartMetadata } from '../charts/ChartRegistry';

interface ChartSelectorProps {
  open: boolean;
  onClose: () => void;
  onSelect: (chartType: ChartType) => void;
}

const ChartSelector: React.FC<ChartSelectorProps> = ({ open, onClose, onSelect }) => {
  const chartTypes = getAvailableChartTypes();

  const handleSelect = (chartType: ChartType) => {
    onSelect(chartType);
    onClose();
  };

  return (
    <Dialog open={open} onClose={onClose} maxWidth="md" fullWidth>
      <DialogTitle>Select Chart Type</DialogTitle>
      <DialogContent>
        <Grid container spacing={2} sx={{ mt: 1 }}>
          {chartTypes.map((type) => {
            const metadata = getChartMetadata(type);
            return (
              <Grid item xs={12} sm={6} md={4} key={type}>
                <Card
                  sx={{
                    cursor: 'pointer',
                    '&:hover': {
                      boxShadow: 4,
                      transform: 'translateY(-2px)',
                      transition: 'all 0.2s',
                    },
                  }}
                  onClick={() => handleSelect(type)}
                >
                  <CardContent>
                    <Typography variant="h6" gutterBottom>
                      {metadata?.name || type}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      {metadata?.description || 'No description'}
                    </Typography>
                    {metadata?.category && (
                      <Box sx={{ mt: 1 }}>
                        <Typography variant="caption" color="primary">
                          {metadata.category}
                        </Typography>
                      </Box>
                    )}
                  </CardContent>
                </Card>
              </Grid>
            );
          })}
        </Grid>
      </DialogContent>
    </Dialog>
  );
};

export default ChartSelector;
