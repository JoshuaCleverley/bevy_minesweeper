use bevy::prelude::Component;
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncover {}