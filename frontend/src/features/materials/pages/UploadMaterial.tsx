import { useState } from "react";
import { useAuthStore } from "../../auth/store";
import {
  Container,
  Paper,
  Typography,
  Button,
  Box,
  Alert,
  CircularProgress,
} from "@mui/material";
import CloudUploadIcon from "@mui/icons-material/CloudUpload";

export function UploadMaterialPage() {
  const token = useAuthStore((state) => state.token);
  const [file, setFile] = useState<File | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      setFile(e.target.files[0]);
      setError(null);
      setSuccess(null);
    }
  };

  const handleUpload = async () => {
    if (!file) {
      setError("Select a file");
      return;
    }
    setLoading(true);
    setError(null);
    setSuccess(null);

    const body = new FormData();
    body.append("file", file);

    try {
      const baseUrl = import.meta.env.VITE_API_BASE_URL ?? "http://localhost:3000";
      const response = await fetch(`${baseUrl}/materials`, {
        method: "POST",
        body,
        headers: {
          ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
      });

      if (!response.ok) {
        let errMsg = await response.text();
        try {
            const errJson = JSON.parse(errMsg);
            if (errJson.error) errMsg = errJson.error;
        } catch {}
        throw new Error(errMsg);
      }
      
      setSuccess("File uploaded successfully!");
      setFile(null);
      // Reset the file input visually by ID
      const fileInput = document.getElementById("file-upload") as HTMLInputElement;
      if (fileInput) fileInput.value = "";
      
    } catch (err) {
      setError(err instanceof Error ? err.message : "Upload failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container maxWidth="sm" sx={{ mt: 4 }}>
      <Paper elevation={3} sx={{ p: 4, textAlign: "center" }}>
        <Typography variant="h4" gutterBottom>
          Upload Material
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          Upload your study material (PDF, Text) to generate questions.
        </Typography>

        <Box sx={{ my: 3 }}>
            <Button
              component="label"
              variant="outlined"
              startIcon={<CloudUploadIcon />}
              sx={{ mb: 2, display: 'block', mx: 'auto' }}
            >
              {file ? "Change File" : "Select File"}
              <input
                id="file-upload"
                type="file"
                hidden
                onChange={handleFileChange}
              />
            </Button>
            {file && (
                <Box>
                    <Typography variant="body1" sx={{ fontWeight: 'bold' }}>
                        {file.name}
                    </Typography>
                    <Typography variant="caption" color="text.secondary">
                        {(file.size / 1024).toFixed(2)} KB
                    </Typography>
                </Box>
            )}
        </Box>

        {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
        {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

        <Button
          variant="contained"
          onClick={handleUpload}
          disabled={!file || loading}
          size="large"
          fullWidth
        >
          {loading ? <CircularProgress size={24} color="inherit" /> : "Upload"}
        </Button>
      </Paper>
    </Container>
  );
}
