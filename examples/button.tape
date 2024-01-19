# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/buttons.tape`
Output "target/button.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 260
Hide
Type "cargo run --example=button"
Enter
Sleep 2s
Show
Sleep 1s
