"""
Main window for the Zappy GUI
"""
from PyQt6.QtCore import Qt, QTimer
from PyQt6.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, QLabel, QPushButton,
    QMessageBox, QInputDialog, QLineEdit, QSplitter, QTabWidget, QStatusBar
)

from gui.core.map_view import MapView
from gui.core.log_viewer import LogViewer
from gui.core.map_controls import MapControls
from gui.core.player.PlayerManager import PlayerManager
from gui.core.server_connection import ServerConnection


class ZappyMainWindow(QMainWindow):
    """Main window for the Zappy GUI application"""
    
    def __init__(self):
        super().__init__()
        
        # Set up the main window
        self.setWindowTitle("Zappy GUI")
        self.resize(1500, 900)
        
        # Initialize components
        self.server_connection : ServerConnection | None = None  # Initialize as None instead of creating an instance
        self.player_manager = PlayerManager()

        self.map_controls = None
        self.log_viewer = None
        self.time_unit_label = None
        self.connection_status = None
        self.map_size_label = None
        self.disconnect_button = None
        self.connect_button = None
        self.port_input = None
        self.host_input = None
        
        # Create central layout with game view and tabs
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        
        # Main layout contains the header, game view and tabs in a vertical arrangement
        main_layout = QVBoxLayout(central_widget)
        main_layout.setContentsMargins(0, 0, 0, 0)
        
        # Add connection header
        self.create_connection_header(main_layout)

        ## Create a splitter to allow resizing between game view and tabs
        splitter = QSplitter(Qt.Orientation.Horizontal)
        main_layout.addWidget(splitter, 1)

        ## Game view on the left
        self.game_view = MapView(self.player_manager)
        splitter.addWidget(self.game_view)

        ## Create tab widget on the right
        self.tabs = QTabWidget()
        splitter.addWidget(self.tabs)

        ## Create all modules as tabs
        self.create_tabs()

        ## Set initial splitter sizes (70% game view, 30% tabs)
        splitter.setSizes([700, 300])

        ## Create status bar
        self.status_bar = QStatusBar()
        self.setStatusBar(self.status_bar)
        self.status_bar.showMessage("Disconnected")

        # Create separate timers for game state updates and camera updates
        self.game_update_timer = QTimer(self)
        self.game_update_timer.timeout.connect(self.update_game_state)
        self.game_update_timer.start(100)  # 100ms fixed interval for game state

        # Camera update timer with higher frequency for smooth tracking
        self.camera_update_timer = QTimer(self)
        self.camera_update_timer.timeout.connect(self.update_camera)
        self.camera_update_timer.start(16)  # ~60 FPS for smooth camera movement
        
        # No automatic connection dialog at startup
        
    def create_connection_header(self, main_layout):
        """Create the connection header at the top of the window"""
        header_widget = QWidget()
        header_widget.setStyleSheet("background-color: #f0f0f0; border-bottom: 1px solid #ddd;")
        header_layout = QHBoxLayout(header_widget)
        
        # Server info section
        server_section = QWidget()
        server_layout = QHBoxLayout(server_section)
        server_layout.setContentsMargins(0, 0, 0, 0)
        
        # Host input
        server_layout.addWidget(QLabel("Host:"))
        self.host_input = QLineEdit("localhost")
        self.host_input.setFixedWidth(150)
        server_layout.addWidget(self.host_input)
        
        # Port input
        server_layout.addWidget(QLabel("Port:"))
        self.port_input = QLineEdit("4242")
        self.port_input.setFixedWidth(80)
        server_layout.addWidget(self.port_input)
        
        # Connect button
        self.connect_button = QPushButton("Connect")
        # noinspection PyUnresolvedReferences
        self.connect_button.clicked.connect(self.connect_from_header)
        server_layout.addWidget(self.connect_button)
        
        # Disconnect button
        self.disconnect_button = QPushButton("Disconnect")
        # noinspection PyUnresolvedReferences
        self.disconnect_button.clicked.connect(self.disconnect_from_server)
        self.disconnect_button.setEnabled(False)
        server_layout.addWidget(self.disconnect_button)
        
        header_layout.addWidget(server_section)
        
        # Status indicator
        self.connection_status = QLabel("Status: Disconnected")
        self.connection_status.setStyleSheet("font-weight: bold; color: #d32f2f;")
        header_layout.addWidget(self.connection_status)

        header_layout.addStretch(1)
        # Add header to main layout
        main_layout.addWidget(header_widget)
    
    def connect_from_header(self):
        """Connect to server using the header input fields"""
        host = self.host_input.text()
        port_text = self.port_input.text()
        
        try:
            port = int(port_text)
            self.connect_to_server(host, port)
        except ValueError:
            QMessageBox.critical(
                self, "Connection Error", "Invalid port number"
            )

    def create_tabs(self):
        """Create tabs for all the modules"""
        
        # Map controls tab
        self.map_controls = MapControls()
        self.map_controls.grid_changed.connect(self.on_show_grid_changed)
        self.map_controls.coords_changed.connect(self.on_show_coords_changed)
        self.map_controls.resources_changed.connect(self.on_show_resources_changed)
        self.map_controls.text_size_changed.connect(self.on_text_size_changed)

        self.map_controls.zoom_in_button.clicked.connect(self.game_view.zoom_in)
        self.map_controls.zoom_out_button.clicked.connect(self.game_view.zoom_out)
        self.map_controls.reset_zoom_button.clicked.connect(self.game_view.reset_view)

        self.map_controls.follow_player_changed.connect(self.on_follow_player_changed)
        self.map_controls.smooth_tracking_changed.connect(self.on_smooth_tracking_changed)
        self.map_controls.tracking_speed_changed.connect(self.on_tracking_speed_changed)


        self.player_manager.player_added.connect(self.update_player_list)
        self.player_manager.player_removed.connect(self.update_player_list)

        self.game_view.use_smooth_tracking = self.map_controls.smooth_tracking_checkbox.isChecked()
        self.game_view.tracking_speed = self.map_controls.tracking_speed_slider.value() / 100.0

        self.tabs.addTab(self.map_controls, "Map Controls")
        
        # Resource viewer tab
        #self.resource_viewer = ResourceViewer()
        #self.tabs.addTab(self.resource_viewer, "Resources")
        
        # Team panel tab
        #self.team_panel = TeamPanel()
        #self.tabs.addTab(self.team_panel, "Teams")
        
        # Player panel tab
        #self.player_panel = PlayerPanel()
        #self.tabs.addTab(self.player_panel, "Players")
        
        # Log viewer tab
        self.log_viewer = LogViewer()
        self.tabs.addTab(self.log_viewer, "Logs")

    def update_player_list(self, _=None):
        """Update the player dropdown with the current player list"""
        self.map_controls.update_player_list(self.player_manager.all_players())

    def on_text_size_changed(self, size):
        """Handle text size changes"""
        self.game_view.set_text_size(size)

    def on_show_grid_changed(self, show):
        """Handle show grid changes"""
        self.game_view.show_grid = show
        self.game_view.draw_map()

    def on_show_coords_changed(self, show):
        """Handle show coordinates changes"""
        self.game_view.show_coordinates = show
        self.game_view.draw_map()

    def on_show_resources_changed(self, show):
        """Handle show resources changes"""
        self.game_view.show_resources = show
        self.game_view.draw_map()
    
    def show_connect_dialog(self):
        """Show dialog to connect to the server"""
        host, ok = QInputDialog.getText(
            self, "Connect to Server", "Server Host:", 
            QLineEdit.EchoMode.Normal, "localhost"
        )
        
        if ok and host:
            port, ok = QInputDialog.getText(
                self, "Connect to Server", "Server Port:",
                QLineEdit.EchoMode.Normal, "4242"
            )
            
            if ok and port:
                try:
                    port = int(port)
                    self.connect_to_server(host, port)
                except ValueError:
                    QMessageBox.critical(
                        self, "Connection Error", "Invalid port number"
                    )

    def update_connection_status(self, connected: bool):
        if connected:
            self.connection_status.setText(
                f"Status: Connected to {self.server_connection.host}:{self.server_connection.port}")
            self.connection_status.setStyleSheet("font-weight: bold; color: #4CAF50;")
            self.status_bar.showMessage(f"Connected to {self.host_input.text()}:{self.port_input.text()}")
            self.connection_status.setText(f"Status: Connected to {self.host_input.text()}:{self.port_input.text()}")
            self.connection_status.setStyleSheet("font-weight: bold; color: #4CAF50;")
            self.disconnect_button.setEnabled(True)
            self.connect_button.setEnabled(False)
        else:
            self.connection_status.setText("Status: Disconnected")
            self.status_bar.showMessage(f"Status: Disconnected")
            self.connection_status.setStyleSheet("font-weight: bold; color: #d32f2f;")
            self.disconnect_button.setEnabled(False)
            self.connect_button.setEnabled(True)

    
    def connect_to_server(self, host, port):
        """Connect to the server with the given host and port"""
        try:
            # Create new connection with host and port parameters
            self.server_connection = ServerConnection(host, port)
            self.server_connection.on_status_change = self.update_connection_status
            self.server_connection.connect()

            #connect emitter to server_connection
            self.map_controls.time_unit_changed.connect(self.server_connection.send_new_time_unit)

            self.map_controls.set_enabled(True)
            
            # Update host/port input fields
            self.host_input.setText(host)
            self.port_input.setText(str(port))
            
            # Log connection
            self.log_viewer.add_log(f"Connected to {host}:{port}", "connection")
        except Exception as e:
            QMessageBox.critical(
                self, "Connection Error", f"Failed to connect: {str(e)}"
            )
    
    def disconnect_from_server(self):
        """Disconnect from server"""
        if self.server_connection:
            self.server_connection.disconnect()
            self.server_connection = None
            
            # Update UI elements
            self.status_bar.showMessage("Disconnected")
            self.connection_status.setText("Status: Disconnected")
            self.connection_status.setStyleSheet("font-weight: bold; color: #d32f2f;")
            self.disconnect_button.setEnabled(False)
            self.connect_button.setEnabled(True)

            self.map_controls.set_enabled(False)

            # Clear the game view
            self.game_view.clear()
            
            # Log disconnection
            self.log_viewer.add_log("Disconnected from server", "connection")

    def update_game_state(self):
        """Update game state with new data from server"""
        if not self.server_connection or not self.server_connection.connected:
            return

        # Get responses from server
        responses = self.server_connection.get_responses()

        # Process responses
        for response in responses:
            self.process_server_response(response)

        # Draw the map
        self.game_view.draw_map()

    
    def process_server_response(self, response):
        """Process a response from the server"""
        # Log the response
        self.log_viewer.add_log(response, "server")
        
        parts = response.split()
        if not parts:
            return
        
        cmd = parts[0]
        
        try:
            # Map size
            #msz X Y\n
            if cmd == "msz" and len(parts) >= 3:
                map_width = int(parts[1])
                map_height = int(parts[2])
                self.game_view.set_map_size(map_width, map_height)


            # In ZappyMainWindow.process_server_response
            elif cmd == "sgt" and len(parts) >= 2:
                freq = int(parts[1])
                self.map_controls.set_time_unit(freq)
                # You might also want to log this
                self.log_viewer.add_log(f"Server time unit set to {freq}", "game")

            elif cmd == "sst" and len(parts) >= 2:
                freq = int(parts[1])
                self.map_controls.set_time_unit(freq)
                self.log_viewer.add_log(f"Server time unit changed to {freq}", "game")
            
            # Team name
            elif cmd == "tna" and len(parts) >= 2:
                pass
            
            # Tile content
            elif cmd == "bct" and len(parts) >= 10:
                x = int(parts[1])
                y = int(parts[2])
                resources = [int(parts[i]) for i in range(3, 10)]

                # Update resources in the game view
                self.game_view.update_tile_resources(x, y, resources)
            
            # New player
            elif cmd == "pnw" and len(parts) >= 7:
                player_id = int(parts[1][1:])  # Remove the # prefix
                x = int(parts[2])
                y = int(parts[3])
                orientation = int(parts[4])
                level = int(parts[5])
                team_name = parts[6]

                # Convert numeric orientation to direction letter
                direction_map = {1: "N", 2: "E", 3: "S", 4: "W"}
                direction = direction_map.get(orientation, "N")

                # Create the player in the player manager
                self.player_manager.add_player(
                    player_id,
                    position=(x, y),
                    direction=direction,
                    level=level,
                    team=team_name
                )

                self.log_viewer.add_player_log(f"Player #{player_id} (team {team_name}) joined at level {level}")

            # Player position
            elif cmd == "ppo" and len(parts) >= 5:
                player_id = int(parts[1][1:])  # Remove the # prefix
                x = int(parts[2])
                y = int(parts[3])
                orientation = int(parts[4])

                # Convert numeric orientation to direction letter
                direction_map = {1: "N", 2: "E", 3: "S", 4: "W"}
                direction = direction_map.get(orientation, "N")

                # Update player position
                self.player_manager.update_player_position(player_id, (x, y), direction)

            # Player death
            elif cmd == "pdi" and len(parts) >= 2:
                player_id = int(parts[1][1:])

                # Remove player
                self.player_manager.remove_player(player_id)
                self.log_viewer.add_player_log(f"Player #{player_id} died")
            
            # Player level
            elif cmd == "plv" and len(parts) >= 3:
                player_id = int(parts[1][1:])  # Remove the # prefix
                level = int(parts[2])
                
                self.log_viewer.add_player_log(f"Player #{player_id} is now level {level}")
            
            # Player inventory
            elif cmd == "pin" and len(parts) >= 10:
                _player_id = int(parts[1][1:])  # Remove the # prefix
                _x = int(parts[2])
                _y = int(parts[3])
                _resources = {
                    "food": int(parts[4]),
                    "linemate": int(parts[5]),
                    "deraumere": int(parts[6]),
                    "sibur": int(parts[7]),
                    "mendiane": int(parts[8]),
                    "phiras": int(parts[9]),
                    "thystame": int(parts[10])
                }
            
            # Start of incantation
            elif cmd == "pic" and len(parts) >= 4:
                x = int(parts[1])
                y = int(parts[2])
                level = int(parts[3])
                players = [int(parts[i][1:]) for i in range(4, len(parts))]
                
                player_str = ", ".join([f"#{p}" for p in players])
                self.log_viewer.add_incantation_log(
                    f"Incantation started at ({x}, {y}) for level {level+1} by players {player_str}"
                )
            
            # End of incantation
            elif cmd == "pie" and len(parts) >= 4:
                x = int(parts[1])
                y = int(parts[2])
                result = int(parts[3])  # 0 = failure, 1 = success

                status = "succeeded" if result == 1 else "failed"
                self.log_viewer.add_incantation_log(
                    f"Incantation at ({x}, {y}) {status}"
                )
                
        except Exception as e:
            print(f"Error processing response: {e} - Response: {response}")
    
    def update_time_unit_in_header(self, time_unit):
        """Update the time unit display in the header"""
        self.time_unit_label.setText(f"{time_unit} ms")

    def on_follow_player_changed(self, player_id):
        """Handle follow player changes"""
        self.game_view.tracked_player_id = player_id
        if player_id is not None:
            self.game_view.center_on_player(player_id)

    def on_smooth_tracking_changed(self, enabled):
        """Handle smooth tracking setting changes"""
        # You'll need to implement smooth tracking in your MapView
        self.game_view.use_smooth_tracking = enabled

    def on_tracking_speed_changed(self, speed):
        """Handle tracking speed changes"""
        # Set the tracking speed in your MapView
        self.game_view.tracking_speed = speed

    def update_camera(self):
        """Update camera position if tracking a player (called at higher frequency)"""
        # Only update camera tracking, not the entire game state
        if hasattr(self.game_view, 'tracked_player_id') and self.game_view.tracked_player_id is not None:
            self.game_view.update_player_tracking()