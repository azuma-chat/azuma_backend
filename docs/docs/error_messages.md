# Error Messages

Errors are returned with a 4XX or 5XX code and the following format:

```json
{
    "code": 418,
    "message": "IM_A_TEAPOT"
}
```

## Possible Errors

### Not Found

This error occurs when you try to access something that doesn't exist.

```json
{
    "code": 404,
    "message": "NOT_FOUND"
}
```

### Already Exists

This error occurs when you try to create something that already exists.

```json
{
    "code": 400,
    "message": "ALREADY_EXISTS"
}
```

### Internal Server Error

This error occurs when something in the backend goes wrong. Try contacting the person which hosts the backend.

```json
{
    "code": 500,
    "message": "INTERNAL_SERVER_ERROR"
}
```

### Unauthorized

This error occurs when you try to access something that you aren't authorized to access. Make sure you're sending a valid [authorization token](/authorization).

```json
{
    "code": 401,
    "message": "UNAUTHORIZED"
}
```

### Unhandled Rejection

This error occurs when the backend wasn't able to categorise an error. Please create a GitHub Issue for it so we can fix it. 

```json
{
    "code": 500,
    "message": "UNHANDLED_REJECTION"
}
```