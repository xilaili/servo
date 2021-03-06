/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![cfg(test)]
#![feature(core_intrinsics)]
#![feature(plugin)]

extern crate app_units;
extern crate cssparser;
extern crate euclid;
#[macro_use] extern crate html5ever_atoms;
#[macro_use] #[allow(unused_extern_crates)] extern crate matches;
extern crate owning_ref;
extern crate parking_lot;
extern crate rustc_serialize;
extern crate selectors;
#[macro_use] extern crate servo_atoms;
extern crate servo_url;
extern crate style;
extern crate style_traits;
extern crate util;

mod atomic_refcell;
mod attr;
mod cache;
mod logical_geometry;
mod media_queries;
mod owning_handle;
mod parsing;
mod properties;
mod str;
mod stylesheets;
mod stylist;
mod value;
mod viewport;

mod writing_modes {
    use style::logical_geometry::WritingMode;
    use style::properties::{INITIAL_SERVO_VALUES, get_writing_mode};

    #[test]
    fn initial_writing_mode_is_empty() {
        assert_eq!(get_writing_mode(INITIAL_SERVO_VALUES.get_inheritedbox()), WritingMode::empty())
    }
}
