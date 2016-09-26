tin_can_telephone
=================
a stateless back-end web service with the purpose of
                     connecting multiple clients' users via either TCP or HTTP
                     connections, ideally running asynchronously.

First Task: How do you do that?

Well first lets just try to talk...

Part 1: Talking to my application -> Conduct a conversation via HTTP requests.

Okay so after searching for a basic outline of a server, I found one in the 
    actual documentation for RUSTful... :D. I made a copy of one which can
    be located in "src/example.rs"; if you'll notice I make an interface which
    reflects the Smash Bros. user interface.

Before checking that out though, here's some information which I found
    valuable whilst researching server and router setup.

So what are all these types?

TreeRouter\<MethodRouter\<Variables>> ~= Do\<What\<Where>>

TreeRouter: 
    Tree shaped router (think -> <i>tree</i>) which selects handlers using
    paths. Think of the MethodRouter & Variables as keys.
MethodRouter:
    Mapping between an http method* and a router 'T', where the router is what
    stores a **Method** and **Variable**. Also remember, 'method'
    as in HTTP method, GET, POST, PUT, etc..
Variables:
    The path variables! Just the path that will be a part of the 'url' so
    to speak.

TreeRouter<MethodRouter<Variables>> Example:

~~~rust
    let mut router = TreeRouter::new();
    
    router.insert(Get, "/", index); // where index is the <i>Handler</i>
    router.insert(Get, "/users", show_peeps);
    router.insert(Get, "/pr", show_power_ranking);
    router.insert(Get, "/about", about_yoshi);

    router.insert(Get, "/*", show_error); // i.e. unknown path / extension...
~~~

Ideally the API will function as any RESTful API should. A client will make
  method call for a particular path or 'Variable', and receive an HTTP response
  of some sort.

Message streams?



