// This file is Copyright its original authors, visible in version control
// history and in the source files from which this was generated.
//
// This file is licensed under the license available in the LICENSE or LICENSE.md
// file in the root of this repository or, if no such file exists, the same
// license as that which applies to the original source files from which this
// source was automatically generated.

//! Wire messages, traits representing wire message handlers, and a few error types live here.
//!
//! For a normal node you probably don't need to use anything here, however, if you wish to split a
//! node into an internet-facing route/message socket handling daemon and a separate daemon (or
//! server entirely) which handles only channel-related messages you may wish to implement
//! [`ChannelMessageHandler`] yourself and use it to re-serialize messages and pass them across
//! daemons/servers.
//!
//! Note that if you go with such an architecture (instead of passing raw socket events to a
//! non-internet-facing system) you trust the frontend internet-facing system to not lie about the
//! source `node_id` of the message, however this does allow you to significantly reduce bandwidth
//! between the systems as routing messages can represent a significant chunk of bandwidth usage
//! (especially for non-channel-publicly-announcing nodes). As an alternate design which avoids
//! this issue, if you have sufficient bidirectional bandwidth between your systems, you may send
//! raw socket events into your non-internet-facing system and then send routing events back to
//! track the network on the less-secure system.

use alloc::str::FromStr;
use core::ffi::c_void;
use core::convert::Infallible;
use bitcoin::hashes::Hash;
use crate::c_types::*;
#[cfg(feature="no-std")]
use alloc::{vec::Vec, boxed::Box};

/// An error in decoding a message or struct.
#[derive(Clone)]
#[must_use]
#[repr(C)]
pub enum DecodeError {
	/// A version byte specified something we don't know how to handle.
	///
	/// Includes unknown realm byte in an onion hop data packet.
	UnknownVersion,
	/// Unknown feature mandating we fail to parse message (e.g., TLV with an even, unknown type)
	UnknownRequiredFeature,
	/// Value was invalid.
	///
	/// For example, a byte which was supposed to be a bool was something other than a 0
	/// or 1, a public key/private key/signature was invalid, text wasn't UTF-8, TLV was
	/// syntactically incorrect, etc.
	InvalidValue,
	/// The buffer to be read was too short.
	ShortRead,
	/// A length descriptor in the packet didn't describe the later data correctly.
	BadLengthDescriptor,
	/// Error from [`std::io`].
	Io(
		crate::c_types::IOError),
	/// The message included zlib-compressed values, which we don't support.
	UnsupportedCompression,
}
use lightning::ln::msgs::DecodeError as DecodeErrorImport;
pub(crate) type nativeDecodeError = DecodeErrorImport;

impl DecodeError {
	#[allow(unused)]
	pub(crate) fn to_native(&self) -> nativeDecodeError {
		match self {
			DecodeError::UnknownVersion => nativeDecodeError::UnknownVersion,
			DecodeError::UnknownRequiredFeature => nativeDecodeError::UnknownRequiredFeature,
			DecodeError::InvalidValue => nativeDecodeError::InvalidValue,
			DecodeError::ShortRead => nativeDecodeError::ShortRead,
			DecodeError::BadLengthDescriptor => nativeDecodeError::BadLengthDescriptor,
			DecodeError::Io (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				nativeDecodeError::Io (
					a_nonref.to_rust_kind(),
				)
			},
			DecodeError::UnsupportedCompression => nativeDecodeError::UnsupportedCompression,
		}
	}
	#[allow(unused)]
	pub(crate) fn into_native(self) -> nativeDecodeError {
		match self {
			DecodeError::UnknownVersion => nativeDecodeError::UnknownVersion,
			DecodeError::UnknownRequiredFeature => nativeDecodeError::UnknownRequiredFeature,
			DecodeError::InvalidValue => nativeDecodeError::InvalidValue,
			DecodeError::ShortRead => nativeDecodeError::ShortRead,
			DecodeError::BadLengthDescriptor => nativeDecodeError::BadLengthDescriptor,
			DecodeError::Io (mut a, ) => {
				nativeDecodeError::Io (
					a.to_rust_kind(),
				)
			},
			DecodeError::UnsupportedCompression => nativeDecodeError::UnsupportedCompression,
		}
	}
	#[allow(unused)]
	pub(crate) fn from_native(native: &nativeDecodeError) -> Self {
		match native {
			nativeDecodeError::UnknownVersion => DecodeError::UnknownVersion,
			nativeDecodeError::UnknownRequiredFeature => DecodeError::UnknownRequiredFeature,
			nativeDecodeError::InvalidValue => DecodeError::InvalidValue,
			nativeDecodeError::ShortRead => DecodeError::ShortRead,
			nativeDecodeError::BadLengthDescriptor => DecodeError::BadLengthDescriptor,
			nativeDecodeError::Io (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				DecodeError::Io (
					crate::c_types::IOError::from_rust_kind(a_nonref),
				)
			},
			nativeDecodeError::UnsupportedCompression => DecodeError::UnsupportedCompression,
		}
	}
	#[allow(unused)]
	pub(crate) fn native_into(native: nativeDecodeError) -> Self {
		match native {
			nativeDecodeError::UnknownVersion => DecodeError::UnknownVersion,
			nativeDecodeError::UnknownRequiredFeature => DecodeError::UnknownRequiredFeature,
			nativeDecodeError::InvalidValue => DecodeError::InvalidValue,
			nativeDecodeError::ShortRead => DecodeError::ShortRead,
			nativeDecodeError::BadLengthDescriptor => DecodeError::BadLengthDescriptor,
			nativeDecodeError::Io (mut a, ) => {
				DecodeError::Io (
					crate::c_types::IOError::from_rust_kind(a),
				)
			},
			nativeDecodeError::UnsupportedCompression => DecodeError::UnsupportedCompression,
		}
	}
}
/// Frees any resources used by the DecodeError
#[no_mangle]
pub extern "C" fn DecodeError_free(this_ptr: DecodeError) { }
/// Creates a copy of the DecodeError
#[no_mangle]
pub extern "C" fn DecodeError_clone(orig: &DecodeError) -> DecodeError {
	orig.clone()
}
#[no_mangle]
/// Utility method to constructs a new UnknownVersion-variant DecodeError
pub extern "C" fn DecodeError_unknown_version() -> DecodeError {
	DecodeError::UnknownVersion}
#[no_mangle]
/// Utility method to constructs a new UnknownRequiredFeature-variant DecodeError
pub extern "C" fn DecodeError_unknown_required_feature() -> DecodeError {
	DecodeError::UnknownRequiredFeature}
#[no_mangle]
/// Utility method to constructs a new InvalidValue-variant DecodeError
pub extern "C" fn DecodeError_invalid_value() -> DecodeError {
	DecodeError::InvalidValue}
#[no_mangle]
/// Utility method to constructs a new ShortRead-variant DecodeError
pub extern "C" fn DecodeError_short_read() -> DecodeError {
	DecodeError::ShortRead}
#[no_mangle]
/// Utility method to constructs a new BadLengthDescriptor-variant DecodeError
pub extern "C" fn DecodeError_bad_length_descriptor() -> DecodeError {
	DecodeError::BadLengthDescriptor}
#[no_mangle]
/// Utility method to constructs a new Io-variant DecodeError
pub extern "C" fn DecodeError_io(a: crate::c_types::IOError) -> DecodeError {
	DecodeError::Io(a, )
}
#[no_mangle]
/// Utility method to constructs a new UnsupportedCompression-variant DecodeError
pub extern "C" fn DecodeError_unsupported_compression() -> DecodeError {
	DecodeError::UnsupportedCompression}
/// Checks if two DecodeErrors contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
#[no_mangle]
pub extern "C" fn DecodeError_eq(a: &DecodeError, b: &DecodeError) -> bool {
	if &a.to_native() == &b.to_native() { true } else { false }
}

use lightning::ln::msgs::Init as nativeInitImport;
pub(crate) type nativeInit = nativeInitImport;

/// An [`init`] message to be sent to or received from a peer.
///
/// [`init`]: https://github.com/lightning/bolts/blob/master/01-messaging.md#the-init-message
#[must_use]
#[repr(C)]
pub struct Init {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeInit,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for Init {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeInit>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the Init, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn Init_free(this_obj: Init) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Init_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeInit) };
}
#[allow(unused)]
impl Init {
	pub(crate) fn get_native_ref(&self) -> &'static nativeInit {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeInit {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeInit {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The relevant features which the sender supports.
#[no_mangle]
pub extern "C" fn Init_get_features(this_ptr: &Init) -> crate::lightning::ln::features::InitFeatures {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().features;
	crate::lightning::ln::features::InitFeatures { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::features::InitFeatures<>) as *mut _) }, is_owned: false }
}
/// The relevant features which the sender supports.
#[no_mangle]
pub extern "C" fn Init_set_features(this_ptr: &mut Init, mut val: crate::lightning::ln::features::InitFeatures) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.features = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The receipient's network address.
///
/// This adds the option to report a remote IP address back to a connecting peer using the init
/// message. A node can decide to use that information to discover a potential update to its
/// public IPv4 address (NAT) and use that for a [`NodeAnnouncement`] update message containing
/// the new address.
#[no_mangle]
pub extern "C" fn Init_get_remote_network_address(this_ptr: &Init) -> crate::c_types::derived::COption_NetAddressZ {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().remote_network_address;
	let mut local_inner_val = if inner_val.is_none() { crate::c_types::derived::COption_NetAddressZ::None } else { crate::c_types::derived::COption_NetAddressZ::Some(/* WARNING: CLONING CONVERSION HERE! &Option<Enum> is otherwise un-expressable. */ { crate::lightning::ln::msgs::NetAddress::native_into((*inner_val.as_ref().unwrap()).clone()) }) };
	local_inner_val
}
/// The receipient's network address.
///
/// This adds the option to report a remote IP address back to a connecting peer using the init
/// message. A node can decide to use that information to discover a potential update to its
/// public IPv4 address (NAT) and use that for a [`NodeAnnouncement`] update message containing
/// the new address.
#[no_mangle]
pub extern "C" fn Init_set_remote_network_address(this_ptr: &mut Init, mut val: crate::c_types::derived::COption_NetAddressZ) {
	let mut local_val = { /*val*/ let val_opt = val; if val_opt.is_none() { None } else { Some({ { { val_opt.take() }.into_native() }})} };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.remote_network_address = local_val;
}
/// Constructs a new Init given each field
#[must_use]
#[no_mangle]
pub extern "C" fn Init_new(mut features_arg: crate::lightning::ln::features::InitFeatures, mut remote_network_address_arg: crate::c_types::derived::COption_NetAddressZ) -> Init {
	let mut local_remote_network_address_arg = { /*remote_network_address_arg*/ let remote_network_address_arg_opt = remote_network_address_arg; if remote_network_address_arg_opt.is_none() { None } else { Some({ { { remote_network_address_arg_opt.take() }.into_native() }})} };
	Init { inner: ObjOps::heap_alloc(nativeInit {
		features: *unsafe { Box::from_raw(features_arg.take_inner()) },
		remote_network_address: local_remote_network_address_arg,
	}), is_owned: true }
}
impl Clone for Init {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeInit>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Init_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeInit)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the Init
pub extern "C" fn Init_clone(orig: &Init) -> Init {
	orig.clone()
}
/// Checks if two Inits contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn Init_eq(a: &Init, b: &Init) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ErrorMessage as nativeErrorMessageImport;
pub(crate) type nativeErrorMessage = nativeErrorMessageImport;

/// An [`error`] message to be sent to or received from a peer.
///
/// [`error`]: https://github.com/lightning/bolts/blob/master/01-messaging.md#the-error-and-warning-messages
#[must_use]
#[repr(C)]
pub struct ErrorMessage {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeErrorMessage,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ErrorMessage {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeErrorMessage>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ErrorMessage, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ErrorMessage_free(this_obj: ErrorMessage) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ErrorMessage_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeErrorMessage) };
}
#[allow(unused)]
impl ErrorMessage {
	pub(crate) fn get_native_ref(&self) -> &'static nativeErrorMessage {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeErrorMessage {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeErrorMessage {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID involved in the error.
///
/// All-0s indicates a general error unrelated to a specific channel, after which all channels
/// with the sending peer should be closed.
#[no_mangle]
pub extern "C" fn ErrorMessage_get_channel_id(this_ptr: &ErrorMessage) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID involved in the error.
///
/// All-0s indicates a general error unrelated to a specific channel, after which all channels
/// with the sending peer should be closed.
#[no_mangle]
pub extern "C" fn ErrorMessage_set_channel_id(this_ptr: &mut ErrorMessage, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// A possibly human-readable error description.
///
/// The string should be sanitized before it is used (e.g., emitted to logs or printed to
/// `stdout`). Otherwise, a well crafted error message may trigger a security vulnerability in
/// the terminal emulator or the logging subsystem.
#[no_mangle]
pub extern "C" fn ErrorMessage_get_data(this_ptr: &ErrorMessage) -> crate::c_types::Str {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().data;
	inner_val.as_str().into()
}
/// A possibly human-readable error description.
///
/// The string should be sanitized before it is used (e.g., emitted to logs or printed to
/// `stdout`). Otherwise, a well crafted error message may trigger a security vulnerability in
/// the terminal emulator or the logging subsystem.
#[no_mangle]
pub extern "C" fn ErrorMessage_set_data(this_ptr: &mut ErrorMessage, mut val: crate::c_types::Str) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.data = val.into_string();
}
/// Constructs a new ErrorMessage given each field
#[must_use]
#[no_mangle]
pub extern "C" fn ErrorMessage_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut data_arg: crate::c_types::Str) -> ErrorMessage {
	ErrorMessage { inner: ObjOps::heap_alloc(nativeErrorMessage {
		channel_id: channel_id_arg.data,
		data: data_arg.into_string(),
	}), is_owned: true }
}
impl Clone for ErrorMessage {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeErrorMessage>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ErrorMessage_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeErrorMessage)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ErrorMessage
pub extern "C" fn ErrorMessage_clone(orig: &ErrorMessage) -> ErrorMessage {
	orig.clone()
}
/// Checks if two ErrorMessages contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ErrorMessage_eq(a: &ErrorMessage, b: &ErrorMessage) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::WarningMessage as nativeWarningMessageImport;
pub(crate) type nativeWarningMessage = nativeWarningMessageImport;

/// A [`warning`] message to be sent to or received from a peer.
///
/// [`warning`]: https://github.com/lightning/bolts/blob/master/01-messaging.md#the-error-and-warning-messages
#[must_use]
#[repr(C)]
pub struct WarningMessage {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeWarningMessage,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for WarningMessage {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeWarningMessage>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the WarningMessage, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn WarningMessage_free(this_obj: WarningMessage) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn WarningMessage_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeWarningMessage) };
}
#[allow(unused)]
impl WarningMessage {
	pub(crate) fn get_native_ref(&self) -> &'static nativeWarningMessage {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeWarningMessage {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeWarningMessage {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID involved in the warning.
///
/// All-0s indicates a warning unrelated to a specific channel.
#[no_mangle]
pub extern "C" fn WarningMessage_get_channel_id(this_ptr: &WarningMessage) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID involved in the warning.
///
/// All-0s indicates a warning unrelated to a specific channel.
#[no_mangle]
pub extern "C" fn WarningMessage_set_channel_id(this_ptr: &mut WarningMessage, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// A possibly human-readable warning description.
///
/// The string should be sanitized before it is used (e.g. emitted to logs or printed to
/// stdout). Otherwise, a well crafted error message may trigger a security vulnerability in
/// the terminal emulator or the logging subsystem.
#[no_mangle]
pub extern "C" fn WarningMessage_get_data(this_ptr: &WarningMessage) -> crate::c_types::Str {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().data;
	inner_val.as_str().into()
}
/// A possibly human-readable warning description.
///
/// The string should be sanitized before it is used (e.g. emitted to logs or printed to
/// stdout). Otherwise, a well crafted error message may trigger a security vulnerability in
/// the terminal emulator or the logging subsystem.
#[no_mangle]
pub extern "C" fn WarningMessage_set_data(this_ptr: &mut WarningMessage, mut val: crate::c_types::Str) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.data = val.into_string();
}
/// Constructs a new WarningMessage given each field
#[must_use]
#[no_mangle]
pub extern "C" fn WarningMessage_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut data_arg: crate::c_types::Str) -> WarningMessage {
	WarningMessage { inner: ObjOps::heap_alloc(nativeWarningMessage {
		channel_id: channel_id_arg.data,
		data: data_arg.into_string(),
	}), is_owned: true }
}
impl Clone for WarningMessage {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeWarningMessage>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn WarningMessage_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeWarningMessage)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the WarningMessage
pub extern "C" fn WarningMessage_clone(orig: &WarningMessage) -> WarningMessage {
	orig.clone()
}
/// Checks if two WarningMessages contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn WarningMessage_eq(a: &WarningMessage, b: &WarningMessage) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::Ping as nativePingImport;
pub(crate) type nativePing = nativePingImport;

/// A [`ping`] message to be sent to or received from a peer.
///
/// [`ping`]: https://github.com/lightning/bolts/blob/master/01-messaging.md#the-ping-and-pong-messages
#[must_use]
#[repr(C)]
pub struct Ping {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativePing,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for Ping {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativePing>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the Ping, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn Ping_free(this_obj: Ping) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Ping_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativePing) };
}
#[allow(unused)]
impl Ping {
	pub(crate) fn get_native_ref(&self) -> &'static nativePing {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativePing {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativePing {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The desired response length.
#[no_mangle]
pub extern "C" fn Ping_get_ponglen(this_ptr: &Ping) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().ponglen;
	*inner_val
}
/// The desired response length.
#[no_mangle]
pub extern "C" fn Ping_set_ponglen(this_ptr: &mut Ping, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.ponglen = val;
}
/// The ping packet size.
///
/// This field is not sent on the wire. byteslen zeros are sent.
#[no_mangle]
pub extern "C" fn Ping_get_byteslen(this_ptr: &Ping) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().byteslen;
	*inner_val
}
/// The ping packet size.
///
/// This field is not sent on the wire. byteslen zeros are sent.
#[no_mangle]
pub extern "C" fn Ping_set_byteslen(this_ptr: &mut Ping, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.byteslen = val;
}
/// Constructs a new Ping given each field
#[must_use]
#[no_mangle]
pub extern "C" fn Ping_new(mut ponglen_arg: u16, mut byteslen_arg: u16) -> Ping {
	Ping { inner: ObjOps::heap_alloc(nativePing {
		ponglen: ponglen_arg,
		byteslen: byteslen_arg,
	}), is_owned: true }
}
impl Clone for Ping {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativePing>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Ping_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativePing)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the Ping
pub extern "C" fn Ping_clone(orig: &Ping) -> Ping {
	orig.clone()
}
/// Checks if two Pings contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn Ping_eq(a: &Ping, b: &Ping) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::Pong as nativePongImport;
pub(crate) type nativePong = nativePongImport;

/// A [`pong`] message to be sent to or received from a peer.
///
/// [`pong`]: https://github.com/lightning/bolts/blob/master/01-messaging.md#the-ping-and-pong-messages
#[must_use]
#[repr(C)]
pub struct Pong {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativePong,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for Pong {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativePong>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the Pong, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn Pong_free(this_obj: Pong) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Pong_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativePong) };
}
#[allow(unused)]
impl Pong {
	pub(crate) fn get_native_ref(&self) -> &'static nativePong {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativePong {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativePong {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The pong packet size.
///
/// This field is not sent on the wire. byteslen zeros are sent.
#[no_mangle]
pub extern "C" fn Pong_get_byteslen(this_ptr: &Pong) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().byteslen;
	*inner_val
}
/// The pong packet size.
///
/// This field is not sent on the wire. byteslen zeros are sent.
#[no_mangle]
pub extern "C" fn Pong_set_byteslen(this_ptr: &mut Pong, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.byteslen = val;
}
/// Constructs a new Pong given each field
#[must_use]
#[no_mangle]
pub extern "C" fn Pong_new(mut byteslen_arg: u16) -> Pong {
	Pong { inner: ObjOps::heap_alloc(nativePong {
		byteslen: byteslen_arg,
	}), is_owned: true }
}
impl Clone for Pong {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativePong>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Pong_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativePong)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the Pong
pub extern "C" fn Pong_clone(orig: &Pong) -> Pong {
	orig.clone()
}
/// Checks if two Pongs contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn Pong_eq(a: &Pong, b: &Pong) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::OpenChannel as nativeOpenChannelImport;
pub(crate) type nativeOpenChannel = nativeOpenChannelImport;

/// An [`open_channel`] message to be sent to or received from a peer.
///
/// [`open_channel`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#the-open_channel-message
#[must_use]
#[repr(C)]
pub struct OpenChannel {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeOpenChannel,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for OpenChannel {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeOpenChannel>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the OpenChannel, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn OpenChannel_free(this_obj: OpenChannel) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn OpenChannel_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeOpenChannel) };
}
#[allow(unused)]
impl OpenChannel {
	pub(crate) fn get_native_ref(&self) -> &'static nativeOpenChannel {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeOpenChannel {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeOpenChannel {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The genesis hash of the blockchain where the channel is to be opened
#[no_mangle]
pub extern "C" fn OpenChannel_get_chain_hash(this_ptr: &OpenChannel) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().chain_hash;
	inner_val.as_inner()
}
/// The genesis hash of the blockchain where the channel is to be opened
#[no_mangle]
pub extern "C" fn OpenChannel_set_chain_hash(this_ptr: &mut OpenChannel, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.chain_hash = ::bitcoin::hash_types::BlockHash::from_slice(&val.data[..]).unwrap();
}
/// A temporary channel ID, until the funding outpoint is announced
#[no_mangle]
pub extern "C" fn OpenChannel_get_temporary_channel_id(this_ptr: &OpenChannel) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().temporary_channel_id;
	inner_val
}
/// A temporary channel ID, until the funding outpoint is announced
#[no_mangle]
pub extern "C" fn OpenChannel_set_temporary_channel_id(this_ptr: &mut OpenChannel, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.temporary_channel_id = val.data;
}
/// The channel value
#[no_mangle]
pub extern "C" fn OpenChannel_get_funding_satoshis(this_ptr: &OpenChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().funding_satoshis;
	*inner_val
}
/// The channel value
#[no_mangle]
pub extern "C" fn OpenChannel_set_funding_satoshis(this_ptr: &mut OpenChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.funding_satoshis = val;
}
/// The amount to push to the counterparty as part of the open, in milli-satoshi
#[no_mangle]
pub extern "C" fn OpenChannel_get_push_msat(this_ptr: &OpenChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().push_msat;
	*inner_val
}
/// The amount to push to the counterparty as part of the open, in milli-satoshi
#[no_mangle]
pub extern "C" fn OpenChannel_set_push_msat(this_ptr: &mut OpenChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.push_msat = val;
}
/// The threshold below which outputs on transactions broadcast by sender will be omitted
#[no_mangle]
pub extern "C" fn OpenChannel_get_dust_limit_satoshis(this_ptr: &OpenChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().dust_limit_satoshis;
	*inner_val
}
/// The threshold below which outputs on transactions broadcast by sender will be omitted
#[no_mangle]
pub extern "C" fn OpenChannel_set_dust_limit_satoshis(this_ptr: &mut OpenChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.dust_limit_satoshis = val;
}
/// The maximum inbound HTLC value in flight towards sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn OpenChannel_get_max_htlc_value_in_flight_msat(this_ptr: &OpenChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().max_htlc_value_in_flight_msat;
	*inner_val
}
/// The maximum inbound HTLC value in flight towards sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn OpenChannel_set_max_htlc_value_in_flight_msat(this_ptr: &mut OpenChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.max_htlc_value_in_flight_msat = val;
}
/// The minimum value unencumbered by HTLCs for the counterparty to keep in the channel
#[no_mangle]
pub extern "C" fn OpenChannel_get_channel_reserve_satoshis(this_ptr: &OpenChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_reserve_satoshis;
	*inner_val
}
/// The minimum value unencumbered by HTLCs for the counterparty to keep in the channel
#[no_mangle]
pub extern "C" fn OpenChannel_set_channel_reserve_satoshis(this_ptr: &mut OpenChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_reserve_satoshis = val;
}
/// The minimum HTLC size incoming to sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn OpenChannel_get_htlc_minimum_msat(this_ptr: &OpenChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_minimum_msat;
	*inner_val
}
/// The minimum HTLC size incoming to sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn OpenChannel_set_htlc_minimum_msat(this_ptr: &mut OpenChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_minimum_msat = val;
}
/// The feerate per 1000-weight of sender generated transactions, until updated by
/// [`UpdateFee`]
#[no_mangle]
pub extern "C" fn OpenChannel_get_feerate_per_kw(this_ptr: &OpenChannel) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().feerate_per_kw;
	*inner_val
}
/// The feerate per 1000-weight of sender generated transactions, until updated by
/// [`UpdateFee`]
#[no_mangle]
pub extern "C" fn OpenChannel_set_feerate_per_kw(this_ptr: &mut OpenChannel, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.feerate_per_kw = val;
}
/// The number of blocks which the counterparty will have to wait to claim on-chain funds if
/// they broadcast a commitment transaction
#[no_mangle]
pub extern "C" fn OpenChannel_get_to_self_delay(this_ptr: &OpenChannel) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().to_self_delay;
	*inner_val
}
/// The number of blocks which the counterparty will have to wait to claim on-chain funds if
/// they broadcast a commitment transaction
#[no_mangle]
pub extern "C" fn OpenChannel_set_to_self_delay(this_ptr: &mut OpenChannel, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.to_self_delay = val;
}
/// The maximum number of inbound HTLCs towards sender
#[no_mangle]
pub extern "C" fn OpenChannel_get_max_accepted_htlcs(this_ptr: &OpenChannel) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().max_accepted_htlcs;
	*inner_val
}
/// The maximum number of inbound HTLCs towards sender
#[no_mangle]
pub extern "C" fn OpenChannel_set_max_accepted_htlcs(this_ptr: &mut OpenChannel, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.max_accepted_htlcs = val;
}
/// The sender's key controlling the funding transaction
#[no_mangle]
pub extern "C" fn OpenChannel_get_funding_pubkey(this_ptr: &OpenChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().funding_pubkey;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The sender's key controlling the funding transaction
#[no_mangle]
pub extern "C" fn OpenChannel_set_funding_pubkey(this_ptr: &mut OpenChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.funding_pubkey = val.into_rust();
}
/// Used to derive a revocation key for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn OpenChannel_get_revocation_basepoint(this_ptr: &OpenChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().revocation_basepoint;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// Used to derive a revocation key for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn OpenChannel_set_revocation_basepoint(this_ptr: &mut OpenChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.revocation_basepoint = val.into_rust();
}
/// A payment key to sender for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn OpenChannel_get_payment_point(this_ptr: &OpenChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().payment_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// A payment key to sender for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn OpenChannel_set_payment_point(this_ptr: &mut OpenChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.payment_point = val.into_rust();
}
/// Used to derive a payment key to sender for transactions broadcast by sender
#[no_mangle]
pub extern "C" fn OpenChannel_get_delayed_payment_basepoint(this_ptr: &OpenChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().delayed_payment_basepoint;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// Used to derive a payment key to sender for transactions broadcast by sender
#[no_mangle]
pub extern "C" fn OpenChannel_set_delayed_payment_basepoint(this_ptr: &mut OpenChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.delayed_payment_basepoint = val.into_rust();
}
/// Used to derive an HTLC payment key to sender
#[no_mangle]
pub extern "C" fn OpenChannel_get_htlc_basepoint(this_ptr: &OpenChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_basepoint;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// Used to derive an HTLC payment key to sender
#[no_mangle]
pub extern "C" fn OpenChannel_set_htlc_basepoint(this_ptr: &mut OpenChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_basepoint = val.into_rust();
}
/// The first to-be-broadcast-by-sender transaction's per commitment point
#[no_mangle]
pub extern "C" fn OpenChannel_get_first_per_commitment_point(this_ptr: &OpenChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().first_per_commitment_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The first to-be-broadcast-by-sender transaction's per commitment point
#[no_mangle]
pub extern "C" fn OpenChannel_set_first_per_commitment_point(this_ptr: &mut OpenChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.first_per_commitment_point = val.into_rust();
}
/// The channel flags to be used
#[no_mangle]
pub extern "C" fn OpenChannel_get_channel_flags(this_ptr: &OpenChannel) -> u8 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_flags;
	*inner_val
}
/// The channel flags to be used
#[no_mangle]
pub extern "C" fn OpenChannel_set_channel_flags(this_ptr: &mut OpenChannel, mut val: u8) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_flags = val;
}
/// The channel type that this channel will represent
///
/// If this is `None`, we derive the channel type from the intersection of our
/// feature bits with our counterparty's feature bits from the [`Init`] message.
///
/// Note that the return value (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn OpenChannel_get_channel_type(this_ptr: &OpenChannel) -> crate::lightning::ln::features::ChannelTypeFeatures {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_type;
	let mut local_inner_val = crate::lightning::ln::features::ChannelTypeFeatures { inner: unsafe { (if inner_val.is_none() { core::ptr::null() } else { ObjOps::nonnull_ptr_to_inner( { (inner_val.as_ref().unwrap()) }) } as *const lightning::ln::features::ChannelTypeFeatures<>) as *mut _ }, is_owned: false };
	local_inner_val
}
/// The channel type that this channel will represent
///
/// If this is `None`, we derive the channel type from the intersection of our
/// feature bits with our counterparty's feature bits from the [`Init`] message.
///
/// Note that val (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn OpenChannel_set_channel_type(this_ptr: &mut OpenChannel, mut val: crate::lightning::ln::features::ChannelTypeFeatures) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.take_inner()) } }) };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_type = local_val;
}
impl Clone for OpenChannel {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeOpenChannel>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn OpenChannel_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeOpenChannel)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the OpenChannel
pub extern "C" fn OpenChannel_clone(orig: &OpenChannel) -> OpenChannel {
	orig.clone()
}
/// Checks if two OpenChannels contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn OpenChannel_eq(a: &OpenChannel, b: &OpenChannel) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::AcceptChannel as nativeAcceptChannelImport;
pub(crate) type nativeAcceptChannel = nativeAcceptChannelImport;

