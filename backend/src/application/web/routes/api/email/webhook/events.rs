use serde_json::Value;

pub enum EmailWebhookEvent {
    Send(EmailSentEvent),
    Delivered(EmailSentEvent),
    SoftBounced(EmailSoftBouncedEvent),
    HardBounced(EmailHardBouncedEvent),
    Opened(EmailOpenedEvent),
    Clicked(EmailClickedEvent),
}

pub struct EmailData {
    pub id: String,
    pub from: String,
    pub subject: String,
}

pub struct RecipientData {
    pub id: String,
    pub email: String,
}

pub struct EmailClickedEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

pub struct EmailOpenedEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

pub struct EmailHardBouncedEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

pub struct EmailSoftBouncedEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

pub struct EmailDeliveredEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

pub struct EmailSentEvent {
    pub email: EmailData,
    pub recipient: RecipientData,
}

impl TryFrom<Value> for EmailWebhookEvent {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
