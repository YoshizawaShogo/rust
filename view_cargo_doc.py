#!/usr/bin/python3

# 用途    : サーバにリモートでアクセスしている時に、そこで生成したrustdocを見るためにhttpサーバを建てる。
# 使用方法: このgitリポジトリのルートディレクトリにて 
#          $ ./view_cargo_doc.py
# 注意事項: http通信なので、秘密にしたいcreateに対してはしない方が良さそう。

import re
import http.server
import socketserver

IP_ADDRESS = "127.0.0.1" # localhost
CANDIDATE_PORTS = range(8000, 8010) # 連続で使用すると、エラー("OSError: [Errno 98] Address already in use")が出たので対策した。
TARGET_DIR = "project/target" # project/.cargo/config.tomlのtarget-dirに対応
SETTINGS_HTML = TARGET_DIR + "/doc/settings.html"

# extract </button><a class="sidebar-logo" href="???">
with open(SETTINGS_HTML, "r") as f:
    match = re.search('</button><a class="sidebar-logo" href="[^"]*">', f.read())
    stripped_match = match.group().replace('</button><a class="sidebar-logo" href="./', '').replace('">', '')
    INDEX_HTML = TARGET_DIR + "/doc/" + stripped_match

# http server
Handler = http.server.SimpleHTTPRequestHandler
for port in CANDIDATE_PORTS:
    try:
        with socketserver.TCPServer((IP_ADDRESS, port), Handler) as httpd:
            print("Serving at port", "http://" + IP_ADDRESS + ":" + str(port) + "/" + INDEX_HTML + ".")
            httpd.serve_forever()
    except OSError as e: # OSEエラーが出たら、次のポート候補で再度挑戦する。
        continue
    except KeyboardInterrupt:
        print('\nCatch KeyboardInterrupt.')
        break
    else: # httpd.serve_forever() は終了しないので、このelseには到達しない。
        print("Error: このprint()関数が実行されることはない。")
        raise
        
