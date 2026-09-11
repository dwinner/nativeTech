# Save the original env.
$goOsValue = $env:GOOS
$goArchValue = $env:GOARCH

try
{
    # Change temporary
    $env:GOOS = "linux"
    $env:GOARCH = "amd64"

    # Run cross compilation
    go build crossCompile.go
}
finally
{
    # Restore previous
    $env:GOOS = $goOsValue
    $env:GOARCH = $goArchValue
    $env:MyVariable = $goOsValue
}