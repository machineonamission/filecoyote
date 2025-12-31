# backend

this entire module (and all submodules) are not visible at all to the client. this is where server-only functions like
databases go and where server-only dependencies can be included. code from here can only be safely used in dioxus server
functions that are able to hide their use from the client.

though the client is visible to the backend