/// An [`accept_channel`] message to be sent to or received from a peer.
///
/// [`accept_channel`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#the-accept_channel-message
#[must_use]
#[repr(C)]
pub struct AcceptChannel {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeAcceptChannel,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for AcceptChannel {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeAcceptChannel>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the AcceptChannel, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn AcceptChannel_free(this_obj: AcceptChannel) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn AcceptChannel_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeAcceptChannel) };
}
#[allow(unused)]
impl AcceptChannel {
	pub(crate) fn get_native_ref(&self) -> &'static nativeAcceptChannel {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeAcceptChannel {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeAcceptChannel {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// A temporary channel ID, until the funding outpoint is announced
#[no_mangle]
pub extern "C" fn AcceptChannel_get_temporary_channel_id(this_ptr: &AcceptChannel) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().temporary_channel_id;
	inner_val
}
/// A temporary channel ID, until the funding outpoint is announced
#[no_mangle]
pub extern "C" fn AcceptChannel_set_temporary_channel_id(this_ptr: &mut AcceptChannel, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.temporary_channel_id = val.data;
}
/// The threshold below which outputs on transactions broadcast by sender will be omitted
#[no_mangle]
pub extern "C" fn AcceptChannel_get_dust_limit_satoshis(this_ptr: &AcceptChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().dust_limit_satoshis;
	*inner_val
}
/// The threshold below which outputs on transactions broadcast by sender will be omitted
#[no_mangle]
pub extern "C" fn AcceptChannel_set_dust_limit_satoshis(this_ptr: &mut AcceptChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.dust_limit_satoshis = val;
}
/// The maximum inbound HTLC value in flight towards sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn AcceptChannel_get_max_htlc_value_in_flight_msat(this_ptr: &AcceptChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().max_htlc_value_in_flight_msat;
	*inner_val
}
/// The maximum inbound HTLC value in flight towards sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn AcceptChannel_set_max_htlc_value_in_flight_msat(this_ptr: &mut AcceptChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.max_htlc_value_in_flight_msat = val;
}
/// The minimum value unencumbered by HTLCs for the counterparty to keep in the channel
#[no_mangle]
pub extern "C" fn AcceptChannel_get_channel_reserve_satoshis(this_ptr: &AcceptChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_reserve_satoshis;
	*inner_val
}
/// The minimum value unencumbered by HTLCs for the counterparty to keep in the channel
#[no_mangle]
pub extern "C" fn AcceptChannel_set_channel_reserve_satoshis(this_ptr: &mut AcceptChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_reserve_satoshis = val;
}
/// The minimum HTLC size incoming to sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn AcceptChannel_get_htlc_minimum_msat(this_ptr: &AcceptChannel) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_minimum_msat;
	*inner_val
}
/// The minimum HTLC size incoming to sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn AcceptChannel_set_htlc_minimum_msat(this_ptr: &mut AcceptChannel, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_minimum_msat = val;
}
/// Minimum depth of the funding transaction before the channel is considered open
#[no_mangle]
pub extern "C" fn AcceptChannel_get_minimum_depth(this_ptr: &AcceptChannel) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().minimum_depth;
	*inner_val
}
/// Minimum depth of the funding transaction before the channel is considered open
#[no_mangle]
pub extern "C" fn AcceptChannel_set_minimum_depth(this_ptr: &mut AcceptChannel, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.minimum_depth = val;
}
/// The number of blocks which the counterparty will have to wait to claim on-chain funds if they broadcast a commitment transaction
#[no_mangle]
pub extern "C" fn AcceptChannel_get_to_self_delay(this_ptr: &AcceptChannel) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().to_self_delay;
	*inner_val
}
/// The number of blocks which the counterparty will have to wait to claim on-chain funds if they broadcast a commitment transaction
#[no_mangle]
pub extern "C" fn AcceptChannel_set_to_self_delay(this_ptr: &mut AcceptChannel, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.to_self_delay = val;
}
/// The maximum number of inbound HTLCs towards sender
#[no_mangle]
pub extern "C" fn AcceptChannel_get_max_accepted_htlcs(this_ptr: &AcceptChannel) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().max_accepted_htlcs;
	*inner_val
}
/// The maximum number of inbound HTLCs towards sender
#[no_mangle]
pub extern "C" fn AcceptChannel_set_max_accepted_htlcs(this_ptr: &mut AcceptChannel, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.max_accepted_htlcs = val;
}
/// The sender's key controlling the funding transaction
#[no_mangle]
pub extern "C" fn AcceptChannel_get_funding_pubkey(this_ptr: &AcceptChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().funding_pubkey;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The sender's key controlling the funding transaction
#[no_mangle]
pub extern "C" fn AcceptChannel_set_funding_pubkey(this_ptr: &mut AcceptChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.funding_pubkey = val.into_rust();
}
/// Used to derive a revocation key for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn AcceptChannel_get_revocation_basepoint(this_ptr: &AcceptChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().revocation_basepoint;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// Used to derive a revocation key for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn AcceptChannel_set_revocation_basepoint(this_ptr: &mut AcceptChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.revocation_basepoint = val.into_rust();
}
/// A payment key to sender for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn AcceptChannel_get_payment_point(this_ptr: &AcceptChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().payment_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// A payment key to sender for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn AcceptChannel_set_payment_point(this_ptr: &mut AcceptChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.payment_point = val.into_rust();
}
/// Used to derive a payment key to sender for transactions broadcast by sender
#[no_mangle]
pub extern "C" fn AcceptChannel_get_delayed_payment_basepoint(this_ptr: &AcceptChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().delayed_payment_basepoint;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// Used to derive a payment key to sender for transactions broadcast by sender
#[no_mangle]
pub extern "C" fn AcceptChannel_set_delayed_payment_basepoint(this_ptr: &mut AcceptChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.delayed_payment_basepoint = val.into_rust();
}
/// Used to derive an HTLC payment key to sender for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn AcceptChannel_get_htlc_basepoint(this_ptr: &AcceptChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_basepoint;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// Used to derive an HTLC payment key to sender for transactions broadcast by counterparty
#[no_mangle]
pub extern "C" fn AcceptChannel_set_htlc_basepoint(this_ptr: &mut AcceptChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_basepoint = val.into_rust();
}
/// The first to-be-broadcast-by-sender transaction's per commitment point
#[no_mangle]
pub extern "C" fn AcceptChannel_get_first_per_commitment_point(this_ptr: &AcceptChannel) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().first_per_commitment_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The first to-be-broadcast-by-sender transaction's per commitment point
#[no_mangle]
pub extern "C" fn AcceptChannel_set_first_per_commitment_point(this_ptr: &mut AcceptChannel, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.first_per_commitment_point = val.into_rust();
}
/// The channel type that this channel will represent.
///
/// If this is `None`, we derive the channel type from the intersection of
/// our feature bits with our counterparty's feature bits from the [`Init`] message.
/// This is required to match the equivalent field in [`OpenChannel::channel_type`].
///
/// Note that the return value (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn AcceptChannel_get_channel_type(this_ptr: &AcceptChannel) -> crate::lightning::ln::features::ChannelTypeFeatures {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_type;
	let mut local_inner_val = crate::lightning::ln::features::ChannelTypeFeatures { inner: unsafe { (if inner_val.is_none() { core::ptr::null() } else { ObjOps::nonnull_ptr_to_inner( { (inner_val.as_ref().unwrap()) }) } as *const lightning::ln::features::ChannelTypeFeatures<>) as *mut _ }, is_owned: false };
	local_inner_val
}
/// The channel type that this channel will represent.
///
/// If this is `None`, we derive the channel type from the intersection of
/// our feature bits with our counterparty's feature bits from the [`Init`] message.
/// This is required to match the equivalent field in [`OpenChannel::channel_type`].
///
/// Note that val (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn AcceptChannel_set_channel_type(this_ptr: &mut AcceptChannel, mut val: crate::lightning::ln::features::ChannelTypeFeatures) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.take_inner()) } }) };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_type = local_val;
}
impl Clone for AcceptChannel {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeAcceptChannel>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn AcceptChannel_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeAcceptChannel)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the AcceptChannel
pub extern "C" fn AcceptChannel_clone(orig: &AcceptChannel) -> AcceptChannel {
	orig.clone()
}
/// Checks if two AcceptChannels contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn AcceptChannel_eq(a: &AcceptChannel, b: &AcceptChannel) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::FundingCreated as nativeFundingCreatedImport;
pub(crate) type nativeFundingCreated = nativeFundingCreatedImport;

/// A [`funding_created`] message to be sent to or received from a peer.
///
/// [`funding_created`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#the-funding_created-message
#[must_use]
#[repr(C)]
pub struct FundingCreated {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeFundingCreated,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for FundingCreated {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeFundingCreated>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the FundingCreated, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn FundingCreated_free(this_obj: FundingCreated) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn FundingCreated_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeFundingCreated) };
}
#[allow(unused)]
impl FundingCreated {
	pub(crate) fn get_native_ref(&self) -> &'static nativeFundingCreated {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeFundingCreated {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeFundingCreated {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// A temporary channel ID, until the funding is established
#[no_mangle]
pub extern "C" fn FundingCreated_get_temporary_channel_id(this_ptr: &FundingCreated) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().temporary_channel_id;
	inner_val
}
/// A temporary channel ID, until the funding is established
#[no_mangle]
pub extern "C" fn FundingCreated_set_temporary_channel_id(this_ptr: &mut FundingCreated, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.temporary_channel_id = val.data;
}
/// The funding transaction ID
#[no_mangle]
pub extern "C" fn FundingCreated_get_funding_txid(this_ptr: &FundingCreated) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().funding_txid;
	inner_val.as_inner()
}
/// The funding transaction ID
#[no_mangle]
pub extern "C" fn FundingCreated_set_funding_txid(this_ptr: &mut FundingCreated, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.funding_txid = ::bitcoin::hash_types::Txid::from_slice(&val.data[..]).unwrap();
}
/// The specific output index funding this channel
#[no_mangle]
pub extern "C" fn FundingCreated_get_funding_output_index(this_ptr: &FundingCreated) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().funding_output_index;
	*inner_val
}
/// The specific output index funding this channel
#[no_mangle]
pub extern "C" fn FundingCreated_set_funding_output_index(this_ptr: &mut FundingCreated, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.funding_output_index = val;
}
/// The signature of the channel initiator (funder) on the initial commitment transaction
#[no_mangle]
pub extern "C" fn FundingCreated_get_signature(this_ptr: &FundingCreated) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().signature;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// The signature of the channel initiator (funder) on the initial commitment transaction
#[no_mangle]
pub extern "C" fn FundingCreated_set_signature(this_ptr: &mut FundingCreated, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.signature = val.into_rust();
}
/// Constructs a new FundingCreated given each field
#[must_use]
#[no_mangle]
pub extern "C" fn FundingCreated_new(mut temporary_channel_id_arg: crate::c_types::ThirtyTwoBytes, mut funding_txid_arg: crate::c_types::ThirtyTwoBytes, mut funding_output_index_arg: u16, mut signature_arg: crate::c_types::Signature) -> FundingCreated {
	FundingCreated { inner: ObjOps::heap_alloc(nativeFundingCreated {
		temporary_channel_id: temporary_channel_id_arg.data,
		funding_txid: ::bitcoin::hash_types::Txid::from_slice(&funding_txid_arg.data[..]).unwrap(),
		funding_output_index: funding_output_index_arg,
		signature: signature_arg.into_rust(),
	}), is_owned: true }
}
impl Clone for FundingCreated {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeFundingCreated>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn FundingCreated_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeFundingCreated)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the FundingCreated
pub extern "C" fn FundingCreated_clone(orig: &FundingCreated) -> FundingCreated {
	orig.clone()
}
/// Checks if two FundingCreateds contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn FundingCreated_eq(a: &FundingCreated, b: &FundingCreated) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::FundingSigned as nativeFundingSignedImport;
pub(crate) type nativeFundingSigned = nativeFundingSignedImport;

/// A [`funding_signed`] message to be sent to or received from a peer.
///
/// [`funding_signed`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#the-funding_signed-message
#[must_use]
#[repr(C)]
pub struct FundingSigned {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeFundingSigned,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for FundingSigned {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeFundingSigned>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the FundingSigned, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn FundingSigned_free(this_obj: FundingSigned) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn FundingSigned_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeFundingSigned) };
}
#[allow(unused)]
impl FundingSigned {
	pub(crate) fn get_native_ref(&self) -> &'static nativeFundingSigned {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeFundingSigned {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeFundingSigned {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn FundingSigned_get_channel_id(this_ptr: &FundingSigned) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn FundingSigned_set_channel_id(this_ptr: &mut FundingSigned, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The signature of the channel acceptor (fundee) on the initial commitment transaction
#[no_mangle]
pub extern "C" fn FundingSigned_get_signature(this_ptr: &FundingSigned) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().signature;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// The signature of the channel acceptor (fundee) on the initial commitment transaction
#[no_mangle]
pub extern "C" fn FundingSigned_set_signature(this_ptr: &mut FundingSigned, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.signature = val.into_rust();
}
/// Constructs a new FundingSigned given each field
#[must_use]
#[no_mangle]
pub extern "C" fn FundingSigned_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut signature_arg: crate::c_types::Signature) -> FundingSigned {
	FundingSigned { inner: ObjOps::heap_alloc(nativeFundingSigned {
		channel_id: channel_id_arg.data,
		signature: signature_arg.into_rust(),
	}), is_owned: true }
}
impl Clone for FundingSigned {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeFundingSigned>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn FundingSigned_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeFundingSigned)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the FundingSigned
pub extern "C" fn FundingSigned_clone(orig: &FundingSigned) -> FundingSigned {
	orig.clone()
}
/// Checks if two FundingSigneds contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn FundingSigned_eq(a: &FundingSigned, b: &FundingSigned) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ChannelReady as nativeChannelReadyImport;
pub(crate) type nativeChannelReady = nativeChannelReadyImport;

/// A [`channel_ready`] message to be sent to or received from a peer.
///
/// [`channel_ready`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#the-channel_ready-message
#[must_use]
#[repr(C)]
pub struct ChannelReady {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeChannelReady,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ChannelReady {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeChannelReady>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ChannelReady, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ChannelReady_free(this_obj: ChannelReady) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ChannelReady_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeChannelReady) };
}
#[allow(unused)]
impl ChannelReady {
	pub(crate) fn get_native_ref(&self) -> &'static nativeChannelReady {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeChannelReady {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeChannelReady {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn ChannelReady_get_channel_id(this_ptr: &ChannelReady) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn ChannelReady_set_channel_id(this_ptr: &mut ChannelReady, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The per-commitment point of the second commitment transaction
#[no_mangle]
pub extern "C" fn ChannelReady_get_next_per_commitment_point(this_ptr: &ChannelReady) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().next_per_commitment_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The per-commitment point of the second commitment transaction
#[no_mangle]
pub extern "C" fn ChannelReady_set_next_per_commitment_point(this_ptr: &mut ChannelReady, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.next_per_commitment_point = val.into_rust();
}
/// If set, provides a `short_channel_id` alias for this channel.
///
/// The sender will accept payments to be forwarded over this SCID and forward them to this
/// messages' recipient.
#[no_mangle]
pub extern "C" fn ChannelReady_get_short_channel_id_alias(this_ptr: &ChannelReady) -> crate::c_types::derived::COption_u64Z {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().short_channel_id_alias;
	let mut local_inner_val = if inner_val.is_none() { crate::c_types::derived::COption_u64Z::None } else { crate::c_types::derived::COption_u64Z::Some( { inner_val.unwrap() }) };
	local_inner_val
}
/// If set, provides a `short_channel_id` alias for this channel.
///
/// The sender will accept payments to be forwarded over this SCID and forward them to this
/// messages' recipient.
#[no_mangle]
pub extern "C" fn ChannelReady_set_short_channel_id_alias(this_ptr: &mut ChannelReady, mut val: crate::c_types::derived::COption_u64Z) {
	let mut local_val = if val.is_some() { Some( { val.take() }) } else { None };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.short_channel_id_alias = local_val;
}
/// Constructs a new ChannelReady given each field
#[must_use]
#[no_mangle]
pub extern "C" fn ChannelReady_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut next_per_commitment_point_arg: crate::c_types::PublicKey, mut short_channel_id_alias_arg: crate::c_types::derived::COption_u64Z) -> ChannelReady {
	let mut local_short_channel_id_alias_arg = if short_channel_id_alias_arg.is_some() { Some( { short_channel_id_alias_arg.take() }) } else { None };
	ChannelReady { inner: ObjOps::heap_alloc(nativeChannelReady {
		channel_id: channel_id_arg.data,
		next_per_commitment_point: next_per_commitment_point_arg.into_rust(),
		short_channel_id_alias: local_short_channel_id_alias_arg,
	}), is_owned: true }
}
impl Clone for ChannelReady {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeChannelReady>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ChannelReady_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeChannelReady)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ChannelReady
pub extern "C" fn ChannelReady_clone(orig: &ChannelReady) -> ChannelReady {
	orig.clone()
}
/// Checks if two ChannelReadys contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ChannelReady_eq(a: &ChannelReady, b: &ChannelReady) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::Shutdown as nativeShutdownImport;
pub(crate) type nativeShutdown = nativeShutdownImport;

/// A [`shutdown`] message to be sent to or received from a peer.
///
/// [`shutdown`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#closing-initiation-shutdown
#[must_use]
#[repr(C)]
pub struct Shutdown {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeShutdown,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for Shutdown {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeShutdown>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the Shutdown, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn Shutdown_free(this_obj: Shutdown) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Shutdown_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeShutdown) };
}
#[allow(unused)]
impl Shutdown {
	pub(crate) fn get_native_ref(&self) -> &'static nativeShutdown {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeShutdown {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeShutdown {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn Shutdown_get_channel_id(this_ptr: &Shutdown) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn Shutdown_set_channel_id(this_ptr: &mut Shutdown, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The destination of this peer's funds on closing.
///
/// Must be in one of these forms: P2PKH, P2SH, P2WPKH, P2WSH, P2TR.
#[no_mangle]
pub extern "C" fn Shutdown_get_scriptpubkey(this_ptr: &Shutdown) -> crate::c_types::u8slice {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().scriptpubkey;
	crate::c_types::u8slice::from_slice(&inner_val[..])
}
/// The destination of this peer's funds on closing.
///
/// Must be in one of these forms: P2PKH, P2SH, P2WPKH, P2WSH, P2TR.
#[no_mangle]
pub extern "C" fn Shutdown_set_scriptpubkey(this_ptr: &mut Shutdown, mut val: crate::c_types::derived::CVec_u8Z) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.scriptpubkey = ::bitcoin::blockdata::script::Script::from(val.into_rust());
}
/// Constructs a new Shutdown given each field
#[must_use]
#[no_mangle]
pub extern "C" fn Shutdown_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut scriptpubkey_arg: crate::c_types::derived::CVec_u8Z) -> Shutdown {
	Shutdown { inner: ObjOps::heap_alloc(nativeShutdown {
		channel_id: channel_id_arg.data,
		scriptpubkey: ::bitcoin::blockdata::script::Script::from(scriptpubkey_arg.into_rust()),
	}), is_owned: true }
}
impl Clone for Shutdown {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeShutdown>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn Shutdown_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeShutdown)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the Shutdown
pub extern "C" fn Shutdown_clone(orig: &Shutdown) -> Shutdown {
	orig.clone()
}
/// Checks if two Shutdowns contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn Shutdown_eq(a: &Shutdown, b: &Shutdown) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ClosingSignedFeeRange as nativeClosingSignedFeeRangeImport;
pub(crate) type nativeClosingSignedFeeRange = nativeClosingSignedFeeRangeImport;

/// The minimum and maximum fees which the sender is willing to place on the closing transaction.
///
/// This is provided in [`ClosingSigned`] by both sides to indicate the fee range they are willing
/// to use.
#[must_use]
#[repr(C)]
pub struct ClosingSignedFeeRange {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeClosingSignedFeeRange,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ClosingSignedFeeRange {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeClosingSignedFeeRange>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ClosingSignedFeeRange, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ClosingSignedFeeRange_free(this_obj: ClosingSignedFeeRange) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ClosingSignedFeeRange_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeClosingSignedFeeRange) };
}
#[allow(unused)]
impl ClosingSignedFeeRange {
	pub(crate) fn get_native_ref(&self) -> &'static nativeClosingSignedFeeRange {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeClosingSignedFeeRange {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeClosingSignedFeeRange {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The minimum absolute fee, in satoshis, which the sender is willing to place on the closing
/// transaction.
#[no_mangle]
pub extern "C" fn ClosingSignedFeeRange_get_min_fee_satoshis(this_ptr: &ClosingSignedFeeRange) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().min_fee_satoshis;
	*inner_val
}
/// The minimum absolute fee, in satoshis, which the sender is willing to place on the closing
/// transaction.
#[no_mangle]
pub extern "C" fn ClosingSignedFeeRange_set_min_fee_satoshis(this_ptr: &mut ClosingSignedFeeRange, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.min_fee_satoshis = val;
}
/// The maximum absolute fee, in satoshis, which the sender is willing to place on the closing
/// transaction.
#[no_mangle]
pub extern "C" fn ClosingSignedFeeRange_get_max_fee_satoshis(this_ptr: &ClosingSignedFeeRange) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().max_fee_satoshis;
	*inner_val
}
/// The maximum absolute fee, in satoshis, which the sender is willing to place on the closing
/// transaction.
#[no_mangle]
pub extern "C" fn ClosingSignedFeeRange_set_max_fee_satoshis(this_ptr: &mut ClosingSignedFeeRange, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.max_fee_satoshis = val;
}
/// Constructs a new ClosingSignedFeeRange given each field
#[must_use]
#[no_mangle]
pub extern "C" fn ClosingSignedFeeRange_new(mut min_fee_satoshis_arg: u64, mut max_fee_satoshis_arg: u64) -> ClosingSignedFeeRange {
	ClosingSignedFeeRange { inner: ObjOps::heap_alloc(nativeClosingSignedFeeRange {
		min_fee_satoshis: min_fee_satoshis_arg,
		max_fee_satoshis: max_fee_satoshis_arg,
	}), is_owned: true }
}
impl Clone for ClosingSignedFeeRange {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeClosingSignedFeeRange>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ClosingSignedFeeRange_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeClosingSignedFeeRange)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ClosingSignedFeeRange
pub extern "C" fn ClosingSignedFeeRange_clone(orig: &ClosingSignedFeeRange) -> ClosingSignedFeeRange {
	orig.clone()
}
/// Checks if two ClosingSignedFeeRanges contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ClosingSignedFeeRange_eq(a: &ClosingSignedFeeRange, b: &ClosingSignedFeeRange) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ClosingSigned as nativeClosingSignedImport;
pub(crate) type nativeClosingSigned = nativeClosingSignedImport;

