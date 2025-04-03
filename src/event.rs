use crate::protocol::Id;
use log::debug;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct TimedEvent<T> {
    pub data: T,
    pub event_id: Id,
    pub expiration_tick: u64,
}

impl<T> Ord for TimedEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.expiration_tick.cmp(&self.expiration_tick)
            .then_with(|| other.event_id.cmp(&self.event_id))
    }
}

impl<T> PartialOrd for TimedEvent<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for TimedEvent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.expiration_tick == other.expiration_tick &&
            self.event_id == other.event_id
    }
}

impl<T> Eq for TimedEvent<T> {}

pub struct EventScheduler<T> {
    events: BinaryHeap<TimedEvent<T>>,
    current_tick: u64,
    next_event_id: Id,
    //players_states: HashMap<Id, >
}

impl<T> EventScheduler<T> {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
            current_tick: 0,
            next_event_id: 0,
        }
    }

    pub fn schedule(&mut self, data: T, ticks_from_now: u64) -> Id {
        let event_id = self.next_event_id;
        self.next_event_id += 1;

        let expiration_tick = self.current_tick + ticks_from_now;
        let event = TimedEvent {
            data,
            event_id,
            expiration_tick,
        };

        debug!("Scheduled event #{} to execute at tick {}", event_id, expiration_tick);
        self.events.push(event);
        event_id
    }

    pub fn tick(&mut self) -> Vec<T> {
        self.current_tick += 1;
        self.get_expired_events()
    }

    pub fn tick_multiple(&mut self, ticks: u64) -> Vec<T> {
        self.current_tick += ticks;
        self.get_expired_events()
    }

    fn get_expired_events(&mut self) -> Vec<T> {
        let mut expired_events = Vec::new();

        while let Some(event) = self.events.peek() {
            if event.expiration_tick <= self.current_tick {
                if let Some(event) = self.events.pop() {
                    debug!("Event #{} executing at tick {}", event.event_id, self.current_tick);
                    expired_events.push(event.data);
                }
            } else {
                break;
            }
        }

        //if !expired_events.is_empty() {
        //    info!("Executed {} events at tick {}", expired_events.len(), self.current_tick);
        //}

        expired_events
    }

    pub fn cancel(&mut self, event_id: Id) -> bool {
        let index = self.events.iter().position(|e| e.event_id == event_id);

        if let Some(index) = index {
            let events = std::mem::take(&mut self.events);
            self.events = events.into_iter()
                .filter(|e| e.event_id != event_id)
                .collect();
            debug!("Cancelled event #{}", event_id);
            true
        } else {
            false
        }
    }

    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    pub fn pending_count(&self) -> usize {
        self.events.len()
    }

    pub fn display_pending_events(&self) -> Vec<(u64, u64)>
    where T: Debug
    {
        // Créer une copie temporaire pour le tri
        let mut events: Vec<&TimedEvent<T>> = self.events.iter().collect();

        // Trier par temps d'expiration croissant
        events.sort_by_key(|e| e.expiration_tick);

        // Préparer les résultats
        let mut result = Vec::new();

        // Afficher chaque événement
        for event in events {
            let remaining_ticks = event.expiration_tick.saturating_sub(self.current_tick);
            result.push((event.event_id, remaining_ticks));

            debug!(
                "Event #{}: exécution dans {} ticks (tick {}), données: {:?}",
                event.event_id,
                remaining_ticks,
                event.expiration_tick,
                event.data
            );
        }

        result
    }
}

/// Exemple d'utilisation du gestionnaire d'événements
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_scheduling() {
        // Événements simples avec des chaînes de caractères
        let mut scheduler = EventScheduler::<String>::new();

        // Programme plusieurs événements
        scheduler.schedule("Event A".to_string(), 5);
        scheduler.schedule("Event B".to_string(), 5); // Même tick que A
        scheduler.schedule("Event C".to_string(), 10);

        // Avance de 5 ticks
        let expired = scheduler.tick_multiple(5);
        assert_eq!(expired.len(), 2);
        assert!(expired.contains(&"Event A".to_string()));
        assert!(expired.contains(&"Event B".to_string()));

        // Avance de 5 ticks supplémentaires
        let expired = scheduler.tick_multiple(5);
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0], "Event C".to_string());
    }

    #[test]
    fn test_event_cancellation() {
        let mut scheduler = EventScheduler::<&str>::new();

        // Programme plusieurs événements
        let id1 = scheduler.schedule("Event A", 5);
        let id2 = scheduler.schedule("Event B", 5);

        // Annule un événement
        assert!(scheduler.cancel(id1));

        // Avance de 5 ticks
        let expired = scheduler.tick_multiple(5);
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0], "Event B");
    }
}