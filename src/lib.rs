//#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![cfg_attr(docsrs, deny(rustdoc::broken_intra_doc_links))]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Copyright 2019+, the Cacao developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Dual-licensed under an MIT/MPL-2.0 license, see the LICENSE files in this repository.

//! # Cacao
//!
//! This library provides safe Rust bindings for `AppKit` on macOS and (eventually) `UIKit` on iOS and tvOS.
//! It tries to do so in a way that, if you've done programming for the framework before (in Swift or
//! Objective-C), will feel familiar. This is tricky in Rust due to the ownership model, but some
//! creative coding and assumptions can get us pretty far.
//!
//! This library is currently _very_ early stage and may have bugs. Your usage of it is at
//! your own risk. With that said, provided you follow the rules (regarding memory/ownership) it's
//! already fine for some apps.
//!
//! _Note that this crate relies on the Objective-C runtime. Interfacing with the runtime *requires*
//! unsafe blocks; this crate handles those unsafe interactions for you and provides a mostly safe wrapper, 
//! but by using this crate you understand that usage of `unsafe` is a given and will be somewhat 
//! rampant for wrapped controls. This does _not_ mean you can't assess, review, or question unsafe 
//! usage - just know it's happening, and in large part it's not going away._
//!
//! # Hello World
//!
//! ```rust,no_run
//! use cacao::macos::{App, AppDelegate};
//! use cacao::macos::window::Window;
//! 
//! #[derive(Default)]
//! struct BasicApp {
//!     window: Window
//! }
//!
//! impl AppDelegate for BasicApp {
//!     fn did_finish_launching(&self) {
//!        self.window.set_minimum_content_size(400., 400.);
//!        self.window.set_title("Hello World!");
//!        self.window.show();
//!     }
//! }
//!
//! fn main() {
//!     App::new("com.hello.world", BasicApp::default()).run();
//! }
//! ```
//!
//! ## Initialization
//!
//! Due to the way that AppKit and UIKit programs typically work, you're encouraged to do the bulk
//! of your work starting from the `did_finish_launching()` method of your `AppDelegate`. This
//! ensures the application has had time to initialize and do any housekeeping necessary behind the
//! scenes.
//!
//! Note that, in order for this framework to be useful, you must always elect one of the following
//! features:
//!
//! - `macos`: Implements macOS-specific APIs (menus, toolbars, windowing, and so on).
//! - `ios`: Implements iOS-specific APIs (scenes, navigation controllers, and so on).
//! - `tvos`: Implements tvOS-specific APIs. Currently barely implemented.
//!
//! The rest of the features in this framework attempt to expose a common API across the three
//! aforementioned feature platforms; if you need something else, you can often implement it
//! yourself by accessing the underlying `objc` property of a control and sending messages to it
//! directly.
//!
//! ## Optional Features
//!
//! The following are a list of [Cargo features][cargo-features] that can be enabled or disabled.
//!
//! - `cloudkit`: Links `CloudKit.framework` and provides some wrappers around CloudKit
//! functionality. Currently not feature complete.
//! - `color_fallbacks`: Provides fallback colors for older systems where `systemColor` types don't
//! exist. This feature is very uncommon and you probably don't need it.
//! - `quicklook`: Links `QuickLook.framework` and offers methods for generating preview images for
//! files.
//! - `user-notifications`: Links `UserNotifications.framework` and provides functionality for
//! emitting notifications on macOS and iOS. Note that this _requires_ your application be
//! code-signed, and will not work without it.
//! - `webview`: Links `WebKit.framework` and provides a `WebView` control backed by `WKWebView`.
//! This feature is not supported on tvOS, as the platform has no webview control.
//! - `webview-downloading-macos`: Enables downloading files from the `WebView` via a private
//! interface. This is not an App-Store-safe feature, so be aware of that before enabling. This
//! feature is not supported on iOS (a user would handle downloads very differently) or tvOS
//! (there's no web browser there at all).
//!
//! [cargo-features]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section

pub use core_foundation;
pub use core_graphics;
pub use objc;
pub use url;
pub use lazy_static;

#[cfg(target_os = "macos")]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
pub mod macos;

#[cfg(target_os = "ios")]
#[cfg_attr(docsrs, doc(cfg(target_os = "ios")))]
pub mod ios;

#[cfg(target_os = "macos")]
pub mod button;

#[cfg(any(feature = "cloudkit", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "cloudkit")))]
pub mod cloudkit;

pub mod color;

#[cfg(target_os = "macos")]
pub mod dragdrop;

pub mod error;

#[cfg(target_os = "macos")]
pub mod events;

pub mod defaults;

#[cfg(target_os = "macos")]
pub mod filesystem;

pub mod foundation;
pub mod geometry;

#[cfg(target_os = "macos")]
pub mod image;

#[cfg(target_os = "macos")]
pub mod input;

pub mod layer;
pub(crate) mod invoker;
pub mod layout;

#[cfg(target_os = "macos")]
pub mod listview;
pub mod networking;
pub mod notification_center;

#[cfg(target_os = "macos")]
pub mod pasteboard;

#[cfg(target_os = "macos")]
pub mod progress;

#[cfg(target_os = "macos")]
pub mod scrollview;

#[cfg(target_os = "macos")]
pub mod switch;

#[cfg(target_os = "macos")]
pub mod text;

#[cfg(feature = "quicklook")]
#[cfg_attr(docsrs, doc(cfg(feature = "quicklook")))]
pub mod quicklook;

#[cfg(feature = "user-notifications")]
#[cfg_attr(docsrs, doc(cfg(feature = "user-notifications")))]
pub mod user_notifications;

pub mod user_activity;
pub mod utils;

pub mod view;

#[cfg(any(feature = "webview", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "webview")))]
pub mod webview;
