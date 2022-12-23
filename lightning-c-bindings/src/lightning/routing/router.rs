// This file is Copyright its original authors, visible in version control
// history and in the source files from which this was generated.
//
// This file is licensed under the license available in the LICENSE or LICENSE.md
// file in the root of this repository or, if no such file exists, the same
// license as that which applies to the original source files from which this
// source was automatically generated.

//! The top-level routing/network map tracking logic lives here.
//!
//! You probably want to create a P2PGossipSync and use that as your RoutingMessageHandler and then
//! interrogate it to get routes for your own payments.

use alloc::str::FromStr;
use core::ffi::c_void;
use core::convert::Infallible;
use bitcoin::hashes::Hash;
use crate::c_types::*;
#[cfg(feature="no-std")]
use alloc::{vec::Vec, boxed::Box};


use lightning::routing::router::DefaultRouter as nativeDefaultRouterImport;
pub(crate) type nativeDefaultRouter = nativeDefaultRouterImport<&'static lightning::routing::gossip::NetworkGraph<crate::lightning::util::logger::Logger>, crate::lightning::util::logger::Logger, crate::lightning::routing::scoring::LockableScore>;

/// A [`Router`] implemented using [`find_route`].
#[must_use]
#[repr(C)]
pub struct DefaultRouter {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeDefaultRouter,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for DefaultRouter {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeDefaultRouter>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the DefaultRouter, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn DefaultRouter_free(this_obj: DefaultRouter) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn DefaultRouter_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeDefaultRouter) };
}
#[allow(unused)]
impl DefaultRouter {
	pub(crate) fn get_native_ref(&self) -> &'static nativeDefaultRouter {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeDefaultRouter {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeDefaultRouter {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// Creates a new router.
#[must_use]
#[no_mangle]
pub extern "C" fn DefaultRouter_new(network_graph: &crate::lightning::routing::gossip::NetworkGraph, mut logger: crate::lightning::util::logger::Logger, mut random_seed_bytes: crate::c_types::ThirtyTwoBytes, mut scorer: crate::lightning::routing::scoring::LockableScore) -> crate::lightning::routing::router::DefaultRouter {
	let mut ret = lightning::routing::router::DefaultRouter::new(network_graph.get_native_ref(), logger, random_seed_bytes.data, scorer);
	crate::lightning::routing::router::DefaultRouter { inner: ObjOps::heap_alloc(ret), is_owned: true }
}

impl From<nativeDefaultRouter> for crate::lightning::routing::router::Router {
	fn from(obj: nativeDefaultRouter) -> Self {
		let mut rust_obj = DefaultRouter { inner: ObjOps::heap_alloc(obj), is_owned: true };
		let mut ret = DefaultRouter_as_Router(&rust_obj);
		// We want to free rust_obj when ret gets drop()'d, not rust_obj, so wipe rust_obj's pointer and set ret's free() fn
		rust_obj.inner = core::ptr::null_mut();
		ret.free = Some(DefaultRouter_free_void);
		ret
	}
}
/// Constructs a new Router which calls the relevant methods on this_arg.
/// This copies the `inner` pointer in this_arg and thus the returned Router must be freed before this_arg is
#[no_mangle]
pub extern "C" fn DefaultRouter_as_Router(this_arg: &DefaultRouter) -> crate::lightning::routing::router::Router {
	crate::lightning::routing::router::Router {
		this_arg: unsafe { ObjOps::untweak_ptr((*this_arg).inner) as *mut c_void },
		free: None,
		find_route: DefaultRouter_Router_find_route,
		find_route_with_id: DefaultRouter_Router_find_route_with_id,
		notify_payment_path_failed: DefaultRouter_Router_notify_payment_path_failed,
		notify_payment_path_successful: DefaultRouter_Router_notify_payment_path_successful,
		notify_payment_probe_successful: DefaultRouter_Router_notify_payment_probe_successful,
		notify_payment_probe_failed: DefaultRouter_Router_notify_payment_probe_failed,
	}
}

#[must_use]
extern "C" fn DefaultRouter_Router_find_route(this_arg: *const c_void, mut payer: crate::c_types::PublicKey, route_params: &crate::lightning::routing::router::RouteParameters, first_hops: *mut crate::c_types::derived::CVec_ChannelDetailsZ, mut inflight_htlcs: crate::lightning::routing::router::InFlightHtlcs) -> crate::c_types::derived::CResult_RouteLightningErrorZ {
	let mut local_first_hops_base = if first_hops == core::ptr::null_mut() { None } else { Some( { let mut local_first_hops_0 = Vec::new(); for mut item in unsafe { &mut *first_hops }.as_slice().iter() { local_first_hops_0.push( { item.get_native_ref() }); }; local_first_hops_0 }) }; let mut local_first_hops = local_first_hops_base.as_ref().map(|a| &a[..]);
	let mut ret = <nativeDefaultRouter as lightning::routing::router::Router<>>::find_route(unsafe { &mut *(this_arg as *mut nativeDefaultRouter) }, &payer.into_rust(), route_params.get_native_ref(), local_first_hops, *unsafe { Box::from_raw(inflight_htlcs.take_inner()) });
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::Route { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::LightningError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_ret
}
#[must_use]
extern "C" fn DefaultRouter_Router_find_route_with_id(this_arg: *const c_void, mut payer: crate::c_types::PublicKey, route_params: &crate::lightning::routing::router::RouteParameters, first_hops: *mut crate::c_types::derived::CVec_ChannelDetailsZ, mut inflight_htlcs: crate::lightning::routing::router::InFlightHtlcs, mut _payment_hash: crate::c_types::ThirtyTwoBytes, mut _payment_id: crate::c_types::ThirtyTwoBytes) -> crate::c_types::derived::CResult_RouteLightningErrorZ {
	let mut local_first_hops_base = if first_hops == core::ptr::null_mut() { None } else { Some( { let mut local_first_hops_0 = Vec::new(); for mut item in unsafe { &mut *first_hops }.as_slice().iter() { local_first_hops_0.push( { item.get_native_ref() }); }; local_first_hops_0 }) }; let mut local_first_hops = local_first_hops_base.as_ref().map(|a| &a[..]);
	let mut ret = <nativeDefaultRouter as lightning::routing::router::Router<>>::find_route_with_id(unsafe { &mut *(this_arg as *mut nativeDefaultRouter) }, &payer.into_rust(), route_params.get_native_ref(), local_first_hops, *unsafe { Box::from_raw(inflight_htlcs.take_inner()) }, ::lightning::ln::PaymentHash(_payment_hash.data), ::lightning::ln::channelmanager::PaymentId(_payment_id.data));
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::Route { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::LightningError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_ret
}
extern "C" fn DefaultRouter_Router_notify_payment_path_failed(this_arg: *const c_void, mut path: crate::c_types::derived::CVec_RouteHopZ, mut short_channel_id: u64) {
	let mut local_path = Vec::new(); for mut item in path.as_slice().iter() { local_path.push( { item.get_native_ref() }); };
	<nativeDefaultRouter as lightning::routing::router::Router<>>::notify_payment_path_failed(unsafe { &mut *(this_arg as *mut nativeDefaultRouter) }, &local_path[..], short_channel_id)
}
extern "C" fn DefaultRouter_Router_notify_payment_path_successful(this_arg: *const c_void, mut path: crate::c_types::derived::CVec_RouteHopZ) {
	let mut local_path = Vec::new(); for mut item in path.as_slice().iter() { local_path.push( { item.get_native_ref() }); };
	<nativeDefaultRouter as lightning::routing::router::Router<>>::notify_payment_path_successful(unsafe { &mut *(this_arg as *mut nativeDefaultRouter) }, &local_path[..])
}
extern "C" fn DefaultRouter_Router_notify_payment_probe_successful(this_arg: *const c_void, mut path: crate::c_types::derived::CVec_RouteHopZ) {
	let mut local_path = Vec::new(); for mut item in path.as_slice().iter() { local_path.push( { item.get_native_ref() }); };
	<nativeDefaultRouter as lightning::routing::router::Router<>>::notify_payment_probe_successful(unsafe { &mut *(this_arg as *mut nativeDefaultRouter) }, &local_path[..])
}
extern "C" fn DefaultRouter_Router_notify_payment_probe_failed(this_arg: *const c_void, mut path: crate::c_types::derived::CVec_RouteHopZ, mut short_channel_id: u64) {
	let mut local_path = Vec::new(); for mut item in path.as_slice().iter() { local_path.push( { item.get_native_ref() }); };
	<nativeDefaultRouter as lightning::routing::router::Router<>>::notify_payment_probe_failed(unsafe { &mut *(this_arg as *mut nativeDefaultRouter) }, &local_path[..], short_channel_id)
}

/// A trait defining behavior for routing a payment.
#[repr(C)]
pub struct Router {
	/// An opaque pointer which is passed to your function implementations as an argument.
	/// This has no meaning in the LDK, and can be NULL or any other value.
	pub this_arg: *mut c_void,
	/// Finds a [`Route`] between `payer` and `payee` for a payment with the given values.
	///
	/// Note that first_hops (or a relevant inner pointer) may be NULL or all-0s to represent None
	#[must_use]
	pub find_route: extern "C" fn (this_arg: *const c_void, payer: crate::c_types::PublicKey, route_params: &crate::lightning::routing::router::RouteParameters, first_hops: *mut crate::c_types::derived::CVec_ChannelDetailsZ, inflight_htlcs: crate::lightning::routing::router::InFlightHtlcs) -> crate::c_types::derived::CResult_RouteLightningErrorZ,
	/// Finds a [`Route`] between `payer` and `payee` for a payment with the given values. Includes
	/// `PaymentHash` and `PaymentId` to be able to correlate the request with a specific payment.
	///
	/// Note that first_hops (or a relevant inner pointer) may be NULL or all-0s to represent None
	#[must_use]
	pub find_route_with_id: extern "C" fn (this_arg: *const c_void, payer: crate::c_types::PublicKey, route_params: &crate::lightning::routing::router::RouteParameters, first_hops: *mut crate::c_types::derived::CVec_ChannelDetailsZ, inflight_htlcs: crate::lightning::routing::router::InFlightHtlcs, _payment_hash: crate::c_types::ThirtyTwoBytes, _payment_id: crate::c_types::ThirtyTwoBytes) -> crate::c_types::derived::CResult_RouteLightningErrorZ,
	/// Lets the router know that payment through a specific path has failed.
	pub notify_payment_path_failed: extern "C" fn (this_arg: *const c_void, path: crate::c_types::derived::CVec_RouteHopZ, short_channel_id: u64),
	/// Lets the router know that payment through a specific path was successful.
	pub notify_payment_path_successful: extern "C" fn (this_arg: *const c_void, path: crate::c_types::derived::CVec_RouteHopZ),
	/// Lets the router know that a payment probe was successful.
	pub notify_payment_probe_successful: extern "C" fn (this_arg: *const c_void, path: crate::c_types::derived::CVec_RouteHopZ),
	/// Lets the router know that a payment probe failed.
	pub notify_payment_probe_failed: extern "C" fn (this_arg: *const c_void, path: crate::c_types::derived::CVec_RouteHopZ, short_channel_id: u64),
	/// Frees any resources associated with this object given its this_arg pointer.
	/// Does not need to free the outer struct containing function pointers and may be NULL is no resources need to be freed.
	pub free: Option<extern "C" fn(this_arg: *mut c_void)>,
}
unsafe impl Send for Router {}
unsafe impl Sync for Router {}
#[no_mangle]
pub(crate) extern "C" fn Router_clone_fields(orig: &Router) -> Router {
	Router {
		this_arg: orig.this_arg,
		find_route: Clone::clone(&orig.find_route),
		find_route_with_id: Clone::clone(&orig.find_route_with_id),
		notify_payment_path_failed: Clone::clone(&orig.notify_payment_path_failed),
		notify_payment_path_successful: Clone::clone(&orig.notify_payment_path_successful),
		notify_payment_probe_successful: Clone::clone(&orig.notify_payment_probe_successful),
		notify_payment_probe_failed: Clone::clone(&orig.notify_payment_probe_failed),
		free: Clone::clone(&orig.free),
	}
}

use lightning::routing::router::Router as rustRouter;
impl rustRouter for Router {
	fn find_route(&self, mut payer: &bitcoin::secp256k1::PublicKey, mut route_params: &lightning::routing::router::RouteParameters, mut first_hops: Option<&[&lightning::ln::channelmanager::ChannelDetails]>, mut inflight_htlcs: lightning::routing::router::InFlightHtlcs) -> Result<lightning::routing::router::Route, lightning::ln::msgs::LightningError> {
		let mut local_first_hops_base = if first_hops.is_none() { SmartPtr::null() } else { SmartPtr::from_obj( { let mut local_first_hops_0 = Vec::new(); for item in (first_hops.unwrap()).iter() { local_first_hops_0.push( { crate::lightning::ln::channelmanager::ChannelDetails { inner: unsafe { ObjOps::nonnull_ptr_to_inner(((*item) as *const lightning::ln::channelmanager::ChannelDetails<>) as *mut _) }, is_owned: false } }); }; local_first_hops_0.into() }) }; let mut local_first_hops = *local_first_hops_base;
		let mut ret = (self.find_route)(self.this_arg, crate::c_types::PublicKey::from_rust(&payer), &crate::lightning::routing::router::RouteParameters { inner: unsafe { ObjOps::nonnull_ptr_to_inner((route_params as *const lightning::routing::router::RouteParameters<>) as *mut _) }, is_owned: false }, local_first_hops, crate::lightning::routing::router::InFlightHtlcs { inner: ObjOps::heap_alloc(inflight_htlcs), is_owned: true });
		let mut local_ret = match ret.result_ok { true => Ok( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).take_inner()) } }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn find_route_with_id(&self, mut payer: &bitcoin::secp256k1::PublicKey, mut route_params: &lightning::routing::router::RouteParameters, mut first_hops: Option<&[&lightning::ln::channelmanager::ChannelDetails]>, mut inflight_htlcs: lightning::routing::router::InFlightHtlcs, mut _payment_hash: lightning::ln::PaymentHash, mut _payment_id: lightning::ln::channelmanager::PaymentId) -> Result<lightning::routing::router::Route, lightning::ln::msgs::LightningError> {
		let mut local_first_hops_base = if first_hops.is_none() { SmartPtr::null() } else { SmartPtr::from_obj( { let mut local_first_hops_0 = Vec::new(); for item in (first_hops.unwrap()).iter() { local_first_hops_0.push( { crate::lightning::ln::channelmanager::ChannelDetails { inner: unsafe { ObjOps::nonnull_ptr_to_inner(((*item) as *const lightning::ln::channelmanager::ChannelDetails<>) as *mut _) }, is_owned: false } }); }; local_first_hops_0.into() }) }; let mut local_first_hops = *local_first_hops_base;
		let mut ret = (self.find_route_with_id)(self.this_arg, crate::c_types::PublicKey::from_rust(&payer), &crate::lightning::routing::router::RouteParameters { inner: unsafe { ObjOps::nonnull_ptr_to_inner((route_params as *const lightning::routing::router::RouteParameters<>) as *mut _) }, is_owned: false }, local_first_hops, crate::lightning::routing::router::InFlightHtlcs { inner: ObjOps::heap_alloc(inflight_htlcs), is_owned: true }, crate::c_types::ThirtyTwoBytes { data: _payment_hash.0 }, crate::c_types::ThirtyTwoBytes { data: _payment_id.0 });
		let mut local_ret = match ret.result_ok { true => Ok( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).take_inner()) } }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn notify_payment_path_failed(&self, mut path: &[&lightning::routing::router::RouteHop], mut short_channel_id: u64) {
		let mut local_path = Vec::new(); for item in path.iter() { local_path.push( { crate::lightning::routing::router::RouteHop { inner: unsafe { ObjOps::nonnull_ptr_to_inner(((*item) as *const lightning::routing::router::RouteHop<>) as *mut _) }, is_owned: false } }); };
		(self.notify_payment_path_failed)(self.this_arg, local_path.into(), short_channel_id)
	}
	fn notify_payment_path_successful(&self, mut path: &[&lightning::routing::router::RouteHop]) {
		let mut local_path = Vec::new(); for item in path.iter() { local_path.push( { crate::lightning::routing::router::RouteHop { inner: unsafe { ObjOps::nonnull_ptr_to_inner(((*item) as *const lightning::routing::router::RouteHop<>) as *mut _) }, is_owned: false } }); };
		(self.notify_payment_path_successful)(self.this_arg, local_path.into())
	}
	fn notify_payment_probe_successful(&self, mut path: &[&lightning::routing::router::RouteHop]) {
		let mut local_path = Vec::new(); for item in path.iter() { local_path.push( { crate::lightning::routing::router::RouteHop { inner: unsafe { ObjOps::nonnull_ptr_to_inner(((*item) as *const lightning::routing::router::RouteHop<>) as *mut _) }, is_owned: false } }); };
		(self.notify_payment_probe_successful)(self.this_arg, local_path.into())
	}
	fn notify_payment_probe_failed(&self, mut path: &[&lightning::routing::router::RouteHop], mut short_channel_id: u64) {
		let mut local_path = Vec::new(); for item in path.iter() { local_path.push( { crate::lightning::routing::router::RouteHop { inner: unsafe { ObjOps::nonnull_ptr_to_inner(((*item) as *const lightning::routing::router::RouteHop<>) as *mut _) }, is_owned: false } }); };
		(self.notify_payment_probe_failed)(self.this_arg, local_path.into(), short_channel_id)
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl core::ops::Deref for Router {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// Calls the free function if one is set
#[no_mangle]
pub extern "C" fn Router_free(this_ptr: Router) { }
impl Drop for Router {
	fn drop(&mut self) {
		if let Some(f) = self.free {
			f(self.this_arg);
		}
	}
}

use lightning::routing::router::ScorerAccountingForInFlightHtlcs as nativeScorerAccountingForInFlightHtlcsImport;
pub(crate) type nativeScorerAccountingForInFlightHtlcs = nativeScorerAccountingForInFlightHtlcsImport<crate::lightning::routing::scoring::Score>;

/// [`Score`] implementation that factors in in-flight HTLC liquidity.
///
/// Useful for custom [`Router`] implementations to wrap their [`Score`] on-the-fly when calling
/// [`find_route`].
///
/// [`Score`]: crate::routing::scoring::Score
#[must_use]
#[repr(C)]
pub struct ScorerAccountingForInFlightHtlcs {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeScorerAccountingForInFlightHtlcs,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ScorerAccountingForInFlightHtlcs {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeScorerAccountingForInFlightHtlcs>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ScorerAccountingForInFlightHtlcs, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ScorerAccountingForInFlightHtlcs_free(this_obj: ScorerAccountingForInFlightHtlcs) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ScorerAccountingForInFlightHtlcs_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeScorerAccountingForInFlightHtlcs) };
}
#[allow(unused)]
impl ScorerAccountingForInFlightHtlcs {
	pub(crate) fn get_native_ref(&self) -> &'static nativeScorerAccountingForInFlightHtlcs {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeScorerAccountingForInFlightHtlcs {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeScorerAccountingForInFlightHtlcs {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// Initialize a new `ScorerAccountingForInFlightHtlcs`.
#[must_use]
#[no_mangle]
pub extern "C" fn ScorerAccountingForInFlightHtlcs_new(mut scorer: crate::lightning::routing::scoring::Score, mut inflight_htlcs: crate::lightning::routing::router::InFlightHtlcs) -> crate::lightning::routing::router::ScorerAccountingForInFlightHtlcs {
	let mut ret = lightning::routing::router::ScorerAccountingForInFlightHtlcs::new(scorer, *unsafe { Box::from_raw(inflight_htlcs.take_inner()) });
	crate::lightning::routing::router::ScorerAccountingForInFlightHtlcs { inner: ObjOps::heap_alloc(ret), is_owned: true }
}

#[no_mangle]
/// Serialize the ScorerAccountingForInFlightHtlcs object into a byte array which can be read by ScorerAccountingForInFlightHtlcs_read
pub extern "C" fn ScorerAccountingForInFlightHtlcs_write(obj: &crate::lightning::routing::router::ScorerAccountingForInFlightHtlcs) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ScorerAccountingForInFlightHtlcs_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeScorerAccountingForInFlightHtlcs) })
}
impl From<nativeScorerAccountingForInFlightHtlcs> for crate::lightning::routing::scoring::Score {
	fn from(obj: nativeScorerAccountingForInFlightHtlcs) -> Self {
		let mut rust_obj = ScorerAccountingForInFlightHtlcs { inner: ObjOps::heap_alloc(obj), is_owned: true };
		let mut ret = ScorerAccountingForInFlightHtlcs_as_Score(&rust_obj);
		// We want to free rust_obj when ret gets drop()'d, not rust_obj, so wipe rust_obj's pointer and set ret's free() fn
		rust_obj.inner = core::ptr::null_mut();
		ret.free = Some(ScorerAccountingForInFlightHtlcs_free_void);
		ret
	}
}
/// Constructs a new Score which calls the relevant methods on this_arg.
/// This copies the `inner` pointer in this_arg and thus the returned Score must be freed before this_arg is
#[no_mangle]
pub extern "C" fn ScorerAccountingForInFlightHtlcs_as_Score(this_arg: &ScorerAccountingForInFlightHtlcs) -> crate::lightning::routing::scoring::Score {
	crate::lightning::routing::scoring::Score {
		this_arg: unsafe { ObjOps::untweak_ptr((*this_arg).inner) as *mut c_void },
		free: None,
		channel_penalty_msat: ScorerAccountingForInFlightHtlcs_Score_channel_penalty_msat,
		payment_path_failed: ScorerAccountingForInFlightHtlcs_Score_payment_path_failed,
		payment_path_successful: ScorerAccountingForInFlightHtlcs_Score_payment_path_successful,
		probe_failed: ScorerAccountingForInFlightHtlcs_Score_probe_failed,
		probe_successful: ScorerAccountingForInFlightHtlcs_Score_probe_successful,
		write: ScorerAccountingForInFlightHtlcs_write_void,
	}
}

