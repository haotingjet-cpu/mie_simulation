# 🚀 1. 環境初始化（強迫使用 TLS 1.2/1.3 與 UTF-8 編碼，防止亂碼與連線失敗）
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# 🚀 2. 載入 .env 設定檔
$EnvPath = Join-Path $PSScriptRoot ".env"
if (Test-Path $EnvPath) {
    Get-Content $EnvPath | ForEach-Object {
        if ($_ -match '^([^#=]+)=(.*)$') {
            $Key = $Matches[1].Trim()
            $Value = $Matches[2].Trim()
            [System.Environment]::SetEnvironmentVariable($Key, $Value, "Process")
        }
    }
} else {
    Write-Error ".env 檔案不存在，請檢查路徑！"
    exit
}

$Username = $env:EMAIL_USER
$Password = $env:EMAIL_PASS

# 🚀 3. Rust 專案編譯與模擬執行
Write-Host "開始編譯 Rust 專案..." -ForegroundColor Cyan
cargo build --release

# 使用 $PSScriptRoot 確保路徑絕對精準
$ExePath = Join-Path $PSScriptRoot "target\release\simulation.exe"

if (Test-Path $ExePath) {
    Write-Host "開始執行米氏散射模擬..." -ForegroundColor Cyan
    # 執行並捕獲輸出，轉為單一字串
    $SmulateString = [string]((& $ExePath --no_progress ) -join "`n")
} else {
    $SmulateString = "錯誤：找不到米氏散射模擬執行檔，此為預設文字。"
}

Write-Host "模擬執行完成！" -ForegroundColor Green

# 🚀 4. 設定郵件與照片附件內容
$From = $Username
$To = $Username
$Subject = "模擬結果回報"

# 修正重複賦值問題
$Body = @"
米氏散射模擬執行完畢，以下為程式標準輸出數據：

$SmulateString

(詳細高解析度 DPI 圖表已夾帶於附件中)
"@

$MailMessage = New-Object System.Net.Mail.MailMessage($From, $To, $Subject, $Body)

# 設定照片附件路徑
$PhotoPath = "C:\rust\mie_simulate\latest_result.png" 

if (Test-Path $PhotoPath) {
    $Attachment = New-Object System.Net.Mail.Attachment($PhotoPath)
    $MailMessage.Attachments.Add($Attachment)
    Write-Host "成功裝載模擬照片附件。" -ForegroundColor Cyan
} else {
    Write-Warning "找不到照片檔案，將不裝載附件發送！"
}

# 🚀 5. SMTP 伺服器連線與發送
$SmtpServer = "smtp.gmail.com"
$SMTPClient = New-Object System.Net.Mail.SmtpClient($SmtpServer, 587)
$SMTPClient.EnableSsl = $true
$SMTPClient.Credentials = New-Object System.Net.NetworkCredential($Username, $Password)

try {
    Write-Host "正在發送郵件至 $To ..." -ForegroundColor Cyan
    $SMTPClient.Send($MailMessage)
    Write-Host "🎉 郵件發送成功！" -ForegroundColor Green
} catch {
    Write-Error "發送失敗！詳細錯誤訊息如下："
    Write-Error $_.Exception.Message
    if ($_.Exception.InnerException) {
        Write-Error "內部異常: $($_.Exception.InnerException.Message)"
    }
} finally {
    # 釋放資源，避免圖片檔案被 PowerShell 鎖死無法被下次模擬覆寫
    $MailMessage.Dispose()
    $SMTPClient.Dispose()
}
