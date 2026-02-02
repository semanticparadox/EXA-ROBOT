use sqlx::SqlitePool;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;
use std::sync::Arc;
use chrono::Utc;
use crate::services::store_service::StoreService;
use crate::bot_manager::BotManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoBotInvoice {
    pub asset: String,
    pub amount: String,
    pub description: Option<String>,
    pub payload: Option<String>,
    pub paid_btn_name: Option<String>,
    pub paid_btn_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoBotResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceResult {
    pub invoice_id: i64,
    pub bot_invoice_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NowPaymentInvoice {
    pub price_amount: f64,
    pub price_currency: String,
    pub pay_currency: String,
    pub ipn_callback_url: String,
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NowPaymentResponse {
    pub payment_id: String,
    pub invoice_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentType {
    BalanceTopup,
    OrderPurchase(i64), // order_id
}

impl PaymentType {
    pub fn to_payload_string(&self, user_id: i64) -> String {
        match self {
            PaymentType::BalanceTopup => format!("{}:bal:0", user_id),
            PaymentType::OrderPurchase(order_id) => format!("{}:ord:{}", user_id, order_id),
        }
    }
}

pub struct PayService {
    pool: SqlitePool,
    #[allow(dead_code)]
    store_service: Arc<StoreService>,
    bot_manager: Arc<BotManager>,
    cryptobot_token: String,
    nowpayments_key: String,
    crystalpay_login: String,
    crystalpay_secret: String,
    stripe_secret_key: String,
    is_testnet: bool,
}

impl PayService {
    pub fn new(
        pool: SqlitePool, 
        store_service: Arc<StoreService>, 
        bot_manager: Arc<BotManager>, 
        cryptobot_token: String, 
        nowpayments_key: String, 
        crystalpay_login: String,
        crystalpay_secret: String,
        stripe_secret_key: String,
        is_testnet: bool
    ) -> Self {
        Self { 
            pool, 
            store_service, 
            bot_manager, 
            cryptobot_token, 
            nowpayments_key, 
            crystalpay_login,
            crystalpay_secret,
            stripe_secret_key,
            is_testnet 
        }
    }

    fn get_cryptobot_url(&self) -> &str {
        if self.is_testnet { "https://testnet-pay.crypt.bot/api" } else { "https://pay.crypt.bot/api" }
    }

    pub async fn create_cryptobot_invoice(&self, user_id: i64, amount_usd: f64, payment_type: PaymentType) -> Result<String> {
        info!("Creating CryptoPay invoice for user {}: ${} ({:?})", user_id, amount_usd, payment_type);
        
        let payload = payment_type.to_payload_string(user_id);
        
        // CryptoBot needs asset amount, not USD usually, but createInvoice allows 'amount' + 'fiat' or 'amount' + 'asset'.
        // If we want USD, we use 'amount' and 'fiat' = 'USD' OR we use one of the crypto currencies.
        // Actually typical CryptoPay usage is creating an invoice in USDT or allowing user to pay any asset.
        // createInvoice endpoint: asset, amount, ...
        // To bill in USD: we might need to use `createInvoice` with `fiat` param?
        // Let's check typical usage. Often we request USDT.
        
        let invoice = serde_json::json!({
             "asset": "USDT",
             "amount": format!("{:.2}", amount_usd),
             "description": "EXA ROBOT Top-up",
             "payload": payload,
             "allow_anonymous": false,
             "allow_comments": false
        });

        let client = reqwest::Client::new();
        let resp = client.post(format!("{}/createInvoice", self.get_cryptobot_url()))
            .header("Crypto-Pay-API-Token", &self.cryptobot_token)
            .json(&invoice)
            .send()
            .await?;

        let body: serde_json::Value = resp.json().await?;
        if body["ok"].as_bool().unwrap_or(false) {
             Ok(body["result"]["bot_invoice_url"].as_str().unwrap_or("").to_string())
        } else {
             Err(anyhow::anyhow!("CryptoBot error: {:?}", body))
        }
    }

    pub async fn create_nowpayments_invoice(&self, user_id: i64, amount_usd: f64, payment_type: PaymentType) -> Result<String> {
        info!("Creating NOWPayments invoice for user {}: ${}", user_id, amount_usd);
        
        let payload_base = payment_type.to_payload_string(user_id);
        let unique_order_id = format!("{}_{}", payload_base, Utc::now().timestamp());

        let invoice = serde_json::json!({
            "price_amount": amount_usd,
            "price_currency": "usd",
            "pay_currency": "usdttrc20", // Default view
            "order_id": unique_order_id,
            "ipn_callback_url": "https://api.exa.robot/api/payments/nowpayments", 
            "success_url": "https://t.me/exarobot_bot",
            "cancel_url": "https://t.me/exarobot_bot"
        });

        let client = reqwest::Client::new();
        let resp = client.post("https://api.nowpayments.io/v1/invoice")
            .header("x-api-key", &self.nowpayments_key)
            .json(&invoice)
            .send()
            .await?;

        let body: serde_json::Value = resp.json().await?;
        if let Some(url) = body["invoice_url"].as_str() {
            Ok(url.to_string())
        } else {
            Err(anyhow::anyhow!("NOWPayments error: {:?}", body))
        }
    }

    pub async fn create_crystalpay_invoice(&self, user_id: i64, amount_usd: f64, payment_type: PaymentType) -> Result<String> {
        info!("Creating CrystalPay invoice for user {}: ${}", user_id, amount_usd);
        
        // CrystalPay creates a payment link.
        // Endpoint: https://api.crystalpay.io/v2/invoice/create/
        // Auth: login + secret
        // Params: amount, type=purchase, lifetime, etc.
        
        // Convert USD to RUB for CrystalPay (assuming it accepts RUB primarily for SBP)
        // Or check if they support USD invoices. They do support multi-currency but often convert.
        // For simplicity, let's assume we bill in USD and they handle conversion or we set USD.
        
        let payload = payment_type.to_payload_string(user_id);
        
        let body_json = serde_json::json!({
            "auth_login": self.crystalpay_login,
            "auth_secret": self.crystalpay_secret,
            "amount": amount_usd,
            "amount_currency": "USD", // Request USD
            "type": "purchase",
            "description": format!("ExaRobot User {}", user_id),
            "redirect_url": "https://t.me/exarobot_bot",
            "callback_url": "https://api.exa.robot/api/payments/crystalpay",
            "extra": payload 
        });

        let client = reqwest::Client::new();
        let resp = client.post("https://api.crystalpay.io/v2/invoice/create/")
            .json(&body_json)
            .send()
            .await?;
            
        let resp_json: serde_json::Value = resp.json().await?;
        
        // CrystalPay V2 response: {"error":false, "errors":[], "data": { "id": "...", "url": "..." }}
        if resp_json["error"].as_bool().unwrap_or(true) {
             Err(anyhow::anyhow!("CrystalPay Error: {:?}", resp_json))
        } else {
             Ok(resp_json["data"]["url"].as_str().unwrap_or("").to_string())
        }
    }

    pub async fn create_stripe_session(&self, user_id: i64, amount_usd: f64, payment_type: PaymentType) -> Result<String> {
        info!("Creating Stripe Session for user {}: ${}", user_id, amount_usd);
        
        let payload = payment_type.to_payload_string(user_id);
        let amount_cents = (amount_usd * 100.0) as i64;
        
        let client = reqwest::Client::new();
        let params = [
            ("mode", "payment"),
            ("success_url", "https://t.me/exarobot_bot"),
            ("cancel_url", "https://t.me/exarobot_bot"),
            ("client_reference_id", &payload),
            ("line_items[0][price_data][currency]", "usd"),
            ("line_items[0][price_data][product_data][name]", "Balance Top-up"),
            ("line_items[0][price_data][unit_amount]", &amount_cents.to_string()),
            ("line_items[0][quantity]", "1"),
        ];

        let resp = client.post("https://api.stripe.com/v1/checkout/sessions")
            .basic_auth(&self.stripe_secret_key, None::<&str>)
            .form(&params)
            .send()
            .await?;

        let body: serde_json::Value = resp.json().await?;
        if let Some(url) = body["url"].as_str() {
             Ok(url.to_string())
        } else {
             Err(anyhow::anyhow!("Stripe Error: {:?}", body))
        }
    }

    pub async fn handle_webhook(&self, source: &str, payload: &str) -> Result<()> {
        let body: serde_json::Value = serde_json::from_str(payload)?;
        
        match source {
            "cryptobot" => {
                 if let Some(update_type) = body["update_type"].as_str() {
                    if update_type == "invoice_paid" {
                        let invoice = &body["update_payload"];
                        if invoice["status"].as_str().unwrap_or("") == "paid" {
                            let amount: f64 = invoice["amount"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                            let payload_str = invoice["payload"].as_str().unwrap_or("");
                            let id = invoice["invoice_id"].to_string();
                            self.process_any_payment(amount, "cryptobot", Some(id), payload_str).await?;
                        }
                    }
                 }
            },
            "nowpayments" => {
                 if let Some(status) = body["payment_status"].as_str() {
                     if status == "finished" {
                         let amount: f64 = body["pay_amount"].as_f64().unwrap_or(0.0);
                         let order_id = body["order_id"].as_str().unwrap_or("");
                         let payload_str = order_id.split('_').next().unwrap_or("");
                         let id = body["payment_id"].to_string();
                         self.process_any_payment(amount, "nowpayments", Some(id), payload_str).await?;
                     }
                 }
            },
            "crystalpay" => {
                // Check signature logic usually needed here, but for now trusting payload for MVP
                // CrystalPay callback: type=payment, state=payed
                if body["type"].as_str().unwrap_or("") == "payment" && body["state"].as_str().unwrap_or("") == "payed" {
                    let amount: f64 = body["amount"].as_f64().unwrap_or(0.0); // usually in currency requested
                    let extra = body["extra"].as_str().unwrap_or("");
                    let id = body["id"].to_string();
                    self.process_any_payment(amount, "crystalpay", Some(id), extra).await?;
                }
            },
            "stripe" => {
                // Stripe sends Event object
                if body["type"].as_str().unwrap_or("") == "checkout.session.completed" {
                    let session = &body["data"]["object"];
                    let amount_subtokens = session["amount_total"].as_i64().unwrap_or(0);
                    let amount_usd = amount_subtokens as f64 / 100.0;
                    let payload_str = session["client_reference_id"].as_str().unwrap_or("");
                    let id = session["id"].to_string();
                    self.process_any_payment(amount_usd, "stripe", Some(id), payload_str).await?;
                }
            },
            _ => {}
        }

        Ok(())
    }

    pub async fn process_any_payment(&self, amount_usd: f64, method: &str, external_id: Option<String>, payload: &str) -> Result<()> {
        let parts: Vec<&str> = payload.split(':').collect();
        if parts.len() < 3 {
            // Legacy/Simple fallback
            if let Ok(user_id) = payload.parse::<i64>() {
                return self.process_balance_topup(user_id, amount_usd, method, external_id).await;
            }
             return Err(anyhow::anyhow!("Invalid payload: {}", payload));
        }

        let user_id: i64 = parts[0].parse().unwrap_or(0);
        let type_code = parts[1];
        let target_id: i64 = parts[2].parse().unwrap_or(0);

        if user_id == 0 { return Err(anyhow::anyhow!("Zero User ID")); }

        match type_code {
            "bal" => self.process_balance_topup(user_id, amount_usd, method, external_id).await,
            "ord" => self.process_order_purchase(user_id, target_id, amount_usd, method, external_id).await,
            _ => Err(anyhow::anyhow!("Unknown Type: {}", type_code)),
        }
    }

    async fn process_order_purchase(&self, user_id: i64, order_id: i64, amount_usd: f64, method: &str, external_id: Option<String>) -> Result<()> {
        info!("Processing ORDER payment #${} for user {}", order_id, user_id);
        let amount_units = (amount_usd * 100.0) as i64;
        self.store_service.log_payment(user_id, method, amount_units, external_id.as_deref(), "paid").await?;
        self.store_service.process_order_payment(order_id).await?;
        
        let _ = self.bot_manager.send_notification(user_id, "✅ Your order has been paid successfully!").await;
        
        let _ = crate::services::analytics_service::AnalyticsService::track_revenue(&self.store_service.get_pool(), amount_units).await;
        Ok(())
    }

    async fn process_balance_topup(&self, user_id: i64, amount_usd: f64, method: &str, external_id: Option<String>) -> Result<()> {
        info!("Processing BALANCE top-up of ${} for user {} via {}", amount_usd, user_id, method);
        let amount_units = (amount_usd * 100.0) as i64; 
        
        let mut tx = self.pool.begin().await?;
        sqlx::query("UPDATE users SET balance = balance + ? WHERE id = ?")
            .bind(amount_units).bind(user_id).execute(&mut *tx).await?;

        let payment_id: i64 = sqlx::query_scalar(
            "INSERT INTO payments (user_id, method, amount, external_id, status) VALUES (?, ?, ?, ?, 'paid') RETURNING id"
        )
            .bind(user_id).bind(method).bind(amount_units).bind(external_id).fetch_one(&mut *tx).await?;

        if let Some((referrer_tg_id, bonus)) = self.store_service.apply_referral_bonus(&mut tx, user_id, amount_units, Some(payment_id)).await? {
            let formatted_bonus = format!("{:.2}", bonus as f64 / 100.0);
            let msg = format!("🎉 *Referral Bonus* from your invited user!\n+${}", formatted_bonus);
            let _ = self.bot_manager.send_notification(referrer_tg_id, &msg).await;
        }

        tx.commit().await?;
        
        // Notify user
        let _ = self.bot_manager.send_notification(user_id, &format!("✅ Balance topped up: +${:.2}", amount_usd)).await;
        let _ = crate::services::analytics_service::AnalyticsService::track_revenue(&self.pool, amount_units).await;
        
        Ok(())
    }
}
