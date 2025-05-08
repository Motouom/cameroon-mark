
import { createContext, useContext, ReactNode, useState, useEffect } from 'react';
import { User } from '../types';
import { authAPI, userAPI } from '../services/api';

interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (name: string, email: string, password: string, role: 'buyer' | 'seller') => Promise<void>;
  logout: () => void;
  resetPassword: (email: string) => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);

  // Check for user session and validate token on initial load
  useEffect(() => {
    const checkUserSession = async () => {
      try {
        const token = localStorage.getItem('cameroon_mark_token');
        const storedUser = localStorage.getItem('cameroon_mark_user');
        
        if (token && storedUser) {
          // Try to get current user to validate the token
          try {
            const userData = await userAPI.getCurrentUser();
            setUser(userData.user);
            setIsAuthenticated(true);
          } catch (error) {
            // If API call fails, token is likely invalid
            console.error('Failed to validate user session:', error);
            localStorage.removeItem('cameroon_mark_token');
            localStorage.removeItem('cameroon_mark_user');
          }
        }
      } catch (error) {
        console.error('Failed to restore user session:', error);
        localStorage.removeItem('cameroon_mark_token');
        localStorage.removeItem('cameroon_mark_user');
      } finally {
        setIsLoading(false);
      }
    };

    checkUserSession();
  }, []);

  const login = async (email: string, password: string) => {
    setIsLoading(true);
    try {
      // Call the actual backend API
      const response = await authAPI.login(email, password);
      
      // Extract user and token from response
      const { user, token } = response;
      
      setUser(user);
      setIsAuthenticated(true);
      
      // Store user and token in localStorage
      localStorage.setItem('cameroon_mark_user', JSON.stringify(user));
      localStorage.setItem('cameroon_mark_token', token);
    } catch (error) {
      console.error('Login failed:', error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const register = async (name: string, email: string, password: string, role: 'buyer' | 'seller') => {
    setIsLoading(true);
    try {
      // Call the actual backend API
      const response = await authAPI.register(name, email, password, role);
      
      // Extract user and token from response
      const { user, token } = response;
      
      setUser(user);
      setIsAuthenticated(true);
      
      // Store user and token in localStorage
      localStorage.setItem('cameroon_mark_user', JSON.stringify(user));
      localStorage.setItem('cameroon_mark_token', token);
    } catch (error: any) {
      console.error('Registration failed:', error);
      // Enhance error with response data if available
      if (error.response?.data) {
        error.details = error.response.data;
      }
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = () => {
    setUser(null);
    setIsAuthenticated(false);
    localStorage.removeItem('cameroon_mark_user');
    localStorage.removeItem('cameroon_mark_token');
  };

  const resetPassword = async (email: string) => {
    setIsLoading(true);
    try {
      // Call the actual backend API
      await authAPI.resetPassword(email);
    } catch (error) {
      console.error('Password reset failed:', error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        isAuthenticated,
        isLoading,
        login,
        register,
        logout,
        resetPassword,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
