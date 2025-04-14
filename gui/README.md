# Zappy GUI Client

A Python-based graphical user interface (GUI) client for the Zappy game server.

## Overview

This GUI client connects to a Zappy server using the GUI protocol and visualizes the game state, including:
- Game map and grid
- Players and their movements
- Resources on the map
- Teams and their players
- Incantations and level-ups
- Eggs and hatching
- All game events

## Requirements

- Python 3.6 or higher
- PyQt6

## Installation

1. Install the required Python dependencies:

```bash
pip install -r gui/requirements.txt
```

## Usage

1. Start the GUI client:

```bash
# Run from the project root
python -m gui

# Alternatively, run from the gui directory
cd gui
python __main__.py
```

2. Enter the server address (default: 127.0.0.1) and port (default: 4242).

3. Click "Connect" to connect to the server.

The GUI client will automatically request the game state from the server and continuously update the display.

## Features

- Real-time visualization of the game map
- Player tracking with level and inventory information
- Team management and statistics
- Resource distribution visualization
- Color-coded event logging
- Incantation monitoring
- Time unit control

## Protocol

The client implements the Zappy GUI protocol as specified in the server_protocol.md file.

## License

This project is part of the Zappy-RS server implementation.