"""Demo of a serverless app using `wasi-http` to handle inbound HTTP requests.

This demonstrates how to use WASI's asynchronous capabilities to manage multiple
concurrent requests and streaming bodies.  It uses a custom `asyncio` event loop
to thread I/O through coroutines.
"""

import asyncio
import hashlib
import poller
import json

from clustering import exports
from clustering.types import Ok
from clustering.imports import (types, clustering)
from clustering.imports.types import (
    MethodGet, MethodPost, Scheme, SchemeHttp, SchemeHttps, SchemeOther, IncomingRequest, ResponseOutparam,
    OutgoingResponse, Fields, OutgoingBody, OutgoingRequest
)
from poller import Stream, Sink, PollLoop
from typing import Tuple
from urllib import parse

class IncomingHandler(exports.IncomingHandler):
    """Implements the `export`ed portion of the `wasi-http` `proxy` world."""

    def handle(self, request: IncomingRequest, response_out: ResponseOutparam):
        """Handle the specified `request`, sending the response to `response_out`.

        """
        # Dispatch the request using `asyncio`, backed by a custom event loop
        # based on WASI's `poll_oneoff` function.
        loop = PollLoop()
        asyncio.set_event_loop(loop)
        loop.run_until_complete(handle_async(request, response_out))

async def handle_async(request: IncomingRequest, response_out: ResponseOutparam):
    """Handle the specified `request`, sending the response to `response_out`."""

    method = request.method()
    path = request.path_with_query()
    headers = request.headers().entries()

    if isinstance(method, MethodPost) and path == "/cluster":
        response = OutgoingResponse(Fields.from_list(list(filter(lambda pair: pair[0] == "content-type", headers))))

        response_body = response.body()


        ResponseOutparam.set(response_out, Ok(response))

        stream = Stream(request.consume())
        sink = Sink(response_body)

        while True:
            chunk = await stream.next()
            if chunk is None:
                break
            else:
                await sink.send(bytes(f"{run_cluster(chunk)}\n", "utf-8"))
        sink.close()
    else:
        response = OutgoingResponse(Fields.from_list([]))
        response.set_status_code(400)
        body = response.body()
        ResponseOutparam.set(response_out, Ok(response))
        OutgoingBody.finish(body, None)

def run_cluster(data: bytes) -> str:
    json_str = data.decode('utf-8')
    json_data = json.loads(json_str)
    points = json_data['points']
    n_clusters = json_data['n_clusters']
    clusters = clustering.run(points, n_clusters)
    return clusters
