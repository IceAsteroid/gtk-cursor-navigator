TODO list:
* Remove hardcoded CSS settings from source code
  * To be configurable in config file.
  * To prevent the label(two letters) size too large in each cell of the grid view
    * which causes cells which are at edge excedding the screen.

* Highlight the first letter when pressed.

* The outer lines should be the same pixelated with the inner lines for the grid view

* Add the ability to configure core ergonomic keys to select the two letters of each cell in config file instead of hardcoded.

* The ergonomic keys should be first letter on left hand, and second letter on right hand.
  * After all combinations are exhausted, reverse all combinations by first letter on right hand, and seond on right.
  * After all of above are exhausted, generate the rest combinations for cells with same handed keys.

* Turn this repository into a toolset for actions like navigation, right/left clicks, text selection, dragging of cursor
  * To be also independent of any specific window managers under wayland.
  * It should work for a window manager that supports the feature like _mode switching_.
