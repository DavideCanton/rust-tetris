param ($conf = 'debug')

if ($conf -eq 'debug') {
    $arg = ''
}
else {
    $arg = '--' + $conf
}

cargo build $arg
Copy-Item -Path "resources" -Destination "target\$conf" -Recurse -Force