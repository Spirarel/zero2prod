use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