/// A [`closing_signed`] message to be sent to or received from a peer.
///
/// [`closing_signed`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#closing-negotiation-closing_signed
#[must_use]
#[repr(C)]
pub struct ClosingSigned {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeClosingSigned,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ClosingSigned {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeClosingSigned>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ClosingSigned, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ClosingSigned_free(this_obj: ClosingSigned) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ClosingSigned_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeClosingSigned) };
}
#[allow(unused)]
impl ClosingSigned {
	pub(crate) fn get_native_ref(&self) -> &'static nativeClosingSigned {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeClosingSigned {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeClosingSigned {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn ClosingSigned_get_channel_id(this_ptr: &ClosingSigned) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn ClosingSigned_set_channel_id(this_ptr: &mut ClosingSigned, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The proposed total fee for the closing transaction
#[no_mangle]
pub extern "C" fn ClosingSigned_get_fee_satoshis(this_ptr: &ClosingSigned) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().fee_satoshis;
	*inner_val
}
/// The proposed total fee for the closing transaction
#[no_mangle]
pub extern "C" fn ClosingSigned_set_fee_satoshis(this_ptr: &mut ClosingSigned, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.fee_satoshis = val;
}
/// A signature on the closing transaction
#[no_mangle]
pub extern "C" fn ClosingSigned_get_signature(this_ptr: &ClosingSigned) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().signature;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// A signature on the closing transaction
#[no_mangle]
pub extern "C" fn ClosingSigned_set_signature(this_ptr: &mut ClosingSigned, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.signature = val.into_rust();
}
/// The minimum and maximum fees which the sender is willing to accept, provided only by new
/// nodes.
///
/// Note that the return value (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn ClosingSigned_get_fee_range(this_ptr: &ClosingSigned) -> crate::lightning::ln::msgs::ClosingSignedFeeRange {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().fee_range;
	let mut local_inner_val = crate::lightning::ln::msgs::ClosingSignedFeeRange { inner: unsafe { (if inner_val.is_none() { core::ptr::null() } else { ObjOps::nonnull_ptr_to_inner( { (inner_val.as_ref().unwrap()) }) } as *const lightning::ln::msgs::ClosingSignedFeeRange<>) as *mut _ }, is_owned: false };
	local_inner_val
}
/// The minimum and maximum fees which the sender is willing to accept, provided only by new
/// nodes.
///
/// Note that val (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn ClosingSigned_set_fee_range(this_ptr: &mut ClosingSigned, mut val: crate::lightning::ln::msgs::ClosingSignedFeeRange) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.take_inner()) } }) };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.fee_range = local_val;
}
/// Constructs a new ClosingSigned given each field
#[must_use]
#[no_mangle]
pub extern "C" fn ClosingSigned_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut fee_satoshis_arg: u64, mut signature_arg: crate::c_types::Signature, mut fee_range_arg: crate::lightning::ln::msgs::ClosingSignedFeeRange) -> ClosingSigned {
	let mut local_fee_range_arg = if fee_range_arg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(fee_range_arg.take_inner()) } }) };
	ClosingSigned { inner: ObjOps::heap_alloc(nativeClosingSigned {
		channel_id: channel_id_arg.data,
		fee_satoshis: fee_satoshis_arg,
		signature: signature_arg.into_rust(),
		fee_range: local_fee_range_arg,
	}), is_owned: true }
}
impl Clone for ClosingSigned {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeClosingSigned>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ClosingSigned_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeClosingSigned)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ClosingSigned
pub extern "C" fn ClosingSigned_clone(orig: &ClosingSigned) -> ClosingSigned {
	orig.clone()
}
/// Checks if two ClosingSigneds contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ClosingSigned_eq(a: &ClosingSigned, b: &ClosingSigned) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::UpdateAddHTLC as nativeUpdateAddHTLCImport;
pub(crate) type nativeUpdateAddHTLC = nativeUpdateAddHTLCImport;

/// An [`update_add_htlc`] message to be sent to or received from a peer.
///
/// [`update_add_htlc`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#adding-an-htlc-update_add_htlc
#[must_use]
#[repr(C)]
pub struct UpdateAddHTLC {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeUpdateAddHTLC,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for UpdateAddHTLC {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeUpdateAddHTLC>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the UpdateAddHTLC, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_free(this_obj: UpdateAddHTLC) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateAddHTLC_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeUpdateAddHTLC) };
}
#[allow(unused)]
impl UpdateAddHTLC {
	pub(crate) fn get_native_ref(&self) -> &'static nativeUpdateAddHTLC {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeUpdateAddHTLC {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeUpdateAddHTLC {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_get_channel_id(this_ptr: &UpdateAddHTLC) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_set_channel_id(this_ptr: &mut UpdateAddHTLC, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The HTLC ID
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_get_htlc_id(this_ptr: &UpdateAddHTLC) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_id;
	*inner_val
}
/// The HTLC ID
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_set_htlc_id(this_ptr: &mut UpdateAddHTLC, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_id = val;
}
/// The HTLC value in milli-satoshi
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_get_amount_msat(this_ptr: &UpdateAddHTLC) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().amount_msat;
	*inner_val
}
/// The HTLC value in milli-satoshi
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_set_amount_msat(this_ptr: &mut UpdateAddHTLC, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.amount_msat = val;
}
/// The payment hash, the pre-image of which controls HTLC redemption
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_get_payment_hash(this_ptr: &UpdateAddHTLC) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().payment_hash;
	&inner_val.0
}
/// The payment hash, the pre-image of which controls HTLC redemption
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_set_payment_hash(this_ptr: &mut UpdateAddHTLC, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.payment_hash = ::lightning::ln::PaymentHash(val.data);
}
/// The expiry height of the HTLC
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_get_cltv_expiry(this_ptr: &UpdateAddHTLC) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().cltv_expiry;
	*inner_val
}
/// The expiry height of the HTLC
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_set_cltv_expiry(this_ptr: &mut UpdateAddHTLC, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.cltv_expiry = val;
}
impl Clone for UpdateAddHTLC {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeUpdateAddHTLC>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateAddHTLC_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeUpdateAddHTLC)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the UpdateAddHTLC
pub extern "C" fn UpdateAddHTLC_clone(orig: &UpdateAddHTLC) -> UpdateAddHTLC {
	orig.clone()
}
/// Checks if two UpdateAddHTLCs contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_eq(a: &UpdateAddHTLC, b: &UpdateAddHTLC) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::OnionMessage as nativeOnionMessageImport;
pub(crate) type nativeOnionMessage = nativeOnionMessageImport;

/// An onion message to be sent to or received from a peer.
///
#[must_use]
#[repr(C)]
pub struct OnionMessage {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeOnionMessage,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for OnionMessage {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeOnionMessage>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the OnionMessage, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn OnionMessage_free(this_obj: OnionMessage) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn OnionMessage_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeOnionMessage) };
}
#[allow(unused)]
impl OnionMessage {
	pub(crate) fn get_native_ref(&self) -> &'static nativeOnionMessage {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeOnionMessage {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeOnionMessage {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// Used in decrypting the onion packet's payload.
#[no_mangle]
pub extern "C" fn OnionMessage_get_blinding_point(this_ptr: &OnionMessage) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().blinding_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// Used in decrypting the onion packet's payload.
#[no_mangle]
pub extern "C" fn OnionMessage_set_blinding_point(this_ptr: &mut OnionMessage, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.blinding_point = val.into_rust();
}
impl Clone for OnionMessage {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeOnionMessage>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn OnionMessage_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeOnionMessage)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the OnionMessage
pub extern "C" fn OnionMessage_clone(orig: &OnionMessage) -> OnionMessage {
	orig.clone()
}
/// Checks if two OnionMessages contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn OnionMessage_eq(a: &OnionMessage, b: &OnionMessage) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::UpdateFulfillHTLC as nativeUpdateFulfillHTLCImport;
pub(crate) type nativeUpdateFulfillHTLC = nativeUpdateFulfillHTLCImport;

/// An [`update_fulfill_htlc`] message to be sent to or received from a peer.
///
/// [`update_fulfill_htlc`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#removing-an-htlc-update_fulfill_htlc-update_fail_htlc-and-update_fail_malformed_htlc
#[must_use]
#[repr(C)]
pub struct UpdateFulfillHTLC {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeUpdateFulfillHTLC,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for UpdateFulfillHTLC {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeUpdateFulfillHTLC>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the UpdateFulfillHTLC, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_free(this_obj: UpdateFulfillHTLC) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateFulfillHTLC_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeUpdateFulfillHTLC) };
}
#[allow(unused)]
impl UpdateFulfillHTLC {
	pub(crate) fn get_native_ref(&self) -> &'static nativeUpdateFulfillHTLC {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeUpdateFulfillHTLC {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeUpdateFulfillHTLC {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_get_channel_id(this_ptr: &UpdateFulfillHTLC) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_set_channel_id(this_ptr: &mut UpdateFulfillHTLC, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The HTLC ID
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_get_htlc_id(this_ptr: &UpdateFulfillHTLC) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_id;
	*inner_val
}
/// The HTLC ID
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_set_htlc_id(this_ptr: &mut UpdateFulfillHTLC, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_id = val;
}
/// The pre-image of the payment hash, allowing HTLC redemption
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_get_payment_preimage(this_ptr: &UpdateFulfillHTLC) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().payment_preimage;
	&inner_val.0
}
/// The pre-image of the payment hash, allowing HTLC redemption
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_set_payment_preimage(this_ptr: &mut UpdateFulfillHTLC, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.payment_preimage = ::lightning::ln::PaymentPreimage(val.data);
}
/// Constructs a new UpdateFulfillHTLC given each field
#[must_use]
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut htlc_id_arg: u64, mut payment_preimage_arg: crate::c_types::ThirtyTwoBytes) -> UpdateFulfillHTLC {
	UpdateFulfillHTLC { inner: ObjOps::heap_alloc(nativeUpdateFulfillHTLC {
		channel_id: channel_id_arg.data,
		htlc_id: htlc_id_arg,
		payment_preimage: ::lightning::ln::PaymentPreimage(payment_preimage_arg.data),
	}), is_owned: true }
}
impl Clone for UpdateFulfillHTLC {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeUpdateFulfillHTLC>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateFulfillHTLC_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeUpdateFulfillHTLC)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the UpdateFulfillHTLC
pub extern "C" fn UpdateFulfillHTLC_clone(orig: &UpdateFulfillHTLC) -> UpdateFulfillHTLC {
	orig.clone()
}
/// Checks if two UpdateFulfillHTLCs contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_eq(a: &UpdateFulfillHTLC, b: &UpdateFulfillHTLC) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::UpdateFailHTLC as nativeUpdateFailHTLCImport;
pub(crate) type nativeUpdateFailHTLC = nativeUpdateFailHTLCImport;

/// An [`update_fail_htlc`] message to be sent to or received from a peer.
///
/// [`update_fail_htlc`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#removing-an-htlc-update_fulfill_htlc-update_fail_htlc-and-update_fail_malformed_htlc
#[must_use]
#[repr(C)]
pub struct UpdateFailHTLC {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeUpdateFailHTLC,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for UpdateFailHTLC {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeUpdateFailHTLC>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the UpdateFailHTLC, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_free(this_obj: UpdateFailHTLC) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateFailHTLC_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeUpdateFailHTLC) };
}
#[allow(unused)]
impl UpdateFailHTLC {
	pub(crate) fn get_native_ref(&self) -> &'static nativeUpdateFailHTLC {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeUpdateFailHTLC {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeUpdateFailHTLC {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_get_channel_id(this_ptr: &UpdateFailHTLC) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_set_channel_id(this_ptr: &mut UpdateFailHTLC, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The HTLC ID
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_get_htlc_id(this_ptr: &UpdateFailHTLC) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_id;
	*inner_val
}
/// The HTLC ID
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_set_htlc_id(this_ptr: &mut UpdateFailHTLC, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_id = val;
}
impl Clone for UpdateFailHTLC {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeUpdateFailHTLC>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateFailHTLC_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeUpdateFailHTLC)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the UpdateFailHTLC
pub extern "C" fn UpdateFailHTLC_clone(orig: &UpdateFailHTLC) -> UpdateFailHTLC {
	orig.clone()
}
/// Checks if two UpdateFailHTLCs contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_eq(a: &UpdateFailHTLC, b: &UpdateFailHTLC) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::UpdateFailMalformedHTLC as nativeUpdateFailMalformedHTLCImport;
pub(crate) type nativeUpdateFailMalformedHTLC = nativeUpdateFailMalformedHTLCImport;

/// An [`update_fail_malformed_htlc`] message to be sent to or received from a peer.
///
/// [`update_fail_malformed_htlc`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#removing-an-htlc-update_fulfill_htlc-update_fail_htlc-and-update_fail_malformed_htlc
#[must_use]
#[repr(C)]
pub struct UpdateFailMalformedHTLC {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeUpdateFailMalformedHTLC,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for UpdateFailMalformedHTLC {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeUpdateFailMalformedHTLC>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the UpdateFailMalformedHTLC, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_free(this_obj: UpdateFailMalformedHTLC) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateFailMalformedHTLC_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeUpdateFailMalformedHTLC) };
}
#[allow(unused)]
impl UpdateFailMalformedHTLC {
	pub(crate) fn get_native_ref(&self) -> &'static nativeUpdateFailMalformedHTLC {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeUpdateFailMalformedHTLC {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeUpdateFailMalformedHTLC {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_get_channel_id(this_ptr: &UpdateFailMalformedHTLC) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_set_channel_id(this_ptr: &mut UpdateFailMalformedHTLC, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The HTLC ID
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_get_htlc_id(this_ptr: &UpdateFailMalformedHTLC) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_id;
	*inner_val
}
/// The HTLC ID
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_set_htlc_id(this_ptr: &mut UpdateFailMalformedHTLC, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_id = val;
}
/// The failure code
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_get_failure_code(this_ptr: &UpdateFailMalformedHTLC) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().failure_code;
	*inner_val
}
/// The failure code
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_set_failure_code(this_ptr: &mut UpdateFailMalformedHTLC, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.failure_code = val;
}
impl Clone for UpdateFailMalformedHTLC {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeUpdateFailMalformedHTLC>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateFailMalformedHTLC_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeUpdateFailMalformedHTLC)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the UpdateFailMalformedHTLC
pub extern "C" fn UpdateFailMalformedHTLC_clone(orig: &UpdateFailMalformedHTLC) -> UpdateFailMalformedHTLC {
	orig.clone()
}
/// Checks if two UpdateFailMalformedHTLCs contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_eq(a: &UpdateFailMalformedHTLC, b: &UpdateFailMalformedHTLC) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::CommitmentSigned as nativeCommitmentSignedImport;
pub(crate) type nativeCommitmentSigned = nativeCommitmentSignedImport;

/// A [`commitment_signed`] message to be sent to or received from a peer.
///
/// [`commitment_signed`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#committing-updates-so-far-commitment_signed
#[must_use]
#[repr(C)]
pub struct CommitmentSigned {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeCommitmentSigned,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for CommitmentSigned {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeCommitmentSigned>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the CommitmentSigned, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn CommitmentSigned_free(this_obj: CommitmentSigned) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn CommitmentSigned_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeCommitmentSigned) };
}
#[allow(unused)]
impl CommitmentSigned {
	pub(crate) fn get_native_ref(&self) -> &'static nativeCommitmentSigned {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeCommitmentSigned {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeCommitmentSigned {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn CommitmentSigned_get_channel_id(this_ptr: &CommitmentSigned) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn CommitmentSigned_set_channel_id(this_ptr: &mut CommitmentSigned, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// A signature on the commitment transaction
#[no_mangle]
pub extern "C" fn CommitmentSigned_get_signature(this_ptr: &CommitmentSigned) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().signature;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// A signature on the commitment transaction
#[no_mangle]
pub extern "C" fn CommitmentSigned_set_signature(this_ptr: &mut CommitmentSigned, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.signature = val.into_rust();
}
/// Signatures on the HTLC transactions
///
/// Returns a copy of the field.
#[no_mangle]
pub extern "C" fn CommitmentSigned_get_htlc_signatures(this_ptr: &CommitmentSigned) -> crate::c_types::derived::CVec_SignatureZ {
	let mut inner_val = this_ptr.get_native_mut_ref().htlc_signatures.clone();
	let mut local_inner_val = Vec::new(); for mut item in inner_val.drain(..) { local_inner_val.push( { crate::c_types::Signature::from_rust(&item) }); };
	local_inner_val.into()
}
/// Signatures on the HTLC transactions
#[no_mangle]
pub extern "C" fn CommitmentSigned_set_htlc_signatures(this_ptr: &mut CommitmentSigned, mut val: crate::c_types::derived::CVec_SignatureZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { item.into_rust() }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_signatures = local_val;
}
/// Constructs a new CommitmentSigned given each field
#[must_use]
#[no_mangle]
pub extern "C" fn CommitmentSigned_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut signature_arg: crate::c_types::Signature, mut htlc_signatures_arg: crate::c_types::derived::CVec_SignatureZ) -> CommitmentSigned {
	let mut local_htlc_signatures_arg = Vec::new(); for mut item in htlc_signatures_arg.into_rust().drain(..) { local_htlc_signatures_arg.push( { item.into_rust() }); };
	CommitmentSigned { inner: ObjOps::heap_alloc(nativeCommitmentSigned {
		channel_id: channel_id_arg.data,
		signature: signature_arg.into_rust(),
		htlc_signatures: local_htlc_signatures_arg,
	}), is_owned: true }
}
impl Clone for CommitmentSigned {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeCommitmentSigned>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn CommitmentSigned_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeCommitmentSigned)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the CommitmentSigned
pub extern "C" fn CommitmentSigned_clone(orig: &CommitmentSigned) -> CommitmentSigned {
	orig.clone()
}
/// Checks if two CommitmentSigneds contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn CommitmentSigned_eq(a: &CommitmentSigned, b: &CommitmentSigned) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::RevokeAndACK as nativeRevokeAndACKImport;
pub(crate) type nativeRevokeAndACK = nativeRevokeAndACKImport;

/// A [`revoke_and_ack`] message to be sent to or received from a peer.
///
/// [`revoke_and_ack`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#completing-the-transition-to-the-updated-state-revoke_and_ack
#[must_use]
#[repr(C)]
pub struct RevokeAndACK {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeRevokeAndACK,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for RevokeAndACK {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeRevokeAndACK>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the RevokeAndACK, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn RevokeAndACK_free(this_obj: RevokeAndACK) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RevokeAndACK_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeRevokeAndACK) };
}
#[allow(unused)]
impl RevokeAndACK {
	pub(crate) fn get_native_ref(&self) -> &'static nativeRevokeAndACK {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeRevokeAndACK {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeRevokeAndACK {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn RevokeAndACK_get_channel_id(this_ptr: &RevokeAndACK) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn RevokeAndACK_set_channel_id(this_ptr: &mut RevokeAndACK, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The secret corresponding to the per-commitment point
#[no_mangle]
pub extern "C" fn RevokeAndACK_get_per_commitment_secret(this_ptr: &RevokeAndACK) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().per_commitment_secret;
	inner_val
}
/// The secret corresponding to the per-commitment point
#[no_mangle]
pub extern "C" fn RevokeAndACK_set_per_commitment_secret(this_ptr: &mut RevokeAndACK, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.per_commitment_secret = val.data;
}
/// The next sender-broadcast commitment transaction's per-commitment point
#[no_mangle]
pub extern "C" fn RevokeAndACK_get_next_per_commitment_point(this_ptr: &RevokeAndACK) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().next_per_commitment_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The next sender-broadcast commitment transaction's per-commitment point
#[no_mangle]
pub extern "C" fn RevokeAndACK_set_next_per_commitment_point(this_ptr: &mut RevokeAndACK, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.next_per_commitment_point = val.into_rust();
}
/// Constructs a new RevokeAndACK given each field
#[must_use]
#[no_mangle]
pub extern "C" fn RevokeAndACK_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut per_commitment_secret_arg: crate::c_types::ThirtyTwoBytes, mut next_per_commitment_point_arg: crate::c_types::PublicKey) -> RevokeAndACK {
	RevokeAndACK { inner: ObjOps::heap_alloc(nativeRevokeAndACK {
		channel_id: channel_id_arg.data,
		per_commitment_secret: per_commitment_secret_arg.data,
		next_per_commitment_point: next_per_commitment_point_arg.into_rust(),
	}), is_owned: true }
}
impl Clone for RevokeAndACK {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeRevokeAndACK>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn RevokeAndACK_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeRevokeAndACK)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the RevokeAndACK
pub extern "C" fn RevokeAndACK_clone(orig: &RevokeAndACK) -> RevokeAndACK {
	orig.clone()
}
/// Checks if two RevokeAndACKs contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn RevokeAndACK_eq(a: &RevokeAndACK, b: &RevokeAndACK) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::UpdateFee as nativeUpdateFeeImport;
pub(crate) type nativeUpdateFee = nativeUpdateFeeImport;

/// An [`update_fee`] message to be sent to or received from a peer
///
/// [`update_fee`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#updating-fees-update_fee
#[must_use]
#[repr(C)]
pub struct UpdateFee {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeUpdateFee,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for UpdateFee {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeUpdateFee>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the UpdateFee, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn UpdateFee_free(this_obj: UpdateFee) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateFee_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeUpdateFee) };
}
#[allow(unused)]
impl UpdateFee {
	pub(crate) fn get_native_ref(&self) -> &'static nativeUpdateFee {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeUpdateFee {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeUpdateFee {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateFee_get_channel_id(this_ptr: &UpdateFee) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn UpdateFee_set_channel_id(this_ptr: &mut UpdateFee, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// Fee rate per 1000-weight of the transaction
#[no_mangle]
pub extern "C" fn UpdateFee_get_feerate_per_kw(this_ptr: &UpdateFee) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().feerate_per_kw;
	*inner_val
}
/// Fee rate per 1000-weight of the transaction
#[no_mangle]
pub extern "C" fn UpdateFee_set_feerate_per_kw(this_ptr: &mut UpdateFee, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.feerate_per_kw = val;
}
/// Constructs a new UpdateFee given each field
#[must_use]
#[no_mangle]
pub extern "C" fn UpdateFee_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut feerate_per_kw_arg: u32) -> UpdateFee {
	UpdateFee { inner: ObjOps::heap_alloc(nativeUpdateFee {
		channel_id: channel_id_arg.data,
		feerate_per_kw: feerate_per_kw_arg,
	}), is_owned: true }
}
impl Clone for UpdateFee {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeUpdateFee>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UpdateFee_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeUpdateFee)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the UpdateFee
pub extern "C" fn UpdateFee_clone(orig: &UpdateFee) -> UpdateFee {
	orig.clone()
}
/// Checks if two UpdateFees contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn UpdateFee_eq(a: &UpdateFee, b: &UpdateFee) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::DataLossProtect as nativeDataLossProtectImport;
pub(crate) type nativeDataLossProtect = nativeDataLossProtectImport;

/// Proof that the sender knows the per-commitment secret of the previous commitment transaction.
///
/// This is used to convince the recipient that the channel is at a certain commitment
/// number even if they lost that data due to a local failure. Of course, the peer may lie
/// and even later commitments may have been revoked.
#[must_use]
#[repr(C)]
pub struct DataLossProtect {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeDataLossProtect,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for DataLossProtect {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeDataLossProtect>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the DataLossProtect, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn DataLossProtect_free(this_obj: DataLossProtect) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn DataLossProtect_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeDataLossProtect) };
}
#[allow(unused)]
impl DataLossProtect {
	pub(crate) fn get_native_ref(&self) -> &'static nativeDataLossProtect {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeDataLossProtect {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeDataLossProtect {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// Proof that the sender knows the per-commitment secret of a specific commitment transaction
/// belonging to the recipient
#[no_mangle]
pub extern "C" fn DataLossProtect_get_your_last_per_commitment_secret(this_ptr: &DataLossProtect) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().your_last_per_commitment_secret;
	inner_val
}
/// Proof that the sender knows the per-commitment secret of a specific commitment transaction
/// belonging to the recipient
#[no_mangle]
pub extern "C" fn DataLossProtect_set_your_last_per_commitment_secret(this_ptr: &mut DataLossProtect, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.your_last_per_commitment_secret = val.data;
}
/// The sender's per-commitment point for their current commitment transaction
#[no_mangle]
pub extern "C" fn DataLossProtect_get_my_current_per_commitment_point(this_ptr: &DataLossProtect) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().my_current_per_commitment_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The sender's per-commitment point for their current commitment transaction
#[no_mangle]
pub extern "C" fn DataLossProtect_set_my_current_per_commitment_point(this_ptr: &mut DataLossProtect, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.my_current_per_commitment_point = val.into_rust();
}
/// Constructs a new DataLossProtect given each field
#[must_use]
#[no_mangle]
pub extern "C" fn DataLossProtect_new(mut your_last_per_commitment_secret_arg: crate::c_types::ThirtyTwoBytes, mut my_current_per_commitment_point_arg: crate::c_types::PublicKey) -> DataLossProtect {
	DataLossProtect { inner: ObjOps::heap_alloc(nativeDataLossProtect {
		your_last_per_commitment_secret: your_last_per_commitment_secret_arg.data,
		my_current_per_commitment_point: my_current_per_commitment_point_arg.into_rust(),
	}), is_owned: true }
}
impl Clone for DataLossProtect {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeDataLossProtect>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn DataLossProtect_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeDataLossProtect)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the DataLossProtect
pub extern "C" fn DataLossProtect_clone(orig: &DataLossProtect) -> DataLossProtect {
	orig.clone()
}
/// Checks if two DataLossProtects contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn DataLossProtect_eq(a: &DataLossProtect, b: &DataLossProtect) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ChannelReestablish as nativeChannelReestablishImport;
pub(crate) type nativeChannelReestablish = nativeChannelReestablishImport;

/// A [`channel_reestablish`] message to be sent to or received from a peer.
///
/// [`channel_reestablish`]: https://github.com/lightning/bolts/blob/master/02-peer-protocol.md#message-retransmission
#[must_use]
#[repr(C)]
pub struct ChannelReestablish {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeChannelReestablish,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ChannelReestablish {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeChannelReestablish>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ChannelReestablish, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ChannelReestablish_free(this_obj: ChannelReestablish) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ChannelReestablish_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeChannelReestablish) };
}
#[allow(unused)]
impl ChannelReestablish {
	pub(crate) fn get_native_ref(&self) -> &'static nativeChannelReestablish {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeChannelReestablish {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeChannelReestablish {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn ChannelReestablish_get_channel_id(this_ptr: &ChannelReestablish) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn ChannelReestablish_set_channel_id(this_ptr: &mut ChannelReestablish, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The next commitment number for the sender
#[no_mangle]
pub extern "C" fn ChannelReestablish_get_next_local_commitment_number(this_ptr: &ChannelReestablish) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().next_local_commitment_number;
	*inner_val
}
/// The next commitment number for the sender
#[no_mangle]
pub extern "C" fn ChannelReestablish_set_next_local_commitment_number(this_ptr: &mut ChannelReestablish, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.next_local_commitment_number = val;
}
/// The next commitment number for the recipient
#[no_mangle]
pub extern "C" fn ChannelReestablish_get_next_remote_commitment_number(this_ptr: &ChannelReestablish) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().next_remote_commitment_number;
	*inner_val
}
/// The next commitment number for the recipient
#[no_mangle]
pub extern "C" fn ChannelReestablish_set_next_remote_commitment_number(this_ptr: &mut ChannelReestablish, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.next_remote_commitment_number = val;
}
impl Clone for ChannelReestablish {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeChannelReestablish>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ChannelReestablish_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeChannelReestablish)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ChannelReestablish
pub extern "C" fn ChannelReestablish_clone(orig: &ChannelReestablish) -> ChannelReestablish {
	orig.clone()
}
/// Checks if two ChannelReestablishs contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ChannelReestablish_eq(a: &ChannelReestablish, b: &ChannelReestablish) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::AnnouncementSignatures as nativeAnnouncementSignaturesImport;
pub(crate) type nativeAnnouncementSignatures = nativeAnnouncementSignaturesImport;

