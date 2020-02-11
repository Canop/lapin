use {
    anyhow::Result,
    crossbeam::channel::{
        self,
        after,
        bounded,
        Receiver,
    },
    std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::{
            Duration,
        },
    },
    termimad::{
        Event,
        EventSource,
    },
};

use lazy_static::lazy_static;

/// a TL initialized from an Arc<AtomicUsize> stays
///  alive as long as the passed arc doesn't change.
/// When it changes, is_expired returns true
#[derive(Debug, Clone)]
pub struct TaskLifetime {
    initial_value: usize,
    external_value: Arc<AtomicUsize>,
}

impl TaskLifetime {
    pub fn new(external_value: Arc<AtomicUsize>) -> TaskLifetime {
        TaskLifetime {
            initial_value: external_value.load(Ordering::Relaxed),
            external_value,
        }
    }
    pub fn unlimited() -> TaskLifetime {
        // Use a global static Arc<AtomicUsize> so that we don't have to
        // allocate more than once
        lazy_static! {
            static ref ZERO: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        }

        TaskLifetime {
            initial_value: 0,
            external_value: ZERO.clone(),
        }
    }
    pub fn is_expired(&self) -> bool {
        self.initial_value != self.external_value.load(Ordering::Relaxed)
    }
}

/// The dam controls the flow of events.
pub struct Dam {
    event_source: EventSource,
    receiver: Receiver<Event>,
    in_dam: Option<Event>,
}

impl Dam {
    pub fn new() -> Result<Self> {
        let event_source = EventSource::new()?;
        let receiver = event_source.receiver();
        Ok(Self {
            event_source,
            receiver,
            in_dam: None,
        })
    }

    //pub fn unlimited() -> Self {
    //    Self::from(channel::never())
    //}

    pub fn try_wait(&mut self, duration: Duration) -> bool {
        select! {
            recv(self.receiver) -> event => {
                // interruption
                debug!("dam interrupts wait");
                self.in_dam = event.ok();
                false
            }
            recv(after(duration)) -> _ => {
                true
            }
        }
    }

    /// non blocking
    pub fn has_event(&self) -> bool {
        !self.receiver.is_empty()
    }

    /// block until next event (including the one which
    ///  may have been pushed back into the dam).
    /// no event means the source is dead (i.e. we
    /// must quit broot)
    /// There's no event kept in dam after this call.
    pub fn next_event(&mut self) -> Option<Event> {
        if self.in_dam.is_some() {
            self.in_dam.take()
        } else {
            match self.receiver.recv() {
                Ok(event) => Some(event),
                Err(_) => {
                    debug!("dead dam"); // should be logged once
                    None
                }
            }
        }
    }

    pub fn unblock(&mut self) {
        self.event_source.unblock(false);
    }

    pub fn kill(&mut self) {
        self.event_source.unblock(true);
        let event_source_end = self.event_source.receiver().recv();
        debug!("event_source_end : {:?}", event_source_end);
    }
}