#[must_use]
extern "C" fn ScorerAccountingForInFlightHtlcs_Score_channel_penalty_msat(this_arg: *const c_void, mut short_channel_id: u64, source: &crate::lightning::routing::gossip::NodeId, target: &crate::lightning::routing::gossip::NodeId, mut usage: crate::lightning::routing::scoring::ChannelUsage) -> u64 {
	let mut ret = <nativeScorerAccountingForInFlightHtlcs as lightning::routing::scoring::Score<>>::channel_penalty_msat(unsafe { &mut *(this_arg as *mut nativeScorerAccountingForInFlightHtlcs) }, short_channel_id, source.get_native_ref(), target.get_native_ref(), *unsafe { Box::from_raw(usage.take_inner()) });
	ret
}
extern "C" fn ScorerAccountingForInFlightHtlcs_Score_payment_path_failed(this_arg: *mut c_void, mut path: crate::c_types::derived::CVec_RouteHopZ, mut short_channel_id: u64) {
	let mut local_path = Vec::new(); for mut item in path.as_slice().iter() { local_path.push( { item.get_native_ref() }); };
	<nativeScorerAccountingForInFlightHtlcs as lightning::routing::scoring::Score<>>::payment_path_failed(unsafe { &mut *(this_arg as *mut nativeScorerAccountingForInFlightHtlcs) }, &local_path[..], short_channel_id)
}
extern "C" fn ScorerAccountingForInFlightHtlcs_Score_payment_path_successful(this_arg: *mut c_void, mut path: crate::c_types::derived::CVec_RouteHopZ) {
	let mut local_path = Vec::new(); for mut item in path.as_slice().iter() { local_path.push( { item.get_native_ref() }); };
	<nativeScorerAccountingForInFlightHtlcs as lightning::routing::scoring::Score<>>::payment_path_successful(unsafe { &mut *(this_arg as *mut nativeScorerAccountingForInFlightHtlcs) }, &local_path[..])
}
extern "C" fn ScorerAccountingForInFlightHtlcs_Score_probe_failed(this_arg: *mut c_void, mut path: crate::c_types::derived::CVec_RouteHopZ, mut short_channel_id: u64) {
	let mut local_path = Vec::new(); for mut item in path.as_slice().iter() { local_path.push( { item.get_native_ref() }); };
	<nativeScorerAccountingForInFlightHtlcs as lightning::routing::scoring::Score<>>::probe_failed(unsafe { &mut *(this_arg as *mut nativeScorerAccountingForInFlightHtlcs) }, &local_path[..], short_channel_id)
}
extern "C" fn ScorerAccountingForInFlightHtlcs_Score_probe_successful(this_arg: *mut c_void, mut path: crate::c_types::derived::CVec_RouteHopZ) {
	let mut local_path = Vec::new(); for mut item in path.as_slice().iter() { local_path.push( { item.get_native_ref() }); };
	<nativeScorerAccountingForInFlightHtlcs as lightning::routing::scoring::Score<>>::probe_successful(unsafe { &mut *(this_arg as *mut nativeScorerAccountingForInFlightHtlcs) }, &local_path[..])
}


