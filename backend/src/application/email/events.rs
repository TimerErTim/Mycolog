#[derive(Debug)]
pub enum EmailWebhookEvent {
    Sent(EmailSentEvent),
    Delivered(EmailDeliveredEvent),
    SoftBounced(EmailSoftBouncedEvent),
    HardBounced(EmailHardBouncedEvent),
    Opened(EmailOpenedEvent),
    Clicked(EmailClickedEvent),
}

impl EmailWebhookEvent {
    pub fn as_data(&self) -> (&EmailData, &RecipientData) {
        match self {
            EmailWebhookEvent::Sent(event) => (event.as_ref(), event.as_ref()),
            EmailWebhookEvent::Delivered(event) => (event.as_ref(), event.as_ref()),
            EmailWebhookEvent::SoftBounced(event) => (event.as_ref(), event.as_ref()),
            EmailWebhookEvent::HardBounced(event) => (event.as_ref(), event.as_ref()),
            EmailWebhookEvent::Opened(event) => (event.as_ref(), event.as_ref()),
            EmailWebhookEvent::Clicked(event) => (event.as_ref(), event.as_ref()),
        }
    }
}

#[derive(Debug)]
pub struct EmailClickedEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

impl AsRef<EmailData> for EmailClickedEvent {
    fn as_ref(&self) -> &EmailData {
        &self.email
    }
}

impl AsRef<RecipientData> for EmailClickedEvent {
    fn as_ref(&self) -> &RecipientData {
        &self.recipient
    }
}

#[derive(Debug)]
pub struct EmailOpenedEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

impl AsRef<EmailData> for EmailOpenedEvent {
    fn as_ref(&self) -> &EmailData {
        &self.email
    }
}

impl AsRef<RecipientData> for EmailOpenedEvent {
    fn as_ref(&self) -> &RecipientData {
        &self.recipient
    }
}

#[derive(Debug)]
pub struct EmailHardBouncedEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

impl AsRef<EmailData> for EmailHardBouncedEvent {
    fn as_ref(&self) -> &EmailData {
        &self.email
    }
}

impl AsRef<RecipientData> for EmailHardBouncedEvent {
    fn as_ref(&self) -> &RecipientData {
        &self.recipient
    }
}

#[derive(Debug)]
pub struct EmailSoftBouncedEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

impl AsRef<EmailData> for EmailSoftBouncedEvent {
    fn as_ref(&self) -> &EmailData {
        &self.email
    }
}

impl AsRef<RecipientData> for EmailSoftBouncedEvent {
    fn as_ref(&self) -> &RecipientData {
        &self.recipient
    }
}

#[derive(Debug)]
pub struct EmailDeliveredEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

impl AsRef<EmailData> for EmailDeliveredEvent {
    fn as_ref(&self) -> &EmailData {
        &self.email
    }
}

impl AsRef<RecipientData> for EmailDeliveredEvent {
    fn as_ref(&self) -> &RecipientData {
        &self.recipient
    }
}

#[derive(Debug)]
pub struct EmailSentEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

impl AsRef<EmailData> for EmailSentEvent {
    fn as_ref(&self) -> &EmailData {
        &self.email
    }
}

impl AsRef<RecipientData> for EmailSentEvent {
    fn as_ref(&self) -> &RecipientData {
        &self.recipient
    }
}

#[derive(Debug)]
pub struct EmailData {
    pub id: String,
    pub from: String,
    pub subject: String,
}

#[derive(Debug)]
pub struct RecipientData {
    pub id: String,
    pub email: String,
}
