# Authorization

For some Azuma endpoints, you have to authorize yourself. Endpoints which require authorization are marked like this:

!!! info "Authorization required"

## Bearer Authorization

Azuma uses Bearer authentication. To authorize, send the `Authorization`-Header with every request:

```
Authorization: Bearer TOKEN
```