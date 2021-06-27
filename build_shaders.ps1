
$frag_shaders = ("src\shaders\main.frag", "src\foo.frag")

foreach ($in_shader in $frag_shaders) {
    echo "hello ${in_shader}"
}