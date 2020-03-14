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
  "token": "vmcG6sPriniFUvFcyIKNI1kjyW4NIErHrByovYbZ7HQWMBCh0Y19tgge43dmISgb",
  "userid": {
    "$oid": "5e626bd89158f3ecad052a8e"
  },
  "expiration": "2020-04-13T18:56:20.055234Z"
}
```

This endpoint is used to get an authentication token.

### HTTP Request

`POST /api/login`

## Registration

> Request:

```json
{
    "name": "user2",
    "password": "user2"
}
```

> Response:

```json
{
  "token": "Fih77K6t3EsMBS85Rv6VyxC9nVaSLOMCC6MOfwbcZ7bRzSve9GeKnRv0vrkn2yCv",
  "userid": {
    "$oid": "5e6d28b700c49dba00213a92"
  },
  "expiration": "2020-04-13T18:55:51.094616900Z"
}
```

This endpoint is used to create a user and get an authentication token for the created user.

### HTTP Request

`POST /api/register`

## Me

> Response:

```json
{
  "_id": {
    "$oid": "5e626bd89158f3ecad052a8e"
  },
  "name": "user1",
  "icon": "icon.here",
  "status": "I'm stuff"
}
```

This endpoint is used to get information about the current user.

### HTTP Request

`GET /api/me`

<aside class="notice">
This method requires authentication.
</aside>