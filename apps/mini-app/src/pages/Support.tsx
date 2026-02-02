import { useNavigate } from 'react-router-dom'
import './Support.css'

export default function Support() {
    const navigate = useNavigate()

    const faqs = [
        {
            q: "How do I connect?",
            a: "Go to the 'Subscription' page, copy the link, and paste it into your VPN client (like Hiddify or Sing-box)."
        },
        {
            q: "Which server is faster?",
            a: "Use the 'Servers' page to test latency and find the best server for your location."
        },
        {
            q: "How do I renew?",
            a: "Your subscription auto-renews if you have balance. You can top up in the Billing section."
        }
    ]

    return (
        <div className="page support-page">
            <header className="page-header">
                <button className="back-button" onClick={() => navigate(-1)}>
                    ← Back
                </button>
                <h2>Support</h2>
            </header>

            <div className="support-actions">
                <button className="contact-btn" onClick={() => window.open('https://t.me/SupportBot', '_blank')}>
                    💬 Chat with Support
                </button>
            </div>

            <div className="faq-section">
                <h3>Frequently Asked Questions</h3>
                <div className="faq-list">
                    {faqs.map((faq, i) => (
                        <div key={i} className="faq-item">
                            <div className="faq-q">{faq.q}</div>
                            <div className="faq-a">{faq.a}</div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    )
}
