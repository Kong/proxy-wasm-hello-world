## Get Started:

Dependencies:
* [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
* [httpie](https://httpie.io/docs/cli/installation)
* [docker](https://www.docker.com/products/docker-desktop/)

1. Make sure you have all dependencies

    <details>
    <summary>Click to expand dependency test command</summary>
    
    ```bash
    bash -c "cat <<EOF | bash -es --
    git --version
    httpie --version
    docker --version
    EOF"
    ```
    
    </details>

1. Clone a Wasm Filter repo:

    ```bash
    git clone https://github.com/usrbinkat/proxy-wasm-hello-world wasm
    ```

2. Run Kong Gateway

    ```bash
    curl -Ls https://get.konghq.com/quickstart | bash -s -- -i kong-gateway-internal -t 3.0.0.0-wasmer \
      -e "KONG_WASM=on" -e "KONG_WASM_MODULES=/wasm/proxy_wasm_hello_world.wasm" -v $(pwd)/wasm:/wasm
    ```

3. Create a mockbin service

    ```bash
    http POST :8001/services/ name="mockbin" host="mockbin.com" path="/bin/ccb2968e-08e8-43af-babd-878c9f269486" protocol="http"
    ```

4. Create a mockbin route
    ```bash
    http POST :8001/services/mockbin/routes name="mockbin" "paths[]=/mockbin"
    ```

5. Configure Proxy Wasm `proxy_wasm_hello_world` filter on the mockbin route
    ```bash
    http POST :8001/services/mockbin/plugins name="proxy-wasm" "config[filters][0][name]=proxy_wasm_hello_world"
    ```

6. Check for hello world header
    ```bash
    http GET :8000/mockbin
    ```

## Result:
```bash
❯ http GET :8000/mockbin
HTTP/1.1 200 OK
Connection: keep-alive
Content-Length: 15
Date: Wed, 21 Sep 2022 22:04:25 GMT
Hello: World
Powered-By: proxy-wasm
Server: nginx
Via: kong/3.0.0.0-enterprise-edition
X-Kong-Proxy-Latency: 145
X-Kong-Upstream-Latency: 0

Hello, World!

```