use super::helper::messages::Messages;

#[derive(Debug, Clone)]
pub enum Event {
    Connected,
    Disconnected,
    ErrSendingEvents,
    ValOpen,
    WaitingForVal,
    SendEvents,
    MessageReceived(Messages),
}
