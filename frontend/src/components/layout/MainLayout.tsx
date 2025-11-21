/**
 * Main Layout Component
 * Basic layout wrapper for the application
 */

import React from 'react';
import { Box, Container } from '@mui/material';

interface MainLayoutProps {
  children: React.ReactNode;
}

const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        minHeight: '100vh',
        backgroundColor: 'background.default',
      }}
    >
      <Container maxWidth={false} sx={{ flex: 1, py: 2 }}>
        {children}
      </Container>
    </Box>
  );
};

export default MainLayout;
