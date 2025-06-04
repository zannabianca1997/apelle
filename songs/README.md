# `songs`

This is the service responsible for handling song data, and recovering them from
multiple sources, identified by an urn. Multiple _providers_ can register
themselves and will be queried when a song request arrive with that urn (see the
_provider protocol_ in the `providers` directory).

## Private endpoints

### `POST /sources`
This endpoint can be used to register new sources.

### `POST /providers`
This endpoint can be user to register itself as a provider for a group of
sources.

## Public endpoints

### `GET /sources?[page_size={int}]&[page={string}]`
List of sources known to the service.

### `GET /search?q={string}&[page_size={int}]&[page={string}]&[source={string}[&...]]`
Search for a song in all the sources. Should return a paginated list of songs.
If `page` is specified, then it should come from a previous query `next` or
`prev` fields.

If `source` is specified, then only that source will be used. Can be specified
several times to search in multiple sources.

### `POST /resolve`
Request for resolving a song. Arbitrary data are included as provided from
`/search`. Should redirect to the `/solved` endpoint for the song with a 301
status code.

### `GET /solved/{id}`
Obtain a song that has been resolved by the service.

### `DELETE /solved/{id}`
Ask the service to delete the data associated to a song