/// An [`announcement_signatures`] message to be sent to or received from a peer.
///
/// [`announcement_signatures`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-announcement_signatures-message
#[must_use]
#[repr(C)]
pub struct AnnouncementSignatures {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeAnnouncementSignatures,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for AnnouncementSignatures {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeAnnouncementSignatures>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the AnnouncementSignatures, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_free(this_obj: AnnouncementSignatures) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn AnnouncementSignatures_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeAnnouncementSignatures) };
}
#[allow(unused)]
impl AnnouncementSignatures {
	pub(crate) fn get_native_ref(&self) -> &'static nativeAnnouncementSignatures {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeAnnouncementSignatures {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeAnnouncementSignatures {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The channel ID
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_get_channel_id(this_ptr: &AnnouncementSignatures) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_id;
	inner_val
}
/// The channel ID
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_set_channel_id(this_ptr: &mut AnnouncementSignatures, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_id = val.data;
}
/// The short channel ID
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_get_short_channel_id(this_ptr: &AnnouncementSignatures) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().short_channel_id;
	*inner_val
}
/// The short channel ID
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_set_short_channel_id(this_ptr: &mut AnnouncementSignatures, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.short_channel_id = val;
}
/// A signature by the node key
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_get_node_signature(this_ptr: &AnnouncementSignatures) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().node_signature;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// A signature by the node key
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_set_node_signature(this_ptr: &mut AnnouncementSignatures, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.node_signature = val.into_rust();
}
/// A signature by the funding key
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_get_bitcoin_signature(this_ptr: &AnnouncementSignatures) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().bitcoin_signature;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// A signature by the funding key
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_set_bitcoin_signature(this_ptr: &mut AnnouncementSignatures, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.bitcoin_signature = val.into_rust();
}
/// Constructs a new AnnouncementSignatures given each field
#[must_use]
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut short_channel_id_arg: u64, mut node_signature_arg: crate::c_types::Signature, mut bitcoin_signature_arg: crate::c_types::Signature) -> AnnouncementSignatures {
	AnnouncementSignatures { inner: ObjOps::heap_alloc(nativeAnnouncementSignatures {
		channel_id: channel_id_arg.data,
		short_channel_id: short_channel_id_arg,
		node_signature: node_signature_arg.into_rust(),
		bitcoin_signature: bitcoin_signature_arg.into_rust(),
	}), is_owned: true }
}
impl Clone for AnnouncementSignatures {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeAnnouncementSignatures>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn AnnouncementSignatures_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeAnnouncementSignatures)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the AnnouncementSignatures
pub extern "C" fn AnnouncementSignatures_clone(orig: &AnnouncementSignatures) -> AnnouncementSignatures {
	orig.clone()
}
/// Checks if two AnnouncementSignaturess contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_eq(a: &AnnouncementSignatures, b: &AnnouncementSignatures) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}
/// An address which can be used to connect to a remote peer.
#[derive(Clone)]
#[must_use]
#[repr(C)]
pub enum NetAddress {
	/// An IPv4 address/port on which the peer is listening.
	IPv4 {
		/// The 4-byte IPv4 address
		addr: crate::c_types::FourBytes,
		/// The port on which the node is listening
		port: u16,
	},
	/// An IPv6 address/port on which the peer is listening.
	IPv6 {
		/// The 16-byte IPv6 address
		addr: crate::c_types::SixteenBytes,
		/// The port on which the node is listening
		port: u16,
	},
	/// An old-style Tor onion address/port on which the peer is listening.
	///
	/// This field is deprecated and the Tor network generally no longer supports V2 Onion
	/// addresses. Thus, the details are not parsed here.
	OnionV2(
		crate::c_types::TwelveBytes),
	/// A new-style Tor onion address/port on which the peer is listening.
	///
	/// To create the human-readable \"hostname\", concatenate the ED25519 pubkey, checksum, and version,
	/// wrap as base32 and append \".onion\".
	OnionV3 {
		/// The ed25519 long-term public key of the peer
		ed25519_pubkey: crate::c_types::ThirtyTwoBytes,
		/// The checksum of the pubkey and version, as included in the onion address
		checksum: u16,
		/// The version byte, as defined by the Tor Onion v3 spec.
		version: u8,
		/// The port on which the node is listening
		port: u16,
	},
	/// A hostname/port on which the peer is listening.
	Hostname {
		/// The hostname on which the node is listening.
		hostname: crate::lightning::util::ser::Hostname,
		/// The port on which the node is listening.
		port: u16,
	},
}
use lightning::ln::msgs::NetAddress as NetAddressImport;
pub(crate) type nativeNetAddress = NetAddressImport;

impl NetAddress {
	#[allow(unused)]
	pub(crate) fn to_native(&self) -> nativeNetAddress {
		match self {
			NetAddress::IPv4 {ref addr, ref port, } => {
				let mut addr_nonref = Clone::clone(addr);
				let mut port_nonref = Clone::clone(port);
				nativeNetAddress::IPv4 {
					addr: addr_nonref.data,
					port: port_nonref,
				}
			},
			NetAddress::IPv6 {ref addr, ref port, } => {
				let mut addr_nonref = Clone::clone(addr);
				let mut port_nonref = Clone::clone(port);
				nativeNetAddress::IPv6 {
					addr: addr_nonref.data,
					port: port_nonref,
				}
			},
			NetAddress::OnionV2 (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				nativeNetAddress::OnionV2 (
					a_nonref.data,
				)
			},
			NetAddress::OnionV3 {ref ed25519_pubkey, ref checksum, ref version, ref port, } => {
				let mut ed25519_pubkey_nonref = Clone::clone(ed25519_pubkey);
				let mut checksum_nonref = Clone::clone(checksum);
				let mut version_nonref = Clone::clone(version);
				let mut port_nonref = Clone::clone(port);
				nativeNetAddress::OnionV3 {
					ed25519_pubkey: ed25519_pubkey_nonref.data,
					checksum: checksum_nonref,
					version: version_nonref,
					port: port_nonref,
				}
			},
			NetAddress::Hostname {ref hostname, ref port, } => {
				let mut hostname_nonref = Clone::clone(hostname);
				let mut port_nonref = Clone::clone(port);
				nativeNetAddress::Hostname {
					hostname: *unsafe { Box::from_raw(hostname_nonref.take_inner()) },
					port: port_nonref,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_native(self) -> nativeNetAddress {
		match self {
			NetAddress::IPv4 {mut addr, mut port, } => {
				nativeNetAddress::IPv4 {
					addr: addr.data,
					port: port,
				}
			},
			NetAddress::IPv6 {mut addr, mut port, } => {
				nativeNetAddress::IPv6 {
					addr: addr.data,
					port: port,
				}
			},
			NetAddress::OnionV2 (mut a, ) => {
				nativeNetAddress::OnionV2 (
					a.data,
				)
			},
			NetAddress::OnionV3 {mut ed25519_pubkey, mut checksum, mut version, mut port, } => {
				nativeNetAddress::OnionV3 {
					ed25519_pubkey: ed25519_pubkey.data,
					checksum: checksum,
					version: version,
					port: port,
				}
			},
			NetAddress::Hostname {mut hostname, mut port, } => {
				nativeNetAddress::Hostname {
					hostname: *unsafe { Box::from_raw(hostname.take_inner()) },
					port: port,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_native(native: &nativeNetAddress) -> Self {
		match native {
			nativeNetAddress::IPv4 {ref addr, ref port, } => {
				let mut addr_nonref = Clone::clone(addr);
				let mut port_nonref = Clone::clone(port);
				NetAddress::IPv4 {
					addr: crate::c_types::FourBytes { data: addr_nonref },
					port: port_nonref,
				}
			},
			nativeNetAddress::IPv6 {ref addr, ref port, } => {
				let mut addr_nonref = Clone::clone(addr);
				let mut port_nonref = Clone::clone(port);
				NetAddress::IPv6 {
					addr: crate::c_types::SixteenBytes { data: addr_nonref },
					port: port_nonref,
				}
			},
			nativeNetAddress::OnionV2 (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				NetAddress::OnionV2 (
					crate::c_types::TwelveBytes { data: a_nonref },
				)
			},
			nativeNetAddress::OnionV3 {ref ed25519_pubkey, ref checksum, ref version, ref port, } => {
				let mut ed25519_pubkey_nonref = Clone::clone(ed25519_pubkey);
				let mut checksum_nonref = Clone::clone(checksum);
				let mut version_nonref = Clone::clone(version);
				let mut port_nonref = Clone::clone(port);
				NetAddress::OnionV3 {
					ed25519_pubkey: crate::c_types::ThirtyTwoBytes { data: ed25519_pubkey_nonref },
					checksum: checksum_nonref,
					version: version_nonref,
					port: port_nonref,
				}
			},
			nativeNetAddress::Hostname {ref hostname, ref port, } => {
				let mut hostname_nonref = Clone::clone(hostname);
				let mut port_nonref = Clone::clone(port);
				NetAddress::Hostname {
					hostname: crate::lightning::util::ser::Hostname { inner: ObjOps::heap_alloc(hostname_nonref), is_owned: true },
					port: port_nonref,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn native_into(native: nativeNetAddress) -> Self {
		match native {
			nativeNetAddress::IPv4 {mut addr, mut port, } => {
				NetAddress::IPv4 {
					addr: crate::c_types::FourBytes { data: addr },
					port: port,
				}
			},
			nativeNetAddress::IPv6 {mut addr, mut port, } => {
				NetAddress::IPv6 {
					addr: crate::c_types::SixteenBytes { data: addr },
					port: port,
				}
			},
			nativeNetAddress::OnionV2 (mut a, ) => {
				NetAddress::OnionV2 (
					crate::c_types::TwelveBytes { data: a },
				)
			},
			nativeNetAddress::OnionV3 {mut ed25519_pubkey, mut checksum, mut version, mut port, } => {
				NetAddress::OnionV3 {
					ed25519_pubkey: crate::c_types::ThirtyTwoBytes { data: ed25519_pubkey },
					checksum: checksum,
					version: version,
					port: port,
				}
			},
			nativeNetAddress::Hostname {mut hostname, mut port, } => {
				NetAddress::Hostname {
					hostname: crate::lightning::util::ser::Hostname { inner: ObjOps::heap_alloc(hostname), is_owned: true },
					port: port,
				}
			},
		}
	}
}
/// Frees any resources used by the NetAddress
#[no_mangle]
pub extern "C" fn NetAddress_free(this_ptr: NetAddress) { }
/// Creates a copy of the NetAddress
#[no_mangle]
pub extern "C" fn NetAddress_clone(orig: &NetAddress) -> NetAddress {
	orig.clone()
}
#[no_mangle]
/// Utility method to constructs a new IPv4-variant NetAddress
pub extern "C" fn NetAddress_ipv4(addr: crate::c_types::FourBytes, port: u16) -> NetAddress {
	NetAddress::IPv4 {
		addr,
		port,
	}
}
#[no_mangle]
/// Utility method to constructs a new IPv6-variant NetAddress
pub extern "C" fn NetAddress_ipv6(addr: crate::c_types::SixteenBytes, port: u16) -> NetAddress {
	NetAddress::IPv6 {
		addr,
		port,
	}
}
#[no_mangle]
/// Utility method to constructs a new OnionV2-variant NetAddress
pub extern "C" fn NetAddress_onion_v2(a: crate::c_types::TwelveBytes) -> NetAddress {
	NetAddress::OnionV2(a, )
}
#[no_mangle]
/// Utility method to constructs a new OnionV3-variant NetAddress
pub extern "C" fn NetAddress_onion_v3(ed25519_pubkey: crate::c_types::ThirtyTwoBytes, checksum: u16, version: u8, port: u16) -> NetAddress {
	NetAddress::OnionV3 {
		ed25519_pubkey,
		checksum,
		version,
		port,
	}
}
#[no_mangle]
/// Utility method to constructs a new Hostname-variant NetAddress
pub extern "C" fn NetAddress_hostname(hostname: crate::lightning::util::ser::Hostname, port: u16) -> NetAddress {
	NetAddress::Hostname {
		hostname,
		port,
	}
}
/// Checks if two NetAddresss contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
#[no_mangle]
pub extern "C" fn NetAddress_eq(a: &NetAddress, b: &NetAddress) -> bool {
	if &a.to_native() == &b.to_native() { true } else { false }
}
#[no_mangle]
/// Serialize the NetAddress object into a byte array which can be read by NetAddress_read
pub extern "C" fn NetAddress_write(obj: &crate::lightning::ln::msgs::NetAddress) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(&unsafe { &*obj }.to_native())
}
#[no_mangle]
/// Read a NetAddress from a byte array, created by NetAddress_write
pub extern "C" fn NetAddress_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_NetAddressDecodeErrorZ {
	let res: Result<lightning::ln::msgs::NetAddress, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::NetAddress::native_into(o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
/// Represents the set of gossip messages that require a signature from a node's identity key.
#[derive(Clone)]
#[must_use]
#[repr(C)]
pub enum UnsignedGossipMessage {
	/// An unsigned channel announcement.
	ChannelAnnouncement(
		crate::lightning::ln::msgs::UnsignedChannelAnnouncement),
	/// An unsigned channel update.
	ChannelUpdate(
		crate::lightning::ln::msgs::UnsignedChannelUpdate),
	/// An unsigned node announcement.
	NodeAnnouncement(
		crate::lightning::ln::msgs::UnsignedNodeAnnouncement),
}
use lightning::ln::msgs::UnsignedGossipMessage as UnsignedGossipMessageImport;
pub(crate) type nativeUnsignedGossipMessage = UnsignedGossipMessageImport;

impl UnsignedGossipMessage {
	#[allow(unused)]
	pub(crate) fn to_native(&self) -> nativeUnsignedGossipMessage {
		match self {
			UnsignedGossipMessage::ChannelAnnouncement (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				nativeUnsignedGossipMessage::ChannelAnnouncement (
					*unsafe { Box::from_raw(a_nonref.take_inner()) },
				)
			},
			UnsignedGossipMessage::ChannelUpdate (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				nativeUnsignedGossipMessage::ChannelUpdate (
					*unsafe { Box::from_raw(a_nonref.take_inner()) },
				)
			},
			UnsignedGossipMessage::NodeAnnouncement (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				nativeUnsignedGossipMessage::NodeAnnouncement (
					*unsafe { Box::from_raw(a_nonref.take_inner()) },
				)
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_native(self) -> nativeUnsignedGossipMessage {
		match self {
			UnsignedGossipMessage::ChannelAnnouncement (mut a, ) => {
				nativeUnsignedGossipMessage::ChannelAnnouncement (
					*unsafe { Box::from_raw(a.take_inner()) },
				)
			},
			UnsignedGossipMessage::ChannelUpdate (mut a, ) => {
				nativeUnsignedGossipMessage::ChannelUpdate (
					*unsafe { Box::from_raw(a.take_inner()) },
				)
			},
			UnsignedGossipMessage::NodeAnnouncement (mut a, ) => {
				nativeUnsignedGossipMessage::NodeAnnouncement (
					*unsafe { Box::from_raw(a.take_inner()) },
				)
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_native(native: &nativeUnsignedGossipMessage) -> Self {
		match native {
			nativeUnsignedGossipMessage::ChannelAnnouncement (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				UnsignedGossipMessage::ChannelAnnouncement (
					crate::lightning::ln::msgs::UnsignedChannelAnnouncement { inner: ObjOps::heap_alloc(a_nonref), is_owned: true },
				)
			},
			nativeUnsignedGossipMessage::ChannelUpdate (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				UnsignedGossipMessage::ChannelUpdate (
					crate::lightning::ln::msgs::UnsignedChannelUpdate { inner: ObjOps::heap_alloc(a_nonref), is_owned: true },
				)
			},
			nativeUnsignedGossipMessage::NodeAnnouncement (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				UnsignedGossipMessage::NodeAnnouncement (
					crate::lightning::ln::msgs::UnsignedNodeAnnouncement { inner: ObjOps::heap_alloc(a_nonref), is_owned: true },
				)
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn native_into(native: nativeUnsignedGossipMessage) -> Self {
		match native {
			nativeUnsignedGossipMessage::ChannelAnnouncement (mut a, ) => {
				UnsignedGossipMessage::ChannelAnnouncement (
					crate::lightning::ln::msgs::UnsignedChannelAnnouncement { inner: ObjOps::heap_alloc(a), is_owned: true },
				)
			},
			nativeUnsignedGossipMessage::ChannelUpdate (mut a, ) => {
				UnsignedGossipMessage::ChannelUpdate (
					crate::lightning::ln::msgs::UnsignedChannelUpdate { inner: ObjOps::heap_alloc(a), is_owned: true },
				)
			},
			nativeUnsignedGossipMessage::NodeAnnouncement (mut a, ) => {
				UnsignedGossipMessage::NodeAnnouncement (
					crate::lightning::ln::msgs::UnsignedNodeAnnouncement { inner: ObjOps::heap_alloc(a), is_owned: true },
				)
			},
		}
	}
}
/// Frees any resources used by the UnsignedGossipMessage
#[no_mangle]
pub extern "C" fn UnsignedGossipMessage_free(this_ptr: UnsignedGossipMessage) { }
/// Creates a copy of the UnsignedGossipMessage
#[no_mangle]
pub extern "C" fn UnsignedGossipMessage_clone(orig: &UnsignedGossipMessage) -> UnsignedGossipMessage {
	orig.clone()
}
#[no_mangle]
/// Utility method to constructs a new ChannelAnnouncement-variant UnsignedGossipMessage
pub extern "C" fn UnsignedGossipMessage_channel_announcement(a: crate::lightning::ln::msgs::UnsignedChannelAnnouncement) -> UnsignedGossipMessage {
	UnsignedGossipMessage::ChannelAnnouncement(a, )
}
#[no_mangle]
/// Utility method to constructs a new ChannelUpdate-variant UnsignedGossipMessage
pub extern "C" fn UnsignedGossipMessage_channel_update(a: crate::lightning::ln::msgs::UnsignedChannelUpdate) -> UnsignedGossipMessage {
	UnsignedGossipMessage::ChannelUpdate(a, )
}
#[no_mangle]
/// Utility method to constructs a new NodeAnnouncement-variant UnsignedGossipMessage
pub extern "C" fn UnsignedGossipMessage_node_announcement(a: crate::lightning::ln::msgs::UnsignedNodeAnnouncement) -> UnsignedGossipMessage {
	UnsignedGossipMessage::NodeAnnouncement(a, )
}
#[no_mangle]
/// Serialize the UnsignedGossipMessage object into a byte array which can be read by UnsignedGossipMessage_read
pub extern "C" fn UnsignedGossipMessage_write(obj: &crate::lightning::ln::msgs::UnsignedGossipMessage) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(&unsafe { &*obj }.to_native())
}

use lightning::ln::msgs::UnsignedNodeAnnouncement as nativeUnsignedNodeAnnouncementImport;
pub(crate) type nativeUnsignedNodeAnnouncement = nativeUnsignedNodeAnnouncementImport;

/// The unsigned part of a [`node_announcement`] message.
///
/// [`node_announcement`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-node_announcement-message
#[must_use]
#[repr(C)]
pub struct UnsignedNodeAnnouncement {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeUnsignedNodeAnnouncement,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for UnsignedNodeAnnouncement {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeUnsignedNodeAnnouncement>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the UnsignedNodeAnnouncement, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_free(this_obj: UnsignedNodeAnnouncement) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UnsignedNodeAnnouncement_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeUnsignedNodeAnnouncement) };
}
#[allow(unused)]
impl UnsignedNodeAnnouncement {
	pub(crate) fn get_native_ref(&self) -> &'static nativeUnsignedNodeAnnouncement {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeUnsignedNodeAnnouncement {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeUnsignedNodeAnnouncement {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The advertised features
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_get_features(this_ptr: &UnsignedNodeAnnouncement) -> crate::lightning::ln::features::NodeFeatures {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().features;
	crate::lightning::ln::features::NodeFeatures { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::features::NodeFeatures<>) as *mut _) }, is_owned: false }
}
/// The advertised features
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_set_features(this_ptr: &mut UnsignedNodeAnnouncement, mut val: crate::lightning::ln::features::NodeFeatures) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.features = *unsafe { Box::from_raw(val.take_inner()) };
}
/// A strictly monotonic announcement counter, with gaps allowed
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_get_timestamp(this_ptr: &UnsignedNodeAnnouncement) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().timestamp;
	*inner_val
}
/// A strictly monotonic announcement counter, with gaps allowed
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_set_timestamp(this_ptr: &mut UnsignedNodeAnnouncement, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.timestamp = val;
}
/// The `node_id` this announcement originated from (don't rebroadcast the `node_announcement` back
/// to this node).
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_get_node_id(this_ptr: &UnsignedNodeAnnouncement) -> crate::lightning::routing::gossip::NodeId {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().node_id;
	crate::lightning::routing::gossip::NodeId { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::routing::gossip::NodeId<>) as *mut _) }, is_owned: false }
}
/// The `node_id` this announcement originated from (don't rebroadcast the `node_announcement` back
/// to this node).
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_set_node_id(this_ptr: &mut UnsignedNodeAnnouncement, mut val: crate::lightning::routing::gossip::NodeId) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.node_id = *unsafe { Box::from_raw(val.take_inner()) };
}
/// An RGB color for UI purposes
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_get_rgb(this_ptr: &UnsignedNodeAnnouncement) -> *const [u8; 3] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().rgb;
	inner_val
}
/// An RGB color for UI purposes
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_set_rgb(this_ptr: &mut UnsignedNodeAnnouncement, mut val: crate::c_types::ThreeBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.rgb = val.data;
}
/// An alias, for UI purposes.
///
/// This should be sanitized before use. There is no guarantee of uniqueness.
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_get_alias(this_ptr: &UnsignedNodeAnnouncement) -> crate::lightning::routing::gossip::NodeAlias {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().alias;
	crate::lightning::routing::gossip::NodeAlias { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::routing::gossip::NodeAlias<>) as *mut _) }, is_owned: false }
}
/// An alias, for UI purposes.
///
/// This should be sanitized before use. There is no guarantee of uniqueness.
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_set_alias(this_ptr: &mut UnsignedNodeAnnouncement, mut val: crate::lightning::routing::gossip::NodeAlias) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.alias = *unsafe { Box::from_raw(val.take_inner()) };
}
/// List of addresses on which this node is reachable
///
/// Returns a copy of the field.
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_get_addresses(this_ptr: &UnsignedNodeAnnouncement) -> crate::c_types::derived::CVec_NetAddressZ {
	let mut inner_val = this_ptr.get_native_mut_ref().addresses.clone();
	let mut local_inner_val = Vec::new(); for mut item in inner_val.drain(..) { local_inner_val.push( { crate::lightning::ln::msgs::NetAddress::native_into(item) }); };
	local_inner_val.into()
}
/// List of addresses on which this node is reachable
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_set_addresses(this_ptr: &mut UnsignedNodeAnnouncement, mut val: crate::c_types::derived::CVec_NetAddressZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { item.into_native() }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.addresses = local_val;
}
impl Clone for UnsignedNodeAnnouncement {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeUnsignedNodeAnnouncement>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UnsignedNodeAnnouncement_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeUnsignedNodeAnnouncement)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the UnsignedNodeAnnouncement
pub extern "C" fn UnsignedNodeAnnouncement_clone(orig: &UnsignedNodeAnnouncement) -> UnsignedNodeAnnouncement {
	orig.clone()
}
/// Checks if two UnsignedNodeAnnouncements contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_eq(a: &UnsignedNodeAnnouncement, b: &UnsignedNodeAnnouncement) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::NodeAnnouncement as nativeNodeAnnouncementImport;
pub(crate) type nativeNodeAnnouncement = nativeNodeAnnouncementImport;

/// A [`node_announcement`] message to be sent to or received from a peer.
///
/// [`node_announcement`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-node_announcement-message
#[must_use]
#[repr(C)]
pub struct NodeAnnouncement {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeNodeAnnouncement,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for NodeAnnouncement {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeNodeAnnouncement>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the NodeAnnouncement, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn NodeAnnouncement_free(this_obj: NodeAnnouncement) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn NodeAnnouncement_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeNodeAnnouncement) };
}
#[allow(unused)]
impl NodeAnnouncement {
	pub(crate) fn get_native_ref(&self) -> &'static nativeNodeAnnouncement {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeNodeAnnouncement {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeNodeAnnouncement {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The signature by the node key
#[no_mangle]
pub extern "C" fn NodeAnnouncement_get_signature(this_ptr: &NodeAnnouncement) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().signature;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// The signature by the node key
#[no_mangle]
pub extern "C" fn NodeAnnouncement_set_signature(this_ptr: &mut NodeAnnouncement, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.signature = val.into_rust();
}
/// The actual content of the announcement
#[no_mangle]
pub extern "C" fn NodeAnnouncement_get_contents(this_ptr: &NodeAnnouncement) -> crate::lightning::ln::msgs::UnsignedNodeAnnouncement {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().contents;
	crate::lightning::ln::msgs::UnsignedNodeAnnouncement { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::msgs::UnsignedNodeAnnouncement<>) as *mut _) }, is_owned: false }
}
/// The actual content of the announcement
#[no_mangle]
pub extern "C" fn NodeAnnouncement_set_contents(this_ptr: &mut NodeAnnouncement, mut val: crate::lightning::ln::msgs::UnsignedNodeAnnouncement) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.contents = *unsafe { Box::from_raw(val.take_inner()) };
}
/// Constructs a new NodeAnnouncement given each field
#[must_use]
#[no_mangle]
pub extern "C" fn NodeAnnouncement_new(mut signature_arg: crate::c_types::Signature, mut contents_arg: crate::lightning::ln::msgs::UnsignedNodeAnnouncement) -> NodeAnnouncement {
	NodeAnnouncement { inner: ObjOps::heap_alloc(nativeNodeAnnouncement {
		signature: signature_arg.into_rust(),
		contents: *unsafe { Box::from_raw(contents_arg.take_inner()) },
	}), is_owned: true }
}
impl Clone for NodeAnnouncement {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeNodeAnnouncement>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn NodeAnnouncement_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeNodeAnnouncement)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the NodeAnnouncement
pub extern "C" fn NodeAnnouncement_clone(orig: &NodeAnnouncement) -> NodeAnnouncement {
	orig.clone()
}
/// Checks if two NodeAnnouncements contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn NodeAnnouncement_eq(a: &NodeAnnouncement, b: &NodeAnnouncement) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::UnsignedChannelAnnouncement as nativeUnsignedChannelAnnouncementImport;
pub(crate) type nativeUnsignedChannelAnnouncement = nativeUnsignedChannelAnnouncementImport;

/// The unsigned part of a [`channel_announcement`] message.
///
/// [`channel_announcement`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-channel_announcement-message
#[must_use]
#[repr(C)]
pub struct UnsignedChannelAnnouncement {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeUnsignedChannelAnnouncement,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for UnsignedChannelAnnouncement {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeUnsignedChannelAnnouncement>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the UnsignedChannelAnnouncement, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_free(this_obj: UnsignedChannelAnnouncement) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UnsignedChannelAnnouncement_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeUnsignedChannelAnnouncement) };
}
#[allow(unused)]
impl UnsignedChannelAnnouncement {
	pub(crate) fn get_native_ref(&self) -> &'static nativeUnsignedChannelAnnouncement {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeUnsignedChannelAnnouncement {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeUnsignedChannelAnnouncement {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The advertised channel features
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_features(this_ptr: &UnsignedChannelAnnouncement) -> crate::lightning::ln::features::ChannelFeatures {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().features;
	crate::lightning::ln::features::ChannelFeatures { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::features::ChannelFeatures<>) as *mut _) }, is_owned: false }
}
/// The advertised channel features
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_features(this_ptr: &mut UnsignedChannelAnnouncement, mut val: crate::lightning::ln::features::ChannelFeatures) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.features = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The genesis hash of the blockchain where the channel is to be opened
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_chain_hash(this_ptr: &UnsignedChannelAnnouncement) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().chain_hash;
	inner_val.as_inner()
}
/// The genesis hash of the blockchain where the channel is to be opened
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_chain_hash(this_ptr: &mut UnsignedChannelAnnouncement, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.chain_hash = ::bitcoin::hash_types::BlockHash::from_slice(&val.data[..]).unwrap();
}
/// The short channel ID
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_short_channel_id(this_ptr: &UnsignedChannelAnnouncement) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().short_channel_id;
	*inner_val
}
/// The short channel ID
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_short_channel_id(this_ptr: &mut UnsignedChannelAnnouncement, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.short_channel_id = val;
}
/// One of the two `node_id`s which are endpoints of this channel
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_node_id_1(this_ptr: &UnsignedChannelAnnouncement) -> crate::lightning::routing::gossip::NodeId {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().node_id_1;
	crate::lightning::routing::gossip::NodeId { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::routing::gossip::NodeId<>) as *mut _) }, is_owned: false }
}
/// One of the two `node_id`s which are endpoints of this channel
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_node_id_1(this_ptr: &mut UnsignedChannelAnnouncement, mut val: crate::lightning::routing::gossip::NodeId) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.node_id_1 = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The other of the two `node_id`s which are endpoints of this channel
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_node_id_2(this_ptr: &UnsignedChannelAnnouncement) -> crate::lightning::routing::gossip::NodeId {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().node_id_2;
	crate::lightning::routing::gossip::NodeId { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::routing::gossip::NodeId<>) as *mut _) }, is_owned: false }
}
/// The other of the two `node_id`s which are endpoints of this channel
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_node_id_2(this_ptr: &mut UnsignedChannelAnnouncement, mut val: crate::lightning::routing::gossip::NodeId) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.node_id_2 = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The funding key for the first node
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_bitcoin_key_1(this_ptr: &UnsignedChannelAnnouncement) -> crate::lightning::routing::gossip::NodeId {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().bitcoin_key_1;
	crate::lightning::routing::gossip::NodeId { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::routing::gossip::NodeId<>) as *mut _) }, is_owned: false }
}
/// The funding key for the first node
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_bitcoin_key_1(this_ptr: &mut UnsignedChannelAnnouncement, mut val: crate::lightning::routing::gossip::NodeId) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.bitcoin_key_1 = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The funding key for the second node
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_bitcoin_key_2(this_ptr: &UnsignedChannelAnnouncement) -> crate::lightning::routing::gossip::NodeId {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().bitcoin_key_2;
	crate::lightning::routing::gossip::NodeId { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::routing::gossip::NodeId<>) as *mut _) }, is_owned: false }
}
/// The funding key for the second node
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_bitcoin_key_2(this_ptr: &mut UnsignedChannelAnnouncement, mut val: crate::lightning::routing::gossip::NodeId) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.bitcoin_key_2 = *unsafe { Box::from_raw(val.take_inner()) };
}
impl Clone for UnsignedChannelAnnouncement {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeUnsignedChannelAnnouncement>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UnsignedChannelAnnouncement_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeUnsignedChannelAnnouncement)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the UnsignedChannelAnnouncement
pub extern "C" fn UnsignedChannelAnnouncement_clone(orig: &UnsignedChannelAnnouncement) -> UnsignedChannelAnnouncement {
	orig.clone()
}
/// Checks if two UnsignedChannelAnnouncements contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_eq(a: &UnsignedChannelAnnouncement, b: &UnsignedChannelAnnouncement) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ChannelAnnouncement as nativeChannelAnnouncementImport;
pub(crate) type nativeChannelAnnouncement = nativeChannelAnnouncementImport;

