use std::sync::Mutex;

use anyhow::anyhow;
use lazy_static::lazy_static;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::application::web::routes::api::email::webhook::events::EmailWebhookEvent;

lazy_static! {
    static ref EVENT_CHANNEL: (
        Sender<EmailWebhookEvent>,
        Mutex<Option<Receiver<EmailWebhookEvent>>>
    ) = {
        let (send, recv) = tokio::sync::mpsc::channel(10);
        (send, Mutex::new(Some(recv)))
    };
}

pub(super) async fn send_event(event: EmailWebhookEvent) -> anyhow::Result<()> {
    EVENT_CHANNEL.0.send(event).await?;
    Ok(())
}

pub fn take_email_event_receiver() -> anyhow::Result<Receiver<EmailWebhookEvent>> {
    let mut receiver_guard = EVENT_CHANNEL
        .1
        .lock()
        .map_err(|_| anyhow!("lock for email webhook event receiver was poisoned"))?;
    let receiver = receiver_guard.take();
    receiver.ok_or(anyhow!("email webhook event receiver was already in use"))
}
