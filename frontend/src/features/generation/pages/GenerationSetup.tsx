import { useState, useEffect } from "react";
import { Link as RouterLink, useNavigate } from "react-router-dom";
import { useAuthStore } from "../../auth/store";
import { createGenerationJob } from "../api";
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
  Slider,
  Grid,
} from "@mui/material";
import AutoAwesomeIcon from "@mui/icons-material/AutoAwesome";

export function GenerationSetupPage() {
  const token = useAuthStore((state) => state.token);
  const navigate = useNavigate();
  
  const [materials, setMaterials] = useState<Material[]>([]);
  const [loadingMaterials, setLoadingMaterials] = useState(false);
  
  const [materialId, setMaterialId] = useState("");
  const [mcqSingle, setMcqSingle] = useState(10);
  const [mcqMulti, setMcqMulti] = useState(5);
  const [fillBlank, setFillBlank] = useState(5);
  
  const [generationLoading, setGenerationLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!token) return;
    
    setLoadingMaterials(true);
    listMaterials(token)
      .then((data) => {
        setMaterials(data);
        if (data.length > 0) {
          setMaterialId(data[0].id);
        }
      })
      .catch((err) => {
        console.error("Failed to load materials", err);
        setError("Failed to load materials. Please convert ensuring backend is running.");
      })
      .finally(() => setLoadingMaterials(false));
  }, [token]);

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
      
      // Redirect to a status page or just show success? 
      // For now, maybe redirect to a list of jobs or stay here with success message?
      // Since there is no job list page yet, let's just alert success or navigate potentially.
       navigate(`/test/${job.id}`); // Assuming we want to view the job status/test
       // NOTE: The user might want to see the job polling status first.
       // But typically we might go to a "Question Set" page. 
       // However, strictly adhering to "refine the generate tests page", I'll just show status here or navigate.
       // Let's assume we want to navigate to the test taking page if ready, or a polling page.
       // The previous implementation showed: setStatus(`Job ${job.id}: ${job.status}`);
       // I'll stick to a simple feedback for now, or maybe navigate to home.
       // The previous turn didn't mention a 'Test' page implementation other than 'GenerationSetup'.
       // I'll just show an alert for MVP.
       alert(`Job Started! ID: ${job.id}`);
       
    } catch (err) {
      setError(err instanceof Error ? err.message : "Generation failed");
    } finally {
      setGenerationLoading(false);
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
                            <MenuItem key={m.id} value={m.id}>
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
                    {generationLoading ? <CircularProgress size={24} color="inherit" /> : "Start Generation"}
                </Button>
            </Box>
        )}
      </Paper>
    </Container>
  );
}
