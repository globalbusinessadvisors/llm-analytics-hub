/**
 * Dashboard List Component
 * Displays a list of available dashboards
 */

import React from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Box,
  Card,
  CardContent,
  CardActions,
  Button,
  Typography,
  Grid,
} from '@mui/material';
import { useDashboardStore } from '@/store/dashboardStore';

const DashboardList: React.FC = () => {
  const navigate = useNavigate();
  const dashboards = useDashboardStore((state) => Object.values(state.dashboards));

  return (
    <Box sx={{ p: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 3 }}>
        <Typography variant="h4">Dashboards</Typography>
        <Button
          variant="contained"
          color="primary"
          onClick={() => navigate('/dashboards/new')}
        >
          Create Dashboard
        </Button>
      </Box>

      <Grid container spacing={3}>
        {dashboards.map((dashboard) => (
          <Grid item xs={12} sm={6} md={4} key={dashboard.id}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  {dashboard.name}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {dashboard.description || 'No description'}
                </Typography>
                <Typography variant="caption" color="text.secondary" sx={{ mt: 1, display: 'block' }}>
                  {dashboard.widgets.length} widget{dashboard.widgets.length !== 1 ? 's' : ''}
                </Typography>
              </CardContent>
              <CardActions>
                <Button
                  size="small"
                  onClick={() => navigate(`/dashboards/${dashboard.id}`)}
                >
                  View
                </Button>
                <Button
                  size="small"
                  onClick={() => navigate(`/dashboards/${dashboard.id}/edit`)}
                >
                  Edit
                </Button>
              </CardActions>
            </Card>
          </Grid>
        ))}

        {dashboards.length === 0 && (
          <Grid item xs={12}>
            <Typography variant="body1" color="text.secondary" align="center">
              No dashboards yet. Create your first dashboard to get started.
            </Typography>
          </Grid>
        )}
      </Grid>
    </Box>
  );
};

export default DashboardList;
