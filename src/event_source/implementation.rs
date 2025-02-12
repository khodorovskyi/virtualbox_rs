use crate::enums::VBoxEventType;
use crate::event::Event;
use crate::event_listener::EventListener;
use crate::utility::macros::macros::{get_function_result_pointer, get_function_result_unit};
use crate::{EventSource, VboxError};
use vbox_raw::sys_lib::{IEvent, IEventListener, IEventSource};

impl EventSource {
    /// Creates a new listener object, useful for passive mode.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`EventListener`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let event_source = vbox.get_event_source().unwrap();
    /// let listener = event_source.create_listener().unwrap();
    pub fn create_listener(&self) -> Result<EventListener, VboxError> {
        let listener =
            get_function_result_pointer!(self.object, CreateListener, *mut IEventListener)?;
        Ok(EventListener::new(listener))
    }

    /// Creates a new listener object, useful for passive mode.
    ///
    /// # Arguments
    ///
    /// * `sources` - Subordinate event source this one aggregates.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`EventSource`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let event_source1 = vbox.get_event_source().unwrap();
    /// let event_source2 = vbox.get_event_source().unwrap();
    /// let event_source3 = vbox.get_event_source().unwrap();
    /// let event_source = event_source1.create_aggregator(vec![&event_source2, &event_source3]);
    pub fn create_aggregator(&self, sources: Vec<&EventSource>) -> Result<EventSource, VboxError> {
        let mut sources_ptr_vec = Vec::new();
        for source_ptr in sources {
            sources_ptr_vec.push(source_ptr.object);
        }
        let sources_ptr = sources_ptr_vec.as_mut_ptr();
        let sources_count: u32 = sources_ptr_vec.len() as u32;
        let source = get_function_result_pointer!(
            self.object,
            CreateAggregator,
            *mut IEventSource,
            sources_count,
            sources_ptr
        )?;
        Ok(EventSource::new(source))
    }

    /// Creates a new listener object, useful for passive mode.
    ///
    /// # Arguments
    ///
    /// * `event_listener` - [`EventListener`] to register.
    /// * `event_types` - Event types listener is interested in. One can use wildcards like - [`VBoxEventType::Any`] to specify wildcards, matching more than one event.
    /// * `active` - Which mode this listener is operating in.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::VBoxEventType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let event_source = vbox.get_event_source().unwrap();
    /// let listener = event_source.create_listener().unwrap();
    /// event_source.register_listener(&listener, vec![VBoxEventType::Any], false).unwrap()
    pub fn register_listener(
        &self,
        event_listener: &EventListener,
        event_types: Vec<VBoxEventType>,
        active: bool,
    ) -> Result<(), VboxError> {
        let mut event_types_ptr_vec: Vec<u32> = Vec::new();
        for event_type in event_types {
            event_types_ptr_vec.push(event_type.into());
        }
        let event_listener_ptr = event_listener.object;
        let active = if active { 1 } else { 0 };
        let event_types_ptr = event_types_ptr_vec.as_mut_ptr();
        let event_types_count: u32 = event_types_ptr_vec.len() as u32;
        get_function_result_unit!(
            self.object,
            RegisterListener,
            event_listener_ptr,
            event_types_count,
            event_types_ptr,
            active
        )
    }

    /// Creates a new listener object, useful for passive mode.
    ///
    /// # Arguments
    ///
    /// * `event_listener` - [`EventListener`]  to unregister.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::VBoxEventType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let event_source = vbox.get_event_source().unwrap();
    /// let listener = event_source.create_listener().unwrap();
    /// event_source.register_listener(&listener, vec![VBoxEventType::Any], false).unwrap();
    /// event_source.unregister_listener(&listener).unwrap();
    pub fn unregister_listener(&self, event_listener: &EventListener) -> Result<(), VboxError> {
        let event_listener_ptr = event_listener.object;
        get_function_result_unit!(self.object, UnregisterListener, event_listener_ptr)
    }

    /// Creates a new listener object, useful for passive mode.
    ///
    /// # Arguments
    ///
    /// * `event_listener` - [`EventListener`]  to unregister.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let event_source = vbox.get_event_source().unwrap();
    /// let listener = event_source.create_listener().unwrap();
    /// let event = event_source.get_event(&listener, 20).unwrap();
    pub fn get_event(
        &self,
        event_listener: &EventListener,
        timeout: i32,
    ) -> Result<Event, VboxError> {
        let event_listener_ptr = event_listener.object;
        let event = get_function_result_pointer!(
            self.object,
            GetEvent,
            *mut IEvent,
            event_listener_ptr,
            timeout
        )?;
        Ok(Event::new(event))
    }

    /// Creates a new listener object, useful for passive mode.
    ///
    /// # Arguments
    ///
    /// * `event_listener` - [`EventListener`]  to unregister.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let event_source = vbox.get_event_source().unwrap();
    /// let listener = event_source.create_listener().unwrap();
    /// let event = event_source.get_event(&listener, 20).unwrap();
    /// event_source.event_processed(&listener, &event).unwrap()
    pub fn event_processed(
        &self,
        event_listener: &EventListener,
        event: &Event,
    ) -> Result<(), VboxError> {
        let event_listener_ptr = event_listener.object;
        let event_ptr = event.object;
        get_function_result_unit!(self.object, EventProcessed, event_listener_ptr, event_ptr)
    }
}
