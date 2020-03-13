---
title: Azuma API Reference

search: true
---

# Introduction

Welcome to the Azuma API! With the Azuma API, you can interact with the Azuma Chat Service - for example, you can create your own client.

# Authentication

> When using header authentication, which is usually not automatically handled by request clients, use it like this:

```shell
curl "AZUMA_HOST"
    -H "Azuma-Session: TOKEN"
```

There are two ways to authenticate:

* By cookie: A cookie with the token is sent with every request
* By header: A header with the token is sent with every request

If you're using cookie authentication, a cookie with the key `azuma_session` and the authentication token as value should be sent with every request.

If you're using header authentication, the `Azuma-Session`-Header with the authentication token as value should be sent with every request.

## Token Refresh
When the current authentication token is gonna expire in the next 7 days, Azuma automatically issues a new one.
How this token is returned is based on the request:

* If cookie authentication was used in the request, Azuma is gonna reply with a `Set-Cookie`-Header, which is automatically gonna set the cookie to the new token.
* If header authentication was used in the request, the reply will contain the `Azuma-Session`-Header with a new token.

<aside class="notice">
When using header authentication, make sure to check every response for the <code>Azuma-Session</code>-Header.
</aside>

# API

## API Version

> Response:

```json
{
    "version": "0.0.0"
}
```

This endpoint is used to get the API version.

### HTTP Request

`GET /api/`

# User

## Login

> Request:

```json
{
    "name": "user1",
    "password": "user1"
}
```

> Response:

```json
{
    "token": "2tKMuDkWYpz78OVkiEJ5GJDc6EVnUjbgPebney7fdEs2J4daTDzekEoDsU2G9u3B",
    "userid": 1,
    "expiration": "2020-04-11T21:48:25.939393900Z"
}
```

This endpoint is used to get an authentication token.

### HTTP Request

`POST /api/login`

## Me

> Response:

```json
{
    "id": 1,
    "name": "user1",
    "icon": "icon.url.here",
    "status": "I'm stuff"
}
```

This endpoint is used to get information about the current user.

### HTTP Request

`GET /api/me`

<aside class="notice">
This method requires authentication.
</aside>