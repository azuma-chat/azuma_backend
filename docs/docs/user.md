# User

All endpoints related to users are documented here.

## Login

This endpoint returns an [authorization token](/authorization) for a user.

```bash
POST /login
```

#### Request

Parameter | Required | Description
--- | :---: | ---
name | ✅ | The name of the user you want to log in.
password | ✅ | The password of the user you want to log in.

#### Response

```json
{
    "id": 1,
    "token": "vmcG6sPriniFUvFcyIKNI1kjyW4NIErHrByovYbZ7HQWMBCh0Y19tgge43dmISgb",
    "userid": 1,
    "expiration": "2020-04-13T18:56:20.055234Z"
}
```

!!! danger "Possible Errors: [Not Found](/error_messages/#not-found), [Internal Server Error](/error_messages/#internal-server-error), [Unauthorized](/error_messages/#unauthorized)"

## Register

This endpoint creates a user and returns an [authorization token](/authorization) for the user.

```bash
POST /register
```

#### Request

Parameter | Required | Description
--- | :---: | ---
name | ✅ | The name of the user you want to register.
password | ✅ | The password of the user you want to register.

#### Response

```json
{
    "id": 1,
    "token": "Fih77K6t3EsMBS85Rv6VyxC9nVaSLOMCC6MOfwbcZ7bRzSve9GeKnRv0vrkn2yCv",
    "userid": 1,
    "expiration": "2020-04-13T18:55:51.094616900Z"
}
```

!!! danger "Possible Errors: [Not Found](/error_messages/#not-found), [Already Exists](/error_messages/#already-exists), [Internal Server Error](/error_messages/#internal-server-error), [Unauthorized](/error_messages/#unauthorized)"

## Me

This endpoint returns data about a user.

```bash
GET /me
```

!!! info "Authorization required"

#### Response

```json
{
    "_id": 1,
    "name": "user1",
    "icon": "icon.here",
    "status": "I'm stuff"
}
```

!!! danger "Possible Errors: [Not Found](/error_messages/#not-found), [Internal Server Error](/error_messages/#internal-server-error), [Unauthorized](/error_messages/#unauthorized)"