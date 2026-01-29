import React from 'react';
import { TestResultResponse } from '../api';
import {
  Box,
  Paper,
  Typography,
  Button,
  Chip,
  Divider,
} from '@mui/material';
import CheckCircleIcon from '@mui/icons-material/CheckCircle';
import CancelIcon from '@mui/icons-material/Cancel';

interface ScoreSummaryProps {
  result: TestResultResponse;
  onHome: () => void;
}

export const ScoreSummary: React.FC<ScoreSummaryProps> = ({ result, onHome }) => {
  return (
    <Box sx={{ maxWidth: 800, mx: 'auto', mt: 4, mb: 4 }}>
      <Paper elevation={3} sx={{ p: 4, textAlign: 'center', mb: 4 }}>
        <Typography variant="h4" gutterBottom>
          Test Completed!
        </Typography>
        
        <Box sx={{ my: 3 }}>
            <Typography variant="h2" color={result.score_percent >= 70 ? 'success.main' : 'warning.main'} fontWeight="bold">
                {result.score_percent.toFixed(0)}%
            </Typography>
        </Box>

        <Paper variant="outlined" sx={{ p: 3, bgcolor: 'primary.50', borderColor: 'primary.100', textAlign: 'left' }}>
            <Typography variant="h6" color="primary.main" gutterBottom>
                AI Feedback
            </Typography>
            <Typography variant="body1" sx={{ whiteSpace: 'pre-wrap' }}>
                {result.feedback}
            </Typography>
        </Paper>
      </Paper>

      <Typography variant="h5" gutterBottom sx={{ mb: 2 }}>
        Answer Key
      </Typography>
      
      <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
        {result.correct_answers.map((ans, idx) => (
          <Paper 
            key={ans.question_id} 
            elevation={1}
            sx={{ 
                p: 3, 
                bgcolor: ans.is_correct ? 'success.50' : 'error.50',
                border: 1,
                borderColor: ans.is_correct ? 'success.200' : 'error.200'
            }}
          >
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', mb: 2 }}>
              <Typography variant="subtitle1" fontWeight="bold">
                Question {idx + 1}
              </Typography>
              <Chip 
                icon={ans.is_correct ? <CheckCircleIcon /> : <CancelIcon />}
                label={ans.is_correct ? "CORRECT" : "INCORRECT"}
                color={ans.is_correct ? "success" : "error"}
                size="small"
              />
            </Box>
            
            {!ans.is_correct && (
              <Box sx={{ mt: 1 }}>
                <Typography variant="subtitle2" color="error.dark">Correct Answer:</Typography>
                <Typography variant="body2">{ans.correct_answer}</Typography>
              </Box>
            )}
            
            {ans.explanation && (
              <>
                <Divider sx={{ my: 2 }} />
                <Typography variant="subtitle2" color="text.secondary">Explanation:</Typography>
                <Typography variant="body2" color="text.secondary">{ans.explanation}</Typography>
              </>
            )}
          </Paper>
        ))}
      </Box>

      <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
        <Button 
          variant="contained" 
          size="large" 
          onClick={onHome}
        >
          Back to Dashboard
        </Button>
      </Box>
    </Box>
  );
};
