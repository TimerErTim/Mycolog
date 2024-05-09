use std::collections::BTreeMap;

use anyhow::{anyhow, bail};
use hmac::digest::block_buffer::Eager;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::json;
use surrealdb_core::sql::Value;
use tokio::sync::{Mutex, Semaphore};
use tracing::{error, info, instrument, warn};

use crate::application::database::DatabaseRootAccess;
use crate::application::email::events::{EmailData, EmailWebhookEvent, RecipientData};
use crate::application::email::files::EmailFile;
use crate::application::email::recipients::Recipient;
use crate::context::MycologContext;
use crate::secrets::MycologSecrets;

pub struct EmailManager {
    db: DatabaseRootAccess,
    sender: String,
    emails: BTreeMap<String, EmailFile>,
    client: Client,
    guard: Mutex<()>,
}

impl EmailManager {
    pub fn new(
        secrets: &MycologSecrets,
        db: DatabaseRootAccess,
        sender: impl Into<String>,
        emails: BTreeMap<String, EmailFile>,
    ) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", secrets.keys.mailersend_api())
                .parse()
                .unwrap(),
        );

        Self {
            db,
            sender: sender.into(),
            emails,
            client: Client::builder().default_headers(headers).build().unwrap(),
            guard: Mutex::new(()),
        }
    }

    #[instrument(skip_all)]
    pub async fn process(&self, event: EmailWebhookEvent) -> anyhow::Result<()> {
        let field = match &event {
            EmailWebhookEvent::Sent(_) => "time_sent",
            EmailWebhookEvent::Delivered(_) => "time_delivered",
            EmailWebhookEvent::SoftBounced(_) | EmailWebhookEvent::HardBounced(_) => {
                "time_rejected"
            }
            EmailWebhookEvent::Opened(_) => "time_opened",
            _ => return Ok(()),
        };
        let (email, recipient) = event.as_data();

        let lock = self.guard.lock().await;
        let query = format!("UPDATE email_sent_to SET {} = time::now() WHERE in.mailersend_email_id = $email_id AND recipient = $recipient_email", field);
        let response: Vec<Value> = self
            .db
            .query(query)
            .bind("email_id", &email.id)
            .bind("recipient_email", &recipient.email)
            .await
            .map_err(|err| anyhow!("email status update query failed: {:?}", err))?
            .take(0)?;
        drop(lock);

        match response.len() {
            0 => warn!(
                email.id,
                recipient.email,
                "email status unable to update due to no corresponding email in database"
            ),
            2.. => error!(
                email.id,
                recipient.email,
                "email status update resulted in {} updates, should not be possible",
                response.len()
            ),
            _ => {}
        }

        Ok(())
    }

    pub async fn sumbit_email(
        &self,
        email_type: &str,
        subject: &str,
        recipients: Vec<Recipient>,
    ) -> anyhow::Result<()> {
        let email_file = self.emails.get(email_type).ok_or(anyhow!(
            "email type `{}` not found in loaded files",
            email_type
        ))?;
        if email_file.text.is_none() && email_file.html.is_none() {
            bail!(
                "email type `{}` has neither text or html content",
                email_type
            );
        }

        let payload = build_payload(&self.sender, subject, &email_file, &recipients);

        let lock = self.guard.lock().await;
        info!(?email_type, "sending email to mailersend...");
        let mailersend_id = post_email(&self.client, payload).await.map_err(|err| {
            anyhow!(
                "error sending email type `{}` to mailersend servers: {:?}",
                email_type,
                err
            )
        })?;
        self.persist_email(email_type, &mailersend_id, subject, &recipients)
            .await
            .map_err(|err| {
                anyhow!(
                    "error writing email of type `{}` to database: {:?}",
                    email_type,
                    err
                )
            })?;
        drop(lock);

        Ok(())
    }

    async fn persist_email(
        &self,
        email_type: &str,
        mailersend_id: &str,
        subject: &str,
        recipients: &Vec<Recipient>,
    ) -> anyhow::Result<()> {
        let mut query = self.db.query("LET $email = ( CREATE ONLY email SET sender = $sender, type = $type, mailersend_email_id = $email_id )")
            .bind("sender", &self.sender)
            .bind("type", email_type)
            .bind("email_id", mailersend_id);
        for (index, recipient) in recipients.iter().enumerate() {
            query = query.query(format!("LET $user_id = SELECT id FROM ONLY user WHERE email = $recipient_email_{index} LIMIT 1"))
                .query(format!("IF $user_id != NULL THEN ( RELATE ($email.id)->email_sent_to->$user_id SET recipient = $recipient_email_{index})"))
                .bind(format!("recipient_email_{index}"), &recipient.email);
        }
        query.await?.checked()?;
        Ok(())
    }
}

fn build_payload(
    sender: &str,
    subject: &str,
    file: &EmailFile,
    recipients: &[Recipient],
) -> serde_json::Value {
    let to_recipients = recipients
        .iter()
        .map(|recipient| {
            json!({
                "email": recipient.email
            })
        })
        .collect::<Vec<_>>();
    let personalization_recipients = recipients
        .iter()
        .map(|recipient| {
            json!({
                "email": recipient.email,
                "data": recipient.variables
            })
        })
        .collect::<Vec<_>>();
    json!({
        "from": {
            "email": sender,
            "name": "Mycolog"
        },
        "to": to_recipients,
        "subject": subject,
        "text": file.text,
        "html": file.html,
        "personalization": personalization_recipients
    })
}

async fn post_email(client: &Client, payload: serde_json::Value) -> anyhow::Result<String> {
    let request = client
        .post("https://api.mailersend.com/v1/email")
        .json(&payload);
    let response = request.send().await?;
    if !response.status().is_success() {
        let statuscode = response.status().to_string();
        let text = response.text().await.unwrap_or("???".to_string());
        error!(%statuscode, response = %text, "mailersend responded with error");
        bail!("mailersend responded with statuscode {}", statuscode);
    }
    let message_id = response
        .headers()
        .get("X-Message-Id")
        .ok_or(anyhow!("mailersend response has no `X-Message-Id` header"))?
        .to_str()
        .map_err(|_| anyhow!("mailersend response `X-Message-Id` header is invalid bytes"))?;
    Ok(message_id.to_string())
}
