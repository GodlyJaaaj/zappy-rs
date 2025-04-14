"""
Main window for the Zappy GUI
"""
import sys
from PyQt6.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, QLabel, QPushButton,
    QDockWidget, QSplitter, QMessageBox, QInputDialog, QLineEdit, QStatusBar
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


class MainWindow(QMainWindow):
    """Main window for the Zappy GUI"""
    
    def __init__(self):
        super().__init__()
        
        # Instance variables
        self.server_connection = None
        self.game_view = None
        self.map_controls = None
        self.resource_viewer = None
        self.log_viewer = None
        self.team_panel = None
        self.player_panel = None
        self.timer = None
        self.map_width = 0
        self.map_height = 0
        self.time_unit = 100  # Default time unit
        self.teams = []
        self.last_update = 0
        self.game_over = False
        self.winning_team = None
        
        # Set up UI
        self.setWindowTitle("Zappy GUI")
        self.setGeometry(100, 100, 1200, 800)
        self.setup_ui()
        
        # Show connect dialog by default
        self.show_connect_dialog()
    
    def setup_ui(self):
        """Set up the main window UI components"""
        # Create central widget with game view
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        
        main_layout = QVBoxLayout(central_widget)
        
        # Add win condition banner (hidden by default)
        self.win_banner = QLabel()
        self.win_banner.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.win_banner.setFont(QFont("Arial", 16, QFont.Weight.Bold))
        self.win_banner.setStyleSheet("background-color: #4CAF50; color: white; padding: 10px;")
        self.win_banner.hide()
        main_layout.addWidget(self.win_banner)
        
        # Game view
        self.game_view = GameView()
        
        # Map controls
        self.map_controls = MapControls()
        self.map_controls.on_speed_changed.connect(self.handle_speed_change)
        
        # Create a splitter for resizable panels
        splitter = QSplitter(Qt.Orientation.Horizontal)
        main_layout.addWidget(splitter)
        
        # Game view and controls in the center
        center_widget = QWidget()
        center_layout = QVBoxLayout(center_widget)
        center_layout.addWidget(self.game_view)
        center_layout.addWidget(self.map_controls)
        splitter.addWidget(center_widget)
        
        # Right panel for resources, game info, etc.
        right_panel = QWidget()
        right_layout = QVBoxLayout(right_panel)
        
        # Game info section
        info_section = QWidget()
        info_layout = QVBoxLayout(info_section)
        
        # Current time unit
        time_unit_layout = QHBoxLayout()
        time_unit_layout.addWidget(QLabel("Time Unit (f):"))
        self.time_unit_label = QLabel("100")
        self.time_unit_label.setFont(QFont("Arial", 10, QFont.Weight.Bold))
        time_unit_layout.addWidget(self.time_unit_label)
        time_unit_layout.addStretch()
        info_layout.addLayout(time_unit_layout)
        
        # Map size
        map_size_layout = QHBoxLayout()
        map_size_layout.addWidget(QLabel("Map Size:"))
        self.map_size_label = QLabel("0 x 0")
        self.map_size_label.setFont(QFont("Arial", 10, QFont.Weight.Bold))
        map_size_layout.addWidget(self.map_size_label)
        map_size_layout.addStretch()
        info_layout.addLayout(map_size_layout)
        
        # Win condition description
        win_cond_label = QLabel("Win Condition: 6 players at level 8")
        win_cond_label.setStyleSheet("font-style: italic;")
        info_layout.addWidget(win_cond_label)
        
        right_layout.addWidget(info_section)
        
        # Resource viewer
        self.resource_viewer = ResourceViewer()
        right_layout.addWidget(self.resource_viewer)
        
        splitter.addWidget(right_panel)
        
        # Set initial splitter sizes
        splitter.setSizes([700, 500])
        
        # Create dockable panels
        self.create_dock_widgets()
        
        # Set up status bar
        self.status_bar = QStatusBar()
        self.setStatusBar(self.status_bar)
        self.status_bar.showMessage("Not connected")
        
        # Set up menu bar
        self.create_menu_bar()
        
        # Timer for updates
        self.timer = QTimer(self)
        self.timer.timeout.connect(self.update_game_state)
        self.timer.start(100)  # Update every 100ms
    
    def create_dock_widgets(self):
        """Create dockable panels"""
        # Team panel
        self.team_panel = TeamPanel()
        team_dock = QDockWidget("Teams", self)
        team_dock.setWidget(self.team_panel)
        team_dock.setFeatures(QDockWidget.DockWidgetFeature.DockWidgetMovable | 
                             QDockWidget.DockWidgetFeature.DockWidgetFloatable)
        self.addDockWidget(Qt.DockWidgetArea.LeftDockWidgetArea, team_dock)
        
        # Player panel
        self.player_panel = PlayerPanel()
        player_dock = QDockWidget("Players", self)
        player_dock.setWidget(self.player_panel)
        player_dock.setFeatures(QDockWidget.DockWidgetFeature.DockWidgetMovable | 
                               QDockWidget.DockWidgetFeature.DockWidgetFloatable)
        self.addDockWidget(Qt.DockWidgetArea.LeftDockWidgetArea, player_dock)
        
        # Log viewer
        self.log_viewer = LogViewer()
        log_dock = QDockWidget("Log", self)
        log_dock.setWidget(self.log_viewer)
        log_dock.setFeatures(QDockWidget.DockWidgetFeature.DockWidgetMovable | 
                            QDockWidget.DockWidgetFeature.DockWidgetFloatable)
        self.addDockWidget(Qt.DockWidgetArea.BottomDockWidgetArea, log_dock)
    
    def create_menu_bar(self):
        """Create menu bar"""
        menu_bar = self.menuBar()
        
        # File menu
        file_menu = menu_bar.addMenu("&File")
        
        # Connect action
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
        
        # Exit action
        exit_action = QAction("E&xit", self)
        exit_action.triggered.connect(self.close)
        file_menu.addAction(exit_action)
        
        # View menu
        view_menu = menu_bar.addMenu("&View")
        
        # Reset camera action
        reset_camera_action = QAction("Reset Camera", self)
        reset_camera_action.triggered.connect(self.reset_camera)
        view_menu.addAction(reset_camera_action)
    
    def show_connect_dialog(self):
        """Show dialog to connect to server"""
        host, ok = QInputDialog.getText(self, "Connect to Server", "Host:", 
                                      QLineEdit.EchoMode.Normal, "localhost")
        if not ok:
            return
        
        port, ok = QInputDialog.getText(self, "Connect to Server", "Port:", 
                                      QLineEdit.EchoMode.Normal, "4242")
        if not ok:
            return
        
        try:
            port = int(port)
        except ValueError:
            QMessageBox.critical(self, "Error", "Invalid port number")
            return
        
        self.connect_to_server(host, port)
    
    def connect_to_server(self, host, port):
        """Connect to Zappy server"""
        # Disconnect if already connected
        if self.server_connection and self.server_connection.connected:
            self.disconnect_from_server()
        
        # Create new connection
        self.server_connection = ServerConnection(host, port)
        
        # Attempt connection
        if self.server_connection.connect():
            self.status_bar.showMessage(f"Connected to {host}:{port}")
            self.disconnect_action.setEnabled(True)
            self.log_viewer.add_log_entry("Connected to server", "server")
        else:
            QMessageBox.critical(self, "Connection Error", f"Could not connect to {host}:{port}")
            self.server_connection = None
    
    def disconnect_from_server(self):
        """Disconnect from server"""
        if self.server_connection:
            self.server_connection.disconnect()
            self.server_connection = None
            self.status_bar.showMessage("Disconnected")
            self.disconnect_action.setEnabled(False)
            self.log_viewer.add_log_entry("Disconnected from server", "server")
    
    def reset_camera(self):
        """Reset game view camera to default position"""
        if self.game_view:
            self.game_view.reset_camera()
    
    def handle_speed_change(self, speed):
        """Handle speed change from map controls"""
        pass  # Future implementation
    
    def update_game_state(self):
        """Update game state with new data from server"""
        if not self.server_connection or not self.server_connection.connected:
            return
        
        # Get responses from server
        responses = self.server_connection.get_responses()
        
        # Process responses
        for response in responses:
            self.process_server_response(response)
    
    def process_server_response(self, response):
        """Process a response from the server"""
        # Log the response
        self.log_viewer.add_log_entry(response, "server")
        
        parts = response.split()
        if not parts:
            return
        
        cmd = parts[0]
        
        try:
            # Map size
            if cmd == "msz" and len(parts) >= 3:
                self.map_width = int(parts[1])
                self.map_height = int(parts[2])
                self.map_size_label.setText(f"{self.map_width} x {self.map_height}")
                
                # Update game view with map size
                if self.game_view:
                    self.game_view.set_map_size(self.map_width, self.map_height)
                
                # Update resource viewer with map size
                if self.resource_viewer:
                    self.resource_viewer.update_map_size(self.map_width, self.map_height)
            
            # Time unit
            elif cmd == "sgt" and len(parts) >= 2:
                self.time_unit = int(parts[1])
                self.time_unit_label.setText(str(self.time_unit))
            
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
                    self.player_panel.add_player(player_id, team_name, level)
                
                # Update team panel
                if self.team_panel:
                    self.team_panel.add_player_to_team(team_name, player_id, level)
            
            # Player position
            elif cmd == "ppo" and len(parts) >= 5:
                player_id = int(parts[1][1:])  # Remove the # prefix
                x = int(parts[2])
                y = int(parts[3])
                orientation = int(parts[4])
                
                # Update game view with player position
                if self.game_view:
                    self.game_view.update_player_position(player_id, x, y, orientation)
            
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
            
            # Player death
            elif cmd == "pdi" and len(parts) >= 2:
                player_id = int(parts[1][1:])  # Remove the # prefix
                
                # Update game view to remove player
                if self.game_view:
                    self.game_view.remove_player(player_id)
                
                # Update player panel
                if self.player_panel:
                    self.player_panel.remove_player(player_id)
                
                # Update team panel
                if self.team_panel:
                    self.team_panel.remove_player(player_id)
            
            # Start of incantation
            elif cmd == "pic" and len(parts) >= 4:
                x = int(parts[1])
                y = int(parts[2])
                level = int(parts[3])
                players = [int(parts[i][1:]) for i in range(4, len(parts))]
                
                # Update game view with incantation
                if self.game_view:
                    self.game_view.start_incantation(x, y, level, players)
            
            # End of incantation
            elif cmd == "pie" and len(parts) >= 4:
                x = int(parts[1])
                y = int(parts[2])
                result = int(parts[3])  # 0 = failure, 1 = success
                
                # Update game view with end of incantation
                if self.game_view:
                    self.game_view.end_incantation(x, y, result)
            
            # End of game
            elif cmd == "seg" and len(parts) >= 2:
                winning_team = parts[1]
                self.handle_game_over(winning_team)
            
            # Win condition (custom event from ServerConnection)
            elif cmd == "win_condition" and len(parts) >= 2:
                winning_team = parts[1]
                self.handle_game_over(winning_team)
                
        except Exception as e:
            print(f"Error processing response: {e} - Response: {response}")
    
    def handle_game_over(self, winning_team):
        """Handle game over event"""
        if self.game_over:
            return
            
        self.game_over = True
        self.winning_team = winning_team
        
        # Show win banner
        self.win_banner.setText(f"Team {winning_team} WINS! 🏆 (6 players reached level 8)")
        self.win_banner.show()
        
        # Log to console
        self.log_viewer.add_log_entry(f"GAME OVER: Team {winning_team} WINS!", "system")
        
        # Show message box
        QMessageBox.information(self, "Game Over", 
                               f"Team {winning_team} has won the game!\n\n"
                               f"Six players from team {winning_team} have reached level 8.")