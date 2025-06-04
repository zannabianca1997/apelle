# `gateway`

The main entry point of the constellation. All the traffic, both inbound and
outbound, goes through this service.

It is currently and `nginx` instance.

## Inbound traffic

All traffic is redirected from the public endpoint to the `/public` endpoint of
the corresponding service. For example, requests to `/users` will be forwarded
to `http://users:8080/public`.

Before the request is forwarded, a subrequest is made to the `/auth` endpoint of
the user service, that will authenticate the request and respond with the
`Apelle-User-*` headers, that the gateway will copy on the request before it is
forwarded.

## Outbound traffic

Outbound traffic is routed through corresponding location, for example
`/googleapis` toward `https://googleapis.com`. Aside isolating the
constellation, this handles https encription avoid having to install a SSL
implementation in the single service.