use lightning::routing::router::InFlightHtlcs as nativeInFlightHtlcsImport;
pub(crate) type nativeInFlightHtlcs = nativeInFlightHtlcsImport;

/// A data structure for tracking in-flight HTLCs. May be used during pathfinding to account for
/// in-use channel liquidity.
#[must_use]
#[repr(C)]
pub struct InFlightHtlcs {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeInFlightHtlcs,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for InFlightHtlcs {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeInFlightHtlcs>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the InFlightHtlcs, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn InFlightHtlcs_free(this_obj: InFlightHtlcs) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn InFlightHtlcs_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeInFlightHtlcs) };
}
#[allow(unused)]
impl InFlightHtlcs {
	pub(crate) fn get_native_ref(&self) -> &'static nativeInFlightHtlcs {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeInFlightHtlcs {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeInFlightHtlcs {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
impl Clone for InFlightHtlcs {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeInFlightHtlcs>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn InFlightHtlcs_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeInFlightHtlcs)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the InFlightHtlcs
pub extern "C" fn InFlightHtlcs_clone(orig: &InFlightHtlcs) -> InFlightHtlcs {
	orig.clone()
}
/// Constructs an empty `InFlightHtlcs`.
#[must_use]
#[no_mangle]
pub extern "C" fn InFlightHtlcs_new() -> crate::lightning::routing::router::InFlightHtlcs {
	let mut ret = lightning::routing::router::InFlightHtlcs::new();
	crate::lightning::routing::router::InFlightHtlcs { inner: ObjOps::heap_alloc(ret), is_owned: true }
}

/// Returns liquidity in msat given the public key of the HTLC source, target, and short channel
/// id.
#[must_use]
#[no_mangle]
pub extern "C" fn InFlightHtlcs_used_liquidity_msat(this_arg: &crate::lightning::routing::router::InFlightHtlcs, source: &crate::lightning::routing::gossip::NodeId, target: &crate::lightning::routing::gossip::NodeId, mut channel_scid: u64) -> crate::c_types::derived::COption_u64Z {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.used_liquidity_msat(source.get_native_ref(), target.get_native_ref(), channel_scid);
	let mut local_ret = if ret.is_none() { crate::c_types::derived::COption_u64Z::None } else { crate::c_types::derived::COption_u64Z::Some( { ret.unwrap() }) };
	local_ret
}

#[no_mangle]
/// Serialize the InFlightHtlcs object into a byte array which can be read by InFlightHtlcs_read
pub extern "C" fn InFlightHtlcs_write(obj: &crate::lightning::routing::router::InFlightHtlcs) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn InFlightHtlcs_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeInFlightHtlcs) })
}
#[no_mangle]
/// Read a InFlightHtlcs from a byte array, created by InFlightHtlcs_write
pub extern "C" fn InFlightHtlcs_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_InFlightHtlcsDecodeErrorZ {
	let res: Result<lightning::routing::router::InFlightHtlcs, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::InFlightHtlcs { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}

use lightning::routing::router::RouteHop as nativeRouteHopImport;
pub(crate) type nativeRouteHop = nativeRouteHopImport;

/// A hop in a route
#[must_use]
#[repr(C)]
pub struct RouteHop {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeRouteHop,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for RouteHop {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeRouteHop>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the RouteHop, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn RouteHop_free(this_obj: RouteHop) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RouteHop_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeRouteHop) };
}
#[allow(unused)]
impl RouteHop {
	pub(crate) fn get_native_ref(&self) -> &'static nativeRouteHop {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeRouteHop {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeRouteHop {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The node_id of the node at this hop.
#[no_mangle]
pub extern "C" fn RouteHop_get_pubkey(this_ptr: &RouteHop) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().pubkey;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The node_id of the node at this hop.
#[no_mangle]
pub extern "C" fn RouteHop_set_pubkey(this_ptr: &mut RouteHop, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.pubkey = val.into_rust();
}
/// The node_announcement features of the node at this hop. For the last hop, these may be
/// amended to match the features present in the invoice this node generated.
#[no_mangle]
pub extern "C" fn RouteHop_get_node_features(this_ptr: &RouteHop) -> crate::lightning::ln::features::NodeFeatures {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().node_features;
	crate::lightning::ln::features::NodeFeatures { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::features::NodeFeatures<>) as *mut _) }, is_owned: false }
}
/// The node_announcement features of the node at this hop. For the last hop, these may be
/// amended to match the features present in the invoice this node generated.
#[no_mangle]
pub extern "C" fn RouteHop_set_node_features(this_ptr: &mut RouteHop, mut val: crate::lightning::ln::features::NodeFeatures) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.node_features = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The channel that should be used from the previous hop to reach this node.
#[no_mangle]
pub extern "C" fn RouteHop_get_short_channel_id(this_ptr: &RouteHop) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().short_channel_id;
	*inner_val
}
/// The channel that should be used from the previous hop to reach this node.
#[no_mangle]
pub extern "C" fn RouteHop_set_short_channel_id(this_ptr: &mut RouteHop, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.short_channel_id = val;
}
/// The channel_announcement features of the channel that should be used from the previous hop
/// to reach this node.
#[no_mangle]
pub extern "C" fn RouteHop_get_channel_features(this_ptr: &RouteHop) -> crate::lightning::ln::features::ChannelFeatures {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_features;
	crate::lightning::ln::features::ChannelFeatures { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::features::ChannelFeatures<>) as *mut _) }, is_owned: false }
}
/// The channel_announcement features of the channel that should be used from the previous hop
/// to reach this node.
#[no_mangle]
pub extern "C" fn RouteHop_set_channel_features(this_ptr: &mut RouteHop, mut val: crate::lightning::ln::features::ChannelFeatures) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_features = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The fee taken on this hop (for paying for the use of the *next* channel in the path).
/// For the last hop, this should be the full value of the payment (might be more than
/// requested if we had to match htlc_minimum_msat).
#[no_mangle]
pub extern "C" fn RouteHop_get_fee_msat(this_ptr: &RouteHop) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().fee_msat;
	*inner_val
}
/// The fee taken on this hop (for paying for the use of the *next* channel in the path).
/// For the last hop, this should be the full value of the payment (might be more than
/// requested if we had to match htlc_minimum_msat).
#[no_mangle]
pub extern "C" fn RouteHop_set_fee_msat(this_ptr: &mut RouteHop, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.fee_msat = val;
}
/// The CLTV delta added for this hop. For the last hop, this should be the full CLTV value
/// expected at the destination, in excess of the current block height.
#[no_mangle]
pub extern "C" fn RouteHop_get_cltv_expiry_delta(this_ptr: &RouteHop) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().cltv_expiry_delta;
	*inner_val
}
/// The CLTV delta added for this hop. For the last hop, this should be the full CLTV value
/// expected at the destination, in excess of the current block height.
#[no_mangle]
pub extern "C" fn RouteHop_set_cltv_expiry_delta(this_ptr: &mut RouteHop, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.cltv_expiry_delta = val;
}
/// Constructs a new RouteHop given each field
#[must_use]
#[no_mangle]
pub extern "C" fn RouteHop_new(mut pubkey_arg: crate::c_types::PublicKey, mut node_features_arg: crate::lightning::ln::features::NodeFeatures, mut short_channel_id_arg: u64, mut channel_features_arg: crate::lightning::ln::features::ChannelFeatures, mut fee_msat_arg: u64, mut cltv_expiry_delta_arg: u32) -> RouteHop {
	RouteHop { inner: ObjOps::heap_alloc(nativeRouteHop {
		pubkey: pubkey_arg.into_rust(),
		node_features: *unsafe { Box::from_raw(node_features_arg.take_inner()) },
		short_channel_id: short_channel_id_arg,
		channel_features: *unsafe { Box::from_raw(channel_features_arg.take_inner()) },
		fee_msat: fee_msat_arg,
		cltv_expiry_delta: cltv_expiry_delta_arg,
	}), is_owned: true }
}
impl Clone for RouteHop {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeRouteHop>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RouteHop_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeRouteHop)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the RouteHop
pub extern "C" fn RouteHop_clone(orig: &RouteHop) -> RouteHop {
	orig.clone()
}
/// Checks if two RouteHops contain equal inner contents.
#[no_mangle]
pub extern "C" fn RouteHop_hash(o: &RouteHop) -> u64 {
	if o.inner.is_null() { return 0; }
	// Note that we'd love to use alloc::collections::hash_map::DefaultHasher but it's not in core
	#[allow(deprecated)]
	let mut hasher = core::hash::SipHasher::new();
	core::hash::Hash::hash(o.get_native_ref(), &mut hasher);
	core::hash::Hasher::finish(&hasher)
}
/// Checks if two RouteHops contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn RouteHop_eq(a: &RouteHop, b: &RouteHop) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}
#[no_mangle]
/// Serialize the RouteHop object into a byte array which can be read by RouteHop_read
pub extern "C" fn RouteHop_write(obj: &crate::lightning::routing::router::RouteHop) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn RouteHop_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeRouteHop) })
}
#[no_mangle]
/// Read a RouteHop from a byte array, created by RouteHop_write
pub extern "C" fn RouteHop_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_RouteHopDecodeErrorZ {
	let res: Result<lightning::routing::router::RouteHop, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::RouteHop { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}

use lightning::routing::router::Route as nativeRouteImport;
pub(crate) type nativeRoute = nativeRouteImport;

/// A route directs a payment from the sender (us) to the recipient. If the recipient supports MPP,
/// it can take multiple paths. Each path is composed of one or more hops through the network.
#[must_use]
#[repr(C)]
pub struct Route {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeRoute,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for Route {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeRoute>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the Route, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn Route_free(this_obj: Route) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Route_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeRoute) };
}
#[allow(unused)]
impl Route {
	pub(crate) fn get_native_ref(&self) -> &'static nativeRoute {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeRoute {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeRoute {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The list of routes taken for a single (potentially-)multi-part payment. The pubkey of the
/// last RouteHop in each path must be the same. Each entry represents a list of hops, NOT
/// INCLUDING our own, where the last hop is the destination. Thus, this must always be at
/// least length one. While the maximum length of any given path is variable, keeping the length
/// of any path less or equal to 19 should currently ensure it is viable.
#[no_mangle]
pub extern "C" fn Route_get_paths(this_ptr: &Route) -> crate::c_types::derived::CVec_CVec_RouteHopZZ {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().paths;
	let mut local_inner_val = Vec::new(); for item in inner_val.iter() { local_inner_val.push( { let mut local_inner_val_0 = Vec::new(); for item in item.iter() { local_inner_val_0.push( { crate::lightning::routing::router::RouteHop { inner: unsafe { ObjOps::nonnull_ptr_to_inner((item as *const lightning::routing::router::RouteHop<>) as *mut _) }, is_owned: false } }); }; local_inner_val_0.into() }); };
	local_inner_val.into()
}
/// The list of routes taken for a single (potentially-)multi-part payment. The pubkey of the
/// last RouteHop in each path must be the same. Each entry represents a list of hops, NOT
/// INCLUDING our own, where the last hop is the destination. Thus, this must always be at
/// least length one. While the maximum length of any given path is variable, keeping the length
/// of any path less or equal to 19 should currently ensure it is viable.
#[no_mangle]
pub extern "C" fn Route_set_paths(this_ptr: &mut Route, mut val: crate::c_types::derived::CVec_CVec_RouteHopZZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { let mut local_val_0 = Vec::new(); for mut item in item.into_rust().drain(..) { local_val_0.push( { *unsafe { Box::from_raw(item.take_inner()) } }); }; local_val_0 }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.paths = local_val;
}
/// The `payment_params` parameter passed to [`find_route`].
/// This is used by `ChannelManager` to track information which may be required for retries,
/// provided back to you via [`Event::PaymentPathFailed`].
///
/// [`Event::PaymentPathFailed`]: crate::util::events::Event::PaymentPathFailed
///
/// Note that the return value (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn Route_get_payment_params(this_ptr: &Route) -> crate::lightning::routing::router::PaymentParameters {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().payment_params;
	let mut local_inner_val = crate::lightning::routing::router::PaymentParameters { inner: unsafe { (if inner_val.is_none() { core::ptr::null() } else { ObjOps::nonnull_ptr_to_inner( { (inner_val.as_ref().unwrap()) }) } as *const lightning::routing::router::PaymentParameters<>) as *mut _ }, is_owned: false };
	local_inner_val
}
/// The `payment_params` parameter passed to [`find_route`].
/// This is used by `ChannelManager` to track information which may be required for retries,
/// provided back to you via [`Event::PaymentPathFailed`].
///
/// [`Event::PaymentPathFailed`]: crate::util::events::Event::PaymentPathFailed
///
/// Note that val (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn Route_set_payment_params(this_ptr: &mut Route, mut val: crate::lightning::routing::router::PaymentParameters) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.take_inner()) } }) };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.payment_params = local_val;
}
/// Constructs a new Route given each field
#[must_use]
#[no_mangle]
pub extern "C" fn Route_new(mut paths_arg: crate::c_types::derived::CVec_CVec_RouteHopZZ, mut payment_params_arg: crate::lightning::routing::router::PaymentParameters) -> Route {
	let mut local_paths_arg = Vec::new(); for mut item in paths_arg.into_rust().drain(..) { local_paths_arg.push( { let mut local_paths_arg_0 = Vec::new(); for mut item in item.into_rust().drain(..) { local_paths_arg_0.push( { *unsafe { Box::from_raw(item.take_inner()) } }); }; local_paths_arg_0 }); };
	let mut local_payment_params_arg = if payment_params_arg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(payment_params_arg.take_inner()) } }) };
	Route { inner: ObjOps::heap_alloc(nativeRoute {
		paths: local_paths_arg,
		payment_params: local_payment_params_arg,
	}), is_owned: true }
}
impl Clone for Route {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeRoute>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Route_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeRoute)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the Route
pub extern "C" fn Route_clone(orig: &Route) -> Route {
	orig.clone()
}
/// Checks if two Routes contain equal inner contents.
#[no_mangle]
pub extern "C" fn Route_hash(o: &Route) -> u64 {
	if o.inner.is_null() { return 0; }
	// Note that we'd love to use alloc::collections::hash_map::DefaultHasher but it's not in core
	#[allow(deprecated)]
	let mut hasher = core::hash::SipHasher::new();
	core::hash::Hash::hash(o.get_native_ref(), &mut hasher);
	core::hash::Hasher::finish(&hasher)
}
/// Checks if two Routes contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn Route_eq(a: &Route, b: &Route) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}
/// Returns the total amount of fees paid on this [`Route`].
///
/// This doesn't include any extra payment made to the recipient, which can happen in excess of
/// the amount passed to [`find_route`]'s `params.final_value_msat`.
#[must_use]
#[no_mangle]
pub extern "C" fn Route_get_total_fees(this_arg: &crate::lightning::routing::router::Route) -> u64 {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.get_total_fees();
	ret
}

