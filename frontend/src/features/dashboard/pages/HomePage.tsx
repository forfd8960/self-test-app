import React from 'react';
import { 
  Box, 
  Card, 
  CardContent, 
  CardActions, 
  Typography, 
  Button, 
  Grid 
} from '@mui/material';
import { 
  CloudUpload as UploadIcon,
  AutoFixHigh as GenerateIcon,
  EmojiEvents as TrophyIcon
} from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../../auth/store';

export const HomePage = () => {
  const navigate = useNavigate();
  const token = useAuthStore(s => s.token);

  return (
    <Box>
      <Typography variant="h4" gutterBottom component="div" sx={{ mb: 4 }}>
        {token ? "Welcome Back!" : "Welcome to Self Test App"}
      </Typography>
      {!token && (
         <Typography variant="body1" sx={{ mb: 4 }}>
           Please login to upload materials and generate tests.
         </Typography>
      )}
      
      <Grid container spacing={3}>
        {/* Quick Actions */}
        <Grid item xs={12} md={4}>
          <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
            <CardContent>
              <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                <UploadIcon color="primary" sx={{ fontSize: 40, mr: 2 }} />
                <Typography variant="h6">Upload Materials</Typography>
              </Box>
              <Typography variant="body2" color="text.secondary">
                Upload new learning materials (PDF, DOCX) to generate questions from.
              </Typography>
            </CardContent>
            <CardActions sx={{ mt: 'auto' }}>
              <Button size="small" onClick={() => navigate('/upload')}>Go to Upload</Button>
            </CardActions>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
            <CardContent>
              <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                <GenerateIcon color="secondary" sx={{ fontSize: 40, mr: 2 }} />
                <Typography variant="h6">Generate Test</Typography>
              </Box>
              <Typography variant="body2" color="text.secondary">
                Create a new personalized test based on your uploaded materials.
              </Typography>
            </CardContent>
            <CardActions sx={{ mt: 'auto' }}>
              <Button size="small" onClick={() => navigate('/generate')}>Create Test</Button>
            </CardActions>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
            <CardContent>
              <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                <TrophyIcon color="warning" sx={{ fontSize: 40, mr: 2 }} />
                <Typography variant="h6">Recent Results</Typography>
              </Box>
              <Typography variant="body2" color="text.secondary">
                View your recent test scores and AI feedback.
              </Typography>
            </CardContent>
            <CardActions sx={{ mt: 'auto' }}>
              <Button size="small" onClick={() => navigate('/history')}>View History</Button>
            </CardActions>
          </Card>
        </Grid>
      </Grid>

      {/* Placeholder for Recent Activity Table */}
      <Box sx={{ mt: 6 }}>
        <Typography variant="h5" gutterBottom>
          Recent Activity
        </Typography>
        <Typography variant="body1" color="text.secondary">
          No recent tests found. Start by uploading a material!
        </Typography>
      </Box>
    </Box>
  );
};
