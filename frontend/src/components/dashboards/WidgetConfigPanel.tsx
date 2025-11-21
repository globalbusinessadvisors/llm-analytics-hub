/**
 * Widget Configuration Panel
 * Panel for editing widget configuration
 */

import React, { useState } from 'react';
import {
  Drawer,
  Box,
  Typography,
  TextField,
  Button,
  Divider,
} from '@mui/material';
import { WidgetConfig } from '@/types/dashboard';

interface WidgetConfigPanelProps {
  widget: WidgetConfig;
  open: boolean;
  onClose: () => void;
  onSave: (config: Partial<WidgetConfig>) => void;
}

const WidgetConfigPanel: React.FC<WidgetConfigPanelProps> = ({
  widget,
  open,
  onClose,
  onSave,
}) => {
  const [title, setTitle] = useState(widget.title);
  const [description, setDescription] = useState(widget.description || '');

  const handleSave = () => {
    onSave({
      title,
      description,
    });
  };

  return (
    <Drawer anchor="right" open={open} onClose={onClose}>
      <Box sx={{ width: 400, p: 3 }}>
        <Typography variant="h6" gutterBottom>
          Widget Configuration
        </Typography>

        <Divider sx={{ my: 2 }} />

        <TextField
          fullWidth
          label="Title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          margin="normal"
        />

        <TextField
          fullWidth
          label="Description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          margin="normal"
          multiline
          rows={3}
        />

        <Box sx={{ mt: 3, display: 'flex', gap: 2 }}>
          <Button variant="contained" color="primary" onClick={handleSave}>
            Save
          </Button>
          <Button variant="outlined" onClick={onClose}>
            Cancel
          </Button>
        </Box>
      </Box>
    </Drawer>
  );
};

export default WidgetConfigPanel;
