import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useAuthStore } from '../../auth/store';
import { getQuestions, Question } from '../../generation/api';
import { submitTest, TestResultResponse } from '../api';
import { ScoreSummary } from '../components/ScoreSummary';
import {
  Box,
  Container,
  Paper,
  Typography,
  Radio,
  RadioGroup,
  FormControlLabel,
  Checkbox,
  Button,
  CircularProgress,
  Alert,
  Chip,
  Divider,
  TextField,
  FormGroup,
} from '@mui/material';

export const TakeTest = () => {
  const { setId } = useParams();
  const token = useAuthStore(s => s.token);
  const [questions, setQuestions] = useState<Question[]>([]);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [result, setResult] = useState<TestResultResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const navigate = useNavigate();

  useEffect(() => {
    if (setId && token) {
        setLoading(true);
      getQuestions(setId, token)
        .then(setQuestions)
        .catch(console.error)
        .finally(() => setLoading(false));
    }
  }, [setId, token]);

  const handleAnswerChange = (questionId: string, value: string) => {
    setAnswers(prev => ({ ...prev, [questionId]: value }));
  };

  const handleMultiSelectChange = (questionId: string, option: string, checked: boolean) => {
    const current = answers[questionId] ? answers[questionId].split(',') : [];
    let next: string[];
    if (checked) {
      next = [...current, option];
    } else {
      next = current.filter(o => o !== option);
    }
    setAnswers(prev => ({ ...prev, [questionId]: next.join(',') }));
  };

  // Helper to check if multi-select option is checked
  const isMultiSelected = (questionId: string, option: string) => {
      const resp = answers[questionId] || '';
      return resp.split(',').includes(option);
  };

  const handleSubmit = async () => {
    if (!setId || !token) return;
    setSubmitting(true);
    const submission = {
        question_set_id: setId,
        answers: Object.entries(answers).map(([qid, resp]) => ({
            question_id: qid,
            response: resp
        }))
    };
    try {
        const res = await submitTest(token, submission);
        setResult(res);
    } catch (e) {
        alert("Failed to submit test");
        console.error(e);
    } finally {
        setSubmitting(false);
    }
  };

  if (result) return <ScoreSummary result={result} onHome={() => navigate('/')} />;
  
  if (loading) {
      return (
          <Box sx={{ display: 'flex', justifyContent: 'center', mt: 8 }}>
              <CircularProgress />
          </Box>
      );
  }

  if (!questions.length) {
      return (
        <Container maxWidth="md" sx={{ mt: 4 }}>
            <Alert severity="warning">No questions found for this test.</Alert>
            <Button onClick={() => navigate('/')} sx={{ mt: 2 }}>Back Home</Button>
        </Container>
      );
  }

  return (
    <Container maxWidth="md" sx={{ my: 4 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 4 }}>
        <Typography variant="h4" component="h1">
           Take Test
        </Typography>
        <Button onClick={() => navigate('/')} variant="outlined" color="inherit">
            Cancel
        </Button>
      </Box>

      <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
        {questions.map((q, idx) => (
          <Paper key={q.id} elevation={2} sx={{ p: 4 }}>
            <Box sx={{ display: 'flex', alignItems: 'flex-start', gap: 2, mb: 2 }}>
              <Chip label={`Q${idx + 1}`} color="primary" size="small" />
              <Typography variant="h6">{q.prompt}</Typography>
            </Box>
            
            <Divider sx={{ mb: 2 }} />

            <Box sx={{ ml: 2 }}>
                {q.question_type === 'single' && (
                <RadioGroup
                    value={answers[q.id] || ''}
                    onChange={(e) => handleAnswerChange(q.id, e.target.value)}
                >
                    {q.options.map((opt) => (
                    <FormControlLabel
                        key={opt}
                        value={opt}
                        control={<Radio />}
                        label={opt}
                        sx={{ mb: 1 }}
                    />
                    ))}
                </RadioGroup>
                )}

                {q.question_type === 'multiple' && (
                <FormGroup>
                    {q.options.map((opt) => (
                    <FormControlLabel
                        key={opt}
                        control={
                        <Checkbox
                            checked={isMultiSelected(q.id, opt)}
                            onChange={(e) => handleMultiSelectChange(q.id, opt, e.target.checked)}
                        />
                        }
                        label={opt}
                    />
                    ))}
                    <Typography variant="caption" color="text.secondary" sx={{ mt: 1, display: 'block' }}>
                        * Select all that apply
                    </Typography>
                </FormGroup>
                )}

                {q.question_type === 'blank' && (
                    <Box sx={{ mt: 2 }}>
                        <TextField 
                            fullWidth
                            label="Your Answer"
                            variant="outlined"
                            value={answers[q.id] || ''}
                            onChange={(e) => handleAnswerChange(q.id, e.target.value)}
                            placeholder="Type your answer here..."
                        />
                    </Box>
                )}
            </Box>
          </Paper>
        ))}
      </Box>

      <Box sx={{ mt: 4, display: 'flex', justifyContent: 'center' }}>
          <Button 
            variant="contained" 
            size="large" 
            onClick={handleSubmit}
            disabled={submitting}
            sx={{ px: 6, py: 1.5, fontSize: '1.1rem' }}
          >
            {submitting ? <CircularProgress size={24} color="inherit" /> : "Submit Test"}
          </Button>
      </Box>
    </Container>
  );
};