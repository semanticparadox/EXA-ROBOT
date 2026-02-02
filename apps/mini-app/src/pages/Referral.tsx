import { useEffect, useState } from 'react'
import { useAuth } from '../context/AuthContext'
import { useNavigate } from 'react-router-dom'
import './Referral.css'

interface ReferralStats {
    referral_code: string;
    referred_count: number;
    referral_link: string;
}

export default function Referral() {
    const { token } = useAuth()
    const navigate = useNavigate()
    const [stats, setStats] = useState<ReferralStats | null>(null)
    const [loading, setLoading] = useState(true)

    useEffect(() => {
        if (!token) return

        const fetchStats = async () => {
            try {
                const response = await fetch('/api/client/user/referrals', {
                    headers: { 'Authorization': `Bearer ${token}` }
                })
                if (response.ok) {
                    const data = await response.json()
                    setStats(data)
                }
            } catch (error) {
                console.error("Failed to fetch referral stats", error)
            } finally {
                setLoading(false)
            }
        }
        fetchStats()
    }, [token])

    const copyToClipboard = () => {
        if (stats) {
            navigator.clipboard.writeText(stats.referral_link)
            alert("Link copied!")
        }
    }

    if (loading) return <div className="page referral-page"><div className="loading">Loading...</div></div>

    return (
        <div className="page referral-page">
            <header className="page-header">
                <button className="back-button" onClick={() => navigate(-1)}>
                    ← Back
                </button>
                <h2>Refer & Earn</h2>
            </header>

            <div className="referral-card">
                <div className="referral-icon">🎁</div>
                <h3>Invite Friends</h3>
                <p className="referral-desc">
                    Share your link and earn bonuses when your friends subscribe!
                </p>

                <div className="stats-row">
                    <div className="stat-box">
                        <span className="stat-value">{stats?.referred_count || 0}</span>
                        <span className="stat-label">Friends Invited</span>
                    </div>
                </div>


                <div className="link-box">
                    <input type="text" readOnly value={stats?.referral_link || ''} />
                    <button onClick={copyToClipboard}>Copy</button>
                </div>
            </div>
        </div>
    )
}
