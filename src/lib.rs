//! All examples in this crate will be using the [`span_tween`] implementation
//! which requires the feature "span_tween" and it is enabled by default.
//!
//! # Getting started
//!
//! [`DefaultTweenPlugins`] provide most the stuff
//! you will need.
//! Add the plugin to your app:
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_tween::*;
//!
//! fn main() {
//!     App::default()
//!         .add_plugins((DefaultPlugins, DefaultTweenPlugins))
//!         .run();
//! }
//! ```
//!
//! # Tween and Tweener
//!
//! Tweener is a made up word to describe an entity that handles the current
//! actual playback timing for any tweens that it's responsible for. It can be
//! understand as a tween runtime or a tween player.
//!
//! Tween is your animation parameters that declares:
//! - "**What**" to interpolate, such as [`TargetComponent`], [`TargetAsset`], and
//!   [`TargetResource`].
//! - "**How**" to interpolate, such as [`interpolate::Translation`] and
//!   [`interpolate::SpriteColor`]. And they're used with something like [`EaseFunction`]
//! - "**When**" to interpolate such as [`TweenTimeSpan`].
//!
//! # Multi-entities architecture
//!
//! This crate will uses multiple entities to provide most of the flexiblity.
//! Generally implemented by using child-parent hierarchy. The exact
//! details is specific to a tweener/tween implementation.
//!
//! See [span tween structure](span_tween#entity-structure).
//!
//! # Examples
//!
//! ## Custom interpolator quick example
//!
//! See ["Your own interpolator"](crate::interpolate#your-own-interpolator).
//! See ["Registering systems"](crate::tween#registering-systems).
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_tween::prelude::*;
//!
//! #[derive(Component)]
//! struct Foo {
//!     a: f32,
//!     b: f32,
//! }
//!
//! struct InterpolateFooA {
//!     start: f32,
//!     end: f32,
//! }
//!
//! impl Interpolator for InterpolateFooA {
//!     type Item = Foo;
//!
//!     fn interpolate(&self, item: &mut Self::Item, value: f32) {
//!         item.a = self.start.lerp(self.end, value);
//!     }
//! }
//!
//! struct InterpolateFooB {
//!     start: f32,
//!     end: f32,
//! }
//!
//! impl Interpolator for InterpolateFooB {
//!     type Item = Foo;
//!
//!     fn interpolate(&self, item: &mut Self::Item, value: f32) {
//!         item.b = self.start.lerp(self.end, value);
//!     }
//! }
//!
//! fn main() {
//!     App::new().add_tween_systems((
//!         bevy_tween::component_tween_system::<BoxedInterpolator<Foo>>(),
//!         bevy_tween::component_tween_system::<InterpolateFooA>(),
//!         bevy_tween::component_tween_system::<InterpolateFooB>(),
//!     ));
//! }
//! ```
//!
//! ## Usages
//!
//! Run `cargo run --example span_tween` to see this in action.
//! ```no_run
#![doc = include_str!("../examples/span_tween/span_tween.rs")]
//! ```
//! 
//! [`Tween`]: tween::Tween
//! [`TweenDyn`]: tween::Tween
//! [`Interpolator`]: interpolate::Interpolator
//! [`Interpolation`]: interpolation::Interpolation
//! [`EaseFunction`]: interpolation::EaseFunction
//! [`TargetComponent`]: tween::TargetComponent
//! [`TargetAsset`]: tween::TargetAsset
//! [`TargetResource`]: tween::TargetResource
//! [`TweenTimeSpan`]: span_tween::TweenTimeSpan
//! [`ComponentTween`]: tween::ComponentTween
//! [`ComponentTweenDyn`]: tween::ComponentTweenDyn
#![allow(clippy::needless_doctest_main)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(missing_docs)]

use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
use bevy::{app::PluginGroupBuilder, prelude::*};

mod utils;

pub mod interpolate;
pub mod interpolation;
pub mod tween;
pub mod tween_timer;

#[cfg(feature = "span_tween")]
pub mod span_tween;

/// Commonly used items
pub mod prelude {
    pub use std::time::Duration;

    pub use crate::interpolate::{self, BoxedInterpolator, Interpolator};
    pub use crate::interpolation::EaseFunction;

    pub use crate::tween_timer::{Repeat, RepeatStyle};

    #[cfg(feature = "span_tween")]
    #[allow(deprecated)]
    pub use crate::span_tween::{
        span_tween, ChildSpanTweenBuilderExt, SpanTweenBundle,
        SpanTweenerBundle, SpanTweenerEnded, SpanTweensBuilderExt,
        WorldChildSpanTweenBuilderExt,
    };

    pub use crate::tween::{TweenEvent, TweenEventData};

    #[cfg(feature = "bevy_asset")]
    pub use crate::tween::AssetDynTween;
    #[cfg(feature = "bevy_asset")]
    pub use crate::tween::AssetTween;

    pub use crate::tween::ComponentDynTween;
    pub use crate::tween::ComponentTween;

    pub use crate::tween::ResourceDynTween;
    pub use crate::tween::ResourceTween;

    pub use crate::BevyTweenRegisterSystems;
    pub use crate::DefaultTweenPlugins;
}

