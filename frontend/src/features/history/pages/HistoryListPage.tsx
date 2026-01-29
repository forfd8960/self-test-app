import { useEffect, useMemo, useState } from "react";
import { Link } from "react-router-dom";
import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  Chip,
  CircularProgress,
  Divider,
  Stack,
  Typography,
} from "@mui/material";
import Grid from "@mui/material/Grid";
import HistoryEduIcon from "@mui/icons-material/HistoryEdu";
import CalendarMonthIcon from "@mui/icons-material/CalendarMonth";
import AccessTimeIcon from "@mui/icons-material/AccessTime";
import ArrowForwardIcon from "@mui/icons-material/ArrowForward";
import { useAuthStore } from "../../auth/store";
import { TestAttempt, getHistory } from "../api";

export function HistoryListPage() {
  const { token } = useAuthStore();
  const [history, setHistory] = useState<TestAttempt[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (token) {
      getHistory(token)
        .then((data) => {
          setHistory(data);
          setLoading(false);
        })
        .catch((err) => {
          setError("Failed to load history");
          setLoading(false);
        });
    }
  }, [token]);

  const completedCount = useMemo(() => history.filter((a) => a.submitted_at).length, [history]);

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight={280}>
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Box maxWidth="lg" mx="auto" p={3} textAlign="center">
        <Alert severity="error">{error}</Alert>
      </Box>
    );
  }

  return (
    <Box maxWidth="lg" mx="auto" p={{ xs: 2, md: 3 }}>
      <Stack spacing={1} sx={{ mb: 3 }}>
        <Typography variant="h4" fontWeight={800}>Your Learning History</Typography>
        <Typography variant="body2" color="text.secondary">
          Track your progress and review past attempts.
        </Typography>
        <Stack direction="row" spacing={1} alignItems="center">
          <Chip label={`${history.length} Attempts`} size="small" variant="outlined" />
          <Chip label={`${completedCount} Completed`} size="small" color="success" variant="outlined" />
        </Stack>
      </Stack>

      {history.length === 0 ? (
        <Card variant="outlined" sx={{ borderRadius: 3 }}>
          <CardContent>
            <Stack spacing={2} alignItems="center" textAlign="center" sx={{ py: 4 }}>
              <HistoryEduIcon color="action" sx={{ fontSize: 48 }} />
              <Box>
                <Typography variant="h6" fontWeight={700}>No tests taken yet</Typography>
                <Typography variant="body2" color="text.secondary">
                  Generate a test to start tracking your progress.
                </Typography>
              </Box>
              <Button component={Link} to="/generate" variant="contained" startIcon={<HistoryEduIcon />}>
                Create New Test
              </Button>
            </Stack>
          </CardContent>
        </Card>
      ) : (
        <Stack spacing={2}>
          {history.map((attempt) => {
            const date = new Date(attempt.started_at);
            const score = attempt.score_percent;
            const passed = (score || 0) >= 70;

            return (
              <Card key={attempt.id} variant="outlined" sx={{ borderRadius: 3 }}>
                <CardContent>
                  <Grid container spacing={2} alignItems="center">
                    <Grid item xs={12} md={8}>
                      <Stack spacing={1}>
                        <Stack direction="row" spacing={1} alignItems="center">
                          <Typography variant="subtitle1" fontWeight={700}>
                            Test Attempt
                          </Typography>
                          <Chip
                            size="small"
                            color={attempt.submitted_at ? "success" : "warning"}
                            label={attempt.submitted_at ? "Completed" : "In Progress"}
                          />
                        </Stack>

                        <Stack direction={{ xs: "column", sm: "row" }} spacing={2} alignItems={{ xs: "flex-start", sm: "center" }}>
                          <Stack direction="row" spacing={0.5} alignItems="center">
                            <CalendarMonthIcon fontSize="small" color="action" />
                            <Typography variant="body2" color="text.secondary">
                              {date.toLocaleDateString()}
                            </Typography>
                          </Stack>
                          <Stack direction="row" spacing={0.5} alignItems="center">
                            <AccessTimeIcon fontSize="small" color="action" />
                            <Typography variant="body2" color="text.secondary">
                              {date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}
                            </Typography>
                          </Stack>
                        </Stack>
                      </Stack>
                    </Grid>

                    <Grid item xs={12} md={4}>
                      <Stack direction="row" spacing={2} alignItems="center" justifyContent={{ xs: "flex-start", md: "flex-end" }}>
                        <Box textAlign={{ xs: "left", md: "right" }}>
                          <Typography variant="caption" color="text.secondary">Score</Typography>
                          <Typography variant="h5" fontWeight={800} color={score == null ? "text.disabled" : passed ? "success.main" : "error.main"}>
                            {score == null ? "--" : `${Math.round(score)}%`}
                          </Typography>
                        </Box>
                        <Button
                          component={Link}
                          to={`/history/${attempt.id}`}
                          variant="outlined"
                          endIcon={<ArrowForwardIcon />}
                          size="small"
                        >
                          View Details
                        </Button>
                      </Stack>
                    </Grid>
                  </Grid>

                  <Divider sx={{ mt: 2 }} />
                </CardContent>
              </Card>
            );
          })}
        </Stack>
      )}
    </Box>
  );
}
