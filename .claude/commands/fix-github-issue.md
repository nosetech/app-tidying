GitHub issueを分析して実行してください: issue番号 $ARGUMENTS

以下の手順で進めてください。

1. `gh issue view` で issue 詳細を取得
2. 問題の理解
3. 関連ファイルの検索
4. コードの実装
5. コミット(developブランチに直接コミットしないこと。feature/\*ブランチにコミットすること。)
6. developブランチへのプルリクエスト作成。プルリクエストには対応したissueについて「Closes #issue no」を書く。
7. サブエージェントcode-reviewer-jpにより修正したコードについてレビューを実施し、結果はプルリクエストのレビューコメントとして書く。