#[cfg(feature = "bevy_asset")]
pub use tween::asset_dyn_tween_system;
#[cfg(feature = "bevy_asset")]
pub use tween::asset_tween_system;
#[cfg(feature = "bevy_asset")]
#[allow(deprecated)]
pub use tween::asset_tween_system_full;

pub use tween::component_dyn_tween_system;
pub use tween::component_tween_system;
#[allow(deprecated)]
pub use tween::component_tween_system_full;

pub use tween::resource_dyn_tween_system;
pub use tween::resource_tween_system;
#[allow(deprecated)]
pub use tween::resource_tween_system_full;

pub use tween::tween_event_system;
pub use tween::tween_event_taking_system;

/// Default plugins for using crate.
///
/// Plugins:
/// - [`TweenCorePlugin`]
/// - [`interpolate::DefaultInterpolatorsPlugin`]
/// - [`interpolate::DefaultDynInterpolatorsPlugin`]
/// - [`interpolation::EaseFunctionPlugin`]
/// - [`span_tween::SpanTweenPlugin`] if `"span_tween"` feature is enabled.
pub struct DefaultTweenPlugins;

impl PluginGroup for DefaultTweenPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let p = PluginGroupBuilder::start::<DefaultTweenPlugins>()
            .add(TweenCorePlugin::default())
            .add(interpolate::DefaultInterpolatorsPlugin)
            .add(interpolate::DefaultDynInterpolatorsPlugin)
            .add(interpolation::EaseFunctionPlugin)
            .add(tween::DefaultTweenEventsPlugin);
        #[cfg(feature = "span_tween")]
        let p = p.add(span_tween::SpanTweenPlugin);
        p
    }
}

/// This resource will be used while initializing tween plugin and systems.
/// [`BevyTweenRegisterSystems`] for example.
#[derive(Resource, Clone)]
pub struct TweenAppResource {
    /// Configured schedule for tween systems.
    pub schedule: InternedScheduleLabel,
}

impl Default for TweenAppResource {
    fn default() -> Self {
        TweenAppResource {
            schedule: PostUpdate.intern(),
        }
    }
}

/// Configure [`TweenSystemSet`] and register types.
///
/// [`TweenSystemSet`] configuration:
/// - In schedule configured by [`TweenAppResource`]:
///   1. [`TickTweener`],
///   2. [`Tweener`],
///   3. [`UpdateInterpolationValue`],
///   4. [`ApplyTween`],
///
///   [`TickTweener`]: [`TweenSystemSet::TickTweene`]
///   [`Tweener`]: [`TweenSystemSet::Tweener`]
///   [`UpdateInterpolationValue`]: [`TweenSystemSet::UpdateInterpolationValue`]
///   [`ApplyTween`]: [`TweenSystemSet::ApplyTween`]
#[derive(Default)]
pub struct TweenCorePlugin {
    /// See [`TweenAppResource`]
    pub app_resource: TweenAppResource,
}

impl Plugin for TweenCorePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            self.app_resource.schedule,
            (
                TweenSystemSet::TickTweener,
                TweenSystemSet::Tweener,
                TweenSystemSet::UpdateInterpolationValue,
                TweenSystemSet::ApplyTween,
            )
                .chain(),
        )
        .insert_resource(self.app_resource.clone())
        .register_type::<tween_timer::TweenTimer>()
        .register_type::<tween_timer::AnimationDirection>()
        .register_type::<tween_timer::Repeat>()
        .register_type::<tween_timer::RepeatStyle>()
        .register_type::<tween::TweenProgress>()
        .register_type::<tween::TweenerMarker>()
        .register_type::<tween::TweenInterpolationValue>();
    }
}

/// Enum of SystemSet in this crate.
/// See [`TweenCorePlugin`] for default system configuration.
#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenSystemSet {
    /// This set is for systems that responsible for ticking any
    /// tweener such as [`span_tween::tick_span_tweener_system`].
    TickTweener,
    /// This set is for systems that responsible for updating any
    /// tweener such as [`span_tween::span_tweener_system`].
    Tweener,
    /// This set is for systems that responsible for updating any
    /// [`tween::TweenInterpolationValue`] such as
    /// [`interpolation::sample_interpolations_system`].
    UpdateInterpolationValue,
    /// This set is for systems that responsible for actually executing any
    /// active tween and setting the value to its respective tweening item such
    /// as these systems:
    /// - [`tween::component_tween_system`]
    /// - [`tween::resource_tween_system`]
    /// - [`tween::asset_tween_system`]
    ApplyTween,
}

/// Helper trait to add systems by this crate to your app and avoid mistake
/// from forgetting to use the intended schedule and set.
pub trait BevyTweenRegisterSystems {
    /// Register tween systems
    fn add_tween_systems<M>(
        &mut self,
        tween_systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self;
}

impl BevyTweenRegisterSystems for App {
    /// Register tween systems in schedule configured in [`TweenAppResource`]
    /// in set [`TweenSystemSet::ApplyTween`]
    ///
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    fn add_tween_systems<M>(
        &mut self,
        tween_systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let app_resource = self
            .world
            .get_resource::<TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        self.add_systems(
            app_resource.schedule,
            tween_systems.in_set(TweenSystemSet::ApplyTween),
        )
    }
}
