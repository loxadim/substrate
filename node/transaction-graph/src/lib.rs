// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.


// #![warn(missing_docs)]
// #![warn(unused_crates)]

extern crate substrate_client as client;
extern crate parity_codec as codec;
extern crate substrate_primitives;
extern crate sr_primitives;
extern crate node_runtime as runtime;
extern crate node_primitives as primitives;
extern crate node_api;
extern crate parking_lot;

#[cfg(test)]
extern crate substrate_keyring;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

mod future;
mod pool;
mod ready;

pub use self::pool::{Transaction, Pool};