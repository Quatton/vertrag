#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ProductsPeriodListQueryParams {
                #[serde(rename = "page")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub page: Option<f64>,
                #[serde(rename = "limit")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub limit: Option<f64>,
    }



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProductsList200Response {
    #[serde(rename = "products")]
    pub products: Vec<models::ProductsList200ResponseProductsInner>,

    #[serde(rename = "total")]
    pub total: f64,

    #[serde(rename = "page")]
    pub page: f64,

    #[serde(rename = "limit")]
    pub limit: f64,

}





impl ProductsList200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(products: Vec<models::ProductsList200ResponseProductsInner>, total: f64, page: f64, limit: f64, ) -> ProductsList200Response {
        ProductsList200Response {
            products,
            total,
            page,
            limit,
        }
    }
}

/// Converts the ProductsList200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ProductsList200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping products in query parameter serialization


            Some("total".to_string()),
            Some(self.total.to_string()),


            Some("page".to_string()),
            Some(self.page.to_string()),


            Some("limit".to_string()),
            Some(self.limit.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ProductsList200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ProductsList200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub products: Vec<Vec<models::ProductsList200ResponseProductsInner>>,
            pub total: Vec<f64>,
            pub page: Vec<f64>,
            pub limit: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ProductsList200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "products" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductsList200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "total" => intermediate_rep.total.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "page" => intermediate_rep.page.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ProductsList200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ProductsList200Response {
            products: intermediate_rep.products.into_iter().next().ok_or_else(|| "products missing in ProductsList200Response".to_string())?,
            total: intermediate_rep.total.into_iter().next().ok_or_else(|| "total missing in ProductsList200Response".to_string())?,
            page: intermediate_rep.page.into_iter().next().ok_or_else(|| "page missing in ProductsList200Response".to_string())?,
            limit: intermediate_rep.limit.into_iter().next().ok_or_else(|| "limit missing in ProductsList200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ProductsList200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ProductsList200Response>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ProductsList200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ProductsList200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ProductsList200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ProductsList200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ProductsList200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProductsList200ResponseProductsInner {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "price")]
    pub price: f64,

    #[serde(rename = "discountedPrice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub discounted_price: Option<f64>,

}





impl ProductsList200ResponseProductsInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, name: String, description: String, price: f64, ) -> ProductsList200ResponseProductsInner {
        ProductsList200ResponseProductsInner {
            id,
            name,
            description,
            price,
            discounted_price: None,
        }
    }
}

/// Converts the ProductsList200ResponseProductsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ProductsList200ResponseProductsInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),


            Some("price".to_string()),
            Some(self.price.to_string()),


            self.discounted_price.as_ref().map(|discounted_price| {
                [
                    "discountedPrice".to_string(),
                    discounted_price.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ProductsList200ResponseProductsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ProductsList200ResponseProductsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub price: Vec<f64>,
            pub discounted_price: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ProductsList200ResponseProductsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "price" => intermediate_rep.price.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "discountedPrice" => intermediate_rep.discounted_price.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ProductsList200ResponseProductsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ProductsList200ResponseProductsInner {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in ProductsList200ResponseProductsInner".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ProductsList200ResponseProductsInner".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in ProductsList200ResponseProductsInner".to_string())?,
            price: intermediate_rep.price.into_iter().next().ok_or_else(|| "price missing in ProductsList200ResponseProductsInner".to_string())?,
            discounted_price: intermediate_rep.discounted_price.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ProductsList200ResponseProductsInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ProductsList200ResponseProductsInner>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ProductsList200ResponseProductsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ProductsList200ResponseProductsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ProductsList200ResponseProductsInner> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ProductsList200ResponseProductsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ProductsList200ResponseProductsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



