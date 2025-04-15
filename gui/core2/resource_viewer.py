"""
Resource viewer component for the Zappy GUI client
"""
from PyQt6.QtWidgets import (QWidget, QVBoxLayout, QHBoxLayout, QLabel, 
                           QGroupBox, QTableWidget, QTableWidgetItem,
                           QHeaderView, QPushButton, QComboBox,
                           QProgressBar, QTabWidget, QSplitter)
from PyQt6.QtCore import Qt, pyqtSignal
from PyQt6.QtGui import QColor, QPainter, QPixmap, QBrush, QPen, QIcon


class ResourceViewer(QWidget):
    """Panel for displaying resource information and statistics"""
    
    def __init__(self):
        super().__init__()
        
        # Resource data
        self.resource_names = ["Food", "Linemate", "Deraumere", "Sibur", 
                              "Mendiane", "Phiras", "Thystame"]
        
        # Resource densities as defined in the project specification
        self.resource_densities = [0.5, 0.3, 0.15, 0.1, 0.1, 0.08, 0.05]
        
        self.resource_colors = [
            QColor(240, 240, 160),  # Food (yellow)
            QColor(180, 180, 180),  # Linemate (gray)
            QColor(139, 69, 19),    # Deraumere (brown)
            QColor(0, 128, 128),    # Sibur (teal)
            QColor(255, 105, 180),  # Mendiane (pink)
            QColor(50, 205, 50),    # Phiras (green)
            QColor(138, 43, 226)    # Thystame (purple)
        ]
        
        # Resource counts
        self.total_resources = [0] * 7  # Total resources on map
        self.player_resources = [0] * 7  # Total resources held by players
        self.map_width = 0
        self.map_height = 0
        
        self.init_ui()
    
    def init_ui(self):
        """Initialize the UI components"""
        layout = QVBoxLayout(self)
        
        # Main tab widget
        tabs = QTabWidget()
        
        # Overview tab
        overview_tab = QWidget()
        overview_layout = QVBoxLayout(overview_tab)
        
        # Resource counters
        counter_group = QGroupBox("Total Resources")
        counter_layout = QVBoxLayout(counter_group)
        
        self.counter_table = QTableWidget(7, 3)  # 7 resources, 3 columns (name, map, players)
        self.counter_table.setHorizontalHeaderLabels(["Resource", "On Map", "With Players"])
        self.counter_table.horizontalHeader().setSectionResizeMode(0, QHeaderView.ResizeMode.Stretch)
        self.counter_table.horizontalHeader().setSectionResizeMode(1, QHeaderView.ResizeMode.Stretch)
        self.counter_table.horizontalHeader().setSectionResizeMode(2, QHeaderView.ResizeMode.Stretch)
        self.counter_table.verticalHeader().setVisible(False)
        
        # Initialize resource names in table
        for i, resource in enumerate(self.resource_names):
            # Create color icon
            pixmap = QPixmap(16, 16)
            pixmap.fill(self.resource_colors[i])
            
            # Create name item with icon - convert QPixmap to QIcon
            name_item = QTableWidgetItem(resource)
            name_item.setIcon(QIcon(pixmap))
            
            # Create count items
            map_item = QTableWidgetItem("0")
            map_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            
            player_item = QTableWidgetItem("0")
            player_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            
            # Set items in table
            self.counter_table.setItem(i, 0, name_item)
            self.counter_table.setItem(i, 1, map_item)
            self.counter_table.setItem(i, 2, player_item)
        
        counter_layout.addWidget(self.counter_table)
        overview_layout.addWidget(counter_group)
        
        # Resource distribution visualization
        dist_group = QGroupBox("Resource Distribution")
        dist_layout = QVBoxLayout(dist_group)
        
        self.distribution_widget = ResourceDistributionWidget(self.resource_names, self.resource_colors)
        dist_layout.addWidget(self.distribution_widget)
        
        overview_layout.addWidget(dist_group)
        
        # Add overview tab
        tabs.addTab(overview_tab, "Overview")
        
        # Resource density map tab
        density_tab = QWidget()
        density_layout = QVBoxLayout(density_tab)
        
        # Resource selection
        selection_layout = QHBoxLayout()
        selection_layout.addWidget(QLabel("Select Resource:"))
        
        self.resource_combo = QComboBox()
        for resource in self.resource_names:
            self.resource_combo.addItem(resource)
        selection_layout.addWidget(self.resource_combo)
        
        density_layout.addLayout(selection_layout)
        
        # Density info table
        self.density_table = QTableWidget(7, 3)
        self.density_table.setHorizontalHeaderLabels(["Resource", "Density", "Expected Count"])
        self.density_table.horizontalHeader().setSectionResizeMode(0, QHeaderView.ResizeMode.Stretch)
        self.density_table.horizontalHeader().setSectionResizeMode(1, QHeaderView.ResizeMode.Stretch)
        self.density_table.horizontalHeader().setSectionResizeMode(2, QHeaderView.ResizeMode.Stretch)
        self.density_table.verticalHeader().setVisible(False)
        
        # Fill density table
        for i, resource in enumerate(self.resource_names):
            # Resource name with color
            pixmap = QPixmap(16, 16)
            pixmap.fill(self.resource_colors[i])
            name_item = QTableWidgetItem(resource)
            name_item.setIcon(QIcon(pixmap))
            
            # Density value
            density_item = QTableWidgetItem(f"{self.resource_densities[i]:.2f}")
            density_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            
            # Expected count (calculated when map size is known)
            count_item = QTableWidgetItem("N/A")
            count_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            
            # Set items
            self.density_table.setItem(i, 0, name_item)
            self.density_table.setItem(i, 1, density_item)
            self.density_table.setItem(i, 2, count_item)
        
        density_layout.addWidget(self.density_table)
        
        # Density map (placeholder)
        self.density_map = QLabel("Resource density map will be shown here")
        self.density_map.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.density_map.setStyleSheet("background-color: #333; color: white; padding: 20px;")
        density_layout.addWidget(self.density_map)
        
        tabs.addTab(density_tab, "Density Info")
        
        # Incantation requirements tab
        incantation_tab = QWidget()
        incantation_layout = QVBoxLayout(incantation_tab)
        
        incantation_title = QLabel("Elevation Ritual Requirements:")
        incantation_title.setStyleSheet("font-weight: bold; font-size: 14px;")
        incantation_layout.addWidget(incantation_title)
        
        self.incantation_table = QTableWidget(7, 8)  # 7 levels, 8 columns (level + 7 resources)
        headers = ["Level"] + self.resource_names
        self.incantation_table.setHorizontalHeaderLabels(headers)
        
        for i in range(8):
            self.incantation_table.horizontalHeader().setSectionResizeMode(i, QHeaderView.ResizeMode.Stretch)
        
        # Fill with incantation requirements
        requirements = [
            [1, 0, 0, 0, 0, 0, 0],  # Level 1 has no requirements
            [1, 1, 0, 0, 0, 0, 0],  # Level 2
            [2, 1, 1, 0, 0, 0, 0],  # Level 3
            [1, 1, 2, 0, 2, 0, 0],  # Level 4
            [1, 1, 1, 3, 0, 1, 0],  # Level 5
            [1, 1, 2, 1, 3, 0, 0],  # Level 6
            [2, 2, 2, 2, 2, 2, 1]   # Level 7
        ]
        
        player_counts = [1, 1, 2, 4, 4, 6, 6]  # Number of players needed for each level
        
        for level in range(7):
            # Level number (display as level + 1 -> level + 2)
            level_item = QTableWidgetItem(f"{level + 1} → {level + 2}")
            level_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            self.incantation_table.setItem(level, 0, level_item)
            
            # Add player count in tooltip
            level_item.setToolTip(f"Requires {player_counts[level]} players")
            level_item.setBackground(QBrush(QColor(220, 220, 220)))
            
            # Resource requirements
            for res in range(7):
                req_item = QTableWidgetItem(str(requirements[level][res]))
                req_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
                
                # Highlight if resource is needed
                if requirements[level][res] > 0:
                    req_item.setBackground(QBrush(self.resource_colors[res].lighter(150)))
                
                self.incantation_table.setItem(level, res + 1, req_item)
        
        incantation_layout.addWidget(self.incantation_table)
        
        # Add note about players
        player_note = QLabel("Note: Each cell shows the required number of resources for elevation.")
        player_note.setStyleSheet("font-style: italic;")
        incantation_layout.addWidget(player_note)
        
        # Add player requirements table
        player_group = QGroupBox("Required Players for Elevation")
        player_layout = QVBoxLayout(player_group)
        
        player_table = QTableWidget(1, 7)
        player_table.setHorizontalHeaderLabels([f"{i+1}→{i+2}" for i in range(7)])
        for i in range(7):
            player_table.horizontalHeader().setSectionResizeMode(i, QHeaderView.ResizeMode.Stretch)
        
        # Fill with player counts
        for i, count in enumerate(player_counts):
            item = QTableWidgetItem(str(count))
            item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            player_table.setItem(0, i, item)
        
        player_layout.addWidget(player_table)
        incantation_layout.addWidget(player_group)
        
        # Win condition information
        win_group = QGroupBox("Win Condition")
        win_layout = QVBoxLayout(win_group)
        win_layout.addWidget(QLabel("The first team to have 6 players reach level 8 wins the game."))
        incantation_layout.addWidget(win_group)
        
        tabs.addTab(incantation_tab, "Incantations")
        
        layout.addWidget(tabs)
    
    def update_resource_counts(self, map_resources, player_resources):
        """Update the resource counts displays"""
        self.total_resources = map_resources
        self.player_resources = player_resources
        
        # Update table
        for i in range(7):
            self.counter_table.item(i, 1).setText(str(map_resources[i]))
            self.counter_table.item(i, 2).setText(str(player_resources[i]))
        
        # Update distribution widget
        total = [map_resources[i] + player_resources[i] for i in range(7)]
        self.distribution_widget.update_data(total)
    
    def set_map_size(self, width, height):
        """Set map dimensions and update expected resource counts"""
        self.map_width = width
        self.map_height = height
        
        if width > 0 and height > 0:
            # Update expected resource counts in density table
            total_tiles = width * height
            for i in range(7):
                expected_count = total_tiles * self.resource_densities[i]
                self.density_table.item(i, 2).setText(f"{int(expected_count)}")
    
    def update_map_size(self, width, height):
        """Alias for set_map_size to maintain compatibility"""
        self.set_map_size(width, height)
    
    def add_tile_resources(self, resources):
        """Add resources from a tile to the total count"""
        # Add each resource type to our counter
        for i in range(7):
            self.total_resources[i] += resources[i]
        
        # Update the UI
        self.update_resource_counts(self.total_resources, self.player_resources)
    
    def clear_resource_counts(self):
        """Clear all resource counts"""
        self.total_resources = [0] * 7
        self.player_resources = [0] * 7
        self.update_resource_counts(self.total_resources, self.player_resources)


