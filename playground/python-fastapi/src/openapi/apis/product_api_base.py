# coding: utf-8

from typing import ClassVar, Dict, List, Tuple  # noqa: F401

from pydantic import StrictFloat, StrictInt
from typing import Optional, Union
from openapi.models.products_create200_response import ProductsCreate200Response
from openapi.models.products_create_request import ProductsCreateRequest
from openapi.models.products_list200_response import ProductsList200Response


class BaseProductApi:
    subclasses: ClassVar[Tuple] = ()

    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        BaseProductApi.subclasses = BaseProductApi.subclasses + (cls,)
    async def products_create(
        self,
        products_create_request: ProductsCreateRequest,
    ) -> ProductsCreate200Response:
        ...


    async def products_list(
        self,
        page: Optional[Union[StrictFloat, StrictInt]],
        limit: Optional[Union[StrictFloat, StrictInt]],
    ) -> ProductsList200Response:
        ...
