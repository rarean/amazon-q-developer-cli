# -*- coding: utf-8 -*-
import os.path

# Volume format (see hdiutil create -help)
format = 'UDBZ'

# Volume size
size = '50M'

# Files to include
files = [ '../target/release/chat_cli' ]

# Symlinks to create
symlinks = { 'Applications': '/Applications' }

# Volume icon
#icon = 'path/to/icon.icns'

# Background
#background = 'path/to/background.png'

# Window bounds
window_rect = ((100, 100), (640, 280))

# Icon size
icon_size = 128

# Text size
text_size = 16

# Icon locations
icon_locations = {
    'chat_cli': (160, 140),
    'Applications': (480, 140)
}
