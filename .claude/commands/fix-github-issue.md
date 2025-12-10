GitHub issueを分析して実行してください: issue番号 $ARGUMENTS

以下の手順で進めてください。

1. `gh issue view` で issue 詳細を取得
2. 問題の理解
3. 関連ファイルの検索
4. コードの実装(テストコードは実装しない)
5. .github/workflows/ci.ymlで実行しているcargo fmt,cargo clippyを実行して、エラーがあれば修正する。
6. サブエージェントtest-code-implementerにより、実装したコードについてのテストコードを実装する。cargo testを実行し、問題があればコードを修正する。
7. コミット(developブランチに直接コミットしないこと。feature/\*ブランチにコミットすること。)
8. developブランチへのプルリクエスト作成。プルリクエストには対応したissueについて「Closes #issue no」を書く。
9. サブエージェントcode-reviewer-jpにより、修正したコードについてレビューを実施してレビュー結果はプルリクエストのレビューコメントとして書く。
