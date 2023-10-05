### `.env`
```js
HOST = "ip address"
PORT = "port"
```
不使用 docker 才需要 .env

### `docker`
1. 建立 image\
`docker build -t <image name> .`
2. 啟動 container\
`docker run -p <your port>:80 <image name>`
