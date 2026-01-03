# Rust LINE Bot

## Local Development environment

### Summary

1. Set up LocalStack
  1. `docker compose up -d`
2. deploy the application to LocalStack with cdklocal
  1. `cdklocal bootstrap`
  2. `cdklocal deploy --all`
3. check the apigateway url
  1. `docker exec -it /bin/bash localstack-main`
  2. execute `awslocal apigateway get-rest-apis` and check `items[].id`
    1. if you want to check the endopoints, we can see with `awslocal apigateway get-resources --rest-api-id <id>`
  3. the endpoint is `http://localhost:4566/restapis/<id>/<stack name>/_user_request_/<path>`
4. ngok
  1. https://dashboard.ngrok.com/
  2. `ngrok http 4566`
5. Set webhook url
  1. https://developers.line.biz/


How to check the log

1. `logs describe-log-groups`
2. `awslocal logs describe-log-streams --log-group-name '<log group name>'`
3. `awslocal logs get-log-events --log-group-name '<log group name>' --log-stream-name '<log stream name>'`

## Referencies

- https://qiita.com/Yuki_Oshima/items/860a859fb85365a609fc
- https://makky12.hatenablog.com/entry/2023/08/21/120500
- https://zenn.dev/mo_ri_regen/articles/aws-cli-setting
- https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/rust-package.html

https://developers.line.biz/ja/docs/line-developers-console/
