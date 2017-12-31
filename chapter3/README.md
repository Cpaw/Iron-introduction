# Sample Web Application for Rust book

以下の手順に従ってセットアップしてください．  
データベースにSQLite3を使用しているので，インストール済みではない場合は，```$ ./setup.sh```でその旨が表示されますので，インストールしてください．

```
$ git clone https://github.com/Cpaw/Iron_introduction
$ cd Iron_introduction/chapter3
$ chmod u+x setup.sh
$ ./setup.sh
$ cargo run
```

上記コマンドを入力後，しばらくするとブラウザで```localhost:3000```にアクセスするとサンプルアプリケーションが稼働しています．

## Author
ariake


## API使い方

メモ一覧表示
```sh
$ curl localhost:3000/memos
```

メモ追加
```sh
$ curl  -X POST -d '{"content": "ぱろっく"}' localhost:3000/memos
```

メモ表示
```sh
$ curl localhost:3000/memos/{id}
```

メモ更新
```sh
$ curl  -X PUT -d '{"content": "ぱろぱろぱろっく"}' localhost:3000/memos/{id}
```

メモ削除
```sh
$ curl -X DELETE localhost:3000/memos/{id}
```





