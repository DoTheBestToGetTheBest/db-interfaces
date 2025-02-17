#![feature(associated_type_defaults)]

pub mod clickhouse;
pub mod errors;
pub mod params;
pub mod tables;

#[cfg(feature = "alloy-types")]
pub mod alloy_types;

#[cfg(feature = "test-utils")]
pub mod test_utils;

use clickhouse::types::ClickhouseQuery;
pub use db_interfaces_macros::remote_clickhouse_table;
use errors::DatabaseError;
use params::BindParameters;
use tables::*;

#[async_trait::async_trait]
pub trait Database: Sync + Send {
    type DBMS;

    async fn insert_one<T: DatabaseTable>(&self, value: &T::DataType) -> Result<(), DatabaseError>;

    async fn insert_many<T: DatabaseTable>(
        &self,
        values: &[T::DataType],
    ) -> Result<(), DatabaseError>;

    async fn query_one<Q: DatabaseQuery, P: BindParameters>(
        &self,
        query: impl AsRef<str> + Send,
        params: &P,
    ) -> Result<Q, DatabaseError>;

    async fn query_one_optional<Q: DatabaseQuery, P: BindParameters>(
        &self,
        query: impl AsRef<str> + Send,
        params: &P,
    ) -> Result<Option<Q>, DatabaseError>;

    async fn query_many<Q: DatabaseQuery, P: BindParameters>(
        &self,
        query: impl AsRef<str> + Send,
        params: &P,
    ) -> Result<Vec<Q>, DatabaseError>;

    async fn query_raw<Q: DatabaseQuery, P: BindParameters>(
        &self,
        query: impl AsRef<str> + Send,
        params: &P,
    ) -> Result<Vec<u8>, DatabaseError>;

    async fn execute_remote<P: BindParameters>(
        &self,
        query: impl AsRef<str> + Send,
        params: &P,
    ) -> Result<(), DatabaseError>;
}

pub trait DatabaseQuery: ClickhouseQuery {}
