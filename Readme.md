tct: tin_can_telephone
======================
a stateless back-end web service with the purpose of
                     connecting multiple clients' users via either TCP or HTTP
                     connections, ideally running asynchronously.

First Task: How do you do that?

Well form a TCP Connection with the messaging service.

Then use this predefined format for sending messages :

```json
    { "<type>" : { ... } }
```

Currently supported JSON objects to be read by server :
```json

    { "message" : { to: "<UserID>", from: "<UserID>", msg: "Hello world" } }

    { "groupMessage" : { to: "<UserID>", from: "<UserID>", msg: "Hello world" } }

    { "request" : { to: "<UserID>", from: "<UserID>", kind: RequestKind } }

    { "response" : { to: "<UserID>", from: "<UserID>", kind: ResponseKind } }

    { "register" : { from: "<UserID>", psw: "password" } }

    { "loginCredentials" : { from: "<UserID>", psw: "password" } }

    { "error" : "" }

    { "EOF" }

```

Note: The "error" json object is only used for the server itself, and is what
      is produced when there was trouble parsing.