/// A [`channel_announcement`] message to be sent to or received from a peer.
///
/// [`channel_announcement`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-channel_announcement-message
#[must_use]
#[repr(C)]
pub struct ChannelAnnouncement {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeChannelAnnouncement,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ChannelAnnouncement {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeChannelAnnouncement>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ChannelAnnouncement, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_free(this_obj: ChannelAnnouncement) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ChannelAnnouncement_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeChannelAnnouncement) };
}
#[allow(unused)]
impl ChannelAnnouncement {
	pub(crate) fn get_native_ref(&self) -> &'static nativeChannelAnnouncement {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeChannelAnnouncement {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeChannelAnnouncement {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// Authentication of the announcement by the first public node
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_get_node_signature_1(this_ptr: &ChannelAnnouncement) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().node_signature_1;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// Authentication of the announcement by the first public node
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_set_node_signature_1(this_ptr: &mut ChannelAnnouncement, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.node_signature_1 = val.into_rust();
}
/// Authentication of the announcement by the second public node
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_get_node_signature_2(this_ptr: &ChannelAnnouncement) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().node_signature_2;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// Authentication of the announcement by the second public node
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_set_node_signature_2(this_ptr: &mut ChannelAnnouncement, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.node_signature_2 = val.into_rust();
}
/// Proof of funding UTXO ownership by the first public node
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_get_bitcoin_signature_1(this_ptr: &ChannelAnnouncement) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().bitcoin_signature_1;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// Proof of funding UTXO ownership by the first public node
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_set_bitcoin_signature_1(this_ptr: &mut ChannelAnnouncement, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.bitcoin_signature_1 = val.into_rust();
}
/// Proof of funding UTXO ownership by the second public node
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_get_bitcoin_signature_2(this_ptr: &ChannelAnnouncement) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().bitcoin_signature_2;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// Proof of funding UTXO ownership by the second public node
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_set_bitcoin_signature_2(this_ptr: &mut ChannelAnnouncement, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.bitcoin_signature_2 = val.into_rust();
}
/// The actual announcement
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_get_contents(this_ptr: &ChannelAnnouncement) -> crate::lightning::ln::msgs::UnsignedChannelAnnouncement {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().contents;
	crate::lightning::ln::msgs::UnsignedChannelAnnouncement { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::msgs::UnsignedChannelAnnouncement<>) as *mut _) }, is_owned: false }
}
/// The actual announcement
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_set_contents(this_ptr: &mut ChannelAnnouncement, mut val: crate::lightning::ln::msgs::UnsignedChannelAnnouncement) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.contents = *unsafe { Box::from_raw(val.take_inner()) };
}
/// Constructs a new ChannelAnnouncement given each field
#[must_use]
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_new(mut node_signature_1_arg: crate::c_types::Signature, mut node_signature_2_arg: crate::c_types::Signature, mut bitcoin_signature_1_arg: crate::c_types::Signature, mut bitcoin_signature_2_arg: crate::c_types::Signature, mut contents_arg: crate::lightning::ln::msgs::UnsignedChannelAnnouncement) -> ChannelAnnouncement {
	ChannelAnnouncement { inner: ObjOps::heap_alloc(nativeChannelAnnouncement {
		node_signature_1: node_signature_1_arg.into_rust(),
		node_signature_2: node_signature_2_arg.into_rust(),
		bitcoin_signature_1: bitcoin_signature_1_arg.into_rust(),
		bitcoin_signature_2: bitcoin_signature_2_arg.into_rust(),
		contents: *unsafe { Box::from_raw(contents_arg.take_inner()) },
	}), is_owned: true }
}
impl Clone for ChannelAnnouncement {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeChannelAnnouncement>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ChannelAnnouncement_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeChannelAnnouncement)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ChannelAnnouncement
pub extern "C" fn ChannelAnnouncement_clone(orig: &ChannelAnnouncement) -> ChannelAnnouncement {
	orig.clone()
}
/// Checks if two ChannelAnnouncements contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_eq(a: &ChannelAnnouncement, b: &ChannelAnnouncement) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::UnsignedChannelUpdate as nativeUnsignedChannelUpdateImport;
pub(crate) type nativeUnsignedChannelUpdate = nativeUnsignedChannelUpdateImport;

/// The unsigned part of a [`channel_update`] message.
///
/// [`channel_update`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-channel_update-message
#[must_use]
#[repr(C)]
pub struct UnsignedChannelUpdate {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeUnsignedChannelUpdate,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for UnsignedChannelUpdate {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeUnsignedChannelUpdate>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the UnsignedChannelUpdate, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_free(this_obj: UnsignedChannelUpdate) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UnsignedChannelUpdate_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeUnsignedChannelUpdate) };
}
#[allow(unused)]
impl UnsignedChannelUpdate {
	pub(crate) fn get_native_ref(&self) -> &'static nativeUnsignedChannelUpdate {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeUnsignedChannelUpdate {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeUnsignedChannelUpdate {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The genesis hash of the blockchain where the channel is to be opened
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_chain_hash(this_ptr: &UnsignedChannelUpdate) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().chain_hash;
	inner_val.as_inner()
}
/// The genesis hash of the blockchain where the channel is to be opened
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_chain_hash(this_ptr: &mut UnsignedChannelUpdate, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.chain_hash = ::bitcoin::hash_types::BlockHash::from_slice(&val.data[..]).unwrap();
}
/// The short channel ID
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_short_channel_id(this_ptr: &UnsignedChannelUpdate) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().short_channel_id;
	*inner_val
}
/// The short channel ID
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_short_channel_id(this_ptr: &mut UnsignedChannelUpdate, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.short_channel_id = val;
}
/// A strictly monotonic announcement counter, with gaps allowed, specific to this channel
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_timestamp(this_ptr: &UnsignedChannelUpdate) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().timestamp;
	*inner_val
}
/// A strictly monotonic announcement counter, with gaps allowed, specific to this channel
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_timestamp(this_ptr: &mut UnsignedChannelUpdate, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.timestamp = val;
}
/// Channel flags
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_flags(this_ptr: &UnsignedChannelUpdate) -> u8 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().flags;
	*inner_val
}
/// Channel flags
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_flags(this_ptr: &mut UnsignedChannelUpdate, mut val: u8) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.flags = val;
}
/// The number of blocks such that if:
/// `incoming_htlc.cltv_expiry < outgoing_htlc.cltv_expiry + cltv_expiry_delta`
/// then we need to fail the HTLC backwards. When forwarding an HTLC, `cltv_expiry_delta` determines
/// the outgoing HTLC's minimum `cltv_expiry` value -- so, if an incoming HTLC comes in with a
/// `cltv_expiry` of 100000, and the node we're forwarding to has a `cltv_expiry_delta` value of 10,
/// then we'll check that the outgoing HTLC's `cltv_expiry` value is at least 100010 before
/// forwarding. Note that the HTLC sender is the one who originally sets this value when
/// constructing the route.
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_cltv_expiry_delta(this_ptr: &UnsignedChannelUpdate) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().cltv_expiry_delta;
	*inner_val
}
/// The number of blocks such that if:
/// `incoming_htlc.cltv_expiry < outgoing_htlc.cltv_expiry + cltv_expiry_delta`
/// then we need to fail the HTLC backwards. When forwarding an HTLC, `cltv_expiry_delta` determines
/// the outgoing HTLC's minimum `cltv_expiry` value -- so, if an incoming HTLC comes in with a
/// `cltv_expiry` of 100000, and the node we're forwarding to has a `cltv_expiry_delta` value of 10,
/// then we'll check that the outgoing HTLC's `cltv_expiry` value is at least 100010 before
/// forwarding. Note that the HTLC sender is the one who originally sets this value when
/// constructing the route.
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_cltv_expiry_delta(this_ptr: &mut UnsignedChannelUpdate, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.cltv_expiry_delta = val;
}
/// The minimum HTLC size incoming to sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_htlc_minimum_msat(this_ptr: &UnsignedChannelUpdate) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_minimum_msat;
	*inner_val
}
/// The minimum HTLC size incoming to sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_htlc_minimum_msat(this_ptr: &mut UnsignedChannelUpdate, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_minimum_msat = val;
}
/// The maximum HTLC value incoming to sender, in milli-satoshi.
///
/// This used to be optional.
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_htlc_maximum_msat(this_ptr: &UnsignedChannelUpdate) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_maximum_msat;
	*inner_val
}
/// The maximum HTLC value incoming to sender, in milli-satoshi.
///
/// This used to be optional.
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_htlc_maximum_msat(this_ptr: &mut UnsignedChannelUpdate, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_maximum_msat = val;
}
/// The base HTLC fee charged by sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_fee_base_msat(this_ptr: &UnsignedChannelUpdate) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().fee_base_msat;
	*inner_val
}
/// The base HTLC fee charged by sender, in milli-satoshi
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_fee_base_msat(this_ptr: &mut UnsignedChannelUpdate, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.fee_base_msat = val;
}
/// The amount to fee multiplier, in micro-satoshi
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_fee_proportional_millionths(this_ptr: &UnsignedChannelUpdate) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().fee_proportional_millionths;
	*inner_val
}
/// The amount to fee multiplier, in micro-satoshi
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_fee_proportional_millionths(this_ptr: &mut UnsignedChannelUpdate, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.fee_proportional_millionths = val;
}
/// Excess data which was signed as a part of the message which we do not (yet) understand how
/// to decode.
///
/// This is stored to ensure forward-compatibility as new fields are added to the lightning gossip protocol.
///
/// Returns a copy of the field.
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_get_excess_data(this_ptr: &UnsignedChannelUpdate) -> crate::c_types::derived::CVec_u8Z {
	let mut inner_val = this_ptr.get_native_mut_ref().excess_data.clone();
	let mut local_inner_val = Vec::new(); for mut item in inner_val.drain(..) { local_inner_val.push( { item }); };
	local_inner_val.into()
}
/// Excess data which was signed as a part of the message which we do not (yet) understand how
/// to decode.
///
/// This is stored to ensure forward-compatibility as new fields are added to the lightning gossip protocol.
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_set_excess_data(this_ptr: &mut UnsignedChannelUpdate, mut val: crate::c_types::derived::CVec_u8Z) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { item }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.excess_data = local_val;
}
/// Constructs a new UnsignedChannelUpdate given each field
#[must_use]
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_new(mut chain_hash_arg: crate::c_types::ThirtyTwoBytes, mut short_channel_id_arg: u64, mut timestamp_arg: u32, mut flags_arg: u8, mut cltv_expiry_delta_arg: u16, mut htlc_minimum_msat_arg: u64, mut htlc_maximum_msat_arg: u64, mut fee_base_msat_arg: u32, mut fee_proportional_millionths_arg: u32, mut excess_data_arg: crate::c_types::derived::CVec_u8Z) -> UnsignedChannelUpdate {
	let mut local_excess_data_arg = Vec::new(); for mut item in excess_data_arg.into_rust().drain(..) { local_excess_data_arg.push( { item }); };
	UnsignedChannelUpdate { inner: ObjOps::heap_alloc(nativeUnsignedChannelUpdate {
		chain_hash: ::bitcoin::hash_types::BlockHash::from_slice(&chain_hash_arg.data[..]).unwrap(),
		short_channel_id: short_channel_id_arg,
		timestamp: timestamp_arg,
		flags: flags_arg,
		cltv_expiry_delta: cltv_expiry_delta_arg,
		htlc_minimum_msat: htlc_minimum_msat_arg,
		htlc_maximum_msat: htlc_maximum_msat_arg,
		fee_base_msat: fee_base_msat_arg,
		fee_proportional_millionths: fee_proportional_millionths_arg,
		excess_data: local_excess_data_arg,
	}), is_owned: true }
}
impl Clone for UnsignedChannelUpdate {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeUnsignedChannelUpdate>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn UnsignedChannelUpdate_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeUnsignedChannelUpdate)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the UnsignedChannelUpdate
pub extern "C" fn UnsignedChannelUpdate_clone(orig: &UnsignedChannelUpdate) -> UnsignedChannelUpdate {
	orig.clone()
}
/// Checks if two UnsignedChannelUpdates contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn UnsignedChannelUpdate_eq(a: &UnsignedChannelUpdate, b: &UnsignedChannelUpdate) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ChannelUpdate as nativeChannelUpdateImport;
pub(crate) type nativeChannelUpdate = nativeChannelUpdateImport;

/// A [`channel_update`] message to be sent to or received from a peer.
///
/// [`channel_update`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-channel_update-message
#[must_use]
#[repr(C)]
pub struct ChannelUpdate {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeChannelUpdate,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ChannelUpdate {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeChannelUpdate>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ChannelUpdate, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ChannelUpdate_free(this_obj: ChannelUpdate) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ChannelUpdate_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeChannelUpdate) };
}
#[allow(unused)]
impl ChannelUpdate {
	pub(crate) fn get_native_ref(&self) -> &'static nativeChannelUpdate {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeChannelUpdate {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeChannelUpdate {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// A signature of the channel update
#[no_mangle]
pub extern "C" fn ChannelUpdate_get_signature(this_ptr: &ChannelUpdate) -> crate::c_types::Signature {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().signature;
	crate::c_types::Signature::from_rust(&inner_val)
}
/// A signature of the channel update
#[no_mangle]
pub extern "C" fn ChannelUpdate_set_signature(this_ptr: &mut ChannelUpdate, mut val: crate::c_types::Signature) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.signature = val.into_rust();
}
/// The actual channel update
#[no_mangle]
pub extern "C" fn ChannelUpdate_get_contents(this_ptr: &ChannelUpdate) -> crate::lightning::ln::msgs::UnsignedChannelUpdate {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().contents;
	crate::lightning::ln::msgs::UnsignedChannelUpdate { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::msgs::UnsignedChannelUpdate<>) as *mut _) }, is_owned: false }
}
/// The actual channel update
#[no_mangle]
pub extern "C" fn ChannelUpdate_set_contents(this_ptr: &mut ChannelUpdate, mut val: crate::lightning::ln::msgs::UnsignedChannelUpdate) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.contents = *unsafe { Box::from_raw(val.take_inner()) };
}
/// Constructs a new ChannelUpdate given each field
#[must_use]
#[no_mangle]
pub extern "C" fn ChannelUpdate_new(mut signature_arg: crate::c_types::Signature, mut contents_arg: crate::lightning::ln::msgs::UnsignedChannelUpdate) -> ChannelUpdate {
	ChannelUpdate { inner: ObjOps::heap_alloc(nativeChannelUpdate {
		signature: signature_arg.into_rust(),
		contents: *unsafe { Box::from_raw(contents_arg.take_inner()) },
	}), is_owned: true }
}
impl Clone for ChannelUpdate {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeChannelUpdate>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ChannelUpdate_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeChannelUpdate)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ChannelUpdate
pub extern "C" fn ChannelUpdate_clone(orig: &ChannelUpdate) -> ChannelUpdate {
	orig.clone()
}
/// Checks if two ChannelUpdates contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ChannelUpdate_eq(a: &ChannelUpdate, b: &ChannelUpdate) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::QueryChannelRange as nativeQueryChannelRangeImport;
pub(crate) type nativeQueryChannelRange = nativeQueryChannelRangeImport;

/// A [`query_channel_range`] message is used to query a peer for channel
/// UTXOs in a range of blocks. The recipient of a query makes a best
/// effort to reply to the query using one or more [`ReplyChannelRange`]
/// messages.
///
/// [`query_channel_range`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-query_channel_range-and-reply_channel_range-messages
#[must_use]
#[repr(C)]
pub struct QueryChannelRange {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeQueryChannelRange,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for QueryChannelRange {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeQueryChannelRange>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the QueryChannelRange, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn QueryChannelRange_free(this_obj: QueryChannelRange) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn QueryChannelRange_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeQueryChannelRange) };
}
#[allow(unused)]
impl QueryChannelRange {
	pub(crate) fn get_native_ref(&self) -> &'static nativeQueryChannelRange {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeQueryChannelRange {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeQueryChannelRange {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The genesis hash of the blockchain being queried
#[no_mangle]
pub extern "C" fn QueryChannelRange_get_chain_hash(this_ptr: &QueryChannelRange) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().chain_hash;
	inner_val.as_inner()
}
/// The genesis hash of the blockchain being queried
#[no_mangle]
pub extern "C" fn QueryChannelRange_set_chain_hash(this_ptr: &mut QueryChannelRange, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.chain_hash = ::bitcoin::hash_types::BlockHash::from_slice(&val.data[..]).unwrap();
}
/// The height of the first block for the channel UTXOs being queried
#[no_mangle]
pub extern "C" fn QueryChannelRange_get_first_blocknum(this_ptr: &QueryChannelRange) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().first_blocknum;
	*inner_val
}
/// The height of the first block for the channel UTXOs being queried
#[no_mangle]
pub extern "C" fn QueryChannelRange_set_first_blocknum(this_ptr: &mut QueryChannelRange, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.first_blocknum = val;
}
/// The number of blocks to include in the query results
#[no_mangle]
pub extern "C" fn QueryChannelRange_get_number_of_blocks(this_ptr: &QueryChannelRange) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().number_of_blocks;
	*inner_val
}
/// The number of blocks to include in the query results
#[no_mangle]
pub extern "C" fn QueryChannelRange_set_number_of_blocks(this_ptr: &mut QueryChannelRange, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.number_of_blocks = val;
}
/// Constructs a new QueryChannelRange given each field
#[must_use]
#[no_mangle]
pub extern "C" fn QueryChannelRange_new(mut chain_hash_arg: crate::c_types::ThirtyTwoBytes, mut first_blocknum_arg: u32, mut number_of_blocks_arg: u32) -> QueryChannelRange {
	QueryChannelRange { inner: ObjOps::heap_alloc(nativeQueryChannelRange {
		chain_hash: ::bitcoin::hash_types::BlockHash::from_slice(&chain_hash_arg.data[..]).unwrap(),
		first_blocknum: first_blocknum_arg,
		number_of_blocks: number_of_blocks_arg,
	}), is_owned: true }
}
impl Clone for QueryChannelRange {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeQueryChannelRange>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn QueryChannelRange_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeQueryChannelRange)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the QueryChannelRange
pub extern "C" fn QueryChannelRange_clone(orig: &QueryChannelRange) -> QueryChannelRange {
	orig.clone()
}
/// Checks if two QueryChannelRanges contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn QueryChannelRange_eq(a: &QueryChannelRange, b: &QueryChannelRange) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ReplyChannelRange as nativeReplyChannelRangeImport;
pub(crate) type nativeReplyChannelRange = nativeReplyChannelRangeImport;

/// A [`reply_channel_range`] message is a reply to a [`QueryChannelRange`]
/// message.
///
/// Multiple `reply_channel_range` messages can be sent in reply
/// to a single [`QueryChannelRange`] message. The query recipient makes a
/// best effort to respond based on their local network view which may
/// not be a perfect view of the network. The `short_channel_id`s in the
/// reply are encoded. We only support `encoding_type=0` uncompressed
/// serialization and do not support `encoding_type=1` zlib serialization.
///
/// [`reply_channel_range`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-query_channel_range-and-reply_channel_range-messages
#[must_use]
#[repr(C)]
pub struct ReplyChannelRange {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeReplyChannelRange,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ReplyChannelRange {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeReplyChannelRange>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ReplyChannelRange, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ReplyChannelRange_free(this_obj: ReplyChannelRange) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ReplyChannelRange_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeReplyChannelRange) };
}
#[allow(unused)]
impl ReplyChannelRange {
	pub(crate) fn get_native_ref(&self) -> &'static nativeReplyChannelRange {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeReplyChannelRange {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeReplyChannelRange {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The genesis hash of the blockchain being queried
#[no_mangle]
pub extern "C" fn ReplyChannelRange_get_chain_hash(this_ptr: &ReplyChannelRange) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().chain_hash;
	inner_val.as_inner()
}
/// The genesis hash of the blockchain being queried
#[no_mangle]
pub extern "C" fn ReplyChannelRange_set_chain_hash(this_ptr: &mut ReplyChannelRange, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.chain_hash = ::bitcoin::hash_types::BlockHash::from_slice(&val.data[..]).unwrap();
}
/// The height of the first block in the range of the reply
#[no_mangle]
pub extern "C" fn ReplyChannelRange_get_first_blocknum(this_ptr: &ReplyChannelRange) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().first_blocknum;
	*inner_val
}
/// The height of the first block in the range of the reply
#[no_mangle]
pub extern "C" fn ReplyChannelRange_set_first_blocknum(this_ptr: &mut ReplyChannelRange, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.first_blocknum = val;
}
/// The number of blocks included in the range of the reply
#[no_mangle]
pub extern "C" fn ReplyChannelRange_get_number_of_blocks(this_ptr: &ReplyChannelRange) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().number_of_blocks;
	*inner_val
}
/// The number of blocks included in the range of the reply
#[no_mangle]
pub extern "C" fn ReplyChannelRange_set_number_of_blocks(this_ptr: &mut ReplyChannelRange, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.number_of_blocks = val;
}
/// True when this is the final reply for a query
#[no_mangle]
pub extern "C" fn ReplyChannelRange_get_sync_complete(this_ptr: &ReplyChannelRange) -> bool {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().sync_complete;
	*inner_val
}
/// True when this is the final reply for a query
#[no_mangle]
pub extern "C" fn ReplyChannelRange_set_sync_complete(this_ptr: &mut ReplyChannelRange, mut val: bool) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.sync_complete = val;
}
/// The `short_channel_id`s in the channel range
///
/// Returns a copy of the field.
#[no_mangle]
pub extern "C" fn ReplyChannelRange_get_short_channel_ids(this_ptr: &ReplyChannelRange) -> crate::c_types::derived::CVec_u64Z {
	let mut inner_val = this_ptr.get_native_mut_ref().short_channel_ids.clone();
	let mut local_inner_val = Vec::new(); for mut item in inner_val.drain(..) { local_inner_val.push( { item }); };
	local_inner_val.into()
}
/// The `short_channel_id`s in the channel range
#[no_mangle]
pub extern "C" fn ReplyChannelRange_set_short_channel_ids(this_ptr: &mut ReplyChannelRange, mut val: crate::c_types::derived::CVec_u64Z) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { item }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.short_channel_ids = local_val;
}
/// Constructs a new ReplyChannelRange given each field
#[must_use]
#[no_mangle]
pub extern "C" fn ReplyChannelRange_new(mut chain_hash_arg: crate::c_types::ThirtyTwoBytes, mut first_blocknum_arg: u32, mut number_of_blocks_arg: u32, mut sync_complete_arg: bool, mut short_channel_ids_arg: crate::c_types::derived::CVec_u64Z) -> ReplyChannelRange {
	let mut local_short_channel_ids_arg = Vec::new(); for mut item in short_channel_ids_arg.into_rust().drain(..) { local_short_channel_ids_arg.push( { item }); };
	ReplyChannelRange { inner: ObjOps::heap_alloc(nativeReplyChannelRange {
		chain_hash: ::bitcoin::hash_types::BlockHash::from_slice(&chain_hash_arg.data[..]).unwrap(),
		first_blocknum: first_blocknum_arg,
		number_of_blocks: number_of_blocks_arg,
		sync_complete: sync_complete_arg,
		short_channel_ids: local_short_channel_ids_arg,
	}), is_owned: true }
}
impl Clone for ReplyChannelRange {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeReplyChannelRange>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ReplyChannelRange_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeReplyChannelRange)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ReplyChannelRange
pub extern "C" fn ReplyChannelRange_clone(orig: &ReplyChannelRange) -> ReplyChannelRange {
	orig.clone()
}
/// Checks if two ReplyChannelRanges contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ReplyChannelRange_eq(a: &ReplyChannelRange, b: &ReplyChannelRange) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::QueryShortChannelIds as nativeQueryShortChannelIdsImport;
pub(crate) type nativeQueryShortChannelIds = nativeQueryShortChannelIdsImport;

