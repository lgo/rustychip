//! Emulator provides interfaces for emulators core emulator primitives.

// Clocked is a core trait for all emulator components. It provides the
// `clock` method to execute a clock cycle of the component.
//
// The approach is a clock accurate type of emulation. It comprimises
// speed and component-level simplicity for accuracy, by forcing
// emulation of components to implement precise clock-level behaviour.
//
// This is also limited to only be plausible for systems which have
// well-understood clock behaviour for components.
pub trait Clocked {
  fn clock(&mut self);
}
