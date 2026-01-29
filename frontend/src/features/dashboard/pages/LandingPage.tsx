import React from 'react';
import { useNavigate } from 'react-router-dom';
import { 
  AppBar, 
  Toolbar, 
  Typography, 
  Button, 
  Box, 
  Container, 
  Grid,
  Card,
  CardContent,
  CssBaseline
} from '@mui/material';
import { AutoFixHigh, CloudUpload, Assessment, School } from '@mui/icons-material';

export const LandingPage = () => {
  const navigate = useNavigate();

  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
      <CssBaseline />
      
      {/* Header */}
      <AppBar position="static" color="default" elevation={1} sx={{ bgcolor: 'white' }}>
        <Toolbar>
           {/* Logo / Brand */}
           <School sx={{ display: { xs: 'none', md: 'flex' }, mr: 1, color: 'primary.main' }} />
           <Typography
            variant="h6"
            noWrap
            component="a"
            href="/"
            sx={{
              mr: 2,
              display: { xs: 'none', md: 'flex' },
              fontFamily: 'monospace',
              fontWeight: 700,
              letterSpacing: '.1rem',
              color: 'primary.main',
              textDecoration: 'none',
              flexGrow: 1
            }}
          >
            Self Test App
          </Typography>
          
          <Box sx={{ flexGrow: 1, display: { xs: 'flex', md: 'none' } }}>
             {/* Mobile spacer if needed */}
          </Box>

          <Button color="inherit" onClick={() => navigate('/login')} sx={{ mr: 1 }}>Login</Button>
          <Button variant="contained" onClick={() => navigate('/register')}>Register</Button>
        </Toolbar>
      </AppBar>

      {/* Main Content (Landing Page which contains App Features) */}
      <Box component="main" sx={{ flexGrow: 1, py: 8 }}>
        <Container maxWidth="lg">
           {/* Hero */}
           <Box sx={{ textAlign: 'center', mb: 8 }}>
             <Typography variant="h2" component="h1" gutterBottom fontWeight="bold" color="text.primary">
               Master Your Learning Materials
             </Typography>
             <Typography variant="h5" component="h2" color="text.secondary" paragraph>
               Upload documents, generate AI-powered tests, and get instant feedback to improve your knowledge.
             </Typography>
             <Button variant="contained" size="large" onClick={() => navigate('/register')} sx={{ mt: 2, px: 4, py: 1.5 }}>
               Get Started for Free
             </Button>
           </Box>

           {/* Features Grid */}
           <Grid container spacing={4} direction="column" alignItems="center">
             <Grid item xs={12} sx={{ width: '100%', maxWidth: 600 }}>
               <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column', textAlign: 'center', p: 3 }}>
                 <Box sx={{ display: 'flex', justifyContent: 'center', mb: 2 }}>
                   <CloudUpload color="primary" sx={{ fontSize: 60 }} />
                 </Box>
                 <CardContent>
                   <Typography gutterBottom variant="h5" component="h3" fontWeight="medium">
                     Upload Materials
                   </Typography>
                   <Typography color="text.secondary">
                     Support for PDF, DOCX, and TXT files. Simply upload your study guides or notes.
                   </Typography>
                 </CardContent>
               </Card>
             </Grid>
             <Grid item xs={12} sx={{ width: '100%', maxWidth: 600 }}>
               <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column', textAlign: 'center', p: 3 }}>
                 <Box sx={{ display: 'flex', justifyContent: 'center', mb: 2 }}>
                   <AutoFixHigh color="secondary" sx={{ fontSize: 60 }} />
                 </Box>
                 <CardContent>
                   <Typography gutterBottom variant="h5" component="h3" fontWeight="medium">
                     AI Generation
                   </Typography>
                   <Typography color="text.secondary">
                     Intelligent question generation tailored to your content. Multiple choice and fill-in-the-blanks.
                   </Typography>
                 </CardContent>
               </Card>
             </Grid>
             <Grid item xs={12} sx={{ width: '100%', maxWidth: 600 }}>
               <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column', textAlign: 'center', p: 3 }}>
                  <Box sx={{ display: 'flex', justifyContent: 'center', mb: 2 }}>
                   <Assessment color="success" sx={{ fontSize: 60 }} />
                 </Box>
                 <CardContent>
                   <Typography gutterBottom variant="h5" component="h3" fontWeight="medium">
                     Smart Feedback
                   </Typography>
                   <Typography color="text.secondary">
                     Get detailed scoring and constructive feedback on where to improve.
                   </Typography>
                 </CardContent>
               </Card>
             </Grid>
           </Grid>
        </Container>
      </Box>

      {/* Footer */}
      <Box component="footer" sx={{ py: 3, px: 2, mt: 'auto', backgroundColor: (theme) => theme.palette.grey[200] }}>
        <Container maxWidth="sm">
          <Typography variant="body2" color="text.secondary" align="center">
            {'Copyright © '}
            {new Date().getFullYear()}
            {' selftestapp, Inc. Built with AI.'}
          </Typography>
        </Container>
      </Box>
    </Box>
  );
};
