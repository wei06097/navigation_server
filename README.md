# 操作說明
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

### `cargo doc`
- 產生文件並開啟伺服器\
`cargo doc --no-deps && cd target/doc && python3 -m http.server <port>`

# 文件
[Rust 程式碼](https://wei06097.github.io/navigation_server/navigation_server/)
## Web API
### `GET` `/navigation/graph`
<details>

- 用途
    - 取得校內地圖節點資訊

- Response Body
    ```JavaScript
    {
        "1": { //節點編號
            "geo_coord": [ //經緯度座標
                121.54026281917675,
                25.012364685321245
            ],
            "img_coord": [ //平面圖像素座標
                5924,
                2418
            ],
            "edges": { //鄰近節點編號:距離
                "2": 28.133623090558125,
                "199": 9.382506193306543,
                "200": 16.590585069977347
            }
        },
        "2": {
            "geo_coord": [
                121.5404747956079,
                25.012529348340735
            ],
            "img_coord": [
                5526,
                2403
            ],
            "edges": {
                "1": 28.133623090558125,
                "148": 12.653853302417646
            }
        },
        ...
    }
    ```

</details>

### `POST` `/navigation/directions`
<details>

- 用途
    - 輸入兩組經緯度，找出最佳路徑和其他相關資訊

- Request Body
    ```JavaScript
    {
        source: [ //起點 (經緯度座標)
            longitude_a,
            latitude_a
        ],
        destination: [ //終點 (經緯度座標)
            longitude_b,
            latitude_b
        ],
    }
    ````
- Response Body
    ```JavaScript
    {
        source_xy: [ //起點 (平面圖像素座標)
            source_x,
            source_Y
        ],
        destination_xy: [ //終點 (平面圖像素座標)
            destination_x,
            destination_Y
        ],
        best_path: [ //最佳路徑 (節點編號)
            "63", "64", "65", "3", "2"
        ],
        total_distance: 487.63, //總距離 (公尺)
    }
    ````

</details>

### `POST` `/navigation/transform`
<details>

- 用途
    - 平面圖像素座標轉經緯度座標
    - 經緯度座標轉平面圖像素座標

- Case 1
    - Request Body
    ```JavaScript
    {
        coord: {
            Img: [x, y]
        }
    }
    ````
    - Response Body
    ```JavaScript
    {
        coord: {
            Geo: [lon, lat]
        }
    }
    ````

- Case 2
    - Request Body
    ```JavaScript
    {
        coord: {
            Geo: [lon, lat]
        }
    }
    ````
    - Response Body
    ```JavaScript
    {
        coord: {
            Img: [x, y]
        }
    }
    ````

</details>
