# This needs to be in the same directory as setwallaper.bat (on system PATH ideally)
param (
    [string]$path
)

# Check if the provided file exists
if (-Not (Test-Path $path)) {
    Write-Host "The file does not exist: $path"
    exit
}

# Check if the file is a valid image type (e.g., .jpg, .png)
$allowedExtensions = @(".jpg", ".jpeg", ".png", ".bmp")
$extension = [System.IO.Path]::GetExtension($path).ToLower()

if (-Not ($allowedExtensions -contains $extension)) {
    Write-Host "Invalid file type. Please use .jpg, .jpeg, .png, or .bmp files."
    exit
}

# Define the DllImport method for SystemParametersInfo
$code = '[DllImport("user32.dll", CharSet=CharSet.Auto)] public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);'
$method = Add-Type -MemberDefinition $code -Name Wallpaper -Namespace WinAPI -PassThru

# Apply the wallpaper and update settings
$result = $method::SystemParametersInfo(0x0014, 0, $path, 0x0001 | 0x0002)

# Check if it was successful
if ($result -eq 0) {
    Write-Host "Failed to set the wallpaper."
} else {
    Write-Host "Wallpaper set successfully!"
}

