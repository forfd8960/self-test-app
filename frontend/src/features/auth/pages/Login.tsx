import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { 
  Box, 
  Button, 
  Card, 
  CardContent, 
  TextField, 
  Typography, 
  Alert, 
  Container 
} from "@mui/material";
import { api } from "../../../lib/api";
import { useAuthStore } from "../store";

export function LoginPage() {
  const navigate = useNavigate();
  const setToken = useAuthStore((state) => state.setToken);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setError(null);

    try {
      const response = await api.post<{ accessToken: string }>("/auth/login", {
        username,
        password,
      });
      setToken(response.accessToken);
      navigate("/");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Login failed");
    }
  };

  return (
    <Container component="main" maxWidth="xs">
      <Box
        sx={{
          marginTop: 8,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
        }}
      >
        <Typography component="h1" variant="h3" color="primary" gutterBottom fontWeight="bold">
          Self Test App
        </Typography>
        <Card sx={{ width: '100%', mt: 3, boxShadow: 3 }}>
          <CardContent sx={{ display: 'flex', flexDirection: 'column', gap: 2, p: 4 }}>
             <Typography component="h2" variant="h5" align="center" gutterBottom>
              Sign In
            </Typography>
            
            {error && <Alert severity="error">{error}</Alert>}
            
            <Box component="form" onSubmit={handleSubmit} noValidate sx={{ mt: 1 }}>
              <TextField
                margin="normal"
                required
                fullWidth
                id="username"
                label="Username"
                name="username"
                autoComplete="username"
                autoFocus
                value={username}
                onChange={(e) => setUsername(e.target.value)}
              />
              <TextField
                margin="normal"
                required
                fullWidth
                name="password"
                label="Password"
                type="password"
                id="password"
                autoComplete="current-password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
              <Button
                type="submit"
                fullWidth
                variant="contained"
                sx={{ mt: 3, mb: 2, py: 1.5, fontSize: '1rem' }}
              >
                Sign In
              </Button>
              <Button
                fullWidth
                variant="text"
                onClick={() => navigate('/register')}
                sx={{ textTransform: 'none' }}
              >
                Don't have an account? Sign Up
              </Button>
            </Box>
          </CardContent>
        </Card>
      </Box>
    </Container>
  );
}
