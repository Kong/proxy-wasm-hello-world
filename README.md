# proxy-wasm-filter-soap-to-json

Add the wasm32 build target if needed:

```
$ rustup target add wasm32-unknown-unknown
```

Build with:

```
$ cargo build --target=wasm32-unknown-unknown
```

And copy the resulting Wasm bytecode to an Nginx prefix:

```
$ cp target/wasm32-unknown-unknown/debug/proxy_wasm_filter_soap_to_json.wasm /etc/nginx
```

Use it as such in `nginx.conf`:

```nginx
# nginx.conf
events {}

wasm {
    module soap_to_json /etc/nginx/proxy_wasm_filter_soap_to_json.wasm;
}

http {
    server {
        listen 9000;

        location / {
            proxy_wasm  soap_to_json;
        }
    }
}
```
