import React, { createContext, useContext, useEffect, useState } from 'react';
import WebApp from '@twa-dev/sdk';

export interface UserStats {
    traffic_used: number;
    total_traffic: number;
    days_left: number;
    plan_name: string;
    balance: number;
    total_download: number; // Add these to match usage
    total_upload: number;   // Add these to match usage
    traffic_limit: number;  // Add helper for uniformity (alias total_traffic)
}

export interface UserSubscription {
    uuid: string;
    subscription_url: string;
}

interface AuthContextType {
    isAuthenticated: boolean;
    token: string | null;
    userStats: UserStats | null;
    subscription: UserSubscription | null;
    isLoading: boolean;
    refreshData: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType>({
    isAuthenticated: false,
    token: null,
    userStats: null,
    subscription: null,
    isLoading: true,
    refreshData: async () => { },
});

export const useAuth = () => useContext(AuthContext);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [token, setToken] = useState<string | null>(localStorage.getItem('jwt_token'));
    const [userStats, setUserStats] = useState<UserStats | null>(null);
    const [subscription, setSubscription] = useState<UserSubscription | null>(null);
    const [isLoading, setIsLoading] = useState(true);

    // Initial Auth
    useEffect(() => {
        const initAuth = async () => {
            // If we have a token, verify it by fetching stats (or just use it)
            // If no token, or invalid, try to auth with initData

            // Check if running in Telegram
            if (WebApp.initData) {
                try {
                    const response = await fetch('/api/client/auth/telegram', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({ initData: WebApp.initData }),
                    });

                    if (response.ok) {
                        const data = await response.json();
                        setToken(data.token);
                        localStorage.setItem('jwt_token', data.token);
                    } else {
                        console.error("Auth failed:", await response.text());
                    }
                } catch (e) {
                    console.error("Auth error:", e);
                }
            } else {
                console.warn("No initData found. Running in standalone mode?");
                // Dev mode mock (if needed)
            }
        };

        if (!token) {
            initAuth();
        }
    }, []);

    // Fetch Data when token is available
    useEffect(() => {
        if (token) {
            refreshData();
        } else {
            // Wait for auth to complete or fail
            // If no initData and no token after 1s, stop loading
            const timer = setTimeout(() => setIsLoading(false), 1000);
            return () => clearTimeout(timer);
        }
    }, [token]);

    const refreshData = async () => {
        if (!token) return;
        setIsLoading(true);
        try {
            const [statsRes, subRes] = await Promise.all([
                fetch('/api/client/user/stats', { headers: { Authorization: `Bearer ${token}` } }),
                fetch('/api/client/user/subscription', { headers: { Authorization: `Bearer ${token}` } })
            ]);

            if (statsRes.ok) {
                setUserStats(await statsRes.json());
            }
            if (subRes.ok) {
                setSubscription(await subRes.json());
            }
        } catch (e) {
            console.error("Data fetch error:", e);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <AuthContext.Provider value={{ isAuthenticated: !!token, token, userStats, subscription, isLoading, refreshData }}>
            {children}
        </AuthContext.Provider>
    );
};
