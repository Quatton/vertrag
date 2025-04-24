use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ProductsPeriodListResponse {
    /// OK
    Status200_OK
    (models::ProductsList200Response)
}


/// Product
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Product<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// ProductsPeriodList - GET /products
    async fn products_period_list(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::ProductsPeriodListQueryParams,
    ) -> Result<ProductsPeriodListResponse, E>;
}
