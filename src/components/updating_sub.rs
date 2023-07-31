use flume::Receiver;
use iced::futures::stream;
use iced::futures::stream::BoxStream;
use iced::futures::StreamExt;
use iced::subscription::Recipe;
use log::warn;
use std::hash::Hasher;

use crate::Message;
use crate::UpdatingMessage;

pub struct UpdatingSubscription {
    receiver: Receiver<f32>,
}

impl UpdatingSubscription {
    pub fn new(receiver: Receiver<f32>) -> Self {
        UpdatingSubscription { receiver }
    }
}

impl<H, I> Recipe<H, I> for UpdatingSubscription
where
    H: Hasher,
{
    type Output = Message;

    fn hash(&self, _state: &mut H) {}

    fn stream(self: Box<Self>, _input: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        let receiver = self.receiver;

        // Create a stream that waits for new progress updates and converts them to GUI messages.
        stream::unfold(receiver, |receiver| async move {
            match receiver.recv() {
                Ok(progress) => Some((
                    Message::Updating(UpdatingMessage::UpdatingProgress(progress)),
                    receiver,
                )),
                Err(error) => {
                    warn!("MPSC err: {:?}", error);
                    None
                } // Channel has been disconnected, so we stop the stream.
            }
        })
        .boxed()
    }
}
