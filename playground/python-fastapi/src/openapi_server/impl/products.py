from typing import List, Optional, Union

from pydantic import StrictFloat, StrictInt
from openapi.apis.product_api_base import BaseProductApi
from openapi.models.products_create200_response import ProductsCreate200Response
from openapi.models.products_list200_response import ProductsList200Response
from openapi.models.products_list200_response_products_inner import (
    ProductsList200ResponseProductsInner as Product,
)
from openapi.models.products_create_request import ProductsCreateRequest

database: List[Product] = []


class ProductApi(BaseProductApi):
    async def products_create(
        self,
        products_create_request: ProductsCreateRequest,
    ) -> ProductsCreate200Response:
        database.append(
            Product(
                id=str(len(database)),
                name=products_create_request.name,
                description=products_create_request.description,
                price=products_create_request.price,
                discountedPrice=products_create_request.discounted_price,
            )
        )
        return ProductsCreate200Response(success=True)

    async def products_list(
        self,
        page: Optional[Union[StrictFloat, StrictInt]],
        limit: Optional[Union[StrictFloat, StrictInt]],
    ) -> ProductsList200Response:
        return ProductsList200Response(
            products=database, page=1, limit=len(database), total=len(database)
        )
