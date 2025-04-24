# coding: utf-8

from fastapi.testclient import TestClient


from pydantic import StrictFloat, StrictInt  # noqa: F401
from typing import Optional, Union  # noqa: F401
from openapi.models.products_list200_response import ProductsList200Response  # noqa: F401


def test_products_list(client: TestClient):
    """Test case for products_list

    
    """
    params = [("page", 3.4),     ("limit", 3.4)]
    headers = {
    }
    # uncomment below to make a request
    #response = client.request(
    #    "GET",
    #    "/products",
    #    headers=headers,
    #    params=params,
    #)

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200

