# coding: utf-8

from typing import ClassVar, Dict, List, Tuple  # noqa: F401

from pydantic import StrictFloat, StrictInt
from typing import Optional, Union
from openapi_server.models.products_list200_response import ProductsList200Response


class BaseProductApi:
    subclasses: ClassVar[Tuple] = ()

    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        BaseProductApi.subclasses = BaseProductApi.subclasses + (cls,)
    async def products_list(
        self,
        page: Optional[Union[StrictFloat, StrictInt]],
        limit: Optional[Union[StrictFloat, StrictInt]],
    ) -> ProductsList200Response:
        ...
