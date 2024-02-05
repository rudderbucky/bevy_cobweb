//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts
use std::hash::Hash;

//-------------------------------------------------------------------------------------------------------------------

/// Tracks metadata for accessing system events.
#[derive(Resource)]
pub(crate) struct SystemEventAccessTracker
{
    /// True when in a system processing a system event.
    currently_reacting: bool,
    /// The entity where system event data is stored.
    data_entity: Entity,
}

impl SystemEventAccessTracker
{
    /// Sets the 'is reacting' flag.
    pub(crate) fn start(&mut self, data_entity: Entity)
    {
        self.currently_reacting = true;
        self.data_entity = data_entity;
    }

    /// Unsets the 'is reacting' flag.
    ///
    /// Returns the data entity so it can be despawned.
    pub(crate) fn end(&mut self) -> Entity
    {
        self.currently_reacting = false;
        self.data_entity
    }

    /// Returns `true` if a system event is currently being processed.
    fn is_reacting(&self) -> bool
    {
        self.currently_reacting
    }

    /// Returns the data entity of the most recent system event.
    fn data_entity(&self) -> Entity
    {
        self.data_entity
    }
}

impl Default for SystemEventAccessTracker
{
    fn default() -> Self
    {
        Self{
            currently_reacting: false,
            data_entity: Entity::from_raw(0u32),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Stores data for a system event.
#[derive(Component)]
pub(crate) struct SystemEventData<T: Send + Sync + 'static>
{
    data: Option<T>,
}

impl<T: Send + Sync + 'static> SystemEventData<T>
{
    /// Makes a new system event data.
    pub(crate) fn new(data: T) -> Self
    {
        Self{ data: Some(data) }
    }

    /// Takes the system event data.
    fn take(&mut self) -> Option<T>
    {
        self.data.take()
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// System parameter for receiving system event data.
#[derive(SystemParam)]
pub struct SystemEvent<'w, 's, T: Send + Sync + 'static>
{
    tracker: Res<'w, SystemEventAccessTracker>,
    data: Query<'w, 's, &'static mut SystemEventData<T>>,
}

impl<'w, 's, T: Send + Sync + 'static> SystemEvent<'w, 's, T>
{
    /// Takes system event data if it exists.
    ///
    /// This will return at most one unique `T` each time a system runs.
    pub fn take(&mut self) -> Option<T>
    {
        if !self.tracker.is_reacting() { return None; }
        let Ok(data) = self.data.get_mut(self.tracker.data_entity()) else { return None; };

        data.take()
    }
}

//-------------------------------------------------------------------------------------------------------------------
