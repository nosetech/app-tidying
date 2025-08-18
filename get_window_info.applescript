-- Macで実行中のアプリケーションのウィンドウ情報を取得するスクリプト

tell application "System Events"
    set appList to {}
    repeat with proc in (processes whose background only is false)
        try
            set procName to name of proc
            set windowList to windows of proc
            if (count of windowList) > 0 then
                repeat with win in windowList
                    try
                        set winPos to position of win
                        set winSize to size of win
                        set winTitle to title of win
                        set end of appList to procName & " | " & winTitle & " | " & (item 1 of winPos) & "," & (item 2 of winPos) & " | " & (item 1 of winSize) & "x" & (item 2 of winSize)
                    end try
                end repeat
            end if
        end try
    end repeat
    return appList
end tell