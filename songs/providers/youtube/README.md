# `songs-youtube`

Provider for songs coming from Youtube. See the `songs` service for the
description of the provider API this serves under the `/provider` endpoint. 

The URN of the youtube source is `urn:apelle:sources/youtube`.

## Needed configuration

Aside the usual database URL, this service requires the URL for the `songs`
service and the URL the song service can reach this service.

Additionally, an URL where the google apis can be reached must be provided. Take
care that while the APIs require https, this service uses only http. A reverse
proxy must be put in the middle to encrypt the traffic (see the `gateway`
service configuration on how nginx takes care of this).

Finally, it needs a google API key. It need access to the Youtube v3 APIs, at
least for listing videos and searching.