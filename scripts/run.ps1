param ($conf = 'debug')

if ($conf -eq 'debug') {
    $arg = ''
}
else {
    $arg = '--' + $conf
}

& "$PSScriptRoot\build.ps1" $conf
cargo run $arg