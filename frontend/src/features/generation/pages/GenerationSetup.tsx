import { useState, useEffect } from "react";
import { Link as RouterLink, useNavigate } from "react-router-dom";
import { useAuthStore } from "../../auth/store";
import { createGenerationJob, getGenerationJob } from "../api";
import { listMaterials, Material } from "../../materials/api";
import {
  Container,
  Paper,
  Typography,
  Button,
  Box,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Alert,
  CircularProgress,
  Grid,
} from "@mui/material";
import AutoAwesomeIcon from "@mui/icons-material/AutoAwesome";

export function GenerationSetupPage() {
  const token = useAuthStore((state) => state.token);
  const navigate = useNavigate();
  
  const [materials, setMaterials] = useState<Material[]>([]);
  const [loadingMaterials, setLoadingMaterials] = useState(false);
  
  const [materialId, setMaterialId] = useState("");
  const [mcqSingle, setMcqSingle] = useState(1);
  const [mcqMulti, setMcqMulti] = useState(1);
  const [fillBlank, setFillBlank] = useState(1);
  
  const [generationLoading, setGenerationLoading] = useState(false);
  const [jobId, setJobId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!token) return;
    
    setLoadingMaterials(true);
    listMaterials(token)
      .then((data) => {
        setMaterials(data);
        if (data.length > 0) {
            // Find first ready material
             const ready = data.find(m => m.extracted_text_status === 'ready');
             if (ready) setMaterialId(ready.id);
             else if (data.length > 0) setMaterialId(data[0].id);
        }
      })
      .catch((err) => {
        console.error("Failed to load materials", err);
        setError("Failed to load materials. Please convert ensuring backend is running.");
      })
      .finally(() => setLoadingMaterials(false));
  }, [token]);

  // Polling Effect
  useEffect(() => {
    if (!jobId || !token) return;

    const interval = setInterval(async () => {
        try {
            const job = await getGenerationJob(jobId, token);
            if (job.status === 'ready' && job.question_set_id) {
                clearInterval(interval);
                navigate(`/test/${job.question_set_id}`);
            } else if (job.status === 'failed') {
                clearInterval(interval);
                setJobId(null);
                setGenerationLoading(false);
                setError(job.error_message || "Generation failed unexpectedly.");
            }
            // If queued or generating, continue polling
        } catch (e) {
            console.error("Polling error", e);
        }
    }, 2000);

    return () => clearInterval(interval);
  }, [jobId, token, navigate]);

  const handleGenerate = async () => {
    if (!materialId) {
        setError("Please select a material first.");
        return;
    }
    
    setGenerationLoading(true);
    setError(null);

    try {
      const job = await createGenerationJob(
        {
          material_id: materialId,
          mcq_single_count: mcqSingle,
          mcq_multi_count: mcqMulti,
          fill_blank_count: fillBlank,
        },
        token
      );
      setJobId(job.id);
      // Don't turn off loading, we are now polling
    } catch (err) {
      setGenerationLoading(false);
      setError(err instanceof Error ? err.message : "Generation failed");
    }
  };

  if (loadingMaterials) {
      return (
          <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
              <CircularProgress />
          </Box>
      );
  }

  return (
    <Container maxWidth="md" sx={{ mt: 4, mb: 4 }}>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
            <AutoAwesomeIcon color="primary" sx={{ fontSize: 32, mr: 1 }} />
            <Typography variant="h4" component="h1">
            Generate Questions
            </Typography>
        </Box>
        
        <Typography variant="body1" color="text.secondary" paragraph>
          Select a study material and configure how many questions you want the AI to generate.
        </Typography>

        {error && <Alert severity="error" sx={{ mb: 3 }}>{error}</Alert>}

        {materials.length === 0 ? (
            <Alert severity="warning">
                You haven't uploaded any materials yet. 
                <Button component={RouterLink} to="/upload" sx={{ ml: 1 }}>
                    Go to Upload
                </Button>
            </Alert>
        ) : (
            <Box component="form" noValidate sx={{ mt: 1 }}>
                <FormControl fullWidth margin="normal">
                    <InputLabel id="material-select-label">Select Material</InputLabel>
                    <Select
                        labelId="material-select-label"
                        value={materialId}
                        label="Select Material"
                        onChange={(e) => setMaterialId(e.target.value)}
                    >
                        {materials.map((m) => (
                            <MenuItem key={m.id} value={m.id} disabled={m.extracted_text_status !== 'ready'}>
                                {m.original_filename} ({m.extracted_text_status})
                            </MenuItem>
                        ))}
                    </Select>
                </FormControl>

                <Typography variant="h6" sx={{ mt: 3, mb: 1 }}>
                    Configuration
                </Typography>

                <Grid container spacing={3}>
                    <Grid item xs={12} sm={4}>
                         <TextField
                            label="Single Choice (MCQ)"
                            type="number"
                            fullWidth
                            value={mcqSingle}
                            onChange={(e) => setMcqSingle(Number(e.target.value))}
                            InputProps={{ inputProps: { min: 0, max: 20 } }}
                        />
                    </Grid>
                    <Grid item xs={12} sm={4}>
                        <TextField
                            label="Multiple Choice (MCQ)"
                            type="number"
                            fullWidth
                            value={mcqMulti}
                            onChange={(e) => setMcqMulti(Number(e.target.value))}
                             InputProps={{ inputProps: { min: 0, max: 20 } }}
                        />
                    </Grid>
                    <Grid item xs={12} sm={4}>
                        <TextField
                            label="Fill in the Blank"
                            type="number"
                            fullWidth
                            value={fillBlank}
                            onChange={(e) => setFillBlank(Number(e.target.value))}
                             InputProps={{ inputProps: { min: 0, max: 20 } }}
                        />
                    </Grid>
                </Grid>

                <Button
                    fullWidth
                    variant="contained"
                    size="large"
                    onClick={handleGenerate}
                    disabled={generationLoading || !materialId}
                    sx={{ mt: 4, py: 1.5 }}
                >
                    {generationLoading ? (
                        <>
                            <CircularProgress size={24} color="inherit" sx={{ mr: 1 }} />
                            Generating...
                        </>
                    ) : (
                        "Start Generation"
                    )}
                </Button>
            </Box>
        )}
      </Paper>
    </Container>
  );
}
