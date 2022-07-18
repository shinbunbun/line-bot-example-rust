# line-bot-example-rust

本リポジトリは<https://github.com/shinbunbun/aizuhack-bot>をRustで書き直したものになります。

botディレクトリがLINEBot本体のコード、line-bot-sdkディレクトリが試験的に実装したRustのLINEBot SDKです。

## SDKについて

本SDKはα版のため、大量の不具合があるかと思われます。見つかったらPRを投げていただけると助かります。
新機能追加も大歓迎です。

また、破壊的アップデートが頻繁に行われる可能性があります。

【SDK未実装の機能】
- [ ] LINE Things全般
- [x] [Webhook設定](https://developers.line.biz/ja/reference/messaging-api/#webhook-settings)
- [ ] [チャネルアクセストーン](https://developers.line.biz/ja/reference/messaging-api/#channel-access-token)
- [ ] [メッセージ（応答メッセージを除く）](https://developers.line.biz/ja/reference/messaging-api/#send-broadcast-message)
- [ ] [オーディエンス管理](https://developers.line.biz/ja/reference/messaging-api/#manage-audience-group)
- [ ] [分析](https://developers.line.biz/ja/reference/messaging-api/#get-insight)
- [ ] [LINE公式アカウントを友だち追加したユーザーのリストを取得する](https://developers.line.biz/ja/reference/messaging-api/#get-follower-ids)
- [ ] [ボットの情報を取得する](https://developers.line.biz/ja/reference/messaging-api/#get-bot-info)
- [ ] [グループトーク](https://developers.line.biz/ja/reference/messaging-api/#group)
- [ ] [複数人トーク](https://developers.line.biz/ja/reference/messaging-api/#group)
- [ ] [リッチメニュー](https://developers.line.biz/ja/reference/messaging-api/#rich-menu)
- [ ] [アカウント連携](https://developers.line.biz/ja/reference/messaging-api/#account-link)
