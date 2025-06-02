# `songs`

This is the service responsible for handling song data, and recovering them from
multiple sources, identified by an urn. Multiple _providers_ can register
themselves and will be queried when a song request arrive with that urn (see the
_provider protocol_).

## Private endpoints

## `/sources`
This endpoint can be used to register new sources using `POST`.

## `/providers`
This endpoint can be user to register itself as a provider for some sources


## The provider protocol

Every provider should first of all ensure all the sources he want to provide for
are known to the songs service. It should make a POST request for each one to
the `/sources` endpoint, and ensure all return 204. This action is idempotent,
so repeating it is not a problem.

Once done, it should register itself as a provider. A POST request is made to
the `/providers` endpoint, that will then check if the provided webhook is
working. If successfull the provider can then wait for demands.

From then on the service is consumed only through the url provided, that should
serve the API described in the following paragraph

### `GET  /`
Simple healthchek. Should return `204 No Content` or `200 Ok` the body of which
is ignored

### `POST /retrieve?public={true|false}`
Request for searching a song. Arbitrary data are included as provided from the
frontend. Should return a dto of the `SearchResponse` type (see
`apelle-songs-dtos`). If `public` is set, the `public` field should be populated
with the data that would be returned from the `GET /songs/{id}` request.

If the service need to store additional data, they can be provided in the
`callback` field. A subsequent call to the `PUT` endpoint will contain those as
a body.

### `PUT  /songs/{id}`
Signal that a songs has been created through this service data. If `callback`
was populated, this call will contain that data as a body.

### `GET  /songs/{id}`
Request the data bound to a specific songs. Should have the same format to the
`public` field returned from the first search. 