/// A [`query_short_channel_ids`] message is used to query a peer for
/// routing gossip messages related to one or more `short_channel_id`s.
///
/// The query recipient will reply with the latest, if available,
/// [`ChannelAnnouncement`], [`ChannelUpdate`] and [`NodeAnnouncement`] messages
/// it maintains for the requested `short_channel_id`s followed by a
/// [`ReplyShortChannelIdsEnd`] message. The `short_channel_id`s sent in
/// this query are encoded. We only support `encoding_type=0` uncompressed
/// serialization and do not support `encoding_type=1` zlib serialization.
///
/// [`query_short_channel_ids`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-query_short_channel_idsreply_short_channel_ids_end-messages
#[must_use]
#[repr(C)]
pub struct QueryShortChannelIds {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeQueryShortChannelIds,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for QueryShortChannelIds {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeQueryShortChannelIds>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the QueryShortChannelIds, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn QueryShortChannelIds_free(this_obj: QueryShortChannelIds) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn QueryShortChannelIds_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeQueryShortChannelIds) };
}
#[allow(unused)]
impl QueryShortChannelIds {
	pub(crate) fn get_native_ref(&self) -> &'static nativeQueryShortChannelIds {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeQueryShortChannelIds {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeQueryShortChannelIds {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The genesis hash of the blockchain being queried
#[no_mangle]
pub extern "C" fn QueryShortChannelIds_get_chain_hash(this_ptr: &QueryShortChannelIds) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().chain_hash;
	inner_val.as_inner()
}
/// The genesis hash of the blockchain being queried
#[no_mangle]
pub extern "C" fn QueryShortChannelIds_set_chain_hash(this_ptr: &mut QueryShortChannelIds, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.chain_hash = ::bitcoin::hash_types::BlockHash::from_slice(&val.data[..]).unwrap();
}
/// The short_channel_ids that are being queried
///
/// Returns a copy of the field.
#[no_mangle]
pub extern "C" fn QueryShortChannelIds_get_short_channel_ids(this_ptr: &QueryShortChannelIds) -> crate::c_types::derived::CVec_u64Z {
	let mut inner_val = this_ptr.get_native_mut_ref().short_channel_ids.clone();
	let mut local_inner_val = Vec::new(); for mut item in inner_val.drain(..) { local_inner_val.push( { item }); };
	local_inner_val.into()
}
/// The short_channel_ids that are being queried
#[no_mangle]
pub extern "C" fn QueryShortChannelIds_set_short_channel_ids(this_ptr: &mut QueryShortChannelIds, mut val: crate::c_types::derived::CVec_u64Z) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { item }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.short_channel_ids = local_val;
}
/// Constructs a new QueryShortChannelIds given each field
#[must_use]
#[no_mangle]
pub extern "C" fn QueryShortChannelIds_new(mut chain_hash_arg: crate::c_types::ThirtyTwoBytes, mut short_channel_ids_arg: crate::c_types::derived::CVec_u64Z) -> QueryShortChannelIds {
	let mut local_short_channel_ids_arg = Vec::new(); for mut item in short_channel_ids_arg.into_rust().drain(..) { local_short_channel_ids_arg.push( { item }); };
	QueryShortChannelIds { inner: ObjOps::heap_alloc(nativeQueryShortChannelIds {
		chain_hash: ::bitcoin::hash_types::BlockHash::from_slice(&chain_hash_arg.data[..]).unwrap(),
		short_channel_ids: local_short_channel_ids_arg,
	}), is_owned: true }
}
impl Clone for QueryShortChannelIds {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeQueryShortChannelIds>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn QueryShortChannelIds_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeQueryShortChannelIds)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the QueryShortChannelIds
pub extern "C" fn QueryShortChannelIds_clone(orig: &QueryShortChannelIds) -> QueryShortChannelIds {
	orig.clone()
}
/// Checks if two QueryShortChannelIdss contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn QueryShortChannelIds_eq(a: &QueryShortChannelIds, b: &QueryShortChannelIds) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::ReplyShortChannelIdsEnd as nativeReplyShortChannelIdsEndImport;
pub(crate) type nativeReplyShortChannelIdsEnd = nativeReplyShortChannelIdsEndImport;

/// A [`reply_short_channel_ids_end`] message is sent as a reply to a
/// message. The query recipient makes a best
/// effort to respond based on their local network view which may not be
/// a perfect view of the network.
///
/// [`reply_short_channel_ids_end`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-query_short_channel_idsreply_short_channel_ids_end-messages
#[must_use]
#[repr(C)]
pub struct ReplyShortChannelIdsEnd {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeReplyShortChannelIdsEnd,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for ReplyShortChannelIdsEnd {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeReplyShortChannelIdsEnd>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the ReplyShortChannelIdsEnd, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn ReplyShortChannelIdsEnd_free(this_obj: ReplyShortChannelIdsEnd) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ReplyShortChannelIdsEnd_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeReplyShortChannelIdsEnd) };
}
#[allow(unused)]
impl ReplyShortChannelIdsEnd {
	pub(crate) fn get_native_ref(&self) -> &'static nativeReplyShortChannelIdsEnd {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeReplyShortChannelIdsEnd {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeReplyShortChannelIdsEnd {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The genesis hash of the blockchain that was queried
#[no_mangle]
pub extern "C" fn ReplyShortChannelIdsEnd_get_chain_hash(this_ptr: &ReplyShortChannelIdsEnd) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().chain_hash;
	inner_val.as_inner()
}
/// The genesis hash of the blockchain that was queried
#[no_mangle]
pub extern "C" fn ReplyShortChannelIdsEnd_set_chain_hash(this_ptr: &mut ReplyShortChannelIdsEnd, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.chain_hash = ::bitcoin::hash_types::BlockHash::from_slice(&val.data[..]).unwrap();
}
/// Indicates if the query recipient maintains up-to-date channel
/// information for the `chain_hash`
#[no_mangle]
pub extern "C" fn ReplyShortChannelIdsEnd_get_full_information(this_ptr: &ReplyShortChannelIdsEnd) -> bool {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().full_information;
	*inner_val
}
/// Indicates if the query recipient maintains up-to-date channel
/// information for the `chain_hash`
#[no_mangle]
pub extern "C" fn ReplyShortChannelIdsEnd_set_full_information(this_ptr: &mut ReplyShortChannelIdsEnd, mut val: bool) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.full_information = val;
}
/// Constructs a new ReplyShortChannelIdsEnd given each field
#[must_use]
#[no_mangle]
pub extern "C" fn ReplyShortChannelIdsEnd_new(mut chain_hash_arg: crate::c_types::ThirtyTwoBytes, mut full_information_arg: bool) -> ReplyShortChannelIdsEnd {
	ReplyShortChannelIdsEnd { inner: ObjOps::heap_alloc(nativeReplyShortChannelIdsEnd {
		chain_hash: ::bitcoin::hash_types::BlockHash::from_slice(&chain_hash_arg.data[..]).unwrap(),
		full_information: full_information_arg,
	}), is_owned: true }
}
impl Clone for ReplyShortChannelIdsEnd {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeReplyShortChannelIdsEnd>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn ReplyShortChannelIdsEnd_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeReplyShortChannelIdsEnd)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the ReplyShortChannelIdsEnd
pub extern "C" fn ReplyShortChannelIdsEnd_clone(orig: &ReplyShortChannelIdsEnd) -> ReplyShortChannelIdsEnd {
	orig.clone()
}
/// Checks if two ReplyShortChannelIdsEnds contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn ReplyShortChannelIdsEnd_eq(a: &ReplyShortChannelIdsEnd, b: &ReplyShortChannelIdsEnd) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}

use lightning::ln::msgs::GossipTimestampFilter as nativeGossipTimestampFilterImport;
pub(crate) type nativeGossipTimestampFilter = nativeGossipTimestampFilterImport;

/// A [`gossip_timestamp_filter`] message is used by a node to request
/// gossip relay for messages in the requested time range when the
/// `gossip_queries` feature has been negotiated.
///
/// [`gossip_timestamp_filter`]: https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-gossip_timestamp_filter-message
#[must_use]
#[repr(C)]
pub struct GossipTimestampFilter {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeGossipTimestampFilter,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for GossipTimestampFilter {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeGossipTimestampFilter>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the GossipTimestampFilter, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_free(this_obj: GossipTimestampFilter) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn GossipTimestampFilter_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeGossipTimestampFilter) };
}
#[allow(unused)]
impl GossipTimestampFilter {
	pub(crate) fn get_native_ref(&self) -> &'static nativeGossipTimestampFilter {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeGossipTimestampFilter {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeGossipTimestampFilter {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// The genesis hash of the blockchain for channel and node information
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_get_chain_hash(this_ptr: &GossipTimestampFilter) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().chain_hash;
	inner_val.as_inner()
}
/// The genesis hash of the blockchain for channel and node information
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_set_chain_hash(this_ptr: &mut GossipTimestampFilter, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.chain_hash = ::bitcoin::hash_types::BlockHash::from_slice(&val.data[..]).unwrap();
}
/// The starting unix timestamp
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_get_first_timestamp(this_ptr: &GossipTimestampFilter) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().first_timestamp;
	*inner_val
}
/// The starting unix timestamp
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_set_first_timestamp(this_ptr: &mut GossipTimestampFilter, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.first_timestamp = val;
}
/// The range of information in seconds
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_get_timestamp_range(this_ptr: &GossipTimestampFilter) -> u32 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().timestamp_range;
	*inner_val
}
/// The range of information in seconds
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_set_timestamp_range(this_ptr: &mut GossipTimestampFilter, mut val: u32) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.timestamp_range = val;
}
/// Constructs a new GossipTimestampFilter given each field
#[must_use]
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_new(mut chain_hash_arg: crate::c_types::ThirtyTwoBytes, mut first_timestamp_arg: u32, mut timestamp_range_arg: u32) -> GossipTimestampFilter {
	GossipTimestampFilter { inner: ObjOps::heap_alloc(nativeGossipTimestampFilter {
		chain_hash: ::bitcoin::hash_types::BlockHash::from_slice(&chain_hash_arg.data[..]).unwrap(),
		first_timestamp: first_timestamp_arg,
		timestamp_range: timestamp_range_arg,
	}), is_owned: true }
}
impl Clone for GossipTimestampFilter {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeGossipTimestampFilter>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn GossipTimestampFilter_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeGossipTimestampFilter)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the GossipTimestampFilter
pub extern "C" fn GossipTimestampFilter_clone(orig: &GossipTimestampFilter) -> GossipTimestampFilter {
	orig.clone()
}
/// Checks if two GossipTimestampFilters contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn GossipTimestampFilter_eq(a: &GossipTimestampFilter, b: &GossipTimestampFilter) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}
/// Used to put an error message in a [`LightningError`].
#[derive(Clone)]
#[must_use]
#[repr(C)]
pub enum ErrorAction {
	/// The peer took some action which made us think they were useless. Disconnect them.
	DisconnectPeer {
		/// An error message which we should make an effort to send before we disconnect.
		///
		/// Note that this (or a relevant inner pointer) may be NULL or all-0s to represent None
		msg: crate::lightning::ln::msgs::ErrorMessage,
	},
	/// The peer did something harmless that we weren't able to process, just log and ignore
	IgnoreError,
	/// The peer did something harmless that we weren't able to meaningfully process.
	/// If the error is logged, log it at the given level.
	IgnoreAndLog(
		crate::lightning::util::logger::Level),
	/// The peer provided us with a gossip message which we'd already seen. In most cases this
	/// should be ignored, but it may result in the message being forwarded if it is a duplicate of
	/// our own channel announcements.
	IgnoreDuplicateGossip,
	/// The peer did something incorrect. Tell them.
	SendErrorMessage {
		/// The message to send.
		msg: crate::lightning::ln::msgs::ErrorMessage,
	},
	/// The peer did something incorrect. Tell them without closing any channels.
	SendWarningMessage {
		/// The message to send.
		msg: crate::lightning::ln::msgs::WarningMessage,
		/// The peer may have done something harmless that we weren't able to meaningfully process,
		/// though we should still tell them about it.
		/// If this event is logged, log it at the given level.
		log_level: crate::lightning::util::logger::Level,
	},
}
use lightning::ln::msgs::ErrorAction as ErrorActionImport;
pub(crate) type nativeErrorAction = ErrorActionImport;

impl ErrorAction {
	#[allow(unused)]
	pub(crate) fn to_native(&self) -> nativeErrorAction {
		match self {
			ErrorAction::DisconnectPeer {ref msg, } => {
				let mut msg_nonref = Clone::clone(msg);
				let mut local_msg_nonref = if msg_nonref.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(msg_nonref.take_inner()) } }) };
				nativeErrorAction::DisconnectPeer {
					msg: local_msg_nonref,
				}
			},
			ErrorAction::IgnoreError => nativeErrorAction::IgnoreError,
			ErrorAction::IgnoreAndLog (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				nativeErrorAction::IgnoreAndLog (
					a_nonref.into_native(),
				)
			},
			ErrorAction::IgnoreDuplicateGossip => nativeErrorAction::IgnoreDuplicateGossip,
			ErrorAction::SendErrorMessage {ref msg, } => {
				let mut msg_nonref = Clone::clone(msg);
				nativeErrorAction::SendErrorMessage {
					msg: *unsafe { Box::from_raw(msg_nonref.take_inner()) },
				}
			},
			ErrorAction::SendWarningMessage {ref msg, ref log_level, } => {
				let mut msg_nonref = Clone::clone(msg);
				let mut log_level_nonref = Clone::clone(log_level);
				nativeErrorAction::SendWarningMessage {
					msg: *unsafe { Box::from_raw(msg_nonref.take_inner()) },
					log_level: log_level_nonref.into_native(),
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_native(self) -> nativeErrorAction {
		match self {
			ErrorAction::DisconnectPeer {mut msg, } => {
				let mut local_msg = if msg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(msg.take_inner()) } }) };
				nativeErrorAction::DisconnectPeer {
					msg: local_msg,
				}
			},
			ErrorAction::IgnoreError => nativeErrorAction::IgnoreError,
			ErrorAction::IgnoreAndLog (mut a, ) => {
				nativeErrorAction::IgnoreAndLog (
					a.into_native(),
				)
			},
			ErrorAction::IgnoreDuplicateGossip => nativeErrorAction::IgnoreDuplicateGossip,
			ErrorAction::SendErrorMessage {mut msg, } => {
				nativeErrorAction::SendErrorMessage {
					msg: *unsafe { Box::from_raw(msg.take_inner()) },
				}
			},
			ErrorAction::SendWarningMessage {mut msg, mut log_level, } => {
				nativeErrorAction::SendWarningMessage {
					msg: *unsafe { Box::from_raw(msg.take_inner()) },
					log_level: log_level.into_native(),
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_native(native: &nativeErrorAction) -> Self {
		match native {
			nativeErrorAction::DisconnectPeer {ref msg, } => {
				let mut msg_nonref = Clone::clone(msg);
				let mut local_msg_nonref = crate::lightning::ln::msgs::ErrorMessage { inner: if msg_nonref.is_none() { core::ptr::null_mut() } else {  { ObjOps::heap_alloc((msg_nonref.unwrap())) } }, is_owned: true };
				ErrorAction::DisconnectPeer {
					msg: local_msg_nonref,
				}
			},
			nativeErrorAction::IgnoreError => ErrorAction::IgnoreError,
			nativeErrorAction::IgnoreAndLog (ref a, ) => {
				let mut a_nonref = Clone::clone(a);
				ErrorAction::IgnoreAndLog (
					crate::lightning::util::logger::Level::native_into(a_nonref),
				)
			},
			nativeErrorAction::IgnoreDuplicateGossip => ErrorAction::IgnoreDuplicateGossip,
			nativeErrorAction::SendErrorMessage {ref msg, } => {
				let mut msg_nonref = Clone::clone(msg);
				ErrorAction::SendErrorMessage {
					msg: crate::lightning::ln::msgs::ErrorMessage { inner: ObjOps::heap_alloc(msg_nonref), is_owned: true },
				}
			},
			nativeErrorAction::SendWarningMessage {ref msg, ref log_level, } => {
				let mut msg_nonref = Clone::clone(msg);
				let mut log_level_nonref = Clone::clone(log_level);
				ErrorAction::SendWarningMessage {
					msg: crate::lightning::ln::msgs::WarningMessage { inner: ObjOps::heap_alloc(msg_nonref), is_owned: true },
					log_level: crate::lightning::util::logger::Level::native_into(log_level_nonref),
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn native_into(native: nativeErrorAction) -> Self {
		match native {
			nativeErrorAction::DisconnectPeer {mut msg, } => {
				let mut local_msg = crate::lightning::ln::msgs::ErrorMessage { inner: if msg.is_none() { core::ptr::null_mut() } else {  { ObjOps::heap_alloc((msg.unwrap())) } }, is_owned: true };
				ErrorAction::DisconnectPeer {
					msg: local_msg,
				}
			},
			nativeErrorAction::IgnoreError => ErrorAction::IgnoreError,
			nativeErrorAction::IgnoreAndLog (mut a, ) => {
				ErrorAction::IgnoreAndLog (
					crate::lightning::util::logger::Level::native_into(a),
				)
			},
			nativeErrorAction::IgnoreDuplicateGossip => ErrorAction::IgnoreDuplicateGossip,
			nativeErrorAction::SendErrorMessage {mut msg, } => {
				ErrorAction::SendErrorMessage {
					msg: crate::lightning::ln::msgs::ErrorMessage { inner: ObjOps::heap_alloc(msg), is_owned: true },
				}
			},
			nativeErrorAction::SendWarningMessage {mut msg, mut log_level, } => {
				ErrorAction::SendWarningMessage {
					msg: crate::lightning::ln::msgs::WarningMessage { inner: ObjOps::heap_alloc(msg), is_owned: true },
					log_level: crate::lightning::util::logger::Level::native_into(log_level),
				}
			},
		}
	}
}
/// Frees any resources used by the ErrorAction
#[no_mangle]
pub extern "C" fn ErrorAction_free(this_ptr: ErrorAction) { }
/// Creates a copy of the ErrorAction
#[no_mangle]
pub extern "C" fn ErrorAction_clone(orig: &ErrorAction) -> ErrorAction {
	orig.clone()
}
#[no_mangle]
/// Utility method to constructs a new DisconnectPeer-variant ErrorAction
pub extern "C" fn ErrorAction_disconnect_peer(msg: crate::lightning::ln::msgs::ErrorMessage) -> ErrorAction {
	ErrorAction::DisconnectPeer {
		msg,
	}
}
#[no_mangle]
/// Utility method to constructs a new IgnoreError-variant ErrorAction
pub extern "C" fn ErrorAction_ignore_error() -> ErrorAction {
	ErrorAction::IgnoreError}
#[no_mangle]
/// Utility method to constructs a new IgnoreAndLog-variant ErrorAction
pub extern "C" fn ErrorAction_ignore_and_log(a: crate::lightning::util::logger::Level) -> ErrorAction {
	ErrorAction::IgnoreAndLog(a, )
}
#[no_mangle]
/// Utility method to constructs a new IgnoreDuplicateGossip-variant ErrorAction
pub extern "C" fn ErrorAction_ignore_duplicate_gossip() -> ErrorAction {
	ErrorAction::IgnoreDuplicateGossip}
#[no_mangle]
/// Utility method to constructs a new SendErrorMessage-variant ErrorAction
pub extern "C" fn ErrorAction_send_error_message(msg: crate::lightning::ln::msgs::ErrorMessage) -> ErrorAction {
	ErrorAction::SendErrorMessage {
		msg,
	}
}
#[no_mangle]
/// Utility method to constructs a new SendWarningMessage-variant ErrorAction
pub extern "C" fn ErrorAction_send_warning_message(msg: crate::lightning::ln::msgs::WarningMessage, log_level: crate::lightning::util::logger::Level) -> ErrorAction {
	ErrorAction::SendWarningMessage {
		msg,
		log_level,
	}
}

use lightning::ln::msgs::LightningError as nativeLightningErrorImport;
pub(crate) type nativeLightningError = nativeLightningErrorImport;

/// An Err type for failure to process messages.
#[must_use]
#[repr(C)]
pub struct LightningError {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeLightningError,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for LightningError {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeLightningError>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the LightningError, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn LightningError_free(this_obj: LightningError) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn LightningError_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeLightningError) };
}
#[allow(unused)]
impl LightningError {
	pub(crate) fn get_native_ref(&self) -> &'static nativeLightningError {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeLightningError {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeLightningError {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// A human-readable message describing the error
#[no_mangle]
pub extern "C" fn LightningError_get_err(this_ptr: &LightningError) -> crate::c_types::Str {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().err;
	inner_val.as_str().into()
}
/// A human-readable message describing the error
#[no_mangle]
pub extern "C" fn LightningError_set_err(this_ptr: &mut LightningError, mut val: crate::c_types::Str) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.err = val.into_string();
}
/// The action which should be taken against the offending peer.
#[no_mangle]
pub extern "C" fn LightningError_get_action(this_ptr: &LightningError) -> crate::lightning::ln::msgs::ErrorAction {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().action;
	crate::lightning::ln::msgs::ErrorAction::from_native(inner_val)
}
/// The action which should be taken against the offending peer.
#[no_mangle]
pub extern "C" fn LightningError_set_action(this_ptr: &mut LightningError, mut val: crate::lightning::ln::msgs::ErrorAction) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.action = val.into_native();
}
/// Constructs a new LightningError given each field
#[must_use]
#[no_mangle]
pub extern "C" fn LightningError_new(mut err_arg: crate::c_types::Str, mut action_arg: crate::lightning::ln::msgs::ErrorAction) -> LightningError {
	LightningError { inner: ObjOps::heap_alloc(nativeLightningError {
		err: err_arg.into_string(),
		action: action_arg.into_native(),
	}), is_owned: true }
}
impl Clone for LightningError {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeLightningError>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn LightningError_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeLightningError)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the LightningError
pub extern "C" fn LightningError_clone(orig: &LightningError) -> LightningError {
	orig.clone()
}

use lightning::ln::msgs::CommitmentUpdate as nativeCommitmentUpdateImport;
pub(crate) type nativeCommitmentUpdate = nativeCommitmentUpdateImport;

/// Struct used to return values from [`RevokeAndACK`] messages, containing a bunch of commitment
/// transaction updates if they were pending.
#[must_use]
#[repr(C)]
pub struct CommitmentUpdate {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeCommitmentUpdate,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for CommitmentUpdate {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeCommitmentUpdate>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the CommitmentUpdate, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn CommitmentUpdate_free(this_obj: CommitmentUpdate) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn CommitmentUpdate_free_void(this_ptr: *mut c_void) {
	let _ = unsafe { Box::from_raw(this_ptr as *mut nativeCommitmentUpdate) };
}
#[allow(unused)]
impl CommitmentUpdate {
	pub(crate) fn get_native_ref(&self) -> &'static nativeCommitmentUpdate {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeCommitmentUpdate {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeCommitmentUpdate {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// `update_add_htlc` messages which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_update_add_htlcs(this_ptr: &CommitmentUpdate) -> crate::c_types::derived::CVec_UpdateAddHTLCZ {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().update_add_htlcs;
	let mut local_inner_val = Vec::new(); for item in inner_val.iter() { local_inner_val.push( { crate::lightning::ln::msgs::UpdateAddHTLC { inner: unsafe { ObjOps::nonnull_ptr_to_inner((item as *const lightning::ln::msgs::UpdateAddHTLC<>) as *mut _) }, is_owned: false } }); };
	local_inner_val.into()
}
/// `update_add_htlc` messages which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_add_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::derived::CVec_UpdateAddHTLCZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.update_add_htlcs = local_val;
}
/// `update_fulfill_htlc` messages which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_update_fulfill_htlcs(this_ptr: &CommitmentUpdate) -> crate::c_types::derived::CVec_UpdateFulfillHTLCZ {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().update_fulfill_htlcs;
	let mut local_inner_val = Vec::new(); for item in inner_val.iter() { local_inner_val.push( { crate::lightning::ln::msgs::UpdateFulfillHTLC { inner: unsafe { ObjOps::nonnull_ptr_to_inner((item as *const lightning::ln::msgs::UpdateFulfillHTLC<>) as *mut _) }, is_owned: false } }); };
	local_inner_val.into()
}
/// `update_fulfill_htlc` messages which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fulfill_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::derived::CVec_UpdateFulfillHTLCZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.update_fulfill_htlcs = local_val;
}
/// `update_fail_htlc` messages which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_update_fail_htlcs(this_ptr: &CommitmentUpdate) -> crate::c_types::derived::CVec_UpdateFailHTLCZ {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().update_fail_htlcs;
	let mut local_inner_val = Vec::new(); for item in inner_val.iter() { local_inner_val.push( { crate::lightning::ln::msgs::UpdateFailHTLC { inner: unsafe { ObjOps::nonnull_ptr_to_inner((item as *const lightning::ln::msgs::UpdateFailHTLC<>) as *mut _) }, is_owned: false } }); };
	local_inner_val.into()
}
/// `update_fail_htlc` messages which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fail_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::derived::CVec_UpdateFailHTLCZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.update_fail_htlcs = local_val;
}
/// `update_fail_malformed_htlc` messages which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_update_fail_malformed_htlcs(this_ptr: &CommitmentUpdate) -> crate::c_types::derived::CVec_UpdateFailMalformedHTLCZ {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().update_fail_malformed_htlcs;
	let mut local_inner_val = Vec::new(); for item in inner_val.iter() { local_inner_val.push( { crate::lightning::ln::msgs::UpdateFailMalformedHTLC { inner: unsafe { ObjOps::nonnull_ptr_to_inner((item as *const lightning::ln::msgs::UpdateFailMalformedHTLC<>) as *mut _) }, is_owned: false } }); };
	local_inner_val.into()
}
/// `update_fail_malformed_htlc` messages which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fail_malformed_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::derived::CVec_UpdateFailMalformedHTLCZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.update_fail_malformed_htlcs = local_val;
}
/// An `update_fee` message which should be sent
///
/// Note that the return value (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_update_fee(this_ptr: &CommitmentUpdate) -> crate::lightning::ln::msgs::UpdateFee {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().update_fee;
	let mut local_inner_val = crate::lightning::ln::msgs::UpdateFee { inner: unsafe { (if inner_val.is_none() { core::ptr::null() } else { ObjOps::nonnull_ptr_to_inner( { (inner_val.as_ref().unwrap()) }) } as *const lightning::ln::msgs::UpdateFee<>) as *mut _ }, is_owned: false };
	local_inner_val
}
/// An `update_fee` message which should be sent
///
/// Note that val (or a relevant inner pointer) may be NULL or all-0s to represent None
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fee(this_ptr: &mut CommitmentUpdate, mut val: crate::lightning::ln::msgs::UpdateFee) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.take_inner()) } }) };
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.update_fee = local_val;
}
/// A `commitment_signed` message which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_commitment_signed(this_ptr: &CommitmentUpdate) -> crate::lightning::ln::msgs::CommitmentSigned {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().commitment_signed;
	crate::lightning::ln::msgs::CommitmentSigned { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::ln::msgs::CommitmentSigned<>) as *mut _) }, is_owned: false }
}
/// A `commitment_signed` message which should be sent
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_commitment_signed(this_ptr: &mut CommitmentUpdate, mut val: crate::lightning::ln::msgs::CommitmentSigned) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.commitment_signed = *unsafe { Box::from_raw(val.take_inner()) };
}
/// Constructs a new CommitmentUpdate given each field
#[must_use]
#[no_mangle]
pub extern "C" fn CommitmentUpdate_new(mut update_add_htlcs_arg: crate::c_types::derived::CVec_UpdateAddHTLCZ, mut update_fulfill_htlcs_arg: crate::c_types::derived::CVec_UpdateFulfillHTLCZ, mut update_fail_htlcs_arg: crate::c_types::derived::CVec_UpdateFailHTLCZ, mut update_fail_malformed_htlcs_arg: crate::c_types::derived::CVec_UpdateFailMalformedHTLCZ, mut update_fee_arg: crate::lightning::ln::msgs::UpdateFee, mut commitment_signed_arg: crate::lightning::ln::msgs::CommitmentSigned) -> CommitmentUpdate {
	let mut local_update_add_htlcs_arg = Vec::new(); for mut item in update_add_htlcs_arg.into_rust().drain(..) { local_update_add_htlcs_arg.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	let mut local_update_fulfill_htlcs_arg = Vec::new(); for mut item in update_fulfill_htlcs_arg.into_rust().drain(..) { local_update_fulfill_htlcs_arg.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	let mut local_update_fail_htlcs_arg = Vec::new(); for mut item in update_fail_htlcs_arg.into_rust().drain(..) { local_update_fail_htlcs_arg.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	let mut local_update_fail_malformed_htlcs_arg = Vec::new(); for mut item in update_fail_malformed_htlcs_arg.into_rust().drain(..) { local_update_fail_malformed_htlcs_arg.push( { *unsafe { Box::from_raw(item.take_inner()) } }); };
	let mut local_update_fee_arg = if update_fee_arg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(update_fee_arg.take_inner()) } }) };
	CommitmentUpdate { inner: ObjOps::heap_alloc(nativeCommitmentUpdate {
		update_add_htlcs: local_update_add_htlcs_arg,
		update_fulfill_htlcs: local_update_fulfill_htlcs_arg,
		update_fail_htlcs: local_update_fail_htlcs_arg,
		update_fail_malformed_htlcs: local_update_fail_malformed_htlcs_arg,
		update_fee: local_update_fee_arg,
		commitment_signed: *unsafe { Box::from_raw(commitment_signed_arg.take_inner()) },
	}), is_owned: true }
}
impl Clone for CommitmentUpdate {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeCommitmentUpdate>::is_null(self.inner) { core::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn CommitmentUpdate_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeCommitmentUpdate)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the CommitmentUpdate
pub extern "C" fn CommitmentUpdate_clone(orig: &CommitmentUpdate) -> CommitmentUpdate {
	orig.clone()
}
/// Checks if two CommitmentUpdates contain equal inner contents.
/// This ignores pointers and is_owned flags and looks at the values in fields.
/// Two objects with NULL inner values will be considered "equal" here.
#[no_mangle]
pub extern "C" fn CommitmentUpdate_eq(a: &CommitmentUpdate, b: &CommitmentUpdate) -> bool {
	if a.inner == b.inner { return true; }
	if a.inner.is_null() || b.inner.is_null() { return false; }
	if a.get_native_ref() == b.get_native_ref() { true } else { false }
}
/// A trait to describe an object which can receive channel messages.
///
/// Messages MAY be called in parallel when they originate from different `their_node_ids`, however
/// they MUST NOT be called in parallel when the two calls have the same `their_node_id`.
#[repr(C)]
pub struct ChannelMessageHandler {
	/// An opaque pointer which is passed to your function implementations as an argument.
	/// This has no meaning in the LDK, and can be NULL or any other value.
	pub this_arg: *mut c_void,
	/// Handle an incoming `open_channel` message from the given peer.
	pub handle_open_channel: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::OpenChannel),
	/// Handle an incoming `accept_channel` message from the given peer.
	pub handle_accept_channel: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::AcceptChannel),
	/// Handle an incoming `funding_created` message from the given peer.
	pub handle_funding_created: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::FundingCreated),
	/// Handle an incoming `funding_signed` message from the given peer.
	pub handle_funding_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::FundingSigned),
	/// Handle an incoming `channel_ready` message from the given peer.
	pub handle_channel_ready: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::ChannelReady),
	/// Handle an incoming `shutdown` message from the given peer.
	pub handle_shutdown: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::Shutdown),
	/// Handle an incoming `closing_signed` message from the given peer.
	pub handle_closing_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::ClosingSigned),
	/// Handle an incoming `update_add_htlc` message from the given peer.
	pub handle_update_add_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::UpdateAddHTLC),
	/// Handle an incoming `update_fulfill_htlc` message from the given peer.
	pub handle_update_fulfill_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::UpdateFulfillHTLC),
	/// Handle an incoming `update_fail_htlc` message from the given peer.
	pub handle_update_fail_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::UpdateFailHTLC),
	/// Handle an incoming `update_fail_malformed_htlc` message from the given peer.
	pub handle_update_fail_malformed_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::UpdateFailMalformedHTLC),
	/// Handle an incoming `commitment_signed` message from the given peer.
	pub handle_commitment_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::CommitmentSigned),
	/// Handle an incoming `revoke_and_ack` message from the given peer.
	pub handle_revoke_and_ack: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::RevokeAndACK),
	/// Handle an incoming `update_fee` message from the given peer.
	pub handle_update_fee: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::UpdateFee),
	/// Handle an incoming `announcement_signatures` message from the given peer.
	pub handle_announcement_signatures: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::AnnouncementSignatures),
	/// Indicates a connection to the peer failed/an existing connection was lost.
	pub peer_disconnected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey),
	/// Handle a peer reconnecting, possibly generating `channel_reestablish` message(s).
	///
	/// May return an `Err(())` if the features the peer supports are not sufficient to communicate
	/// with us. Implementors should be somewhat conservative about doing so, however, as other
	/// message handlers may still wish to communicate with this peer.
	#[must_use]
	pub peer_connected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::Init, inbound: bool) -> crate::c_types::derived::CResult_NoneNoneZ,
	/// Handle an incoming `channel_reestablish` message from the given peer.
	pub handle_channel_reestablish: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::ChannelReestablish),
	/// Handle an incoming `channel_update` message from the given peer.
	pub handle_channel_update: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::ChannelUpdate),
	/// Handle an incoming `error` message from the given peer.
	pub handle_error: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::ErrorMessage),
	/// Gets the node feature flags which this handler itself supports. All available handlers are
	/// queried similarly and their feature flags are OR'd together to form the [`NodeFeatures`]
	/// which are broadcasted in our [`NodeAnnouncement`] message.
	#[must_use]
	pub provided_node_features: extern "C" fn (this_arg: *const c_void) -> crate::lightning::ln::features::NodeFeatures,
	/// Gets the init feature flags which should be sent to the given peer. All available handlers
	/// are queried similarly and their feature flags are OR'd together to form the [`InitFeatures`]
	/// which are sent in our [`Init`] message.
	///
	/// Note that this method is called before [`Self::peer_connected`].
	#[must_use]
	pub provided_init_features: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey) -> crate::lightning::ln::features::InitFeatures,
	/// Implementation of MessageSendEventsProvider for this object.
	pub MessageSendEventsProvider: crate::lightning::events::MessageSendEventsProvider,
	/// Frees any resources associated with this object given its this_arg pointer.
	/// Does not need to free the outer struct containing function pointers and may be NULL is no resources need to be freed.
	pub free: Option<extern "C" fn(this_arg: *mut c_void)>,
}
unsafe impl Send for ChannelMessageHandler {}
unsafe impl Sync for ChannelMessageHandler {}
#[no_mangle]
pub(crate) extern "C" fn ChannelMessageHandler_clone_fields(orig: &ChannelMessageHandler) -> ChannelMessageHandler {
	ChannelMessageHandler {
		this_arg: orig.this_arg,
		handle_open_channel: Clone::clone(&orig.handle_open_channel),
		handle_accept_channel: Clone::clone(&orig.handle_accept_channel),
		handle_funding_created: Clone::clone(&orig.handle_funding_created),
		handle_funding_signed: Clone::clone(&orig.handle_funding_signed),
		handle_channel_ready: Clone::clone(&orig.handle_channel_ready),
		handle_shutdown: Clone::clone(&orig.handle_shutdown),
		handle_closing_signed: Clone::clone(&orig.handle_closing_signed),
		handle_update_add_htlc: Clone::clone(&orig.handle_update_add_htlc),
		handle_update_fulfill_htlc: Clone::clone(&orig.handle_update_fulfill_htlc),
		handle_update_fail_htlc: Clone::clone(&orig.handle_update_fail_htlc),
		handle_update_fail_malformed_htlc: Clone::clone(&orig.handle_update_fail_malformed_htlc),
		handle_commitment_signed: Clone::clone(&orig.handle_commitment_signed),
		handle_revoke_and_ack: Clone::clone(&orig.handle_revoke_and_ack),
		handle_update_fee: Clone::clone(&orig.handle_update_fee),
		handle_announcement_signatures: Clone::clone(&orig.handle_announcement_signatures),
		peer_disconnected: Clone::clone(&orig.peer_disconnected),
		peer_connected: Clone::clone(&orig.peer_connected),
		handle_channel_reestablish: Clone::clone(&orig.handle_channel_reestablish),
		handle_channel_update: Clone::clone(&orig.handle_channel_update),
		handle_error: Clone::clone(&orig.handle_error),
		provided_node_features: Clone::clone(&orig.provided_node_features),
		provided_init_features: Clone::clone(&orig.provided_init_features),
		MessageSendEventsProvider: crate::lightning::events::MessageSendEventsProvider_clone_fields(&orig.MessageSendEventsProvider),
		free: Clone::clone(&orig.free),
	}
}
impl lightning::events::MessageSendEventsProvider for ChannelMessageHandler {
	fn get_and_clear_pending_msg_events(&self) -> Vec<lightning::events::MessageSendEvent> {
		let mut ret = (self.MessageSendEventsProvider.get_and_clear_pending_msg_events)(self.MessageSendEventsProvider.this_arg);
		let mut local_ret = Vec::new(); for mut item in ret.into_rust().drain(..) { local_ret.push( { item.into_native() }); };
		local_ret
	}
}

