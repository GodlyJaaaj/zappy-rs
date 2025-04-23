use crate::protocol::Id;
use crate::resources::{LevelRequirement, Resource};
use crate::vec2::UPosition;
use log::{debug, trace, warn};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Debug;

const MAX_SIMULTANEOUS_EVENTS: u64 = 10;

#[derive(Debug)]
pub enum Event {
    Broadcast(String),
    Forward,
    Right,
    Left,
    Look,
    Inventory,
    ConnectNbr,
    Fork,
    Eject,
    Take(Resource),
    Set(Resource),
    Incantation,

    //Can't be sent by IA
    Ko,
    Phantom, // Phantom Event, does almost nothing, only exists to make a client wait for this event
    IncantationEnd(Vec<Id>, &'static LevelRequirement, UPosition),
}

#[derive(Debug, Clone)]
pub struct TimedEvent<T> {
    pub data: T,
    pub event_id: Id,
    pub player_id: Id,
    pub expiration_tick: u64,
}

impl<T> Ord for TimedEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .expiration_tick
            .cmp(&self.expiration_tick)
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
        self.expiration_tick == other.expiration_tick && self.event_id == other.event_id
    }
}

impl<T> Eq for TimedEvent<T> {}

struct PlayerState {
    nb_events: u64,
    last_action_tick: u64,
}

impl PlayerState {
    fn new(nb_events: u64, last_action_tick: u64) -> Self {
        Self {
            nb_events,
            last_action_tick,
        }
    }
}

pub struct EventScheduler<T> {
    events: BinaryHeap<TimedEvent<T>>,
    current_tick: u64,
    next_event_id: Id,
}

impl<T> EventScheduler<T> {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
            current_tick: 0,
            next_event_id: 0,
        }
    }

    pub fn get_nb_events_by_player_id(&self, player_id: Id) -> (u64, u64) {
        let mut nb_events: u64 = 0;
        let mut last_action_tick = self.current_tick;

        for event in self.events.iter() {
            if event.player_id == player_id {
                nb_events += 1;
                if event.expiration_tick > last_action_tick {
                    last_action_tick = event.expiration_tick;
                }
            }
        }

        (nb_events, last_action_tick)
    }

    pub fn force_schedule(&mut self, data: T, event_ticks: u64, player_id: Id) -> Id {
        let event_id = self.next_event_id;
        self.next_event_id += 1;

        let expiration_tick = self.current_tick + event_ticks;

        let event = TimedEvent {
            data,
            event_id,
            player_id,
            expiration_tick,
        };

        debug!(
            "Force scheduled event #{} for player {} to execute at tick {}",
            event_id, player_id, expiration_tick
        );

        self.events.push(event);
        event_id
    }

    pub fn schedule(&mut self, data: T, event_ticks: u64, player_id: Id) -> Id {
        let event_id = self.next_event_id;
        self.next_event_id += 1;

        let (nb_events, last_tick) = self.get_nb_events_by_player_id(player_id);
        if nb_events > MAX_SIMULTANEOUS_EVENTS {
            warn!("Client {} reached max nb_events", player_id);
            return 0;
        }

        let expiration_tick = last_tick + event_ticks;

        let event = TimedEvent {
            data,
            event_id,
            player_id,
            expiration_tick,
        };

        //debug!(
        //    "Scheduled event #{} to execute at tick {}",
        //    event_id, expiration_tick
        //);
        self.events.push(event);
        event_id
    }

    pub fn shift_client_events(&mut self, player_id: Id, shift_ticks: i64) {
        let mut all_events: Vec<TimedEvent<T>> = Vec::new();
        let mut client_events: Vec<TimedEvent<T>> = Vec::new();

        while let Some(event) = self.events.pop() {
            if event.player_id == player_id {
                client_events.push(event);
            } else {
                all_events.push(event);
            }
        }

        for mut event in client_events {
            let new_expiration_tick = if shift_ticks < 0 {
                event.expiration_tick.saturating_sub(-shift_ticks as u64)
            } else {
                event.expiration_tick.saturating_add(shift_ticks as u64)
            };
            event.expiration_tick = new_expiration_tick.max(self.current_tick);
            self.events.push(event);
        }

        for event in all_events {
            self.events.push(event);
        }
    }

    pub fn tick(&mut self) -> Vec<TimedEvent<T>> {
        self.current_tick += 1;
        self.get_expired_events()
    }

    pub fn tick_multiple(&mut self, ticks: u64) -> Vec<TimedEvent<T>> {
        self.current_tick += ticks;
        self.get_expired_events()
    }

    fn get_expired_events(&mut self) -> Vec<TimedEvent<T>> {
        let mut expired_events = Vec::new();

        while let Some(event) = self.events.peek() {
            if event.expiration_tick <= self.current_tick {
                if let Some(event) = self.events.pop() {
                    //debug!(
                    //    "Event #{} executing at tick {}",
                    //    event.event_id, self.current_tick
                    //);
                    expired_events.push(event);
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

        if index.is_some() {
            let events = std::mem::take(&mut self.events);
            self.events = events
                .into_iter()
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
    where
        T: Debug,
    {
        let mut events: Vec<&TimedEvent<T>> = self.events.iter().collect();
        events.sort_by_key(|e| e.expiration_tick);
        let mut result = Vec::new();
        for event in events {
            let remaining_ticks = event.expiration_tick.saturating_sub(self.current_tick);
            result.push((event.event_id, remaining_ticks));

            trace!(
                "Event #{} by Client {}: exécution dans {} ticks, données: {:?}",
                event.event_id, event.player_id, remaining_ticks, event.data
            );
        }

        result
    }
}
