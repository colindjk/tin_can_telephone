tct: tin_can_telephone
======================
a stateless back-end web service with the purpose of
                     connecting multiple clients' users via either TCP or HTTP
                     connections, ideally running asynchronously.

First Task: How do you do that?

Well form a TCP Connection with the messaging service.

Then use this predefined format for sending messages :

```json
    { "<Type>" : { ... } }
```

Currently supported JSON objects to be read by server :
```json

{ "Message" : { "to": "<UserID>", "from": "<UserID>", "msg": "Hello world" } }

{ "GroupMessage" : { "to": "<UserID>", "from": "<UserID>", "msg": "Hello world" } }

{ "Request" : { "to": "<UserID>", "from": "<UserID>", "kind": "<RequestKind>" } }

{ "Response" : { "to": "<UserID>", "from": "<UserID>", "kind": "<ResponseKind>" } }

{ "Register" : { "user": "<UserID>", "psw": "password" } }

{ "RegisterGroup" : { "group": "<UserID>", "admin": "<UserID>" } }

{ "LoginCredentials" : { "user": "<UserID>", "psw": "password" } }

{ "Error" : "Hello error!" }

{ "EOF" }

```

Note: The "error" json object is only used for the server itself, and is what
      is produced when there was trouble parsing.

RequestKind : This field evaluates to a string, currently supported requests:

```javascript
"UserInfo"      // get info about a particular user
"ChatHistory"   // get chat history between a 'to' and 'from'
"GroupHistory"  // group history for 'to'
"GroupInvite"   // Invites a 'to'
"Friends"       // Shows friends
```

ResponseKind : Produces a JSON object with a single field, which
               is the name of the request.

The value they are set equal to is either an array or a hashmap. Arrays will
be used for *History, as messages occur in linear time. HashMaps will be used
for information regarding a particular user / group.

```json
"UserInfo":[]
"ChatHistory":[]
"GroupHistory":[]
"GroupInvite":int // Accepted or declined? true or false / 1 or 0.
"Friends":[]
```


