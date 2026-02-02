use sqlx::SqlitePool;
use serde::Serialize;
use anyhow::Result;

#[derive(Serialize, sqlx::FromRow)]
pub struct LeaderboardEntry {
    pub username: Option<String>,
    pub referral_count: i64,
}

#[derive(Serialize)]
pub struct LeaderboardDisplayEntry {
    pub rank: usize,
    pub username: String,
    pub referral_count: i64,
    pub medal: Option<String>,
}

pub struct ReferralService;

impl ReferralService {
    /// Get top referrers
    pub async fn get_leaderboard(pool: &SqlitePool, limit: i64) -> Result<Vec<LeaderboardDisplayEntry>> {
        // Query to count referrals per user
        let rows: Vec<LeaderboardEntry> = sqlx::query_as(r#"
            SELECT 
                u.username,
                COUNT(r.id) as referral_count
            FROM users u
            JOIN users r ON u.id = r.referred_by
            GROUP BY u.id
            ORDER BY referral_count DESC
            LIMIT ?
        "#)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let mut display_rows = Vec::new();
        for (index, row) in rows.into_iter().enumerate() {
            let rank = index + 1;
            let medal = match rank {
                1 => Some("🥇".to_string()),
                2 => Some("🥈".to_string()),
                3 => Some("🥉".to_string()),
                _ => None,
            };

            let safe_username = row.username.unwrap_or_else(|| "Anonymous".to_string());
            let masked_username = Self::mask_username(&safe_username);

            display_rows.push(LeaderboardDisplayEntry {
                rank,
                username: masked_username,
                referral_count: row.referral_count,
                medal,
            });
        }

        Ok(display_rows)
    }

    fn mask_username(username: &str) -> String {
        if username.len() <= 3 {
             return "***".to_string();
        }
        let len = username.len();
        let visible = if len > 6 { 3 } else { 1 };
        format!("{}***", &username[0..visible])
    }
}
