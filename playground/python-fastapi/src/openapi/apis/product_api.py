# coding: utf-8

from typing import Dict, List  # noqa: F401
import importlib
import pkgutil

from openapi_server.apis.product_api_base import BaseProductApi
import openapi_server.impl

from fastapi import (  # noqa: F401
    APIRouter,
    Body,
    Cookie,
    Depends,
    Form,
    Header,
    HTTPException,
    Path,
    Query,
    Response,
    Security,
    status,
)

from openapi_server.models.extra_models import TokenModel  # noqa: F401
from pydantic import StrictFloat, StrictInt
from typing import Optional, Union
from openapi_server.models.products_list200_response import ProductsList200Response


router = APIRouter()

ns_pkg = openapi_server.impl
for _, name, _ in pkgutil.iter_modules(ns_pkg.__path__, ns_pkg.__name__ + "."):
    importlib.import_module(name)


@router.get(
    "/products",
    responses={
        200: {"model": ProductsList200Response, "description": "OK"},
    },
    tags=["Product"],
    response_model_by_alias=True,
)
async def products_list(
    page: Optional[Union[StrictFloat, StrictInt]] = Query(None, description="", alias="page"),
    limit: Optional[Union[StrictFloat, StrictInt]] = Query(None, description="", alias="limit"),
) -> ProductsList200Response:
    if not BaseProductApi.subclasses:
        raise HTTPException(status_code=500, detail="Not implemented")
    return await BaseProductApi.subclasses[0]().products_list(page, limit)
