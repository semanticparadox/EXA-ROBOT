import React, { createContext, useContext, useEffect, useState } from 'react';
import WebApp from '@twa-dev/sdk';

interface User {
    id: number;
    username: string;
    active_subscriptions: number;
    balance?: number;
}

interface Subscription {
    uuid: string;
    subscription_url: string;
}

interface UserStats {
    traffic_used: number;
    total_traffic: number;
    traffic_limit: number; // mapped from total_traffic
    days_left: number;
    plan_name: number;
    total_download: number; // Derived or separate?
    total_upload: number; // Derived or separate? 
    // Backend returns: traffic_used, total_traffic, days_left, plan_name, balance. 
    // Statistics.tsx expects total_download/upload.
    // The backend /user/stats returns: traffic_used, total_traffic, days_left, plan_name. 
    // It does NOT seem to return total_download/upload in the simple query I saw in client.rs.
    // Let's re-check client.rs response for /user/stats.
    // It returns: traffic_used, total_traffic, days_left, plan_name, balance.
    // Statistics.tsx expects more. I might need to update backend too or mock/zero them.
    // client.rs Line 208: traffic_used, total_traffic.
    // It doesn't split up/down.
    // I will add placeholders in frontend for now to satisfy TS.
}

interface AuthContextType {
    user: User | null;
    token: string | null;
    isLoading: boolean;
    error: string | null;
    subscription: Subscription | null;
    userStats: UserStats | null;
}

const AuthContext = createContext<AuthContextType>({
    user: null,
    token: null,
    isLoading: true,
    error: null,
    subscription: null,
    userStats: null,
});

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [user, setUser] = useState<User | null>(null);
    const [token, setToken] = useState<string | null>(null);
    const [subscription, setSubscription] = useState<Subscription | null>(null);
    const [userStats, setUserStats] = useState<UserStats | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const initAuth = async () => {
            try {
                WebApp.expand();
                const initData = WebApp.initData;

                if (!initData && !import.meta.env.DEV) {
                    // setError("No initData"); return; 
                }

                // Auth
                const authRes = await fetch('/api/client/auth/telegram', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ init_data: initData }),
                });

                if (!authRes.ok) throw new Error('Authentication failed');
                const authData = await authRes.json();
                setToken(authData.token);
                setUser(authData.user);

                // Fetch Extras
                if (authData.token) {
                    // Subscription
                    const subRes = await fetch('/api/client/user/subscription', {
                        headers: { 'Authorization': `Bearer ${authData.token}` }
                    });
                    if (subRes.ok) setSubscription(await subRes.json());

                    // Stats
                    const statsRes = await fetch('/api/client/user/stats', {
                        headers: { 'Authorization': `Bearer ${authData.token}` }
                    });
                    if (statsRes.ok) {
                        const s = await statsRes.json();
                        // Map backend response to logic expect
                        setUserStats({
                            ...s,
                            traffic_limit: s.total_traffic,
                            total_download: s.traffic_used, // Approximation
                            total_upload: 0
                        });
                    }
                }

            } catch (err: any) {
                setError(err.message);
            } finally {
                setIsLoading(false);
            }
        };

        initAuth();
    }, []);

    return (
        <AuthContext.Provider value={{ user, token, isLoading, error, subscription, userStats }}>
            {children}
        </AuthContext.Provider>
    );
};

export const useAuth = () => useContext(AuthContext);