class ResourceDistributionWidget(QWidget):
    """Widget for visualizing resource distribution as a pie chart"""
    
    def __init__(self, resource_names, resource_colors):
        super().__init__()
        self.resource_names = resource_names
        self.resource_colors = resource_colors
        self.data = [1] * 7  # Initialize with equal values
        self.setMinimumHeight(200)
    
    def update_data(self, data):
        """Update the chart data"""
        self.data = data
        self.update()
    
    def paintEvent(self, event):
        """Paint the pie chart"""
        painter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)
        
        # Calculate total for percentages
        total = sum(self.data)
        if total == 0:
            # If no resources, just draw empty circle
            painter.setPen(QPen(Qt.GlobalColor.gray))
            painter.drawEllipse(self.rect().adjusted(10, 10, -10, -10))
            return
        
        # Calculate chart geometry
        center_x = self.width() / 2
        center_y = self.height() / 2
        radius = min(center_x, center_y) - 10
        
        # Draw slices
        start_angle = 0
        for i, value in enumerate(self.data):
            if value == 0:
                continue
                
            span_angle = 360 * value / total
            
            # Draw slice
            painter.setPen(QPen(self.resource_colors[i].darker(120)))
            painter.setBrush(QBrush(self.resource_colors[i]))
            
            # Convert angles to 16ths of a degree (required by drawPie)
            painter.drawPie(
                int(center_x - radius), 
                int(center_y - radius),
                int(radius * 2), 
                int(radius * 2), 
                int(start_angle * 16), 
                int(span_angle * 16)
            )
            
            # Draw label if slice is big enough
            if span_angle > 15:
                # Calculate position for text
                angle_rad = (start_angle + span_angle / 2) * 3.14159 / 180
                text_x = center_x + (radius * 0.7) * -1 * (angle_rad - 3.14159) if angle_rad > 3.14159 else center_x + (radius * 0.7) * angle_rad
                text_y = center_y + (radius * 0.7) * (angle_rad + 1.5707) if angle_rad > 4.71239 or angle_rad < 1.5707 else center_y + (radius * 0.7) * (angle_rad - 1.5707)
                
                # Draw text
                painter.setPen(QPen(Qt.GlobalColor.white))
                resource_name = self.resource_names[i] if i < len(self.resource_names) else f"Resource {i}"
                painter.drawText(int(text_x), int(text_y), f"{resource_name}")
            
            start_angle += span_angle