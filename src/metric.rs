// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause
//! The purpose of this module is to provide abstractions for working with
//! metrics in the context of rust-vmm components where there is a strong need
//! to have metrics as an optional feature.
//!
//! As multiple stakeholders are using these components, there are also
//! questions regarding the serialization format, as metrics are expected to be
//! flexible enough to allow different formatting, serialization and writers.
//! When using the rust-vmm metrics, the expectation is that VMMs built on top
//! of these components can choose what metrics they’re interested in and also
//! can add their own custom metrics without the need to maintain forks.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Abstraction over the common metric operations.
///
/// An object implementing `Metric` is expected to have an inner counter that
/// can be incremented and reset.
pub trait Metric {
    /// Adds `value` to the current counter.
    fn add(&self, value: usize);
    /// Increments by 1 unit the current counter.
    fn inc(&self) {
        self.add(1);
    }
    /// Returns current value of the counter.
    fn count(&self) -> usize;
    /// Resets the metric counter.
    fn reset(&self) {}
}

/// A dummy `Metric` implementation that can be used in components that do not
/// expose metrics.
///
/// This allows replacing `Metric` with a NOP.
///
/// # Example
/// ```rust
/// use vmm_sys_util::metric::Metric;
///
/// struct ObjWithMetric<T: Metric> {
///     obj_metric: T,
///     foo: String,
/// }
///
/// impl ObjWithMetric<()> {
///     fn new(foo: String) -> ObjWithMetric<()> {
///         ObjWithMetric {
///             obj_metric: (),
///             foo,
///         }
///     }
/// }
///
/// // Even though ObjWithMetric has a type parameter, we do not need to
/// // explicitly define it because it was replaced with `()`.
/// let obj_no_metric = ObjWithMetric::new(String::from("foo"));
/// ```
impl Metric for () {
    fn add(&self, _: usize) {}

    fn count(&self) -> usize {
        0
    }
}

impl Metric for AtomicUsize {
    /// Adds `value` to the current counter.
    fn add(&self, value: usize) {
        self.fetch_add(value, Ordering::Relaxed);
    }

    /// Returns current value of the counter.
    fn count(&self) -> usize {
        self.load(Ordering::Relaxed)
    }
}
