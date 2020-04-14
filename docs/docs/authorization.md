# Authorization

For some Azuma endpoints, you have to authorize yourself. Endpoints which require authorization are marked like this:

!!! info "Authorization required"

## Bearer Authorization

Azuma uses Bearer authentication. To authorize, send the `Authorization`-Header with every request:

```
Authorization: Bearer TOKEN
```

## Refresh Tokens

When the current authentication token is gonna expire in the next 7 days, Azuma automatically issues a new one. The reply will contain the Authorization-Header with a new token:

```
Authorization: Bearer NEW_TOKEN
```

!!! warning
    Make sure to check every response for the Authorization-Header.