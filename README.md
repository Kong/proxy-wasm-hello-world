# Proxy Wasm Hello World

Wasm enables developers to write Wasm packet filters in their [preferred language](https://github.com/proxy-wasm/spec#sdks) and deploy them to various dataplanes.    
    
## Benefits

* Wasm enables powerful and performant extensibility of your datapath.
* Sharing a common extension ecosystem across proxy implementations strengthens community growth.

## What is [WebAssembly](https://webassembly.org)?

First designed as an enhancement for modern web browsers, WebAssembly (Wasm) joined HTML, CSS, and JavaScript to be announced as the fourth [W3C recommended](https://www.w3.org/2019/12/pressrelease-wasm-rec.html.en) "language for the Web". Wasm enabled Ahead of Time compiled (AOT) binaries to run in the browser unlocking new capabilities in the client rendered internet.

The [Wasm System Interface](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/) ([WASI](https://github.com/WebAssembly/WASI)) project extended Wasm beyond the browser by establishing the needed [Application Binary Interface](https://en.wikipedia.org/wiki/Application_binary_interface) (ABI) [standardization](https://github.com/WebAssembly/WASI) effort to formally build ecosystems of server side Wasm applications.

## What is [WebAssembly for Proxies](https://github.com/proxy-wasm/spec)?

Wasm extensions depend on a proxy exposing an [embedded Wasm runtime](https://github.com/proxy-wasm/spec/blob/master/docs/WebAssembly-in-Envoy.md#runtimes). First featured in the [Envoy Proxy](https://opensource.googleblog.com/2020/03/webassembly-brings-extensibility-to.html), [Wasm for Proxies](https://github.com/proxy-wasm) defines a standard integration spec to facilitate communication between the proxy host program and  data filter extensions over the Wasi ABI.

## Get Started:

Dependencies:
* [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
* [httpie](https://httpie.io/docs/cli/installation)
* [docker](https://www.docker.com/products/docker-desktop/)

1. Make sure you have all dependencies

    <details>
    <summary>Click to expand dependency check command</summary>
    
    ```bash
    bash -c "cat <<EOF | bash -es --
    git --version
    httpie --version
    docker --version
    EOF"
    ```
    
    </details>

<space>
</space>

2. Clone Wasm Filter repo for: [`proxy_wasm_hello_world`](https://github.com/kong/proxy-wasm-hello-world)

    ```bash
    git clone https://github.com/kong/proxy-wasm-hello-world wasm
    ```

3. Run [Kong API Gateway](https://docs.konghq.com/gateway/latest/):

    ```bash
    curl -Ls https://get.konghq.com/quickstart | bash -s -- -i incubator -t gateway-wasmer-3.0.0.0 \
      -e "KONG_WASM=on" -e "KONG_WASM_MODULES=/wasm/proxy_wasm_hello_world.wasm" -v $(pwd)/wasm:/wasm
    ```

4. Create a [Mockbin.com](https://mockbin.com) demo [Service](https://docs.konghq.com/gateway/latest/get-started/services-and-routes/#managing-services):

    ```bash
    http POST :8001/services/ name="mockbin" host="mockbin.com" path="/bin/ccb2968e-08e8-43af-babd-878c9f269486" protocol="http"
    ```

5. Create a [Mockbin.com](https://mockbin.com) demo [Route](https://docs.konghq.com/gateway/latest/get-started/services-and-routes/#managing-routes):
    ```bash
    http POST :8001/services/mockbin/routes name="mockbin" "paths[]=/mockbin"
    ```

6. Configure Proxy Wasm filter `proxy_wasm_hello_world` on the mockbin route:
    ```bash
    http POST :8001/services/mockbin/plugins name="proxy-wasm" "config[filters][0][name]=proxy_wasm_hello_world"
    ```

7. Check for header: `Cowsay: Hello World`
    ```bash
    http GET :8000/mockbin
    ```
    * Result:
    ```bash
    ❯ http GET :8000/mockbin
    HTTP/1.1 200 OK
    Connection: keep-alive
    Content-Length: 15
    Cowsay: Hello World
    Date: Wed, 21 Sep 2022 22:46:01 GMT
    Powered-By: proxy-wasm
    Server: nginx
    Via: kong/3.0.0.0-enterprise-edition
    X-Kong-Proxy-Latency: 340
    X-Kong-Upstream-Latency: 0
    
    Hello, World!
    ```
