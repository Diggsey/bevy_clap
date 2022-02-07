use std::{ffi::OsString, marker::PhantomData};

use bevy::{ecs::system::Resource, prelude::Plugin};
use clap::Parser;

pub use clap;

pub struct ClapPlugin<T> {
    args: Option<Vec<OsString>>,
    phantom: PhantomData<fn() -> T>,
}

impl<T> Default for ClapPlugin<T> {
    fn default() -> Self {
        Self {
            args: None,
            phantom: PhantomData,
        }
    }
}

impl<T> ClapPlugin<T> {
    pub fn with_args<I: Into<OsString>>(args: impl IntoIterator<Item = I>) -> Self {
        Self {
            args: Some(args.into_iter().map(Into::into).collect()),
            phantom: PhantomData,
        }
    }
}

impl<T: Parser + Resource> Plugin for ClapPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(if let Some(args) = self.args.clone() {
            T::parse_from(args)
        } else {
            T::parse()
        });
    }
}
