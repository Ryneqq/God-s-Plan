mod widgets;
mod gods_ui;

pub use gods_ui::GodsUI;
pub use widgets::{ GodsButton, GodsFpsCounter };

use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin, EguiSettings};