"""
Player panel component for the Zappy GUI client
"""
from PyQt6.QtWidgets import (QWidget, QVBoxLayout, QHBoxLayout, QLabel,
                            QTableWidget, QTableWidgetItem, QComboBox,
                            QPushButton, QHeaderView)
from PyQt6.QtCore import Qt, pyqtSignal


class PlayerPanel(QWidget):
    """Panel for displaying player information"""
    
    # Signals
    player_selected = pyqtSignal(int)  # Player ID
    
    def __init__(self):
        super().__init__()
        
        self.players = {}  # player_id -> player data
        self.resource_names = ["food", "linemate", "deraumere", "sibur", 
                              "mendiane", "phiras", "thystame"]
        
        self.init_ui()
    
    def init_ui(self):
        """Initialize the UI components"""
        layout = QVBoxLayout(self)
        
        # Player selection
        select_layout = QHBoxLayout()
        select_layout.addWidget(QLabel("Select Player:"))
        
        self.player_combo = QComboBox()
        self.player_combo.currentIndexChanged.connect(self.on_player_selected)
        select_layout.addWidget(self.player_combo)
        
        layout.addLayout(select_layout)
        
        # Player details
        self.details_layout = QVBoxLayout()
        self.player_info_label = QLabel("No player selected")
        self.details_layout.addWidget(self.player_info_label)
        
        # Inventory table
        self.inventory_table = QTableWidget(7, 2)  # 7 resources, 2 columns (name, count)
        self.inventory_table.setHorizontalHeaderLabels(["Resource", "Count"])
        self.inventory_table.horizontalHeader().setSectionResizeMode(0, QHeaderView.ResizeMode.Stretch)
        self.inventory_table.horizontalHeader().setSectionResizeMode(1, QHeaderView.ResizeMode.Stretch)
        self.inventory_table.verticalHeader().setVisible(False)
        
        # Initialize resource names in table
        for i, resource in enumerate(self.resource_names):
            self.inventory_table.setItem(i, 0, QTableWidgetItem(resource.capitalize()))
            self.inventory_table.setItem(i, 1, QTableWidgetItem("0"))
        
        self.details_layout.addWidget(self.inventory_table)
        layout.addLayout(self.details_layout)
        
        # Add follow button
        self.follow_button = QPushButton("Follow this player")
        self.follow_button.clicked.connect(self.on_follow_clicked)
        layout.addWidget(self.follow_button)
        
        # Fill space at bottom
        layout.addStretch(1)
    
    def add_player(self, player_id, x, y, orientation, level, team_name):
        """Add a new player to the panel"""
        if player_id in self.players:
            return
            
        self.players[player_id] = {
            'id': player_id,
            'x': x,
            'y': y,
            'orientation': orientation,
            'level': level,
            'team': team_name,
            'inventory': {}
        }
        
        # Add to combo box
        self.player_combo.addItem(f"Player #{player_id} ({team_name})", player_id)
    
    def update_player_position(self, player_id, x, y, orientation):
        """Update a player's position information"""
        if player_id not in self.players:
            return
            
        self.players[player_id]['x'] = x
        self.players[player_id]['y'] = y
        self.players[player_id]['orientation'] = orientation
        
        # Update display if this player is selected
        selected_id = self.get_selected_player_id()
        if selected_id == player_id:
            self.update_player_display(player_id)
    
    def update_player_level(self, player_id, level):
        """Update a player's level"""
        if player_id not in self.players:
            return
            
        self.players[player_id]['level'] = level
        
        # Update combo box text
        team = self.players[player_id]['team']
        index = self.find_player_combo_index(player_id)
        if index >= 0:
            self.player_combo.setItemText(index, f"Player #{player_id} L{level} ({team})")
        
        # Update display if this player is selected
        selected_id = self.get_selected_player_id()
        if selected_id == player_id:
            self.update_player_display(player_id)
    
    def update_player_inventory(self, player_id, resources):
        """Update a player's inventory"""
        if player_id not in self.players:
            return
            
        # Convert resource array to a dict
        inventory = {}
        for i, count in enumerate(resources):
            if i < len(self.resource_names):
                inventory[self.resource_names[i]] = count
        
        self.players[player_id]['inventory'] = inventory
        
        # Update display if this player is selected
        selected_id = self.get_selected_player_id()
        if selected_id == player_id:
            self.update_player_display(player_id)
    
    def remove_player(self, player_id):
        """Remove a player from the panel"""
        if player_id not in self.players:
            return
            
        # Remove from combo box
        index = self.find_player_combo_index(player_id)
        if index >= 0:
            self.player_combo.removeItem(index)
            
        # Remove from players dict
        del self.players[player_id]
        
        # Clear display if this player was selected
        selected_id = self.get_selected_player_id()
        if selected_id is None or selected_id == player_id:
            self.player_info_label.setText("No player selected")
            
            # Reset inventory table
            for i in range(len(self.resource_names)):
                self.inventory_table.item(i, 1).setText("0")
    
    def get_selected_player_id(self):
        """Get the currently selected player ID"""
        if self.player_combo.count() == 0:
            return None
            
        return self.player_combo.currentData()
    
    def find_player_combo_index(self, player_id):
        """Find the index of a player in the combo box"""
        for i in range(self.player_combo.count()):
            if self.player_combo.itemData(i) == player_id:
                return i
        return -1
    
    def update_player_display(self, player_id):
        """Update the display for a specific player"""
        if player_id not in self.players:
            return
            
        player = self.players[player_id]
        
        # Update info label
        orientation_names = {1: "North", 2: "East", 3: "South", 4: "West"}
        orientation = orientation_names.get(player['orientation'], str(player['orientation']))
        
        info_text = (f"Player #{player_id}\n"
                    f"Team: {player['team']}\n"
                    f"Level: {player['level']}\n"
                    f"Position: ({player['x']}, {player['y']})\n"
                    f"Facing: {orientation}")
        self.player_info_label.setText(info_text)
        
        # Update inventory table
        inventory = player.get('inventory', {})
        for i, resource in enumerate(self.resource_names):
            count = inventory.get(resource, 0)
            self.inventory_table.item(i, 1).setText(str(count))
    
    def on_player_selected(self, index):
        """Handle player selection from combo box"""
        if index < 0:
            self.player_info_label.setText("No player selected")
            return
            
        player_id = self.player_combo.itemData(index)
        self.update_player_display(player_id)
        self.player_selected.emit(player_id)
    
    def on_follow_clicked(self):
        """Handle follow button click"""
        player_id = self.get_selected_player_id()
        if player_id is not None:
            self.player_selected.emit(player_id)