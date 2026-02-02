import { useEffect, useState } from 'react'
import { useAuth } from '../context/AuthContext'
import { useNavigate } from 'react-router-dom'

interface Payment {
    id: number;
    amount: number;
    method: string;
    status: string;
    created_at: number;
}

export default function Billing() {
    const { user, token } = useAuth()
    const navigate = useNavigate()
    const [payments, setPayments] = useState<Payment[]>([])
    const [loading, setLoading] = useState(true)

    useEffect(() => {
        if (!token) return

        const fetchPayments = async () => {
            try {
                const response = await fetch('/api/client/user/payments', {
                    headers: { 'Authorization': `Bearer ${token}` }
                })
                if (response.ok) {
                    const data = await response.json()
                    setPayments(data)
                }
            } catch (error) {
                console.error("Failed to fetch payments", error)
            } finally {
                setLoading(false)
            }
        }
        fetchPayments()
    }, [token])

    const formatDate = (timestamp: number) => {
        return new Date(timestamp * 1000).toLocaleDateString()
    }

    const formatCurrency = (amount: number) => {
        return new Intl.NumberFormat('en-US', {
            style: 'currency',
            currency: 'USD', // Assuming USD for now, could be dynamic
        }).format(amount)
    }

    return (
        <div className="page billing-page">
            <div className="header">
                <h1>Billing</h1>
                <button className="back-button" onClick={() => navigate('/')}>Back</button>
            </div>

            <div className="balance-card">
                <h3>Current Balance</h3>
                <div className="balance-amount">
                    {user ? formatCurrency(user.balance || 0) : '...'}
                </div>
                <button className="deposit-button" disabled>
                    Deposit (Coming Soon)
                </button>
            </div>

            <div className="transactions-section">
                <h3>Recent Transactions</h3>
                {loading ? (
                    <div className="loading">Loading history...</div>
                ) : payments.length > 0 ? (
                    <div className="transactions-list">
                        {payments.map(payment => (
                            <div key={payment.id} className="transaction-item">
                                <div className="transaction-info">
                                    <span className="method">{payment.method}</span>
                                    <span className="date">{formatDate(payment.created_at)}</span>
                                </div>
                                <div className="transaction-status">
                                    <span className={`amount ${payment.amount > 0 ? 'positive' : 'negative'}`}>
                                        {payment.amount > 0 ? '+' : ''}{formatCurrency(payment.amount)}
                                    </span>
                                    <span className={`status ${payment.status.toLowerCase()}`}>
                                        {payment.status}
                                    </span>
                                </div>
                            </div>
                        ))}
                    </div>
                ) : (
                    <div className="empty-state">No transactions found</div>
                )}
            </div>

            <style>{`
                .billing-page {
                    padding: 20px;
                    color: white;
                    min-height: 100vh;
                    background: var(--bg-color, #1a1a1a);
                }
                .header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 20px;
                }
                .back-button {
                    background: rgba(255,255,255,0.1);
                    border: none;
                    color: white;
                    padding: 8px 16px;
                    border-radius: 8px;
                    cursor: pointer;
                }
                .balance-card {
                    background: linear-gradient(135deg, #6366f1, #8b5cf6);
                    padding: 24px;
                    border-radius: 16px;
                    text-align: center;
                    margin-bottom: 30px;
                    box-shadow: 0 4px 15px rgba(99, 102, 241, 0.3);
                }
                .balance-amount {
                    font-size: 36px;
                    font-weight: bold;
                    margin: 10px 0 20px 0;
                }
                .deposit-button {
                    background: rgba(255,255,255,0.2);
                    border: 1px solid rgba(255,255,255,0.4);
                    color: white;
                    padding: 10px 24px;
                    border-radius: 20px;
                    font-weight: 600;
                    cursor: not-allowed;
                    opacity: 0.8;
                }
                .transactions-section h3 {
                   margin-bottom: 15px;
                   color: #aaa;
                   font-size: 14px;
                   text-transform: uppercase;
                   letter-spacing: 1px;
                }
                .transaction-item {
                    background: rgba(255,255,255,0.05);
                    padding: 15px;
                    border-radius: 12px;
                    margin-bottom: 10px;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                .transaction-info {
                    display: flex;
                    flex-direction: column;
                }
                .method {
                    font-weight: 500;
                    font-size: 16px;
                }
                .date {
                    color: #888;
                    font-size: 12px;
                    margin-top: 4px;
                }
                .transaction-status {
                    text-align: right;
                }
                .amount {
                    display: block;
                    font-weight: bold;
                    font-size: 16px;
                }
                .amount.positive { color: #4ade80; }
                .amount.negative { color: #f87171; }
                
                .status {
                    font-size: 10px;
                    padding: 2px 6px;
                    border-radius: 4px;
                    text-transform: uppercase;
                }
                .status.completed { background: rgba(74, 222, 128, 0.2); color: #4ade80; }
                .status.pending { background: rgba(251, 191, 36, 0.2); color: #fbbf24; }
                .status.failed { background: rgba(248, 113, 113, 0.2); color: #f87171; }
                
                .empty-state {
                    text-align: center;
                    color: #666;
                    padding: 40px 0;
                }
            `}</style>
        </div>
    )
}
