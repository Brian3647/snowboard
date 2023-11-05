# TLS Example

Note that this will NOT work unless an `identity.pfx` file is created.
You can create one using `openssl`:

```sh
$ openssl genrsa -out key.pem 2048 && \
    openssl req -new -x509 -key key.pem -out cert.pem -days 365 && \
    openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem

# You will get prompted with some personal info for security.
```

In this example, the password "1234" is used, but if you want to make a safe server,
consider getting one generated with a password generator like [avast's](https://www.avast.com/random-password-generator).
(not sponsored, just the first web I found)

You might also get something like this:

```rs
Error: Custom { kind: Other, error: Error { code: ErrorCode(1), cause: Some(Ssl(ErrorStack([Error { code: ..., library: "SSL routines", function: "ssl3_read_bytes", reason: "sslv3 alert bad certificate", file: "ssl/record/rec_layer_s3.c", line: ..., data: "SSL alert number 42" }]))) } }
```

this is probably due to a self-signed certificate. If you persistently have this error, please do open an issue.
