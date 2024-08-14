// Copyright (c) 2024 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(non_camel_case_types)]

use iceoryx2::port::port_identifiers::{
    UniqueListenerId, UniqueNotifierId, UniquePublisherId, UniqueSubscriberId,
};

// BEGIN types definition

/// The system-wide unique id of a [`iox2_publisher_t`].
pub type iox2_unique_publisher_id_t = UniquePublisherId;

/// The system-wide unique id of a [`iox2_subscriber_t`].
pub type iox2_unique_subscriber_id_t = UniqueSubscriberId;

/// The system-wide unique id of a [`iox2_listener_t`].
pub type iox2_unique_listener_id_t = UniqueListenerId;

/// The system-wide unique id of a [`iox2_notifier_t`].
pub type iox2_unique_notifier_id_t = UniqueNotifierId;

// END types definition

// BEGIN C API

/// Checks two [`iox2_unique_publisher_id_t`] for equality.
///
/// # Safety
///
/// * `lhs` - A valid [`iox2_unique_publisher_id_t`]
/// * `rhs` - A valid [`iox2_unique_publisher_id_t`]
#[no_mangle]
pub unsafe extern "C" fn iox2_unique_publisher_id_eq(
    lhs: *const iox2_unique_publisher_id_t,
    rhs: *const iox2_unique_publisher_id_t,
) -> bool {
    debug_assert!(!lhs.is_null());
    debug_assert!(!rhs.is_null());

    (*lhs) == (*rhs)
}

/// Checks the ordering of two [`iox2_unique_publisher_id_t`].
///
/// # Safety
///
/// * `lhs` - A valid [`iox2_unique_publisher_id_t`]
/// * `rhs` - A valid [`iox2_unique_publisher_id_t`]
#[no_mangle]
pub unsafe extern "C" fn iox2_unique_publisher_id_less(
    lhs: *const iox2_unique_publisher_id_t,
    rhs: *const iox2_unique_publisher_id_t,
) -> bool {
    debug_assert!(!lhs.is_null());
    debug_assert!(!rhs.is_null());

    (*lhs) < (*rhs)
}

/// Checks two [`iox2_unique_subscriber_id_t`] for equality.
///
/// # Safety
///
/// * `lhs` - A valid [`iox2_unique_subscriber_id_t`]
/// * `rhs` - A valid [`iox2_unique_subscriber_id_t`]
#[no_mangle]
pub unsafe extern "C" fn iox2_unique_subscriber_id_eq(
    lhs: *const iox2_unique_subscriber_id_t,
    rhs: *const iox2_unique_subscriber_id_t,
) -> bool {
    debug_assert!(!lhs.is_null());
    debug_assert!(!rhs.is_null());

    (*lhs) == (*rhs)
}

/// Checks the ordering of two [`iox2_unique_subscriber_id_t`].
///
/// # Safety
///
/// * `lhs` - A valid [`iox2_unique_subscriber_id_t`]
/// * `rhs` - A valid [`iox2_unique_subscriber_id_t`]
#[no_mangle]
pub unsafe extern "C" fn iox2_unique_subscriber_id_less(
    lhs: *const iox2_unique_subscriber_id_t,
    rhs: *const iox2_unique_subscriber_id_t,
) -> bool {
    debug_assert!(!lhs.is_null());
    debug_assert!(!rhs.is_null());

    (*lhs) < (*rhs)
}

/// Checks two [`iox2_unique_notifier_id_t`] for equality.
///
/// # Safety
///
/// * `lhs` - A valid [`iox2_unique_notifier_id_t`]
/// * `rhs` - A valid [`iox2_unique_notifier_id_t`]
#[no_mangle]
pub unsafe extern "C" fn iox2_unique_notifier_id_eq(
    lhs: *const iox2_unique_notifier_id_t,
    rhs: *const iox2_unique_notifier_id_t,
) -> bool {
    debug_assert!(!lhs.is_null());
    debug_assert!(!rhs.is_null());

    (*lhs) == (*rhs)
}

/// Checks the ordering of two [`iox2_unique_notifier_id_t`].
///
/// # Safety
///
/// * `lhs` - A valid [`iox2_unique_notifier_id_t`]
/// * `rhs` - A valid [`iox2_unique_notifier_id_t`]
#[no_mangle]
pub unsafe extern "C" fn iox2_unique_notifier_id_less(
    lhs: *const iox2_unique_notifier_id_t,
    rhs: *const iox2_unique_notifier_id_t,
) -> bool {
    debug_assert!(!lhs.is_null());
    debug_assert!(!rhs.is_null());

    (*lhs) < (*rhs)
}

/// Checks two [`iox2_unique_listener_id_t`] for equality.
///
/// # Safety
///
/// * `lhs` - A valid [`iox2_unique_listener_id_t`]
/// * `rhs` - A valid [`iox2_unique_listener_id_t`]
#[no_mangle]
pub unsafe extern "C" fn iox2_unique_listener_id_eq(
    lhs: *const iox2_unique_listener_id_t,
    rhs: *const iox2_unique_listener_id_t,
) -> bool {
    debug_assert!(!lhs.is_null());
    debug_assert!(!rhs.is_null());

    (*lhs) == (*rhs)
}

/// Checks the ordering of two [`iox2_unique_listener_id_t`].
///
/// # Safety
///
/// * `lhs` - A valid [`iox2_unique_listener_id_t`]
/// * `rhs` - A valid [`iox2_unique_listener_id_t`]
#[no_mangle]
pub unsafe extern "C" fn iox2_unique_listener_id_less(
    lhs: *const iox2_unique_listener_id_t,
    rhs: *const iox2_unique_listener_id_t,
) -> bool {
    debug_assert!(!lhs.is_null());
    debug_assert!(!rhs.is_null());

    (*lhs) < (*rhs)
}

// END C API
