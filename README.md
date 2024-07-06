# Proxyl
Proxy outgoing requests to alternate servers with fine-grained control


## Setup
Install using cargo
```sh
$ git clone https://github.com/sanket143/proxyl
$ cd proxyl
$ cargo install --path . --locked
```
### Create certificate
This will create a certificate and key with expiry of 30 days
```sh
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 30 -subj "/C=ZZ/ST=ZZ/L=ZZ/O=Proxyl/CN=Proxyl"
```
### Add certificate for proxyl to use
```sh
$ proxyl add-certificate --cert-path ./cert.pem --key-path ./key.pem
```

<details>
  <summary>
    Add and trust certificate in the browser or keychain
  </summary>
  
![image](https://github.com/sanket143/proxyl/assets/26973649/012fc3cb-60db-434d-a72e-cbbbf230373f)
![image](https://github.com/sanket143/proxyl/assets/26973649/e602ed9a-3aa1-4dca-8377-465527f62471)
</details>
<details>
  <summary>
    Complete the server setup by running the server and adding the server address in proxy address
  </summary>

  ![image](https://github.com/sanket143/proxyl/assets/26973649/91c1b8a9-55e2-4b95-8688-65b6c50ece27)
</details>

<details>
  <summary>
    Add `~/.config/proxyl/config.toml` to fine-grain the proxy server
  </summary>

Example:
  ```toml
# Rules are the fundamental part of the configuration
# If the request matches any of the defined rule, it'll redirect the request to the said url
[rules.example-sanket143]
uri = "https://example.com:443/api"
method = "POST"
redirect_to = "http://localhost:3000/api"
headers = { "origin" = "https://example.com" }
body_re = "username=sanket143" # regex that will be applied with the content of the request body
enabled = false # whether the rule is active or not, by default the value is `true`
  ```

If there happened to be multiple rules with similar attributes, it can become repetitive.
In that case, we can define config and extend those in the rules
```toml
[config.example]
uri = "https://example.com:443/api"
method = "POST"
redirect_to = "http://localhost:3000/api"
headers = { "origin" = "https://example.com" }

[rules.example-sanket143]
config = "example"
body_re = "username=sanket143"

[rules.example-octacat]
config = "example"
body_re = "username=octacat"
enabled = false
```
</details>

### NOTES
- Make sure you mention port number in the uri. NOTE **443** in `https://example.com:443/api`
