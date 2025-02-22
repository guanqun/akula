#![feature(
    bool_to_option,
    destructuring_assignment,
    generic_associated_types,
    never_type,
    trait_alias,
    min_type_alias_impl_trait
)]
#![recursion_limit = "256"]
#![allow(
    incomplete_features,
    clippy::mutable_key_type,
    clippy::unused_io_amount
)]

pub mod accessors;
pub mod adapter;
mod changeset;
mod common;
mod dbutils;
mod kv;
mod models;
pub mod stagedsync;
pub mod stages;
mod state;
pub mod txdb;

pub use changeset::ChangeSet;
pub use dbutils::{tables, DupSort, Table};
pub use kv::{
    mdbx::*,
    new_mem_database,
    remote::{kv_client::KvClient as RemoteKvClient, RemoteCursor, RemoteTransaction},
    traits::{
        Cursor, CursorDupSort, MutableCursor, MutableCursorDupSort, MutableTransaction, Transaction,
    },
};
pub use stagedsync::stages::SyncStage;
pub use state::*;
