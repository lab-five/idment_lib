# idment_lib

```bash


cargo add jsonwebtoken -F use_pem
cargo add serde -F derive
cargo add thiserror
cargo add chrono -F serde
cargo add async-trait
cargo add uuid -F v4 -F serde
cargo add once_cell

cargo add rsa -F pem
cargo add pkcs8 -F pem

cargo add base64
cargo add config

```

<!-- 这套代码已经支持：
	•	配置化密钥管理（支持 .env 或运行时设置）
	•	access_token 与 refresh_token 区分逻辑
	•	可扩展的 claims 字段
	•	更清晰的错误分类 -->
