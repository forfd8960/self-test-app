import React from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { LoginPage as Login } from './features/auth/pages/Login';
import { RegisterPage as Register } from './features/auth/pages/Register';
import { UploadMaterialPage as UploadMaterial } from './features/materials/pages/UploadMaterial';
import { GenerationSetupPage as GenerationSetup } from './features/generation/pages/GenerationSetup';
import { QuestionListPage } from './features/generation/pages/QuestionListPage';
import { TakeTest } from './features/tests/pages/TakeTest';
import { useAuthStore } from './features/auth/store';
import { DashboardLayout } from './layouts/DashboardLayout';
import { HomePage } from './features/dashboard/pages/HomePage';
import { HistoryListPage } from './features/history/pages/HistoryListPage';
import { HistoryDetailPage } from './features/history/pages/HistoryDetailPage';
import { LandingPage } from './features/dashboard/pages/LandingPage';

// Simple protected route wrapper
const ProtectedRoute = ({ children }: { children: React.ReactNode }) => {
  const token = useAuthStore((state) => state.token);
  
  if (!token) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
};

const App = () => {
  const token = useAuthStore((state) => state.token);

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={token ? <Navigate to="/home" replace /> : <LandingPage />} />
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<Register />} />
        
        {/* Layout wrapper for both public and private routes that share the shell */}
        <Route element={<DashboardLayout />}>
           <Route path="/home" element={<ProtectedRoute><HomePage /></ProtectedRoute>} />
           
           <Route path="/upload" element={<ProtectedRoute><UploadMaterial /></ProtectedRoute>} />
           <Route path="/generate" element={<ProtectedRoute><GenerationSetup /></ProtectedRoute>} />
           <Route path="/history" element={<ProtectedRoute><HistoryListPage /></ProtectedRoute>} />
           <Route path="/history/:id" element={<ProtectedRoute><HistoryDetailPage /></ProtectedRoute>} />
           <Route path="/questions/:setId" element={<ProtectedRoute><QuestionListPage /></ProtectedRoute>} />
           <Route path="/test/:setId" element={<ProtectedRoute><TakeTest /></ProtectedRoute>} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
};

export default App;
