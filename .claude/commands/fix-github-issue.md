GitHub issueを分析して実行してください: issue番号 $ARGUMENTS

以下の手順で進めてください。

1. **Issue 詳細の取得**
   - `gh issue view <issue-number>` で issue 詳細を取得

2. **問題の理解**
   - Issue の説明、背景、要件を理解する

3. **関連ファイルの検索**
   - 実装に必要なファイルを特定する

4. **コードの実装**
   - テストコードは実装しない
   - feature/\* ブランチで実装を進める

5. **コード品質チェック（初回）**
   - `cargo fmt` を実行して、フォーマットを修正する
   - `cargo clippy --all-targets --all-features` を実行して、警告を修正する
   - エラーがあれば修正する

6. **テストコード実装**
   - サブエージェント `test-code-implementer` を使用して、実装したコードのテストコードを実装する
   - `cargo test` を実行して、テストが通ることを確認する
   - 問題があればコードを修正する

7. **コード品質チェック（最終）**
   - `cargo fmt` と `cargo clippy --all-targets --all-features` を再度実行する
   - エラーがあれば修正する
   - 問題なければ feature ブランチにコミットする

8. **プルリクエスト作成**
   - develop ブランチへのプルリクエストを作成する
   - PR の説明に「Closes #issue-number」を記載する
   - **PR 作成完了をコンソールに表示する**

9. **CI 完了待機と確認**
   - GitHub Actions による CI が実行される
   - CI 完了を待つ（`gh pr view <PR-number> --json statusCheckRollup` で確認）
   - **CI が SUCCESS で完了したことを確認してから次へ進む**
   - エラーがあれば、原因を調査して修正する

10. **コードレビュー実施**
    - サブエージェント `code-reviewer-jp` を使用して、実装コードの詳細レビューを実施する
    - 重大な問題、改善提案を含むレビュー結果を取得する

11. **レビュー結果をプルリクエストに投稿**
    - 取得したレビュー結果を、以下のコマンドでプルリクエストに投稿する：
    ```bash
    gh pr comment <PR-number> --body "レビュー結果のテキスト"
    ```
    - **コメント投稿完了まで、このステップは完了とはみなさない**
    - コメント投稿後、PR の URL をコンソールに表示する

12. **Issue クローズと PR マージ（オプション）**
    - PR をレビューして問題がなければマージする
    - `gh pr merge <PR-number>` でマージする
    - Issue が自動クローズされることを確認する
