# `users`

This is the authentication service, handling the user registration and authentications.
It is commonly reachable under the `/users` endpoint

## Private endpoints

### `GET /auth`
This endpoint handles user authentication. It parses the `Authorization` header
and responds with an appropriate error code, or with 200 if authentication data
is correct.

It also sets the `Apelle-User-*` headers on the response, that must be copied on
the request before it is forwarded to its true destination. See the `gateway`
service configuration on how `nginx` handles this.


## Public endpoints

### `POST /public/users`
Creates a new user

### `/public/users/me`
`GET`s, `PATCH`es and `DELETE`s the current user.