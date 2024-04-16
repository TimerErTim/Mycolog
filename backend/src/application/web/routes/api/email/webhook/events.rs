use anyhow::{anyhow, bail};
use serde_json::Value;

use crate::application::email::events::*;

impl TryFrom<Value> for EmailWebhookEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let data = value
            .get("data")
            .ok_or(anyhow!("no `data` object in json object"))?;
        let type_str = data
            .get("type")
            .ok_or(anyhow!("no `data.type` key in json object"))?
            .as_str()
            .ok_or(anyhow!("`data.type` is no string in json object"))?;
        let (email, recipient) = json_to_data(
            data.get("email")
                .ok_or(anyhow!("no `data.email` key in json object"))?,
        )?;
        let event = match type_str.as_ref() {
            "sent" => Self::Sent(EmailSentEvent { email, recipient }),
            "delivered" => Self::Delivered(EmailDeliveredEvent { email, recipient }),
            "soft_bounced" => Self::SoftBounced(EmailSoftBouncedEvent { email, recipient }),
            "hard_bounced" => Self::HardBounced(EmailHardBouncedEvent { email, recipient }),
            "opened_unique" => Self::Opened(EmailOpenedEvent { email, recipient }),
            "clicked_unique" => Self::Clicked(EmailClickedEvent { email, recipient }),
            _ => bail!("no supported `data.type` in json object"),
        };
        Ok(event)
    }
}

fn json_to_data(value: &Value) -> anyhow::Result<(EmailData, RecipientData)> {
    let email_id = value
        .get("id")
        .ok_or(anyhow!("no `id` key in email json object"))?
        .as_str()
        .ok_or(anyhow!("`id` is no string in email json object"))?;
    let email_from = value
        .get("from")
        .ok_or(anyhow!("no `from` key in email json object"))?
        .as_str()
        .ok_or(anyhow!("`from` is no string in email json object"))?;
    let email_subject = value
        .get("subject")
        .ok_or(anyhow!("no `subject` key in email json object"))?
        .as_str()
        .ok_or(anyhow!("`subject` is no string in email json object"))?;

    let recipient = value
        .get("recipient")
        .ok_or(anyhow!("no `recipient` key in email json object"))?;
    let recipient_id = recipient
        .get("id")
        .ok_or(anyhow!("no `recipient.id` key in email json object"))?
        .as_str()
        .ok_or(anyhow!("`recipient.id` is no string in email json object"))?;
    let recipient_email = recipient
        .get("email")
        .ok_or(anyhow!("no `recipient.email` key in email json object"))?
        .as_str()
        .ok_or(anyhow!(
            "`recipient.email` is no string in email json object"
        ))?;

    Ok((
        EmailData {
            id: email_id.to_string(),
            from: email_from.to_string(),
            subject: email_subject.to_string(),
        },
        RecipientData {
            id: recipient_id.to_string(),
            email: recipient_email.to_string(),
        },
    ))
}
