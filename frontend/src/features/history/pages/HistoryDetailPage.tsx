import { useEffect, useMemo, useState } from "react";
import { Link, useParams } from "react-router-dom";
import {
    Alert,
    Box,
    Card,
    CardContent,
    Chip,
    CircularProgress,
    Divider,
    Grid,
    Stack,
    Typography,
} from "@mui/material";
import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import CancelIcon from "@mui/icons-material/Cancel";
import CalendarMonthIcon from "@mui/icons-material/CalendarMonth";
import PsychologyIcon from "@mui/icons-material/Psychology";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import { useAuthStore } from "../../auth/store";
import { TestAttemptDetail, getHistoryDetail } from "../api";

export function HistoryDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { token } = useAuthStore();
  const [detail, setDetail] = useState<TestAttemptDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (token && id) {
      getHistoryDetail(token, id)
        .then((data) => {
          setDetail(data);
          setLoading(false);
        })
        .catch((err) => {
          setError("Failed to load test details");
          setLoading(false);
        });
    }
  }, [token, id]);

    const attempt = detail?.attempt;
    const answers = detail?.answers ?? [];
    const score = attempt?.score_percent ?? 0;
    const passed = score >= 70;

    const formattedDate = useMemo(() => {
        if (!attempt) return "";
        const date = new Date(attempt.started_at);
        return date.toLocaleString(undefined, {
            weekday: "long",
            year: "numeric",
            month: "long",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
        });
    }, [attempt?.started_at]);

    const feedback = useMemo(() => {
        if (!attempt?.feedback_summary) return null;
        return attempt.feedback_summary.replace(/<think>[\s\S]*?<\/think>/g, "").trim();
    }, [attempt?.feedback_summary]);

    if (loading) {
        return (
            <Box display="flex" justifyContent="center" alignItems="center" minHeight={280}>
                <CircularProgress />
            </Box>
        );
    }

    if (error || !detail || !attempt) {
        return (
            <Box maxWidth="lg" mx="auto" p={3} textAlign="center">
                <Alert severity="error">{error || "Test details not found"}</Alert>
            </Box>
        );
    }

    return (
        <Box maxWidth="lg" mx="auto" p={{ xs: 2, md: 3 }}>
            <Stack direction="row" alignItems="center" spacing={1} sx={{ mb: 2 }} component={Link} to="/history" style={{ textDecoration: "none" }}>
                <ArrowBackIcon color="action" fontSize="small" />
                <Typography variant="body2" color="text.secondary">Back to History</Typography>
            </Stack>

            <Card variant="outlined" sx={{ mb: 3, borderRadius: 3 }}>
                <Box sx={{ height: 6, bgcolor: passed ? "success.main" : "warning.main" }} />
                <CardContent>
                    <Stack direction={{ xs: "column", md: "row" }} spacing={2} justifyContent="space-between" alignItems={{ xs: "flex-start", md: "center" }}>
                        <Box>
                            <Typography variant="h4" fontWeight={700}>Test Results</Typography>
                            <Stack direction="row" spacing={1} alignItems="center" sx={{ mt: 1 }}>
                                <CalendarMonthIcon fontSize="small" color="action" />
                                <Typography variant="body2" color="text.secondary">{formattedDate}</Typography>
                            </Stack>
                        </Box>

                        <Stack direction="row" spacing={2} alignItems="center" sx={{ bgcolor: "grey.50", p: 2, borderRadius: 2, border: 1, borderColor: "grey.100" }}>
                            <Box textAlign="right">
                                <Typography variant="overline" color="text.secondary">Total Score</Typography>
                                <Typography variant="h4" fontWeight={800} color={passed ? "success.main" : "warning.main"}>
                                    {attempt.score_percent != null ? Math.round(score) : "--"}%
                                </Typography>
                            </Box>
                            {passed ? (
                                <CheckCircleIcon sx={{ fontSize: 40, color: "success.main" }} />
                            ) : (
                                <CancelIcon sx={{ fontSize: 40, color: "warning.main" }} />
                            )}
                        </Stack>
                    </Stack>

                    {feedback && (
                        <Card variant="outlined" sx={{ mt: 3, borderRadius: 2, bgcolor: "primary.50", borderColor: "primary.100" }}>
                            <CardContent>
                                <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
                                    <PsychologyIcon color="primary" fontSize="small" />
                                    <Typography variant="subtitle1" fontWeight={700} color="primary.dark">
                                        AI Feedback
                                    </Typography>
                                </Stack>
                                <Typography variant="body2" color="text.primary" sx={{ whiteSpace: "pre-wrap" }}>
                                    {feedback}
                                </Typography>
                            </CardContent>
                        </Card>
                    )}
                </CardContent>
            </Card>

            <Stack direction="row" alignItems="center" justifyContent="space-between" sx={{ mb: 2 }}>
                <Typography variant="h6" fontWeight={700}>Question Review</Typography>
                <Chip label={`${answers.filter(a => a.is_correct).length} / ${answers.length} Correct`} variant="outlined" />
            </Stack>

            <Stack spacing={2}>
                {answers.map((ans, idx) => (
                    <Card key={ans.question_id} variant="outlined" sx={{ borderRadius: 3, overflow: "hidden" }}>
                        <CardContent>
                            <Stack direction="row" justifyContent="space-between" alignItems="center" sx={{ mb: 1 }}>
                                <Typography variant="overline" color="text.secondary">Question {idx + 1}</Typography>
                                <Chip
                                    size="small"
                                    color={ans.is_correct ? "success" : "error"}
                                    label={ans.is_correct ? "Correct" : "Incorrect"}
                                />
                            </Stack>

                            <Typography variant="subtitle1" fontWeight={600} sx={{ mb: 2 }}>
                                {ans.prompt}
                            </Typography>

                            <Grid container spacing={2}>
                                <Grid item xs={12} md={ans.is_correct ? 12 : 6}>
                                    <Typography variant="caption" color="text.secondary" display="block" sx={{ mb: 0.5 }}>
                                        Your Answer
                                    </Typography>
                                    <Box sx={{ p: 1.5, borderRadius: 2, border: 1, borderColor: ans.is_correct ? "success.light" : "error.light", bgcolor: ans.is_correct ? "success.50" : "error.50" }}>
                                        <Typography variant="body2" color="text.primary">
                                            {ans.user_response || "No answer provided"}
                                        </Typography>
                                    </Box>
                                </Grid>

                                {!ans.is_correct && (
                                    <Grid item xs={12} md={6}>
                                        <Typography variant="caption" color="text.secondary" display="block" sx={{ mb: 0.5 }}>
                                            Correct Answer
                                        </Typography>
                                        <Box sx={{ p: 1.5, borderRadius: 2, border: 1, borderColor: "grey.200", bgcolor: "grey.50" }}>
                                            <Typography variant="body2" color="text.primary">{ans.correct_answer}</Typography>
                                        </Box>
                                    </Grid>
                                )}
                            </Grid>

                            {ans.explanation && (
                                <>
                                    <Divider sx={{ my: 2 }} />
                                    <Stack direction="row" spacing={1} alignItems="flex-start">
                                        <InfoOutlinedIcon color="warning" fontSize="small" />
                                        <Box>
                                            <Typography variant="subtitle2" fontWeight={600} color="warning.dark">
                                                Explanation
                                            </Typography>
                                            <Typography variant="body2" color="text.secondary">
                                                {ans.explanation}
                                            </Typography>
                                        </Box>
                                    </Stack>
                                </>
                            )}
                        </CardContent>
                    </Card>
                ))}
            </Stack>
        </Box>
    );
}