/// Returns the total amount paid on this [`Route`], excluding the fees.
#[must_use]
#[no_mangle]
pub extern "C" fn Route_get_total_amount(this_arg: &crate::lightning::routing::router::Route) -> u64 {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.get_total_amount();
	ret
}

#[no_mangle]
/// Serialize the Route object into a byte array which can be read by Route_read
pub extern "C" fn Route_write(obj: &crate::lightning::routing::router::Route) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn Route_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeRoute) })
}
#[no_mangle]
/// Read a Route from a byte array, created by Route_write
pub extern "C" fn Route_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_RouteDecodeErrorZ {
	let res: Result<lightning::routing::router::Route, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::Route { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}

use lightning::routing::router::RouteParameters as nativeRouteParametersImport;
pub(crate) type nativeRouteParameters = nativeRouteParametersImport;

/// Parameters needed to find a [`Route`].
///
/// Passed to [`find_route`] and [`build_route_from_hops`], but also provided in
/// [`Event::PaymentPathFailed`] for retrying a failed payment path.
///
/// [`Event::PaymentPathFailed`]: crate::util::events::Event::PaymentPathFailed
#[must_use]
#[repr(C)]
pub struct RouteParameters {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeRouteParameters,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for RouteParameters {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeRouteParameters>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the RouteParameters, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn RouteParameters_free(this_obj: RouteParameters) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RouteParameters_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeRouteParameters) };
}
#[allow(unused)]
impl RouteParameters {
	pub(crate) fn get_native_ref(&self) -> &'static nativeRouteParameters {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeRouteParameters {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeRouteParameters {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The parameters of the failed payment path.
#[no_mangle]
pub extern "C" fn RouteParameters_get_payment_params(this_ptr: &RouteParameters) -> crate::lightning::routing::router::PaymentParameters {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().payment_params;
	crate::lightning::routing::router::PaymentParameters { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::routing::router::PaymentParameters<>) as *mut _) }, is_owned: false }
}
/// The parameters of the failed payment path.
#[no_mangle]
pub extern "C" fn RouteParameters_set_payment_params(this_ptr: &mut RouteParameters, mut val: crate::lightning::routing::router::PaymentParameters) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.payment_params = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The amount in msats sent on the failed payment path.
#[no_mangle]
pub extern "C" fn RouteParameters_get_final_value_msat(this_ptr: &RouteParameters) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().final_value_msat;
	*inner_val
}
/// The amount in msats sent on the failed payment path.
#[no_mangle]
pub extern "C" fn RouteParameters_set_final_value_msat(this_ptr: &mut RouteParameters, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.final_value_msat = val;
}
/// The CLTV on the final hop of the failed payment path.
#[no_mangle]
pub extern "C" fn RouteParameters_get_final_cltv_expiry_delta(this_ptr: &RouteParameters) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().final_cltv_expiry_delta;
	*inner_val
}
/// The CLTV on the final hop of the failed payment path.
#[no_mangle]
pub extern "C" fn RouteParameters_set_final_cltv_expiry_delta(this_ptr: &mut RouteParameters, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.final_cltv_expiry_delta = val;
}
/// Constructs a new RouteParameters given each field
#[must_use]
#[no_mangle]
pub extern "C" fn RouteParameters_new(mut payment_params_arg: crate::lightning::routing::router::PaymentParameters, mut final_value_msat_arg: u64, mut final_cltv_expiry_delta_arg: u32) -> RouteParameters {
	RouteParameters { inner: ObjOps::heap_alloc(nativeRouteParameters {
		payment_params: *unsafe { Box::from_raw(payment_params_arg.take_inner()) },
		final_value_msat: final_value_msat_arg,
		final_cltv_expiry_delta: final_cltv_expiry_delta_arg,
	}), is_owned: true }
}
impl Clone for RouteParameters {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeRouteParameters>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RouteParameters_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeRouteParameters)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the RouteParameters
pub extern "C" fn RouteParameters_clone(orig: &RouteParameters) -> RouteParameters {
	orig.clone()
}
#[no_mangle]
/// Serialize the RouteParameters object into a byte array which can be read by RouteParameters_read
pub extern "C" fn RouteParameters_write(obj: &crate::lightning::routing::router::RouteParameters) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn RouteParameters_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeRouteParameters) })
}
#[no_mangle]
/// Read a RouteParameters from a byte array, created by RouteParameters_write
pub extern "C" fn RouteParameters_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_RouteParametersDecodeErrorZ {
	let res: Result<lightning::routing::router::RouteParameters, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::RouteParameters { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
/// Maximum total CTLV difference we allow for a full payment path.

#[no_mangle]
pub static DEFAULT_MAX_TOTAL_CLTV_EXPIRY_DELTA: u32 = lightning::routing::router::DEFAULT_MAX_TOTAL_CLTV_EXPIRY_DELTA;
/// Maximum number of paths we allow an (MPP) payment to have.

#[no_mangle]
pub static DEFAULT_MAX_PATH_COUNT: u8 = lightning::routing::router::DEFAULT_MAX_PATH_COUNT;

use lightning::routing::router::PaymentParameters as nativePaymentParametersImport;
pub(crate) type nativePaymentParameters = nativePaymentParametersImport;

/// The recipient of a payment.
#[must_use]
#[repr(C)]
pub struct PaymentParameters {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativePaymentParameters,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for PaymentParameters {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativePaymentParameters>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the PaymentParameters, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn PaymentParameters_free(this_obj: PaymentParameters) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn PaymentParameters_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativePaymentParameters) };
}
#[allow(unused)]
impl PaymentParameters {
	pub(crate) fn get_native_ref(&self) -> &'static nativePaymentParameters {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativePaymentParameters {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativePaymentParameters {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The node id of the payee.
#[no_mangle]
pub extern "C" fn PaymentParameters_get_payee_pubkey(this_ptr: &PaymentParameters) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().payee_pubkey;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The node id of the payee.
#[no_mangle]
pub extern "C" fn PaymentParameters_set_payee_pubkey(this_ptr: &mut PaymentParameters, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.payee_pubkey = val.into_rust();
}
/// Features supported by the payee.
///
/// May be set from the payee's invoice or via [`for_keysend`]. May be `None` if the invoice
/// does not contain any features.
///
/// [`for_keysend`]: Self::for_keysend
///
/// Note that the return value (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn PaymentParameters_get_features(this_ptr: &PaymentParameters) -> crate::lightning::ln::features::InvoiceFeatures {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().features;
	let mut local_inner_val = crate::lightning::ln::features::InvoiceFeatures { inner: unsafe { (if inner_val.is_none() { core::ptr::null() } else { ObjOps::nonnull_ptr_to_inner( { (inner_val.as_ref().unwrap()) }) } as *const lightning::ln::features::InvoiceFeatures<>) as *mut _ }, is_owned: false };
	local_inner_val
}
/// Features supported by the payee.
///
/// May be set from the payee's invoice or via [`for_keysend`]. May be `None` if the invoice
/// does not contain any features.
///
/// [`for_keysend`]: Self::for_keysend
///
/// Note that val (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn PaymentParameters_set_features(this_ptr: &mut PaymentParameters, mut val: crate::lightning::ln::features::InvoiceFeatures) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.take_inner()) } }) };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.features = local_val;
}
/// Hints for routing to the payee, containing channels connecting the payee to public nodes.
#[no_mangle]
pub extern "C" fn PaymentParameters_get_route_hints(this_ptr: &PaymentParameters) -> crate::c_types::derived::CVec_RouteHintZ {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().route_hints;
	let mut local_inner_val = Vec::new(); for item in inner_val.iter() { local_inner_val.push( { crate::lightning::routing::router::RouteHint { inner: unsafe { ObjOps::nonnull_ptr_to_inner((item as *const lightning::routing::router::RouteHint<>) as *mut _) }, is_owned: false } }); };
	local_inner_val.into()
}
/// Hints for routing to the payee, containing channels connecting the payee to public nodes.
#[no_mangle]
pub extern "C" fn PaymentParameters_set_route_hints(this_ptr: &mut PaymentParameters, mut val: crate::c_types::derived::CVec_RouteHintZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.route_hints = local_val;
}
/// Expiration of a payment to the payee, in seconds relative to the UNIX epoch.
#[no_mangle]
pub extern "C" fn PaymentParameters_get_expiry_time(this_ptr: &PaymentParameters) -> crate::c_types::derived::COption_u64Z {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().expiry_time;
	let mut local_inner_val = if inner_val.is_none() { crate::c_types::derived::COption_u64Z::None } else { crate::c_types::derived::COption_u64Z::Some( { inner_val.unwrap() }) };
	local_inner_val
}
/// Expiration of a payment to the payee, in seconds relative to the UNIX epoch.
#[no_mangle]
pub extern "C" fn PaymentParameters_set_expiry_time(this_ptr: &mut PaymentParameters, mut val: crate::c_types::derived::COption_u64Z) {
	let mut local_val = if val.is_some() { Some( { val.take() }) } else { None };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.expiry_time = local_val;
}
/// The maximum total CLTV delta we accept for the route.
/// Defaults to [`DEFAULT_MAX_TOTAL_CLTV_EXPIRY_DELTA`].
#[no_mangle]
pub extern "C" fn PaymentParameters_get_max_total_cltv_expiry_delta(this_ptr: &PaymentParameters) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().max_total_cltv_expiry_delta;
	*inner_val
}
/// The maximum total CLTV delta we accept for the route.
/// Defaults to [`DEFAULT_MAX_TOTAL_CLTV_EXPIRY_DELTA`].
#[no_mangle]
pub extern "C" fn PaymentParameters_set_max_total_cltv_expiry_delta(this_ptr: &mut PaymentParameters, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.max_total_cltv_expiry_delta = val;
}
/// The maximum number of paths that may be used by (MPP) payments.
/// Defaults to [`DEFAULT_MAX_PATH_COUNT`].
#[no_mangle]
pub extern "C" fn PaymentParameters_get_max_path_count(this_ptr: &PaymentParameters) -> u8 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().max_path_count;
	*inner_val
}
/// The maximum number of paths that may be used by (MPP) payments.
/// Defaults to [`DEFAULT_MAX_PATH_COUNT`].
#[no_mangle]
pub extern "C" fn PaymentParameters_set_max_path_count(this_ptr: &mut PaymentParameters, mut val: u8) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.max_path_count = val;
}
/// Selects the maximum share of a channel's total capacity which will be sent over a channel,
/// as a power of 1/2. A higher value prefers to send the payment using more MPP parts whereas
/// a lower value prefers to send larger MPP parts, potentially saturating channels and
/// increasing failure probability for those paths.
///
/// Note that this restriction will be relaxed during pathfinding after paths which meet this
/// restriction have been found. While paths which meet this criteria will be searched for, it
/// is ultimately up to the scorer to select them over other paths.
///
/// A value of 0 will allow payments up to and including a channel's total announced usable
/// capacity, a value of one will only use up to half its capacity, two 1/4, etc.
///
/// Default value: 2
#[no_mangle]
pub extern "C" fn PaymentParameters_get_max_channel_saturation_power_of_half(this_ptr: &PaymentParameters) -> u8 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().max_channel_saturation_power_of_half;
	*inner_val
}
/// Selects the maximum share of a channel's total capacity which will be sent over a channel,
/// as a power of 1/2. A higher value prefers to send the payment using more MPP parts whereas
/// a lower value prefers to send larger MPP parts, potentially saturating channels and
/// increasing failure probability for those paths.
///
/// Note that this restriction will be relaxed during pathfinding after paths which meet this
/// restriction have been found. While paths which meet this criteria will be searched for, it
/// is ultimately up to the scorer to select them over other paths.
///
/// A value of 0 will allow payments up to and including a channel's total announced usable
/// capacity, a value of one will only use up to half its capacity, two 1/4, etc.
///
/// Default value: 2
#[no_mangle]
pub extern "C" fn PaymentParameters_set_max_channel_saturation_power_of_half(this_ptr: &mut PaymentParameters, mut val: u8) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.max_channel_saturation_power_of_half = val;
}
/// A list of SCIDs which this payment was previously attempted over and which caused the
/// payment to fail. Future attempts for the same payment shouldn't be relayed through any of
/// these SCIDs.
///
/// Returns a copy of the field.
#[no_mangle]
pub extern "C" fn PaymentParameters_get_previously_failed_channels(this_ptr: &PaymentParameters) -> crate::c_types::derived::CVec_u64Z {
	let mut inner_val = this_ptr.get_native_mut_ref().previously_failed_channels.clone();
	let mut local_inner_val = Vec::new(); for mut item in inner_val.drain(..) { local_inner_val.push( { item }); };
	local_inner_val.into()
}
/// A list of SCIDs which this payment was previously attempted over and which caused the
/// payment to fail. Future attempts for the same payment shouldn't be relayed through any of
/// these SCIDs.
#[no_mangle]
pub extern "C" fn PaymentParameters_set_previously_failed_channels(this_ptr: &mut PaymentParameters, mut val: crate::c_types::derived::CVec_u64Z) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { item }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.previously_failed_channels = local_val;
}
/// Constructs a new PaymentParameters given each field
#[must_use]
#[no_mangle]
pub extern "C" fn PaymentParameters_new(mut payee_pubkey_arg: crate::c_types::PublicKey, mut features_arg: crate::lightning::ln::features::InvoiceFeatures, mut route_hints_arg: crate::c_types::derived::CVec_RouteHintZ, mut expiry_time_arg: crate::c_types::derived::COption_u64Z, mut max_total_cltv_expiry_delta_arg: u32, mut max_path_count_arg: u8, mut max_channel_saturation_power_of_half_arg: u8, mut previously_failed_channels_arg: crate::c_types::derived::CVec_u64Z) -> PaymentParameters {
	let mut local_features_arg = if features_arg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(features_arg.take_inner()) } }) };
	let mut local_route_hints_arg = Vec::new(); for mut item in route_hints_arg.into_rust().drain(..) { local_route_hints_arg.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	let mut local_expiry_time_arg = if expiry_time_arg.is_some() { Some( { expiry_time_arg.take() }) } else { None };
	let mut local_previously_failed_channels_arg = Vec::new(); for mut item in previously_failed_channels_arg.into_rust().drain(..) { local_previously_failed_channels_arg.push( { item }); };
	PaymentParameters { inner: ObjOps::heap_alloc(nativePaymentParameters {
		payee_pubkey: payee_pubkey_arg.into_rust(),
		features: local_features_arg,
		route_hints: local_route_hints_arg,
		expiry_time: local_expiry_time_arg,
		max_total_cltv_expiry_delta: max_total_cltv_expiry_delta_arg,
		max_path_count: max_path_count_arg,
		max_channel_saturation_power_of_half: max_channel_saturation_power_of_half_arg,
		previously_failed_channels: local_previously_failed_channels_arg,
	}), is_owned: true }
}
impl Clone for PaymentParameters {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativePaymentParameters>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn PaymentParameters_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativePaymentParameters)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the PaymentParameters
pub extern "C" fn PaymentParameters_clone(orig: &PaymentParameters) -> PaymentParameters {
	orig.clone()
}
/// Checks if two PaymentParameterss contain equal inner contents.
#[no_mangle]
pub extern "C" fn PaymentParameters_hash(o: &PaymentParameters) -> u64 {
	if o.inner.is_null() { return 0; }
	// Note that we'd love to use alloc::collections::hash_map::DefaultHasher but it's not in core
	#[allow(deprecated)]
	let mut hasher = core::hash::SipHasher::new();
	core::hash::Hash::hash(o.get_native_ref(), &mut hasher);
	core::hash::Hasher::finish(&hasher)
}
/// Checks if two PaymentParameterss contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn PaymentParameters_eq(a: &PaymentParameters, b: &PaymentParameters) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}
#[no_mangle]
/// Serialize the PaymentParameters object into a byte array which can be read by PaymentParameters_read
pub extern "C" fn PaymentParameters_write(obj: &crate::lightning::routing::router::PaymentParameters) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn PaymentParameters_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativePaymentParameters) })
}
#[no_mangle]
/// Read a PaymentParameters from a byte array, created by PaymentParameters_write
pub extern "C" fn PaymentParameters_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_PaymentParametersDecodeErrorZ {
	let res: Result<lightning::routing::router::PaymentParameters, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::PaymentParameters { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
/// Creates a payee with the node id of the given `pubkey`.
#[must_use]
#[no_mangle]
pub extern "C" fn PaymentParameters_from_node_id(mut payee_pubkey: crate::c_types::PublicKey) -> crate::lightning::routing::router::PaymentParameters {
	let mut ret = lightning::routing::router::PaymentParameters::from_node_id(payee_pubkey.into_rust());
	crate::lightning::routing::router::PaymentParameters { inner: ObjOps::heap_alloc(ret), is_owned: true }
}

/// Creates a payee with the node id of the given `pubkey` to use for keysend payments.
#[must_use]
#[no_mangle]
pub extern "C" fn PaymentParameters_for_keysend(mut payee_pubkey: crate::c_types::PublicKey) -> crate::lightning::routing::router::PaymentParameters {
	let mut ret = lightning::routing::router::PaymentParameters::for_keysend(payee_pubkey.into_rust());
	crate::lightning::routing::router::PaymentParameters { inner: ObjOps::heap_alloc(ret), is_owned: true }
}


use lightning::routing::router::RouteHint as nativeRouteHintImport;
pub(crate) type nativeRouteHint = nativeRouteHintImport;

/// A list of hops along a payment path terminating with a channel to the recipient.
#[must_use]
#[repr(C)]
pub struct RouteHint {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeRouteHint,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for RouteHint {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeRouteHint>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the RouteHint, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn RouteHint_free(this_obj: RouteHint) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RouteHint_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeRouteHint) };
}
#[allow(unused)]
impl RouteHint {
	pub(crate) fn get_native_ref(&self) -> &'static nativeRouteHint {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeRouteHint {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeRouteHint {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
#[no_mangle]
pub extern "C" fn RouteHint_get_a(this_ptr: &RouteHint) -> crate::c_types::derived::CVec_RouteHintHopZ {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().0;
	let mut local_inner_val = Vec::new(); for item in inner_val.iter() { local_inner_val.push( { crate::lightning::routing::router::RouteHintHop { inner: unsafe { ObjOps::nonnull_ptr_to_inner((item as *const lightning::routing::router::RouteHintHop<>) as *mut _) }, is_owned: false } }); };
	local_inner_val.into()
}
#[no_mangle]
pub extern "C" fn RouteHint_set_a(this_ptr: &mut RouteHint, mut val: crate::c_types::derived::CVec_RouteHintHopZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.0 = local_val;
}
/// Constructs a new RouteHint given each field
#[must_use]
#[no_mangle]
pub extern "C" fn RouteHint_new(mut a_arg: crate::c_types::derived::CVec_RouteHintHopZ) -> RouteHint {
	let mut local_a_arg = Vec::new(); for mut item in a_arg.into_rust().drain(..) { local_a_arg.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	RouteHint { inner: ObjOps::heap_alloc(lightning::routing::router::RouteHint (
		local_a_arg,
	)), is_owned: true }
}
impl Clone for RouteHint {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeRouteHint>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RouteHint_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeRouteHint)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the RouteHint
pub extern "C" fn RouteHint_clone(orig: &RouteHint) -> RouteHint {
	orig.clone()
}
/// Checks if two RouteHints contain equal inner contents.
#[no_mangle]
pub extern "C" fn RouteHint_hash(o: &RouteHint) -> u64 {
	if o.inner.is_null() { return 0; }
	// Note that we'd love to use alloc::collections::hash_map::DefaultHasher but it's not in core
	#[allow(deprecated)]
	let mut hasher = core::hash::SipHasher::new();
	core::hash::Hash::hash(o.get_native_ref(), &mut hasher);
	core::hash::Hasher::finish(&hasher)
}
/// Checks if two RouteHints contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn RouteHint_eq(a: &RouteHint, b: &RouteHint) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}
#[no_mangle]
/// Serialize the RouteHint object into a byte array which can be read by RouteHint_read
pub extern "C" fn RouteHint_write(obj: &crate::lightning::routing::router::RouteHint) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn RouteHint_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeRouteHint) })
}
#[no_mangle]
/// Read a RouteHint from a byte array, created by RouteHint_write
pub extern "C" fn RouteHint_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_RouteHintDecodeErrorZ {
	let res: Result<lightning::routing::router::RouteHint, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::RouteHint { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}

use lightning::routing::router::RouteHintHop as nativeRouteHintHopImport;
pub(crate) type nativeRouteHintHop = nativeRouteHintHopImport;

/// A channel descriptor for a hop along a payment path.
#[must_use]
#[repr(C)]
pub struct RouteHintHop {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeRouteHintHop,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for RouteHintHop {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeRouteHintHop>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the RouteHintHop, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn RouteHintHop_free(this_obj: RouteHintHop) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RouteHintHop_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeRouteHintHop) };
}
#[allow(unused)]
impl RouteHintHop {
	pub(crate) fn get_native_ref(&self) -> &'static nativeRouteHintHop {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeRouteHintHop {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeRouteHintHop {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The node_id of the non-target end of the route
#[no_mangle]
pub extern "C" fn RouteHintHop_get_src_node_id(this_ptr: &RouteHintHop) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().src_node_id;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The node_id of the non-target end of the route
#[no_mangle]
pub extern "C" fn RouteHintHop_set_src_node_id(this_ptr: &mut RouteHintHop, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.src_node_id = val.into_rust();
}
/// The short_channel_id of this channel
#[no_mangle]
pub extern "C" fn RouteHintHop_get_short_channel_id(this_ptr: &RouteHintHop) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().short_channel_id;
	*inner_val
}
/// The short_channel_id of this channel
#[no_mangle]
pub extern "C" fn RouteHintHop_set_short_channel_id(this_ptr: &mut RouteHintHop, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.short_channel_id = val;
}
/// The fees which must be paid to use this channel
#[no_mangle]
pub extern "C" fn RouteHintHop_get_fees(this_ptr: &RouteHintHop) -> crate::lightning::routing::gossip::RoutingFees {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().fees;
	crate::lightning::routing::gossip::RoutingFees { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::routing::gossip::RoutingFees<>) as *mut _) }, is_owned: false }
}
/// The fees which must be paid to use this channel
#[no_mangle]
pub extern "C" fn RouteHintHop_set_fees(this_ptr: &mut RouteHintHop, mut val: crate::lightning::routing::gossip::RoutingFees) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.fees = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The difference in CLTV values between this node and the next node.
#[no_mangle]
pub extern "C" fn RouteHintHop_get_cltv_expiry_delta(this_ptr: &RouteHintHop) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().cltv_expiry_delta;
	*inner_val
}
/// The difference in CLTV values between this node and the next node.
#[no_mangle]
pub extern "C" fn RouteHintHop_set_cltv_expiry_delta(this_ptr: &mut RouteHintHop, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.cltv_expiry_delta = val;
}
/// The minimum value, in msat, which must be relayed to the next hop.
#[no_mangle]
pub extern "C" fn RouteHintHop_get_htlc_minimum_msat(this_ptr: &RouteHintHop) -> crate::c_types::derived::COption_u64Z {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_minimum_msat;
	let mut local_inner_val = if inner_val.is_none() { crate::c_types::derived::COption_u64Z::None } else { crate::c_types::derived::COption_u64Z::Some( { inner_val.unwrap() }) };
	local_inner_val
}
/// The minimum value, in msat, which must be relayed to the next hop.
#[no_mangle]
pub extern "C" fn RouteHintHop_set_htlc_minimum_msat(this_ptr: &mut RouteHintHop, mut val: crate::c_types::derived::COption_u64Z) {
	let mut local_val = if val.is_some() { Some( { val.take() }) } else { None };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_minimum_msat = local_val;
}
/// The maximum value in msat available for routing with a single HTLC.
#[no_mangle]
pub extern "C" fn RouteHintHop_get_htlc_maximum_msat(this_ptr: &RouteHintHop) -> crate::c_types::derived::COption_u64Z {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_maximum_msat;
	let mut local_inner_val = if inner_val.is_none() { crate::c_types::derived::COption_u64Z::None } else { crate::c_types::derived::COption_u64Z::Some( { inner_val.unwrap() }) };
	local_inner_val
}
/// The maximum value in msat available for routing with a single HTLC.
#[no_mangle]
pub extern "C" fn RouteHintHop_set_htlc_maximum_msat(this_ptr: &mut RouteHintHop, mut val: crate::c_types::derived::COption_u64Z) {
	let mut local_val = if val.is_some() { Some( { val.take() }) } else { None };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_maximum_msat = local_val;
}
/// Constructs a new RouteHintHop given each field
#[must_use]
#[no_mangle]
pub extern "C" fn RouteHintHop_new(mut src_node_id_arg: crate::c_types::PublicKey, mut short_channel_id_arg: u64, mut fees_arg: crate::lightning::routing::gossip::RoutingFees, mut cltv_expiry_delta_arg: u16, mut htlc_minimum_msat_arg: crate::c_types::derived::COption_u64Z, mut htlc_maximum_msat_arg: crate::c_types::derived::COption_u64Z) -> RouteHintHop {
	let mut local_htlc_minimum_msat_arg = if htlc_minimum_msat_arg.is_some() { Some( { htlc_minimum_msat_arg.take() }) } else { None };
	let mut local_htlc_maximum_msat_arg = if htlc_maximum_msat_arg.is_some() { Some( { htlc_maximum_msat_arg.take() }) } else { None };
	RouteHintHop { inner: ObjOps::heap_alloc(nativeRouteHintHop {
		src_node_id: src_node_id_arg.into_rust(),
		short_channel_id: short_channel_id_arg,
		fees: *unsafe { Box::from_raw(fees_arg.take_inner()) },
		cltv_expiry_delta: cltv_expiry_delta_arg,
		htlc_minimum_msat: local_htlc_minimum_msat_arg,
		htlc_maximum_msat: local_htlc_maximum_msat_arg,
	}), is_owned: true }
}
impl Clone for RouteHintHop {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeRouteHintHop>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RouteHintHop_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeRouteHintHop)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the RouteHintHop
pub extern "C" fn RouteHintHop_clone(orig: &RouteHintHop) -> RouteHintHop {
	orig.clone()
}
/// Checks if two RouteHintHops contain equal inner contents.
#[no_mangle]
pub extern "C" fn RouteHintHop_hash(o: &RouteHintHop) -> u64 {
	if o.inner.is_null() { return 0; }
	// Note that we'd love to use alloc::collections::hash_map::DefaultHasher but it's not in core
	#[allow(deprecated)]
	let mut hasher = core::hash::SipHasher::new();
	core::hash::Hash::hash(o.get_native_ref(), &mut hasher);
	core::hash::Hasher::finish(&hasher)
}
/// Checks if two RouteHintHops contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn RouteHintHop_eq(a: &RouteHintHop, b: &RouteHintHop) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}
#[no_mangle]
/// Serialize the RouteHintHop object into a byte array which can be read by RouteHintHop_read
pub extern "C" fn RouteHintHop_write(obj: &crate::lightning::routing::router::RouteHintHop) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn RouteHintHop_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeRouteHintHop) })
}
#[no_mangle]
/// Read a RouteHintHop from a byte array, created by RouteHintHop_write
pub extern "C" fn RouteHintHop_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_RouteHintHopDecodeErrorZ {
	let res: Result<lightning::routing::router::RouteHintHop, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::RouteHintHop { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
/// Finds a route from us (payer) to the given target node (payee).
///
/// If the payee provided features in their invoice, they should be provided via `params.payee`.
/// Without this, MPP will only be used if the payee's features are available in the network graph.
///
/// Private routing paths between a public node and the target may be included in `params.payee`.
///
/// If some channels aren't announced, it may be useful to fill in `first_hops` with the results
/// from [`ChannelManager::list_usable_channels`]. If it is filled in, the view of these channels
/// from `network_graph` will be ignored, and only those in `first_hops` will be used.
///
/// The fees on channels from us to the next hop are ignored as they are assumed to all be equal.
/// However, the enabled/disabled bit on such channels as well as the `htlc_minimum_msat` /
/// `htlc_maximum_msat` *are* checked as they may change based on the receiving node.
///
/// # Note
///
/// May be used to re-compute a [`Route`] when handling a [`Event::PaymentPathFailed`]. Any
/// adjustments to the [`NetworkGraph`] and channel scores should be made prior to calling this
/// function.
///
/// # Panics
///
/// Panics if first_hops contains channels without short_channel_ids;
/// [`ChannelManager::list_usable_channels`] will never include such channels.
///
/// [`ChannelManager::list_usable_channels`]: crate::ln::channelmanager::ChannelManager::list_usable_channels
/// [`Event::PaymentPathFailed`]: crate::util::events::Event::PaymentPathFailed
/// [`NetworkGraph`]: crate::routing::gossip::NetworkGraph
///
/// Note that first_hops (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn find_route(mut our_node_pubkey: crate::c_types::PublicKey, route_params: &crate::lightning::routing::router::RouteParameters, network_graph: &crate::lightning::routing::gossip::NetworkGraph, first_hops: *mut crate::c_types::derived::CVec_ChannelDetailsZ, mut logger: crate::lightning::util::logger::Logger, scorer: &crate::lightning::routing::scoring::Score, random_seed_bytes: *const [u8; 32]) -> crate::c_types::derived::CResult_RouteLightningErrorZ {
	let mut local_first_hops_base = if first_hops == core::ptr::null_mut() { None } else { Some( { let mut local_first_hops_0 = Vec::new(); for mut item in unsafe { &mut *first_hops }.as_slice().iter() { local_first_hops_0.push( { item.get_native_ref() }); }; local_first_hops_0 }) }; let mut local_first_hops = local_first_hops_base.as_ref().map(|a| &a[..]);
	let mut ret = lightning::routing::router::find_route::<crate::lightning::util::logger::Logger, crate::lightning::util::logger::Logger, crate::lightning::routing::scoring::Score>(&our_node_pubkey.into_rust(), route_params.get_native_ref(), network_graph.get_native_ref(), local_first_hops, logger, scorer, unsafe { &*random_seed_bytes});
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::Route { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::LightningError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_ret
}

/// Construct a route from us (payer) to the target node (payee) via the given hops (which should
/// exclude the payer, but include the payee). This may be useful, e.g., for probing the chosen path.
///
/// Re-uses logic from `find_route`, so the restrictions described there also apply here.
#[no_mangle]
pub extern "C" fn build_route_from_hops(mut our_node_pubkey: crate::c_types::PublicKey, mut hops: crate::c_types::derived::CVec_PublicKeyZ, route_params: &crate::lightning::routing::router::RouteParameters, network_graph: &crate::lightning::routing::gossip::NetworkGraph, mut logger: crate::lightning::util::logger::Logger, random_seed_bytes: *const [u8; 32]) -> crate::c_types::derived::CResult_RouteLightningErrorZ {
	let mut local_hops = Vec::new(); for mut item in hops.into_rust().drain(..) { local_hops.push( { item.into_rust() }); };
	let mut ret = lightning::routing::router::build_route_from_hops::<crate::lightning::util::logger::Logger, crate::lightning::util::logger::Logger>(&our_node_pubkey.into_rust(), &local_hops[..], route_params.get_native_ref(), network_graph.get_native_ref(), logger, unsafe { &*random_seed_bytes});
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::routing::router::Route { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::LightningError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_ret
}

