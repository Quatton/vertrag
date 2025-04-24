use async_trait::async_trait;
use axum::{
    body::Body,
    http::{Method, StatusCode},
    response::Response,
};
use axum_extra::extract::{CookieJar, Host};
use openapi::{
    apis::{
        ErrorHandler,
        default::{Default, ProductsPeriodListResponse},
    },
    models,
};

#[derive(Clone)]
pub struct ApiImpl {}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        self
    }
}

#[async_trait]
impl Default for ApiImpl {
    async fn products_period_list(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _query: &models::ProductsPeriodListQueryParams,
    ) -> Result<ProductsPeriodListResponse, ()> {
        return Ok(ProductsPeriodListResponse::Status200_OK(
            models::ProductsList200Response {
                products: vec![
                    models::ProductsList200ResponseProductsInner {
                        id: 2.to_string(),
                        name: "Product 1".to_string(),
                        price: 10.0,
                        description: "Description of Product 1".to_string(),
                    },
                    models::ProductsList200ResponseProductsInner {
                        id: 2.to_string(),
                        name: "Product 2".to_string(),
                        price: 20.0,
                        description: "Description of Product 2".to_string(),
                    },
                ],
                limit: 10.0,
                total: 2.0,
                page: 1.0,
            },
        ));
    }
}

#[async_trait]
impl ErrorHandler for ApiImpl {
    async fn handle_error(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _error: (),
    ) -> Result<Response, StatusCode> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}
