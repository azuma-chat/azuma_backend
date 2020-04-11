---
title: Azuma API Reference

search: true
---

# Introduction

Welcome to the Azuma API! With the Azuma API, you can interact with the Azuma Chat Service - for example, you can create your own client.

# Authentication

> You can authorize like this:

```shell
curl "AZUMA_HOST"
    -H "Authorization: Bearer TOKEN"
```

Azuma uses Bearer authentication. To authorize, send the `Authentication`-Header with every request.

## Token Refresh
When the current authentication token is gonna expire in the next 7 days, Azuma automatically issues a new one. The reply will contain the `Authorization`-Header with a new token.

<aside class="notice">
Make sure to check every response for the <code>Authorization</code>-Header.
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