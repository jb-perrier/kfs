$foldersToRemove = @("./bin", "./build", "./kernel/target")

foreach ($folder in $foldersToRemove) {
    if (Test-Path $folder) {
        Remove-Item -Path $folder -Recurse -Force
        Write-Host "Removed folder: $folder"
    } else {
        Write-Host "Folder not found: $folder"
    }
}