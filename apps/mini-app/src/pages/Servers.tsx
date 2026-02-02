import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import './Servers.css'
import { useAuth } from '../context/AuthContext'
import { QRCodeSVG } from 'qrcode.react'

interface ServerInfo {
    id: number
    name: string
    country_code: string
    flag: string
    latency?: number
    status: string // 'online' | 'offline'
    distance_km?: number
}



export default function Servers() {
    const navigate = useNavigate()
    const { token, subscription } = useAuth()
    const [servers, setServers] = useState<ServerInfo[]>([])
    const [loading, setLoading] = useState(true)

    // Modal state
    const [selectedServer, setSelectedServer] = useState<ServerInfo | null>(null)
    const [clientType, setClientType] = useState('singbox')
    const [configUrl, setConfigUrl] = useState('')

    useEffect(() => {
        if (!token) return;

        const fetchData = async () => {
            try {
                // Fetch Servers
                const apiUrl = '/api/client/servers'; // Correct endpoint
                const nodesRes = await fetch(apiUrl, {
                    headers: { 'Authorization': `Bearer ${token}` }
                });
                if (nodesRes.ok) {
                    setServers(await nodesRes.json());
                }
            } catch (error) {
                console.error("Failed to fetch data", error);
            } finally {
                setLoading(false);
            }
        };
        fetchData();
    }, [token]);

    const handleGetConfig = (server: ServerInfo) => {
        if (!subscription) return;
        setSelectedServer(server);
        updateConfigUrl(server.id, clientType);
    }

    const updateConfigUrl = (nodeId: number, type: string) => {
        if (!subscription) return;
        // Construct URL: /sub/:uuid?node_id=...&client=...
        // Use backend logic (if we returned full URL in sub object, we'd use that base)
        // But subscription.subscription_url is a full URL.
        // We can parse it or just blindly append/replace?
        // Let's assume subscription_url is like "https://panel.com/sub/UUID"
        // We want "https://panel.com/sub/UUID?client=type&node_id=id"

        let baseUrl = subscription.subscription_url;
        // Check if it already has query params? (Unlikely)
        const separator = baseUrl.includes('?') ? '&' : '?';
        const url = `${baseUrl}${separator}client=${type}&node_id=${nodeId}`;
        setConfigUrl(url);
    }

    const handleClientChange = (type: string) => {
        setClientType(type);
        if (selectedServer) {
            updateConfigUrl(selectedServer.id, type);
        }
    }

    if (loading) {
        return <div className="page servers-page"><div className="loading">Loading servers...</div></div>;
    }



    return (
        <div className="page servers-page">
            <header className="page-header">
                <button className="back-button" onClick={() => navigate(-1)}>
                    ← Back
                </button>
                <h2>Server Selection</h2>
            </header>

            <div className="servers-list">
                {servers.map((server, index) => (
                    <div
                        key={server.id}
                        className={`server-card ${index === 0 ? 'best' : ''}`}
                    >
                        <div className="card-content">
                            <div className="server-flag">{server.flag}</div>
                            <div className="server-info">
                                <div className="server-name">{server.name}</div>
                                <div className="server-country">
                                    {server.country_code}
                                    {server.distance_km !== undefined && (
                                        <span className="distance-badge"> • {server.distance_km} km</span>
                                    )}
                                </div>
                            </div>
                            <div className="server-stats">
                                <div className={`status ${server.status}`}>
                                    {server.status === 'online' ? '🟢' : '🔴'}
                                </div>
                            </div>
                        </div>
                        <button className="get-link-btn" onClick={() => handleGetConfig(server)}>
                            🔗 Get Config
                        </button>

                        {index === 0 && server.latency && (
                            <div className="best-badge">⭐ Best</div>
                        )}
                    </div>
                ))}
            </div>

            {selectedServer && (
                <div className="modal-overlay" onClick={() => setSelectedServer(null)}>
                    <div className="modal-content" onClick={e => e.stopPropagation()}>
                        <h3>Config for {selectedServer.name}</h3>
                        <div className="client-selector">
                            <button className={clientType === 'singbox' ? 'active' : ''} onClick={() => handleClientChange('singbox')}>Sing-box</button>
                            <button className={clientType === 'v2ray' ? 'active' : ''} onClick={() => handleClientChange('v2ray')}>V2Ray</button>
                            <button className={clientType === 'clash' ? 'active' : ''} onClick={() => handleClientChange('clash')}>Clash</button>
                        </div>

                        <div className="qr-container">
                            <QRCodeSVG value={configUrl} size={180} />
                        </div>

                        <div className="url-box">
                            <input type="text" readOnly value={configUrl} />
                            <button onClick={() => navigator.clipboard.writeText(configUrl)}>Copy</button>
                        </div>

                        <button className="close-btn" onClick={() => setSelectedServer(null)}>Close</button>
                    </div>
                </div>
            )}

            <style>{`
                .get-link-btn {
                    width: 100%;
                    margin-top: 10px;
                    padding: 8px;
                    background: rgba(255, 255, 255, 0.1);
                    border: 1px solid rgba(255, 255, 255, 0.2);
                    border-radius: 8px;
                    color: white;
                    cursor: pointer;
                    font-size: 14px;
                }
                .get-link-btn:hover {
                    background: rgba(255, 255, 255, 0.2);
                }
                .card-content {
                    display: flex;
                    align-items: center;
                    gap: 15px;
                }
                
                /* Modal Styles */
                .modal-overlay {
                    position: fixed;
                    top: 0; left: 0; right: 0; bottom: 0;
                    background: rgba(0, 0, 0, 0.8);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    z-index: 1000;
                    padding: 20px;
                }
                .modal-content {
                    background: #222;
                    padding: 24px;
                    border-radius: 16px;
                    width: 100%;
                    max-width: 320px;
                    text-align: center;
                    border: 1px solid #333;
                }
                .client-selector {
                    display: flex;
                    gap: 8px;
                    margin: 15px 0;
                    justify-content: center;
                }
                .client-selector button {
                    background: #333;
                    border: none;
                    color: #aaa;
                    padding: 6px 12px;
                    border-radius: 20px;
                    font-size: 12px;
                }
                .client-selector button.active {
                    background: #6366f1;
                    color: white;
                }
                .qr-container {
                    background: white;
                    padding: 10px;
                    border-radius: 12px;
                    display: inline-block;
                    margin-bottom: 15px;
                }
                .url-box {
                    display: flex;
                    gap: 8px;
                    margin-bottom: 15px;
                }
                .url-box input {
                    flex: 1;
                    background: #111;
                    border: 1px solid #333;
                    color: #888;
                    padding: 8px;
                    border-radius: 6px;
                    font-size: 12px;
                }
                .url-box button {
                    background: #444;
                    color: white;
                    border: none;
                    padding: 0 12px;
                    border-radius: 6px;
                }
                .close-btn {
                    background: transparent;
                    color: #888;
                    border: none;
                    width: 100%;
                    padding: 10px;
                }
            `}</style>
        </div>
    )
}
