"""
Main window for the Zappy GUI
"""
import sys
from PyQt6.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, QLabel, QPushButton,
    QDockWidget, QSplitter, QMessageBox, QInputDialog, QLineEdit, QStatusBar,
    QTabWidget
)
from PyQt6.QtCore import Qt, QTimer
from PyQt6.QtGui import QIcon, QFont, QAction

from .server_connection import ServerConnection
from .game_view import GameView
from .map_controls import MapControls
from .resource_viewer import ResourceViewer
from .log_viewer import LogViewer
from .team_panel import TeamPanel
from .player_panel import PlayerPanel


class ZappyMainWindow(QMainWindow):
    """Main window for the Zappy GUI application"""
    
    def __init__(self):
        super().__init__()
        
        # Set up the main window
        self.setWindowTitle("Zappy GUI")
        self.resize(1200, 800)
        
        # Initialize components
        self.server_connection : ServerConnection | None = None  # Initialize as None instead of creating an instance
        self.map_width = 0
        self.map_height = 0
        self.teams = []
        self.game_over = False
        self.winning_team = None
        
        # Create central layout with game view and tabs
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        
        # Main layout contains the header, game view and tabs in a vertical arrangement
        main_layout = QVBoxLayout(central_widget)
        main_layout.setContentsMargins(0, 0, 0, 0)
        
        # Add connection header
        self.create_connection_header(main_layout)
        
        # Add win condition banner (hidden by default)
        self.win_banner = QLabel()
        self.win_banner.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.win_banner.setFont(QFont("Arial", 16, QFont.Weight.Bold))
        self.win_banner.setStyleSheet("background-color: #4CAF50; color: white; padding: 10px;")
        self.win_banner.hide()
        main_layout.addWidget(self.win_banner)
        
        # Create a splitter to allow resizing between game view and tabs
        splitter = QSplitter(Qt.Orientation.Horizontal)
        main_layout.addWidget(splitter, 1)
        
        # Game view on the left
        self.game_view = GameView()
        splitter.addWidget(self.game_view)
        
        # Create tab widget on the right
        self.tabs = QTabWidget()
        splitter.addWidget(self.tabs)
        
        # Create all modules as tabs
        self.create_tabs()
        
        # Set initial splitter sizes (70% game view, 30% tabs)
        splitter.setSizes([700, 300])
        
        # Create status bar
        self.status_bar = QStatusBar()
        self.setStatusBar(self.status_bar)
        self.status_bar.showMessage("Disconnected")
        
        # Create menu bar
        self.create_menu_bar()
        
        # Timer for updates
        self.timer = QTimer(self)
        self.timer.timeout.connect(self.update_game_state)
        self.timer.start(100)  # Update every 100ms
        
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
        self.connect_button.clicked.connect(self.connect_from_header)
        server_layout.addWidget(self.connect_button)
        
        # Disconnect button
        self.disconnect_button = QPushButton("Disconnect")
        self.disconnect_button.clicked.connect(self.disconnect_from_server)
        self.disconnect_button.setEnabled(False)
        server_layout.addWidget(self.disconnect_button)
        
        header_layout.addWidget(server_section)
        
        # Status indicator
        self.connection_status = QLabel("Status: Disconnected")
        self.connection_status.setStyleSheet("font-weight: bold; color: #d32f2f;")
        header_layout.addWidget(self.connection_status)
        
        # Game info section (map size, time unit)
        game_info_section = QWidget()
        game_info_layout = QHBoxLayout(game_info_section)
        game_info_layout.setContentsMargins(0, 0, 0, 0)
        
        # Map size
        game_info_layout.addWidget(QLabel("Map:"))
        self.map_size_label = QLabel("0 x 0")
        game_info_layout.addWidget(self.map_size_label)
        
        # Time unit
        game_info_layout.addWidget(QLabel("Time Unit:"))
        self.time_unit_label = QLabel("UNDEFINED")
        game_info_layout.addWidget(self.time_unit_label)
        
        header_layout.addWidget(game_info_section)
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
        self.tabs.addTab(self.map_controls, "Map Controls")
        
        # Resource viewer tab
        self.resource_viewer = ResourceViewer()
        self.tabs.addTab(self.resource_viewer, "Resources")
        
        # Team panel tab
        self.team_panel = TeamPanel()
        self.tabs.addTab(self.team_panel, "Teams")
        
        # Player panel tab
        self.player_panel = PlayerPanel()
        self.tabs.addTab(self.player_panel, "Players")
        
        # Log viewer tab
        self.log_viewer = LogViewer()
        self.tabs.addTab(self.log_viewer, "Logs")
    
    def create_menu_bar(self):
        """Create the main window menu bar"""
        menu_bar = self.menuBar()
        
        # File menu
        file_menu = menu_bar.addMenu("&File")
        
        connect_action = QAction("&Connect to Server", self)
        connect_action.triggered.connect(self.show_connect_dialog)
        file_menu.addAction(connect_action)
        
        # Disconnect action
        disconnect_action = QAction("&Disconnect", self)
        disconnect_action.triggered.connect(self.disconnect_from_server)
        disconnect_action.setEnabled(False)  # Disabled until connected
        self.disconnect_action = disconnect_action
        file_menu.addAction(disconnect_action)
        
        file_menu.addSeparator()
        
        exit_action = QAction("&Exit", self)
        exit_action.triggered.connect(self.close)
        file_menu.addAction(exit_action)
        
        # View menu
        view_menu = menu_bar.addMenu("&View")
        
        # Reset camera action
        reset_camera_action = QAction("Reset Camera", self)
        reset_camera_action.triggered.connect(self.reset_camera)
        view_menu.addAction(reset_camera_action)
        
        # Help menu
        help_menu = menu_bar.addMenu("&Help")
        
        about_action = QAction("&About", self)
        about_action.triggered.connect(self.show_about_dialog)
        help_menu.addAction(about_action)
    
    def reset_camera(self):
        """Reset game view camera to default position"""
        if self.game_view:
            self.game_view.reset_camera()
    
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
            self.game_view.clear()
            self.game_view.scene.clear()

    
    def connect_to_server(self, host, port):
        """Connect to the server with the given host and port"""
        try:
            # Create new connection with host and port parameters
            self.server_connection = ServerConnection(host, port)
            self.server_connection.on_status_change = self.update_connection_status
            self.server_connection.connect()

            #connect emitter to server_connection
            self.map_controls.time_unit_changed.connect(self.server_connection.send_new_time_unit)
            
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
            self.disconnect_action.setEnabled(False)
            self.disconnect_button.setEnabled(False)
            self.connect_button.setEnabled(True)
            
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

        self.game_view.redraw_map()
    
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
            if cmd == "msz" and len(parts) >= 3:
                self.map_width = int(parts[1])
                self.map_height = int(parts[2])
                
                # Update game view with map size
                if self.game_view:
                    self.game_view.set_map_size(self.map_width, self.map_height)
                
                # Update map controls with map size
                if self.map_controls:
                    self.map_controls.set_map_size(self.map_width, self.map_height)
                
                # Update resource viewer with map size
                if self.resource_viewer:
                    self.resource_viewer.update_map_size(self.map_width, self.map_height)
                
                # Update map size in header
                self.update_map_size_in_header()

            if cmd == "sgt" and len(parts) >= 2:
                freq = int(parts[1])
                self.map_controls.set_time_unit(freq)
                self.update_time_unit_in_header(freq)
            
            # Team name
            elif cmd == "tna" and len(parts) >= 2:
                team_name = parts[1]
                if team_name not in self.teams:
                    self.teams.append(team_name)
                    # Update team panel
                    if self.team_panel:
                        self.team_panel.add_team(team_name)
            
            # Tile content
            elif cmd == "bct" and len(parts) >= 10:
                x = int(parts[1])
                y = int(parts[2])
                resources = [int(parts[i]) for i in range(3, 10)]
                
                # Update game view with tile resources
                if self.game_view:
                    self.game_view.update_tile(x, y, resources)
            
            # New player
            elif cmd == "pnw" and len(parts) >= 7:
                player_id = int(parts[1][1:])  # Remove the # prefix
                x = int(parts[2])
                y = int(parts[3])
                orientation = int(parts[4])
                level = int(parts[5])
                team_name = parts[6]
                
                # Update game view with player
                if self.game_view:
                    self.game_view.add_player(player_id, x, y, orientation, level, team_name)
                
                # Update player panel
                if self.player_panel:
                    self.player_panel.add_player(player_id, x, y, orientation, level, team_name)
                
                # Update team panel
                if self.team_panel:
                    self.team_panel.add_player_to_team(team_name, player_id)
                
                # Add player to map controls follow list
                if self.map_controls:
                    self.map_controls.add_player_to_follow(player_id, team_name)
                
                self.log_viewer.add_player_log(f"Player #{player_id} (team {team_name}) joined at level {level}")
            
            # Player position
            elif cmd == "ppo" and len(parts) >= 5:
                player_id = int(parts[1][1:])  # Remove the # prefix
                x = int(parts[2])
                y = int(parts[3])
                orientation = int(parts[4])
                
                # Update game view with player position
                if self.game_view:
                    self.game_view.update_player_position(player_id, x, y, orientation)
                
                # Update player panel
                if self.player_panel:
                    self.player_panel.update_player_position(player_id, x, y, orientation)
            
            # Player level
            elif cmd == "plv" and len(parts) >= 3:
                player_id = int(parts[1][1:])  # Remove the # prefix
                level = int(parts[2])
                
                # Update game view with player level
                if self.game_view:
                    self.game_view.update_player_level(player_id, level)
                
                # Update player panel
                if self.player_panel:
                    self.player_panel.update_player_level(player_id, level)
                
                # Update team panel
                if self.team_panel:
                    self.team_panel.update_player_level(player_id, level)
                
                self.log_viewer.add_player_log(f"Player #{player_id} is now level {level}")
            
            # Player inventory
            elif cmd == "pin" and len(parts) >= 10:
                player_id = int(parts[1][1:])  # Remove the # prefix
                x = int(parts[2])
                y = int(parts[3])
                resources = {
                    "food": int(parts[4]),
                    "linemate": int(parts[5]),
                    "deraumere": int(parts[6]),
                    "sibur": int(parts[7]),
                    "mendiane": int(parts[8]),
                    "phiras": int(parts[9]),
                    "thystame": int(parts[10])
                }
                
                if self.player_panel:
                    self.player_panel.update_player_inventory(player_id, resources)
            
            # Player death
            elif cmd == "pdi" and len(parts) >= 2:
                player_id = int(parts[1][1:])
                self.team_panel.remove_player_by_id(player_id)

                # Update game view to remove player
                if self.game_view:
                    self.game_view.remove_player(player_id)
                
                # Update player panel
                if self.player_panel:
                    self.player_panel.remove_player(player_id)
                
                # Update team panel
                if self.team_panel:
                    self.team_panel.remove_player_from_team(player_id)
                
                # Remove player from follow list in map controls
                if self.map_controls:
                    self.map_controls.remove_player_from_follow(player_id)
                
                self.log_viewer.add_player_log(f"Player #{player_id} died")
            
            # Start of incantation
            elif cmd == "pic" and len(parts) >= 4:
                x = int(parts[1])
                y = int(parts[2])
                level = int(parts[3])
                players = [int(parts[i][1:]) for i in range(4, len(parts))]
                
                # Update game view with incantation
                if self.game_view:
                    self.game_view.start_incantation(x, y, level, players)
                
                player_str = ", ".join([f"#{p}" for p in players])
                self.log_viewer.add_incantation_log(
                    f"Incantation started at ({x}, {y}) for level {level+1} by players {player_str}"
                )
            
            # End of incantation
            elif cmd == "pie" and len(parts) >= 4:
                x = int(parts[1])
                y = int(parts[2])
                result = int(parts[3])  # 0 = failure, 1 = success
                
                # Update game view with end of incantation
                if self.game_view:
                    self.game_view.end_incantation(x, y, result)
                
                status = "succeeded" if result == 1 else "failed"
                self.log_viewer.add_incantation_log(
                    f"Incantation at ({x}, {y}) {status}"
                )
                
        except Exception as e:
            print(f"Error processing response: {e} - Response: {response}")
    
    def update_map_size_in_header(self):
        """Update the map size display in the header"""
        if hasattr(self, 'map_size_label'):
            self.map_size_label.setText(f"{self.map_width} x {self.map_height}")
    
    def update_time_unit_in_header(self, time_unit):
        """Update the time unit display in the header"""
        self.time_unit_label.setText(f"{time_unit} ms")

    def show_about_dialog(self):
        """Show about dialog with information about the application"""
        QMessageBox.about(
            self, "About Zappy GUI",
            "Zappy GUI Client\n\n"
            "A graphical client for visualizing the Zappy game server."
        )