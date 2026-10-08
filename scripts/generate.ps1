$inputFile = ".\assets\languages.json"
$outputDir = ".\assets"

$languages = Get-Content $inputFile -Raw | ConvertFrom-Json

foreach ($language in $languages) {
    $langDir = Join-Path $outputDir $language.id

    New-Item -ItemType Directory -Path $langDir -Force | Out-Null

    $indexFile = Join-Path $langDir "index.json"

    "{}" | Set-Content -Path $indexFile -Encoding UTF8

    Write-Host "Created: $indexFile"
}