use lightning::ln::msgs::ChannelMessageHandler as rustChannelMessageHandler;
impl rustChannelMessageHandler for ChannelMessageHandler {
	fn handle_open_channel(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::OpenChannel) {
		(self.handle_open_channel)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::OpenChannel { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::OpenChannel<>) as *mut _) }, is_owned: false })
	}
	fn handle_accept_channel(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::AcceptChannel) {
		(self.handle_accept_channel)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::AcceptChannel { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::AcceptChannel<>) as *mut _) }, is_owned: false })
	}
	fn handle_funding_created(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::FundingCreated) {
		(self.handle_funding_created)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::FundingCreated { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::FundingCreated<>) as *mut _) }, is_owned: false })
	}
	fn handle_funding_signed(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::FundingSigned) {
		(self.handle_funding_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::FundingSigned { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::FundingSigned<>) as *mut _) }, is_owned: false })
	}
	fn handle_channel_ready(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::ChannelReady) {
		(self.handle_channel_ready)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::ChannelReady { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::ChannelReady<>) as *mut _) }, is_owned: false })
	}
	fn handle_shutdown(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::Shutdown) {
		(self.handle_shutdown)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::Shutdown { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::Shutdown<>) as *mut _) }, is_owned: false })
	}
	fn handle_closing_signed(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::ClosingSigned) {
		(self.handle_closing_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::ClosingSigned { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::ClosingSigned<>) as *mut _) }, is_owned: false })
	}
	fn handle_update_add_htlc(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::UpdateAddHTLC) {
		(self.handle_update_add_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::UpdateAddHTLC { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::UpdateAddHTLC<>) as *mut _) }, is_owned: false })
	}
	fn handle_update_fulfill_htlc(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::UpdateFulfillHTLC) {
		(self.handle_update_fulfill_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::UpdateFulfillHTLC { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::UpdateFulfillHTLC<>) as *mut _) }, is_owned: false })
	}
	fn handle_update_fail_htlc(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::UpdateFailHTLC) {
		(self.handle_update_fail_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::UpdateFailHTLC { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::UpdateFailHTLC<>) as *mut _) }, is_owned: false })
	}
	fn handle_update_fail_malformed_htlc(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::UpdateFailMalformedHTLC) {
		(self.handle_update_fail_malformed_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::UpdateFailMalformedHTLC { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::UpdateFailMalformedHTLC<>) as *mut _) }, is_owned: false })
	}
	fn handle_commitment_signed(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::CommitmentSigned) {
		(self.handle_commitment_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::CommitmentSigned { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::CommitmentSigned<>) as *mut _) }, is_owned: false })
	}
	fn handle_revoke_and_ack(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::RevokeAndACK) {
		(self.handle_revoke_and_ack)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::RevokeAndACK { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::RevokeAndACK<>) as *mut _) }, is_owned: false })
	}
	fn handle_update_fee(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::UpdateFee) {
		(self.handle_update_fee)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::UpdateFee { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::UpdateFee<>) as *mut _) }, is_owned: false })
	}
	fn handle_announcement_signatures(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::AnnouncementSignatures) {
		(self.handle_announcement_signatures)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::AnnouncementSignatures { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::AnnouncementSignatures<>) as *mut _) }, is_owned: false })
	}
	fn peer_disconnected(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey) {
		(self.peer_disconnected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id))
	}
	fn peer_connected(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::Init, mut inbound: bool) -> Result<(), ()> {
		let mut ret = (self.peer_connected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::Init { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::Init<>) as *mut _) }, is_owned: false }, inbound);
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn handle_channel_reestablish(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::ChannelReestablish) {
		(self.handle_channel_reestablish)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::ChannelReestablish { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::ChannelReestablish<>) as *mut _) }, is_owned: false })
	}
	fn handle_channel_update(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::ChannelUpdate) {
		(self.handle_channel_update)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::ChannelUpdate { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::ChannelUpdate<>) as *mut _) }, is_owned: false })
	}
	fn handle_error(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::ErrorMessage) {
		(self.handle_error)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::ErrorMessage { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::ErrorMessage<>) as *mut _) }, is_owned: false })
	}
	fn provided_node_features(&self) -> lightning::ln::features::NodeFeatures {
		let mut ret = (self.provided_node_features)(self.this_arg);
		*unsafe { Box::from_raw(ret.take_inner()) }
	}
	fn provided_init_features(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey) -> lightning::ln::features::InitFeatures {
		let mut ret = (self.provided_init_features)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id));
		*unsafe { Box::from_raw(ret.take_inner()) }
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl core::ops::Deref for ChannelMessageHandler {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// Calls the free function if one is set
#[no_mangle]
pub extern "C" fn ChannelMessageHandler_free(this_ptr: ChannelMessageHandler) { }
impl Drop for ChannelMessageHandler {
	fn drop(&mut self) {
		if let Some(f) = self.free {
			f(self.this_arg);
		}
	}
}
/// A trait to describe an object which can receive routing messages.
///
/// # Implementor DoS Warnings
///
/// For messages enabled with the `gossip_queries` feature there are potential DoS vectors when
/// handling inbound queries. Implementors using an on-disk network graph should be aware of
/// repeated disk I/O for queries accessing different parts of the network graph.
#[repr(C)]
pub struct RoutingMessageHandler {
	/// An opaque pointer which is passed to your function implementations as an argument.
	/// This has no meaning in the LDK, and can be NULL or any other value.
	pub this_arg: *mut c_void,
	/// Handle an incoming `node_announcement` message, returning `true` if it should be forwarded on,
	/// `false` or returning an `Err` otherwise.
	#[must_use]
	pub handle_node_announcement: extern "C" fn (this_arg: *const c_void, msg: &crate::lightning::ln::msgs::NodeAnnouncement) -> crate::c_types::derived::CResult_boolLightningErrorZ,
	/// Handle a `channel_announcement` message, returning `true` if it should be forwarded on, `false`
	/// or returning an `Err` otherwise.
	#[must_use]
	pub handle_channel_announcement: extern "C" fn (this_arg: *const c_void, msg: &crate::lightning::ln::msgs::ChannelAnnouncement) -> crate::c_types::derived::CResult_boolLightningErrorZ,
	/// Handle an incoming `channel_update` message, returning true if it should be forwarded on,
	/// `false` or returning an `Err` otherwise.
	#[must_use]
	pub handle_channel_update: extern "C" fn (this_arg: *const c_void, msg: &crate::lightning::ln::msgs::ChannelUpdate) -> crate::c_types::derived::CResult_boolLightningErrorZ,
	/// Gets channel announcements and updates required to dump our routing table to a remote node,
	/// starting at the `short_channel_id` indicated by `starting_point` and including announcements
	/// for a single channel.
	#[must_use]
	pub get_next_channel_announcement: extern "C" fn (this_arg: *const c_void, starting_point: u64) -> crate::c_types::derived::COption_C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZZ,
	/// Gets a node announcement required to dump our routing table to a remote node, starting at
	/// the node *after* the provided pubkey and including up to one announcement immediately
	/// higher (as defined by `<PublicKey as Ord>::cmp`) than `starting_point`.
	/// If `None` is provided for `starting_point`, we start at the first node.
	///
	/// Note that starting_point (or a relevant inner pointer) may be NULL or all-0s to represent None
	/// Note that the return value (or a relevant inner pointer) may be NULL or all-0s to represent None
	#[must_use]
	pub get_next_node_announcement: extern "C" fn (this_arg: *const c_void, starting_point: crate::lightning::routing::gossip::NodeId) -> crate::lightning::ln::msgs::NodeAnnouncement,
	/// Called when a connection is established with a peer. This can be used to
	/// perform routing table synchronization using a strategy defined by the
	/// implementor.
	///
	/// May return an `Err(())` if the features the peer supports are not sufficient to communicate
	/// with us. Implementors should be somewhat conservative about doing so, however, as other
	/// message handlers may still wish to communicate with this peer.
	#[must_use]
	pub peer_connected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, init: &crate::lightning::ln::msgs::Init, inbound: bool) -> crate::c_types::derived::CResult_NoneNoneZ,
	/// Handles the reply of a query we initiated to learn about channels
	/// for a given range of blocks. We can expect to receive one or more
	/// replies to a single query.
	#[must_use]
	pub handle_reply_channel_range: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: crate::lightning::ln::msgs::ReplyChannelRange) -> crate::c_types::derived::CResult_NoneLightningErrorZ,
	/// Handles the reply of a query we initiated asking for routing gossip
	/// messages for a list of channels. We should receive this message when
	/// a node has completed its best effort to send us the pertaining routing
	/// gossip messages.
	#[must_use]
	pub handle_reply_short_channel_ids_end: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: crate::lightning::ln::msgs::ReplyShortChannelIdsEnd) -> crate::c_types::derived::CResult_NoneLightningErrorZ,
	/// Handles when a peer asks us to send a list of `short_channel_id`s
	/// for the requested range of blocks.
	#[must_use]
	pub handle_query_channel_range: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: crate::lightning::ln::msgs::QueryChannelRange) -> crate::c_types::derived::CResult_NoneLightningErrorZ,
	/// Handles when a peer asks us to send routing gossip messages for a
	/// list of `short_channel_id`s.
	#[must_use]
	pub handle_query_short_channel_ids: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: crate::lightning::ln::msgs::QueryShortChannelIds) -> crate::c_types::derived::CResult_NoneLightningErrorZ,
	/// Indicates that there are a large number of [`ChannelAnnouncement`] (or other) messages
	/// pending some async action. While there is no guarantee of the rate of future messages, the
	/// caller should seek to reduce the rate of new gossip messages handled, especially
	/// [`ChannelAnnouncement`]s.
	#[must_use]
	pub processing_queue_high: extern "C" fn (this_arg: *const c_void) -> bool,
	/// Gets the node feature flags which this handler itself supports. All available handlers are
	/// queried similarly and their feature flags are OR'd together to form the [`NodeFeatures`]
	/// which are broadcasted in our [`NodeAnnouncement`] message.
	#[must_use]
	pub provided_node_features: extern "C" fn (this_arg: *const c_void) -> crate::lightning::ln::features::NodeFeatures,
	/// Gets the init feature flags which should be sent to the given peer. All available handlers
	/// are queried similarly and their feature flags are OR'd together to form the [`InitFeatures`]
	/// which are sent in our [`Init`] message.
	///
	/// Note that this method is called before [`Self::peer_connected`].
	#[must_use]
	pub provided_init_features: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey) -> crate::lightning::ln::features::InitFeatures,
	/// Implementation of MessageSendEventsProvider for this object.
	pub MessageSendEventsProvider: crate::lightning::events::MessageSendEventsProvider,
	/// Frees any resources associated with this object given its this_arg pointer.
	/// Does not need to free the outer struct containing function pointers and may be NULL is no resources need to be freed.
	pub free: Option<extern "C" fn(this_arg: *mut c_void)>,
}
unsafe impl Send for RoutingMessageHandler {}
unsafe impl Sync for RoutingMessageHandler {}
#[no_mangle]
pub(crate) extern "C" fn RoutingMessageHandler_clone_fields(orig: &RoutingMessageHandler) -> RoutingMessageHandler {
	RoutingMessageHandler {
		this_arg: orig.this_arg,
		handle_node_announcement: Clone::clone(&orig.handle_node_announcement),
		handle_channel_announcement: Clone::clone(&orig.handle_channel_announcement),
		handle_channel_update: Clone::clone(&orig.handle_channel_update),
		get_next_channel_announcement: Clone::clone(&orig.get_next_channel_announcement),
		get_next_node_announcement: Clone::clone(&orig.get_next_node_announcement),
		peer_connected: Clone::clone(&orig.peer_connected),
		handle_reply_channel_range: Clone::clone(&orig.handle_reply_channel_range),
		handle_reply_short_channel_ids_end: Clone::clone(&orig.handle_reply_short_channel_ids_end),
		handle_query_channel_range: Clone::clone(&orig.handle_query_channel_range),
		handle_query_short_channel_ids: Clone::clone(&orig.handle_query_short_channel_ids),
		processing_queue_high: Clone::clone(&orig.processing_queue_high),
		provided_node_features: Clone::clone(&orig.provided_node_features),
		provided_init_features: Clone::clone(&orig.provided_init_features),
		MessageSendEventsProvider: crate::lightning::events::MessageSendEventsProvider_clone_fields(&orig.MessageSendEventsProvider),
		free: Clone::clone(&orig.free),
	}
}
impl lightning::events::MessageSendEventsProvider for RoutingMessageHandler {
	fn get_and_clear_pending_msg_events(&self) -> Vec<lightning::events::MessageSendEvent> {
		let mut ret = (self.MessageSendEventsProvider.get_and_clear_pending_msg_events)(self.MessageSendEventsProvider.this_arg);
		let mut local_ret = Vec::new(); for mut item in ret.into_rust().drain(..) { local_ret.push( { item.into_native() }); };
		local_ret
	}
}

