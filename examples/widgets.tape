# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/buttons.tape`
Output "target/button.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Hide
Type "cargo run --example=widgets"
Enter
Sleep 2s
Show
Sleep 1s
Type @250ms "  "
Sleep 500ms
Right @250ms
Type @250ms "  "
Sleep 500ms
Right @250ms
Type @250ms"  "
Sleep 500ms
Right @250ms
Sleep 1s
Tab @2s 3