use lightning::ln::msgs::RoutingMessageHandler as rustRoutingMessageHandler;
impl rustRoutingMessageHandler for RoutingMessageHandler {
	fn handle_node_announcement(&self, mut msg: &lightning::ln::msgs::NodeAnnouncement) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_node_announcement)(self.this_arg, &crate::lightning::ln::msgs::NodeAnnouncement { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::NodeAnnouncement<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }) }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn handle_channel_announcement(&self, mut msg: &lightning::ln::msgs::ChannelAnnouncement) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_channel_announcement)(self.this_arg, &crate::lightning::ln::msgs::ChannelAnnouncement { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::ChannelAnnouncement<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }) }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn handle_channel_update(&self, mut msg: &lightning::ln::msgs::ChannelUpdate) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_channel_update)(self.this_arg, &crate::lightning::ln::msgs::ChannelUpdate { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::ChannelUpdate<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }) }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn get_next_channel_announcement(&self, mut starting_point: u64) -> Option<(lightning::ln::msgs::ChannelAnnouncement, Option<lightning::ln::msgs::ChannelUpdate>, Option<lightning::ln::msgs::ChannelUpdate>)> {
		let mut ret = (self.get_next_channel_announcement)(self.this_arg, starting_point);
		let mut local_ret = if ret.is_some() { Some( { let (mut orig_ret_0_0, mut orig_ret_0_1, mut orig_ret_0_2) = ret.take().to_rust(); let mut local_orig_ret_0_1 = if orig_ret_0_1.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(orig_ret_0_1.take_inner()) } }) }; let mut local_orig_ret_0_2 = if orig_ret_0_2.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(orig_ret_0_2.take_inner()) } }) }; let mut local_ret_0 = (*unsafe { Box::from_raw(orig_ret_0_0.take_inner()) }, local_orig_ret_0_1, local_orig_ret_0_2); local_ret_0 }) } else { None };
		local_ret
	}
	fn get_next_node_announcement(&self, mut starting_point: Option<&lightning::routing::gossip::NodeId>) -> Option<lightning::ln::msgs::NodeAnnouncement> {
		let mut local_starting_point = crate::lightning::routing::gossip::NodeId { inner: unsafe { (if starting_point.is_none() { core::ptr::null() } else { ObjOps::nonnull_ptr_to_inner( { (starting_point.unwrap()) }) } as *const lightning::routing::gossip::NodeId<>) as *mut _ }, is_owned: false };
		let mut ret = (self.get_next_node_announcement)(self.this_arg, local_starting_point);
		let mut local_ret = if ret.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(ret.take_inner()) } }) };
		local_ret
	}
	fn peer_connected(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut init: &lightning::ln::msgs::Init, mut inbound: bool) -> Result<(), ()> {
		let mut ret = (self.peer_connected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::Init { inner: unsafe { ObjOps::nonnull_ptr_to_inner((init as *const lightning::ln::msgs::Init<>) as *mut _) }, is_owned: false }, inbound);
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn handle_reply_channel_range(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: lightning::ln::msgs::ReplyChannelRange) -> Result<(), lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_reply_channel_range)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), crate::lightning::ln::msgs::ReplyChannelRange { inner: ObjOps::heap_alloc(msg), is_owned: true });
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn handle_reply_short_channel_ids_end(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: lightning::ln::msgs::ReplyShortChannelIdsEnd) -> Result<(), lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_reply_short_channel_ids_end)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), crate::lightning::ln::msgs::ReplyShortChannelIdsEnd { inner: ObjOps::heap_alloc(msg), is_owned: true });
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn handle_query_channel_range(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: lightning::ln::msgs::QueryChannelRange) -> Result<(), lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_query_channel_range)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), crate::lightning::ln::msgs::QueryChannelRange { inner: ObjOps::heap_alloc(msg), is_owned: true });
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn handle_query_short_channel_ids(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut msg: lightning::ln::msgs::QueryShortChannelIds) -> Result<(), lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_query_short_channel_ids)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), crate::lightning::ln::msgs::QueryShortChannelIds { inner: ObjOps::heap_alloc(msg), is_owned: true });
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn processing_queue_high(&self) -> bool {
		let mut ret = (self.processing_queue_high)(self.this_arg);
		ret
	}
	fn provided_node_features(&self) -> lightning::ln::features::NodeFeatures {
		let mut ret = (self.provided_node_features)(self.this_arg);
		*unsafe { Box::from_raw(ret.take_inner()) }
	}
	fn provided_init_features(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey) -> lightning::ln::features::InitFeatures {
		let mut ret = (self.provided_init_features)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id));
		*unsafe { Box::from_raw(ret.take_inner()) }
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl core::ops::Deref for RoutingMessageHandler {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// Calls the free function if one is set
#[no_mangle]
pub extern "C" fn RoutingMessageHandler_free(this_ptr: RoutingMessageHandler) { }
impl Drop for RoutingMessageHandler {
	fn drop(&mut self) {
		if let Some(f) = self.free {
			f(self.this_arg);
		}
	}
}
/// A trait to describe an object that can receive onion messages.
#[repr(C)]
pub struct OnionMessageHandler {
	/// An opaque pointer which is passed to your function implementations as an argument.
	/// This has no meaning in the LDK, and can be NULL or any other value.
	pub this_arg: *mut c_void,
	/// Handle an incoming `onion_message` message from the given peer.
	pub handle_onion_message: extern "C" fn (this_arg: *const c_void, peer_node_id: crate::c_types::PublicKey, msg: &crate::lightning::ln::msgs::OnionMessage),
	/// Called when a connection is established with a peer. Can be used to track which peers
	/// advertise onion message support and are online.
	///
	/// May return an `Err(())` if the features the peer supports are not sufficient to communicate
	/// with us. Implementors should be somewhat conservative about doing so, however, as other
	/// message handlers may still wish to communicate with this peer.
	#[must_use]
	pub peer_connected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, init: &crate::lightning::ln::msgs::Init, inbound: bool) -> crate::c_types::derived::CResult_NoneNoneZ,
	/// Indicates a connection to the peer failed/an existing connection was lost. Allows handlers to
	/// drop and refuse to forward onion messages to this peer.
	pub peer_disconnected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey),
	/// Gets the node feature flags which this handler itself supports. All available handlers are
	/// queried similarly and their feature flags are OR'd together to form the [`NodeFeatures`]
	/// which are broadcasted in our [`NodeAnnouncement`] message.
	#[must_use]
	pub provided_node_features: extern "C" fn (this_arg: *const c_void) -> crate::lightning::ln::features::NodeFeatures,
	/// Gets the init feature flags which should be sent to the given peer. All available handlers
	/// are queried similarly and their feature flags are OR'd together to form the [`InitFeatures`]
	/// which are sent in our [`Init`] message.
	///
	/// Note that this method is called before [`Self::peer_connected`].
	#[must_use]
	pub provided_init_features: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey) -> crate::lightning::ln::features::InitFeatures,
	/// Implementation of OnionMessageProvider for this object.
	pub OnionMessageProvider: crate::lightning::events::OnionMessageProvider,
	/// Frees any resources associated with this object given its this_arg pointer.
	/// Does not need to free the outer struct containing function pointers and may be NULL is no resources need to be freed.
	pub free: Option<extern "C" fn(this_arg: *mut c_void)>,
}
unsafe impl Send for OnionMessageHandler {}
unsafe impl Sync for OnionMessageHandler {}
#[no_mangle]
pub(crate) extern "C" fn OnionMessageHandler_clone_fields(orig: &OnionMessageHandler) -> OnionMessageHandler {
	OnionMessageHandler {
		this_arg: orig.this_arg,
		handle_onion_message: Clone::clone(&orig.handle_onion_message),
		peer_connected: Clone::clone(&orig.peer_connected),
		peer_disconnected: Clone::clone(&orig.peer_disconnected),
		provided_node_features: Clone::clone(&orig.provided_node_features),
		provided_init_features: Clone::clone(&orig.provided_init_features),
		OnionMessageProvider: crate::lightning::events::OnionMessageProvider_clone_fields(&orig.OnionMessageProvider),
		free: Clone::clone(&orig.free),
	}
}
impl lightning::events::OnionMessageProvider for OnionMessageHandler {
	fn next_onion_message_for_peer(&self, mut peer_node_id: bitcoin::secp256k1::PublicKey) -> Option<lightning::ln::msgs::OnionMessage> {
		let mut ret = (self.OnionMessageProvider.next_onion_message_for_peer)(self.OnionMessageProvider.this_arg, crate::c_types::PublicKey::from_rust(&peer_node_id));
		let mut local_ret = if ret.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(ret.take_inner()) } }) };
		local_ret
	}
}

use lightning::ln::msgs::OnionMessageHandler as rustOnionMessageHandler;
impl rustOnionMessageHandler for OnionMessageHandler {
	fn handle_onion_message(&self, mut peer_node_id: &bitcoin::secp256k1::PublicKey, mut msg: &lightning::ln::msgs::OnionMessage) {
		(self.handle_onion_message)(self.this_arg, crate::c_types::PublicKey::from_rust(&peer_node_id), &crate::lightning::ln::msgs::OnionMessage { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::OnionMessage<>) as *mut _) }, is_owned: false })
	}
	fn peer_connected(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey, mut init: &lightning::ln::msgs::Init, mut inbound: bool) -> Result<(), ()> {
		let mut ret = (self.peer_connected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::lightning::ln::msgs::Init { inner: unsafe { ObjOps::nonnull_ptr_to_inner((init as *const lightning::ln::msgs::Init<>) as *mut _) }, is_owned: false }, inbound);
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn peer_disconnected(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey) {
		(self.peer_disconnected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id))
	}
	fn provided_node_features(&self) -> lightning::ln::features::NodeFeatures {
		let mut ret = (self.provided_node_features)(self.this_arg);
		*unsafe { Box::from_raw(ret.take_inner()) }
	}
	fn provided_init_features(&self, mut their_node_id: &bitcoin::secp256k1::PublicKey) -> lightning::ln::features::InitFeatures {
		let mut ret = (self.provided_init_features)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id));
		*unsafe { Box::from_raw(ret.take_inner()) }
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl core::ops::Deref for OnionMessageHandler {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// Calls the free function if one is set
#[no_mangle]
pub extern "C" fn OnionMessageHandler_free(this_ptr: OnionMessageHandler) { }
impl Drop for OnionMessageHandler {
	fn drop(&mut self) {
		if let Some(f) = self.free {
			f(self.this_arg);
		}
	}
}
mod fuzzy_internal_msgs {

use alloc::str::FromStr;
use core::ffi::c_void;
use core::convert::Infallible;
use bitcoin::hashes::Hash;
use crate::c_types::*;
#[cfg(feature="no-std")]
use alloc::{vec::Vec, boxed::Box};

}
#[no_mangle]
/// Serialize the AcceptChannel object into a byte array which can be read by AcceptChannel_read
pub extern "C" fn AcceptChannel_write(obj: &crate::lightning::ln::msgs::AcceptChannel) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn AcceptChannel_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeAcceptChannel) })
}
#[no_mangle]
/// Read a AcceptChannel from a byte array, created by AcceptChannel_write
pub extern "C" fn AcceptChannel_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_AcceptChannelDecodeErrorZ {
	let res: Result<lightning::ln::msgs::AcceptChannel, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::AcceptChannel { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the AnnouncementSignatures object into a byte array which can be read by AnnouncementSignatures_read
pub extern "C" fn AnnouncementSignatures_write(obj: &crate::lightning::ln::msgs::AnnouncementSignatures) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn AnnouncementSignatures_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeAnnouncementSignatures) })
}
#[no_mangle]
/// Read a AnnouncementSignatures from a byte array, created by AnnouncementSignatures_write
pub extern "C" fn AnnouncementSignatures_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_AnnouncementSignaturesDecodeErrorZ {
	let res: Result<lightning::ln::msgs::AnnouncementSignatures, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::AnnouncementSignatures { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the ChannelReestablish object into a byte array which can be read by ChannelReestablish_read
pub extern "C" fn ChannelReestablish_write(obj: &crate::lightning::ln::msgs::ChannelReestablish) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ChannelReestablish_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeChannelReestablish) })
}
#[no_mangle]
/// Read a ChannelReestablish from a byte array, created by ChannelReestablish_write
pub extern "C" fn ChannelReestablish_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ChannelReestablishDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ChannelReestablish, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ChannelReestablish { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the ClosingSigned object into a byte array which can be read by ClosingSigned_read
pub extern "C" fn ClosingSigned_write(obj: &crate::lightning::ln::msgs::ClosingSigned) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ClosingSigned_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeClosingSigned) })
}
#[no_mangle]
/// Read a ClosingSigned from a byte array, created by ClosingSigned_write
pub extern "C" fn ClosingSigned_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ClosingSignedDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ClosingSigned, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ClosingSigned { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the ClosingSignedFeeRange object into a byte array which can be read by ClosingSignedFeeRange_read
pub extern "C" fn ClosingSignedFeeRange_write(obj: &crate::lightning::ln::msgs::ClosingSignedFeeRange) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ClosingSignedFeeRange_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeClosingSignedFeeRange) })
}
#[no_mangle]
/// Read a ClosingSignedFeeRange from a byte array, created by ClosingSignedFeeRange_write
pub extern "C" fn ClosingSignedFeeRange_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ClosingSignedFeeRangeDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ClosingSignedFeeRange, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ClosingSignedFeeRange { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the CommitmentSigned object into a byte array which can be read by CommitmentSigned_read
pub extern "C" fn CommitmentSigned_write(obj: &crate::lightning::ln::msgs::CommitmentSigned) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn CommitmentSigned_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeCommitmentSigned) })
}
#[no_mangle]
/// Read a CommitmentSigned from a byte array, created by CommitmentSigned_write
pub extern "C" fn CommitmentSigned_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_CommitmentSignedDecodeErrorZ {
	let res: Result<lightning::ln::msgs::CommitmentSigned, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::CommitmentSigned { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the FundingCreated object into a byte array which can be read by FundingCreated_read
pub extern "C" fn FundingCreated_write(obj: &crate::lightning::ln::msgs::FundingCreated) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn FundingCreated_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeFundingCreated) })
}
#[no_mangle]
/// Read a FundingCreated from a byte array, created by FundingCreated_write
pub extern "C" fn FundingCreated_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_FundingCreatedDecodeErrorZ {
	let res: Result<lightning::ln::msgs::FundingCreated, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::FundingCreated { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the FundingSigned object into a byte array which can be read by FundingSigned_read
pub extern "C" fn FundingSigned_write(obj: &crate::lightning::ln::msgs::FundingSigned) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn FundingSigned_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeFundingSigned) })
}
#[no_mangle]
/// Read a FundingSigned from a byte array, created by FundingSigned_write
pub extern "C" fn FundingSigned_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_FundingSignedDecodeErrorZ {
	let res: Result<lightning::ln::msgs::FundingSigned, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::FundingSigned { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the ChannelReady object into a byte array which can be read by ChannelReady_read
pub extern "C" fn ChannelReady_write(obj: &crate::lightning::ln::msgs::ChannelReady) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ChannelReady_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeChannelReady) })
}
#[no_mangle]
/// Read a ChannelReady from a byte array, created by ChannelReady_write
pub extern "C" fn ChannelReady_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ChannelReadyDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ChannelReady, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ChannelReady { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the Init object into a byte array which can be read by Init_read
pub extern "C" fn Init_write(obj: &crate::lightning::ln::msgs::Init) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn Init_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeInit) })
}
#[no_mangle]
/// Read a Init from a byte array, created by Init_write
pub extern "C" fn Init_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_InitDecodeErrorZ {
	let res: Result<lightning::ln::msgs::Init, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::Init { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the OpenChannel object into a byte array which can be read by OpenChannel_read
pub extern "C" fn OpenChannel_write(obj: &crate::lightning::ln::msgs::OpenChannel) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn OpenChannel_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeOpenChannel) })
}
#[no_mangle]
/// Read a OpenChannel from a byte array, created by OpenChannel_write
pub extern "C" fn OpenChannel_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_OpenChannelDecodeErrorZ {
	let res: Result<lightning::ln::msgs::OpenChannel, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::OpenChannel { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the RevokeAndACK object into a byte array which can be read by RevokeAndACK_read
pub extern "C" fn RevokeAndACK_write(obj: &crate::lightning::ln::msgs::RevokeAndACK) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn RevokeAndACK_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeRevokeAndACK) })
}
#[no_mangle]
/// Read a RevokeAndACK from a byte array, created by RevokeAndACK_write
pub extern "C" fn RevokeAndACK_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_RevokeAndACKDecodeErrorZ {
	let res: Result<lightning::ln::msgs::RevokeAndACK, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::RevokeAndACK { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the Shutdown object into a byte array which can be read by Shutdown_read
pub extern "C" fn Shutdown_write(obj: &crate::lightning::ln::msgs::Shutdown) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn Shutdown_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeShutdown) })
}
#[no_mangle]
/// Read a Shutdown from a byte array, created by Shutdown_write
pub extern "C" fn Shutdown_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ShutdownDecodeErrorZ {
	let res: Result<lightning::ln::msgs::Shutdown, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::Shutdown { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the UpdateFailHTLC object into a byte array which can be read by UpdateFailHTLC_read
pub extern "C" fn UpdateFailHTLC_write(obj: &crate::lightning::ln::msgs::UpdateFailHTLC) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn UpdateFailHTLC_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeUpdateFailHTLC) })
}
#[no_mangle]
/// Read a UpdateFailHTLC from a byte array, created by UpdateFailHTLC_write
pub extern "C" fn UpdateFailHTLC_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_UpdateFailHTLCDecodeErrorZ {
	let res: Result<lightning::ln::msgs::UpdateFailHTLC, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::UpdateFailHTLC { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the UpdateFailMalformedHTLC object into a byte array which can be read by UpdateFailMalformedHTLC_read
pub extern "C" fn UpdateFailMalformedHTLC_write(obj: &crate::lightning::ln::msgs::UpdateFailMalformedHTLC) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn UpdateFailMalformedHTLC_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeUpdateFailMalformedHTLC) })
}
#[no_mangle]
/// Read a UpdateFailMalformedHTLC from a byte array, created by UpdateFailMalformedHTLC_write
pub extern "C" fn UpdateFailMalformedHTLC_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_UpdateFailMalformedHTLCDecodeErrorZ {
	let res: Result<lightning::ln::msgs::UpdateFailMalformedHTLC, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::UpdateFailMalformedHTLC { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the UpdateFee object into a byte array which can be read by UpdateFee_read
pub extern "C" fn UpdateFee_write(obj: &crate::lightning::ln::msgs::UpdateFee) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn UpdateFee_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeUpdateFee) })
}
#[no_mangle]
/// Read a UpdateFee from a byte array, created by UpdateFee_write
pub extern "C" fn UpdateFee_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_UpdateFeeDecodeErrorZ {
	let res: Result<lightning::ln::msgs::UpdateFee, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::UpdateFee { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the UpdateFulfillHTLC object into a byte array which can be read by UpdateFulfillHTLC_read
pub extern "C" fn UpdateFulfillHTLC_write(obj: &crate::lightning::ln::msgs::UpdateFulfillHTLC) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn UpdateFulfillHTLC_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeUpdateFulfillHTLC) })
}
#[no_mangle]
/// Read a UpdateFulfillHTLC from a byte array, created by UpdateFulfillHTLC_write
pub extern "C" fn UpdateFulfillHTLC_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_UpdateFulfillHTLCDecodeErrorZ {
	let res: Result<lightning::ln::msgs::UpdateFulfillHTLC, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::UpdateFulfillHTLC { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the UpdateAddHTLC object into a byte array which can be read by UpdateAddHTLC_read
pub extern "C" fn UpdateAddHTLC_write(obj: &crate::lightning::ln::msgs::UpdateAddHTLC) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn UpdateAddHTLC_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeUpdateAddHTLC) })
}
#[no_mangle]
/// Read a UpdateAddHTLC from a byte array, created by UpdateAddHTLC_write
pub extern "C" fn UpdateAddHTLC_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_UpdateAddHTLCDecodeErrorZ {
	let res: Result<lightning::ln::msgs::UpdateAddHTLC, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::UpdateAddHTLC { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Read a OnionMessage from a byte array, created by OnionMessage_write
pub extern "C" fn OnionMessage_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_OnionMessageDecodeErrorZ {
	let res: Result<lightning::ln::msgs::OnionMessage, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::OnionMessage { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the OnionMessage object into a byte array which can be read by OnionMessage_read
pub extern "C" fn OnionMessage_write(obj: &crate::lightning::ln::msgs::OnionMessage) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn OnionMessage_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeOnionMessage) })
}
#[no_mangle]
/// Serialize the Ping object into a byte array which can be read by Ping_read
pub extern "C" fn Ping_write(obj: &crate::lightning::ln::msgs::Ping) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn Ping_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativePing) })
}
#[no_mangle]
/// Read a Ping from a byte array, created by Ping_write
pub extern "C" fn Ping_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_PingDecodeErrorZ {
	let res: Result<lightning::ln::msgs::Ping, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::Ping { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the Pong object into a byte array which can be read by Pong_read
pub extern "C" fn Pong_write(obj: &crate::lightning::ln::msgs::Pong) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn Pong_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativePong) })
}
#[no_mangle]
/// Read a Pong from a byte array, created by Pong_write
pub extern "C" fn Pong_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_PongDecodeErrorZ {
	let res: Result<lightning::ln::msgs::Pong, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::Pong { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the UnsignedChannelAnnouncement object into a byte array which can be read by UnsignedChannelAnnouncement_read
pub extern "C" fn UnsignedChannelAnnouncement_write(obj: &crate::lightning::ln::msgs::UnsignedChannelAnnouncement) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn UnsignedChannelAnnouncement_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeUnsignedChannelAnnouncement) })
}
#[no_mangle]
/// Read a UnsignedChannelAnnouncement from a byte array, created by UnsignedChannelAnnouncement_write
pub extern "C" fn UnsignedChannelAnnouncement_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_UnsignedChannelAnnouncementDecodeErrorZ {
	let res: Result<lightning::ln::msgs::UnsignedChannelAnnouncement, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::UnsignedChannelAnnouncement { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the ChannelAnnouncement object into a byte array which can be read by ChannelAnnouncement_read
pub extern "C" fn ChannelAnnouncement_write(obj: &crate::lightning::ln::msgs::ChannelAnnouncement) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ChannelAnnouncement_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeChannelAnnouncement) })
}
#[no_mangle]
/// Read a ChannelAnnouncement from a byte array, created by ChannelAnnouncement_write
pub extern "C" fn ChannelAnnouncement_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ChannelAnnouncementDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ChannelAnnouncement, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ChannelAnnouncement { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the UnsignedChannelUpdate object into a byte array which can be read by UnsignedChannelUpdate_read
pub extern "C" fn UnsignedChannelUpdate_write(obj: &crate::lightning::ln::msgs::UnsignedChannelUpdate) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn UnsignedChannelUpdate_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeUnsignedChannelUpdate) })
}
#[no_mangle]
/// Read a UnsignedChannelUpdate from a byte array, created by UnsignedChannelUpdate_write
pub extern "C" fn UnsignedChannelUpdate_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_UnsignedChannelUpdateDecodeErrorZ {
	let res: Result<lightning::ln::msgs::UnsignedChannelUpdate, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::UnsignedChannelUpdate { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the ChannelUpdate object into a byte array which can be read by ChannelUpdate_read
pub extern "C" fn ChannelUpdate_write(obj: &crate::lightning::ln::msgs::ChannelUpdate) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ChannelUpdate_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeChannelUpdate) })
}
#[no_mangle]
/// Read a ChannelUpdate from a byte array, created by ChannelUpdate_write
pub extern "C" fn ChannelUpdate_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ChannelUpdateDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ChannelUpdate, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ChannelUpdate { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the ErrorMessage object into a byte array which can be read by ErrorMessage_read
pub extern "C" fn ErrorMessage_write(obj: &crate::lightning::ln::msgs::ErrorMessage) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ErrorMessage_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeErrorMessage) })
}
#[no_mangle]
/// Read a ErrorMessage from a byte array, created by ErrorMessage_write
pub extern "C" fn ErrorMessage_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ErrorMessageDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ErrorMessage, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ErrorMessage { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the WarningMessage object into a byte array which can be read by WarningMessage_read
pub extern "C" fn WarningMessage_write(obj: &crate::lightning::ln::msgs::WarningMessage) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn WarningMessage_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeWarningMessage) })
}
#[no_mangle]
/// Read a WarningMessage from a byte array, created by WarningMessage_write
pub extern "C" fn WarningMessage_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_WarningMessageDecodeErrorZ {
	let res: Result<lightning::ln::msgs::WarningMessage, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::WarningMessage { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the UnsignedNodeAnnouncement object into a byte array which can be read by UnsignedNodeAnnouncement_read
pub extern "C" fn UnsignedNodeAnnouncement_write(obj: &crate::lightning::ln::msgs::UnsignedNodeAnnouncement) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn UnsignedNodeAnnouncement_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeUnsignedNodeAnnouncement) })
}
#[no_mangle]
/// Read a UnsignedNodeAnnouncement from a byte array, created by UnsignedNodeAnnouncement_write
pub extern "C" fn UnsignedNodeAnnouncement_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_UnsignedNodeAnnouncementDecodeErrorZ {
	let res: Result<lightning::ln::msgs::UnsignedNodeAnnouncement, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::UnsignedNodeAnnouncement { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the NodeAnnouncement object into a byte array which can be read by NodeAnnouncement_read
pub extern "C" fn NodeAnnouncement_write(obj: &crate::lightning::ln::msgs::NodeAnnouncement) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn NodeAnnouncement_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeNodeAnnouncement) })
}
#[no_mangle]
/// Read a NodeAnnouncement from a byte array, created by NodeAnnouncement_write
pub extern "C" fn NodeAnnouncement_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_NodeAnnouncementDecodeErrorZ {
	let res: Result<lightning::ln::msgs::NodeAnnouncement, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::NodeAnnouncement { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Read a QueryShortChannelIds from a byte array, created by QueryShortChannelIds_write
pub extern "C" fn QueryShortChannelIds_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_QueryShortChannelIdsDecodeErrorZ {
	let res: Result<lightning::ln::msgs::QueryShortChannelIds, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::QueryShortChannelIds { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the QueryShortChannelIds object into a byte array which can be read by QueryShortChannelIds_read
pub extern "C" fn QueryShortChannelIds_write(obj: &crate::lightning::ln::msgs::QueryShortChannelIds) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn QueryShortChannelIds_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeQueryShortChannelIds) })
}
#[no_mangle]
/// Serialize the ReplyShortChannelIdsEnd object into a byte array which can be read by ReplyShortChannelIdsEnd_read
pub extern "C" fn ReplyShortChannelIdsEnd_write(obj: &crate::lightning::ln::msgs::ReplyShortChannelIdsEnd) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ReplyShortChannelIdsEnd_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeReplyShortChannelIdsEnd) })
}
#[no_mangle]
/// Read a ReplyShortChannelIdsEnd from a byte array, created by ReplyShortChannelIdsEnd_write
pub extern "C" fn ReplyShortChannelIdsEnd_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ReplyShortChannelIdsEndDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ReplyShortChannelIdsEnd, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ReplyShortChannelIdsEnd { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
/// Calculates the overflow safe ending block height for the query.
///
/// Overflow returns `0xffffffff`, otherwise returns `first_blocknum + number_of_blocks`.
#[must_use]
#[no_mangle]
pub extern "C" fn QueryChannelRange_end_blocknum(this_arg: &crate::lightning::ln::msgs::QueryChannelRange) -> u32 {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.end_blocknum();
	ret
}

#[no_mangle]
/// Serialize the QueryChannelRange object into a byte array which can be read by QueryChannelRange_read
pub extern "C" fn QueryChannelRange_write(obj: &crate::lightning::ln::msgs::QueryChannelRange) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn QueryChannelRange_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeQueryChannelRange) })
}
#[no_mangle]
/// Read a QueryChannelRange from a byte array, created by QueryChannelRange_write
pub extern "C" fn QueryChannelRange_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_QueryChannelRangeDecodeErrorZ {
	let res: Result<lightning::ln::msgs::QueryChannelRange, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::QueryChannelRange { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Read a ReplyChannelRange from a byte array, created by ReplyChannelRange_write
pub extern "C" fn ReplyChannelRange_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_ReplyChannelRangeDecodeErrorZ {
	let res: Result<lightning::ln::msgs::ReplyChannelRange, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::ReplyChannelRange { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the ReplyChannelRange object into a byte array which can be read by ReplyChannelRange_read
pub extern "C" fn ReplyChannelRange_write(obj: &crate::lightning::ln::msgs::ReplyChannelRange) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn ReplyChannelRange_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeReplyChannelRange) })
}
#[no_mangle]
/// Serialize the GossipTimestampFilter object into a byte array which can be read by GossipTimestampFilter_read
pub extern "C" fn GossipTimestampFilter_write(obj: &crate::lightning::ln::msgs::GossipTimestampFilter) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn GossipTimestampFilter_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeGossipTimestampFilter) })
}
#[no_mangle]
/// Read a GossipTimestampFilter from a byte array, created by GossipTimestampFilter_write
pub extern "C" fn GossipTimestampFilter_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_GossipTimestampFilterDecodeErrorZ {
	let res: Result<lightning::ln::msgs::GossipTimestampFilter, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::ln::msgs::GossipTimestampFilter { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
